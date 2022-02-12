#include "cpu_profiler.h"

#include "cpu_profile.h"
#include "environment_data.h"
#include "xpf_v8.h"

namespace xprofiler {
using Nan::New;
using v8::Isolate;
using v8::Local;
using v8::String;

void CpuProfiler::DeleteCpuProfiler(v8::CpuProfiler* profiler) {
#if (NODE_MODULE_VERSION > NODE_8_0_MODULE_VERSION)
  profiler->Dispose();
#endif
}

void CpuProfiler::StartProfiling(v8::Isolate* isolate, std::string t) {
  EnvironmentData* env_data = EnvironmentData::GetCurrent(isolate);
  if (env_data->cpu_profiler == nullptr) {
    env_data->cpu_profiler =
        std::unique_ptr<CpuProfiler>(new CpuProfiler(isolate));
  }
  env_data->cpu_profiler->StartProfiling(t);
}

void CpuProfiler::StopProfiling(v8::Isolate* isolate, std::string t,
                                std::string filename) {
  EnvironmentData* env_data = EnvironmentData::GetCurrent(isolate);
  if (env_data->cpu_profiler == nullptr) {
    return;
  }
  env_data->cpu_profiler->StopProfiling(t, filename);
  if (env_data->cpu_profiler->started_profiles_count() == 0) {
    env_data->cpu_profiler.reset();
  }
}

CpuProfiler::CpuProfiler(v8::Isolate* isolate) : isolate_(isolate) {
#if (NODE_MODULE_VERSION > NODE_8_0_MODULE_VERSION)
  cpu_profiler_ = CpuProfilerPtr(v8::CpuProfiler::New(Isolate::GetCurrent()));
#else
  cpu_profiler_ = CpuProfilerPtr(isolate->GetCpuProfiler());
#endif
}

CpuProfiler::~CpuProfiler() {}

void CpuProfiler::StartProfiling(std::string t) {
  HandleScope scope(isolate_);
  Local<String> title = New<String>(t).ToLocalChecked();

  ++started_profiles_count_;
  cpu_profiler_->StartProfiling(title, true);
}

void CpuProfiler::StopProfiling(std::string t, std::string filename) {
  HandleScope scope(isolate_);
  Local<String> title = New<String>(t).ToLocalChecked();

  CpuProfile::CpuProfilePtr profile =
      CpuProfile::CpuProfilePtr(cpu_profiler_->StopProfiling(title));

  CpuProfile::Serialize(isolate_, std::move(profile), filename);

  --started_profiles_count_;
}
}  // namespace xprofiler
