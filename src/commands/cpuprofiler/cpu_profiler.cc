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

#if (NODE_MODULE_VERSION > NODE_8_0_MODULE_VERSION)
static int started_profiles_count = 0;
static int sampling_interval = 0;
static CpuProfiler* current_cpuprofiler = nullptr;
#endif

void Profiler::StartProfiling(std::string t) {
  HandleScope scope;
  Local<String> title = New<String>(t).ToLocalChecked();

#if (NODE_MODULE_VERSION > NODE_8_0_MODULE_VERSION)
  if (!started_profiles_count) {
    current_cpuprofiler = CpuProfiler::New(Isolate::GetCurrent());
  }
  if (sampling_interval) {
    current_cpuprofiler->SetSamplingInterval(sampling_interval);
  }
  ++started_profiles_count;
  current_cpuprofiler->StartProfiling(title, true);
#else
  Isolate::GetCurrent()->GetCpuProfiler()->StartProfiling(title, true);
#endif
}

void Profiler::StopProfiling(string t, string filename) {
  const CpuProfile* profile;
  HandleScope scope;
  Local<String> title = New<String>(t).ToLocalChecked();

#if (NODE_MODULE_VERSION > NODE_8_0_MODULE_VERSION)
  profile = current_cpuprofiler->StopProfiling(title);
#else
  profile = Isolate::GetCurrent()->GetCpuProfiler()->StopProfiling(title);
#endif

  Profile::Serialize(profile, filename);

#if (NODE_MODULE_VERSION > NODE_8_0_MODULE_VERSION)
  const_cast<CpuProfile*>(profile)->Delete();
  --started_profiles_count;
  if (!started_profiles_count) {
    current_cpuprofiler->Dispose();
    current_cpuprofiler = nullptr;
  }
#endif
}

void Profiler::SetSamplingInterval(uint32_t sample) {
#if (NODE_MODULE_VERSION > NODE_8_0_MODULE_VERSION)
  sampling_interval = sample;
#else
  Isolate::GetCurrent()->GetCpuProfiler()->SetSamplingInterval(sample);
#endif
}
}  // namespace xprofiler
