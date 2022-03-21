#include "environment_data.h"

#include <memory>

#include "logbypass/log.h"
#include "process_data.h"
#include "util-inl.h"
#include "xpf_node.h"
#include "xpf_v8.h"

namespace xprofiler {
using v8::Boolean;
using v8::Context;
using v8::Isolate;
using v8::Local;
using v8::Number;
using v8::Object;

// static
EnvironmentData* EnvironmentData::GetCurrent(v8::Isolate* isolate) {
  EnvironmentRegistry* registry = ProcessData::Get()->environment_registry();
  EnvironmentRegistry::NoExitScope scope(registry);
  return registry->Get(isolate);
}

// static
EnvironmentData* EnvironmentData::GetCurrent(
    const Nan::FunctionCallbackInfo<v8::Value>& info) {
  return EnvironmentData::GetCurrent(info.GetIsolate());
}

// static
void EnvironmentData::Create(v8::Isolate* isolate) {
  EnvironmentRegistry* registry = ProcessData::Get()->environment_registry();
  EnvironmentRegistry::NoExitScope no_exit(registry);

  HandleScope scope(isolate);
  uv_loop_t* loop = node::GetCurrentEventLoop(isolate);
  CHECK_NOT_NULL(loop);

  registry->Register(isolate, std::unique_ptr<EnvironmentData>(
                                  new EnvironmentData(isolate, loop)));
  xprofiler::AtExit(isolate, AtExit, isolate);
}

EnvironmentData::EnvironmentData(v8::Isolate* isolate, uv_loop_t* loop)
    : isolate_(isolate), loop_(loop) {
  CHECK_EQ(0, uv_async_init(loop, &interrupt_async_, InterruptIdleCallback));
  uv_unref(reinterpret_cast<uv_handle_t*>(&interrupt_async_));
  CHECK_EQ(0, uv_async_init(loop, &statistics_async_, CollectStatistics));
  uv_unref(reinterpret_cast<uv_handle_t*>(&statistics_async_));
}

// static
void EnvironmentData::AtExit(void* arg) {
  Isolate* isolate = reinterpret_cast<Isolate*>(arg);
  EnvironmentRegistry* registry = ProcessData::Get()->environment_registry();
  EnvironmentRegistry::NoExitScope scope(registry);
  std::unique_ptr<EnvironmentData> env_data = registry->Unregister(isolate);
  uv_close(reinterpret_cast<uv_handle_t*>(&env_data->interrupt_async_),
           nullptr);
  uv_close(reinterpret_cast<uv_handle_t*>(&env_data->statistics_async_),
           CloseCallback);
  env_data.release();
}

// static
void EnvironmentData::CloseCallback(uv_handle_t* handle) {
  EnvironmentData* env_data =
      ContainerOf(&EnvironmentData::statistics_async_,
                  reinterpret_cast<uv_async_t*>(handle));
  delete env_data;
}

void EnvironmentData::SendCollectStatistics() {
  uv_async_send(&statistics_async_);
}

void EnvironmentData::RequestInterrupt(InterruptCallback interrupt) {
  {
    Mutex::ScopedLock lock(interrupt_mutex_);
    interrupt_requests_.push_back(interrupt);
  }
  isolate_->RequestInterrupt(InterruptBusyCallback, this);
  uv_async_send(&interrupt_async_);
}

// static
void EnvironmentData::InterruptBusyCallback(v8::Isolate* isolate, void* data) {
  EnvironmentData* env_data = static_cast<EnvironmentData*>(data);
  std::list<InterruptCallback> requests;
  {
    Mutex::ScopedLock lock(env_data->interrupt_mutex_);
    requests.swap(env_data->interrupt_requests_);
  }

  for (auto it : requests) {
    it(env_data, InterruptKind::kBusy);
  }
}

// static
void EnvironmentData::InterruptIdleCallback(uv_async_t* handle) {
  EnvironmentData* env_data =
      ContainerOf(&EnvironmentData::interrupt_async_, handle);
  std::list<InterruptCallback> requests;
  {
    Mutex::ScopedLock lock(env_data->interrupt_mutex_);
    requests.swap(env_data->interrupt_requests_);
  }

  for (auto it : requests) {
    it(env_data, InterruptKind::kIdle);
  }
}

// static
void EnvironmentData::CollectStatistics(uv_async_t* handle) {
  EnvironmentData* env_data =
      ContainerOf(&EnvironmentData::statistics_async_, handle);
  CollectMemoryStatistics(env_data);
  CollectLibuvHandleStatistics(env_data);
}

// javascript accessible
// static
void EnvironmentData::JsSetupEnvironmentData(const Nan::FunctionCallbackInfo<v8::Value>& info) {
  Isolate* isolate = info.GetIsolate();
  EnvironmentData* env_data = EnvironmentData::GetCurrent(isolate);
  HandleScope scope(isolate);
  Local<Context> context = isolate->GetCurrentContext();

  Local<Object> data = info[0].As<Object>();
  Local<Number> thread_id =
      data->Get(context, OneByteString(isolate, "threadId"))
          .ToLocalChecked()
          .As<Number>();
  Local<Boolean> is_main_thread =
      data->Get(context, OneByteString(isolate, "isMainThread"))
          .ToLocalChecked()
          .As<Boolean>();

  env_data->thread_id_ = thread_id->Value();
  env_data->is_main_thread_ = is_main_thread->Value();
}

}  // namespace xprofiler
