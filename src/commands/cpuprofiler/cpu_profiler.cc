#include "cpu_profiler.h"

#include "cpu_profile.h"

namespace xprofiler {
using Nan::HandleScope;
using Nan::New;
using v8::CpuProfile;
using v8::CpuProfiler;
using v8::Isolate;
using v8::Local;
using v8::String;

Profiler::Profiler() {}
Profiler::~Profiler() {}

#if (NODE_MODULE_VERSION > 0x0039)
static CpuProfiler *current_cpuprofiler =
    CpuProfiler::New(Isolate::GetCurrent());
#endif

void Profiler::StartProfiling(std::string t) {
  HandleScope scope;
  Local<String> title = New<String>(t).ToLocalChecked();

#if (NODE_MODULE_VERSION > 0x0039)
  current_cpuprofiler->StartProfiling(title, true);
#else
  Isolate::GetCurrent()->GetCpuProfiler()->StartProfiling(title, true);
#endif
}

void Profiler::StopProfiling(string t, string filename) {
  const CpuProfile *profile;
  HandleScope scope;
  Local<String> title = New<String>(t).ToLocalChecked();

#if (NODE_MODULE_VERSION > 0x0039)
  profile = current_cpuprofiler->StopProfiling(title);
#else
  profile = Isolate::GetCurrent()->GetCpuProfiler()->StopProfiling(title);
#endif

  Profile::Serialize(profile, filename);
}

void Profiler::SetSamplingInterval(uint32_t sample) {
#if (NODE_MODULE_VERSION > 0x0039)
  current_cpuprofiler->SetSamplingInterval(sample);
#else
  Isolate::GetCurrent()->GetCpuProfiler()->SetSamplingInterval(sample);
#endif
}
}  // namespace xprofiler
