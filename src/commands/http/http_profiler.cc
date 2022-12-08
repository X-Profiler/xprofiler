#include "http_profiler.h"

#include "environment_data.h"

namespace xprofiler {
void HttpProfiler::StartProfiling(v8::Isolate* isolate) {
  EnvironmentData* env_data = EnvironmentData::GetCurrent(isolate);
  if (env_data == nullptr) {
    return;
  }

  EnableHttpDetailProfiling(env_data);
}

void HttpProfiler::StopProfiling(v8::Isolate* isolate) {
  EnvironmentData* env_data = EnvironmentData::GetCurrent(isolate);
  if (env_data == nullptr) {
    return;
  }

  DisableHttpDetailProfiling(env_data);

  // todo: save http profiling data to file
}
}  // namespace xprofiler