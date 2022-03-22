#ifndef XPROFILER_SRC_ENVIRONMENT_DATA_H
#define XPROFILER_SRC_ENVIRONMENT_DATA_H

#include <functional>
#include <list>

#include "commands/cpuprofiler/cpu_profiler.h"
#include "commands/gcprofiler/gc_profiler.h"
#include "logbypass/gc.h"
#include "logbypass/heap.h"
#include "logbypass/http.h"
#include "logbypass/libuv.h"
#include "nan.h"
#include "xpf_mutex-inl.h"

namespace xprofiler {

enum class InterruptKind {
  kBusy,
  kIdle,
};

using InterruptCallback = std::function<void(EnvironmentData*, InterruptKind)>;

class EnvironmentData {
 public:
  static EnvironmentData* GetCurrent(v8::Isolate* isolate);
  static EnvironmentData* GetCurrent(
      const Nan::FunctionCallbackInfo<v8::Value>& info);
  static void Create(v8::Isolate* isolate);

  static void JsSetupEnvironmentData(
      const Nan::FunctionCallbackInfo<v8::Value>& info);

  void SendCollectStatistics();

  void RequestInterrupt(InterruptCallback interrupt);

  inline v8::Isolate* isolate() { return isolate_; }
  inline uv_loop_t* loop() { return loop_; }

  inline bool is_main_thread() { return is_main_thread_; }
  inline double thread_id() { return thread_id_; }

  inline GcStatistics* gc_statistics() { return &gc_statistics_; }
  inline HttpStatistics* http_statistics() { return &http_statistics_; }
  inline MemoryStatistics* memory_statistics() { return &memory_statistics_; }
  inline UvHandleStatistics* uv_handle_statistics() {
    return &uv_handle_statistics_;
  }

  std::unique_ptr<GcProfiler> gc_profiler;
  std::unique_ptr<CpuProfiler> cpu_profiler;

 private:
  static void AtExit(void* arg);
  static void CloseCallback(uv_handle_t* handle);
  static void InterruptBusyCallback(v8::Isolate* isolate, void* data);
  static void InterruptIdleCallback(uv_async_t* handle);

  static void CollectStatistics(uv_async_t* handle);
  EnvironmentData(v8::Isolate* isolate, uv_loop_t* loop);

  v8::Isolate* isolate_;
  uv_loop_t* loop_;
  uv_async_t statistics_async_;

  bool is_main_thread_ = false;
  /* We don't have a native method to get the uint64_t thread id.
   * Use the JavaScript number representation.
   */
  double thread_id_ = -1;

  Mutex interrupt_mutex_;
  std::list<InterruptCallback> interrupt_requests_;
  uv_async_t interrupt_async_;

  GcStatistics gc_statistics_;
  MemoryStatistics memory_statistics_;
  HttpStatistics http_statistics_;
  UvHandleStatistics uv_handle_statistics_;
};

}  // namespace xprofiler

#endif /* XPROFILER_SRC_ENVIRONMENT_DATA_H */
