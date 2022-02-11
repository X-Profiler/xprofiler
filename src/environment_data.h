#ifndef XPROFILER_SRC_ENVIRONMENT_DATA_H
#define XPROFILER_SRC_ENVIRONMENT_DATA_H

#include "commands/gcprofiler/gc_profiler.h"
#include "logbypass/gc.h"
#include "logbypass/heap.h"
#include "logbypass/http.h"
#include "logbypass/libuv.h"
#include "nan.h"

namespace xprofiler {

class EnvironmentData {
 public:
  // TODO(legendecas): remove this no-args GetCurrent.
  static EnvironmentData* GetCurrent();
  static EnvironmentData* GetCurrent(v8::Isolate* isolate);
  static EnvironmentData* GetCurrent(
      const Nan::FunctionCallbackInfo<v8::Value>& info);
  static EnvironmentData* Create(v8::Isolate* isolate);

  void SendCollectStatistics();

  inline v8::Isolate* isolate() { return isolate_; }
  inline uv_loop_t* loop() { return loop_; }

  inline GcStatistics* gc_statistics() { return &gc_statistics_; }
  inline HttpStatistics* http_statistics() { return &http_statistics_; }
  inline MemoryStatistics* memory_statistics() { return &memory_statistics_; }
  inline UvHandleStatistics* uv_handle_statistics() {
    return &uv_handle_statistics_;
  }

  std::unique_ptr<GcProfiler> gc_profiler;

 private:
  static void AtExit(void* arg);
  static void CollectStatistics(uv_async_t* handle);
  EnvironmentData(v8::Isolate* isolate, uv_loop_t* loop);

  v8::Isolate* isolate_;
  uv_loop_t* loop_;
  uv_async_t statistics_async_;

  GcStatistics gc_statistics_;
  MemoryStatistics memory_statistics_;
  HttpStatistics http_statistics_;
  UvHandleStatistics uv_handle_statistics_;
};

}  // namespace xprofiler

#endif /* XPROFILER_SRC_ENVIRONMENT_DATA_H */
