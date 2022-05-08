#include "log.h"

#include <memory>

#include "configure-inl.h"
#include "cpu.h"
#include "environment_data.h"
#include "environment_registry.h"
#include "gc.h"
#include "heap.h"
#include "http.h"
#include "library/utils.h"
#include "libuv.h"
#include "logger.h"
#include "process_data.h"
#include "uv.h"

namespace xprofiler {
using Nan::False;
using Nan::FunctionCallbackInfo;
using Nan::ThrowTypeError;
using Nan::True;
using v8::Value;

void LogByPass::ThreadEntry(uv_loop_t* loop) {
  CHECK_EQ(0, uv_timer_init(loop, &cpu_interval_));
  CHECK_EQ(0, uv_timer_init(loop, &log_interval_));

  CHECK_EQ(0, uv_timer_start(&cpu_interval_, OnCpuInterval, 1000, 1000));
  CHECK_EQ(0, uv_timer_start(&log_interval_, OnLogInterval,
                             GetLogInterval() * 1000, false));
}

void LogByPass::ThreadAtExit() {
  uv_close(reinterpret_cast<uv_handle_t*>(&cpu_interval_), nullptr);
  uv_close(reinterpret_cast<uv_handle_t*>(&log_interval_), nullptr);
}

// static
void LogByPass::OnCpuInterval(uv_timer_t* handle) {
  // set now cpu usage
  SetNowCpuUsage();
}

// static
void LogByPass::OnLogInterval(uv_timer_t* handle) {
  LogByPass* that = ContainerOf(&LogByPass::log_interval_, handle);
  if (!that->next_log_) {
    that->next_log_ = true;
    that->SendCollectStatistics();
    CHECK_EQ(0,
             uv_timer_start(&that->log_interval_, OnLogInterval, 1000, false));
    return;
  }
  that->next_log_ = false;
  that->CollectStatistics();
  CHECK_EQ(0, uv_timer_start(&that->log_interval_, OnLogInterval,
                             GetLogInterval() * 1000, false));
}

void LogByPass::SendCollectStatistics() {
  EnvironmentRegistry* registry = ProcessData::Get()->environment_registry();
  EnvironmentRegistry::NoExitScope scope(registry);

  for (EnvironmentData* env_data : *registry) {
    env_data->SendCollectStatistics();
  }
}

void LogByPass::CollectStatistics() {
  EnvironmentRegistry* registry = ProcessData::Get()->environment_registry();
  EnvironmentRegistry::NoExitScope scope(registry);
  bool log_format_alinode = GetFormatAsAlinode();

  // write cpu info
  WriteCpuUsageInPeriod(log_format_alinode);

  if (log_format_alinode) {
    EnvironmentData* env_data = registry->GetMainThread();
    if (env_data == nullptr) return;
    Write(env_data, log_format_alinode);
    return;
  }

  for (EnvironmentData* env_data : *registry) {
    Write(env_data, log_format_alinode);
  }
}

void LogByPass::Write(EnvironmentData* env_data, bool log_format_alinode) {
  // write heap memory info
  WriteMemoryInfoToLog(env_data, log_format_alinode);

  // write gc status
  WriteGcStatusToLog(env_data, log_format_alinode);

  // write libuv handle info
  WriteLibuvHandleInfoToLog(env_data, log_format_alinode);

  // write http status
  WriteHttpStatus(env_data, log_format_alinode, GetPatchHttpTimeout());
}

void RunLogBypass(const FunctionCallbackInfo<Value>& info) {
  EnvironmentData* env_data = EnvironmentData::GetCurrent(info);
  ThreadId thread_id = env_data->thread_id();
  // init gc hooks
  InitGcStatusHooks(env_data);
  InfoT("init", thread_id, "logbypass: gc hooks setted.");

  // init log thread
  ProcessData* data = ProcessData::Get();
  Mutex::ScopedLock lock(data->log_by_pass_mutex);
  if (data->log_by_pass == nullptr) {
    data->log_by_pass = std::unique_ptr<LogByPass>(new LogByPass());
    data->log_by_pass->StartIfNeeded();
    InfoT("init", thread_id, "logbypass: log thread created.");
  }

  info.GetReturnValue().Set(True());
}

}  // namespace xprofiler
