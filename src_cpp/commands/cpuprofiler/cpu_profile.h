#ifndef XPROFILER_SRC_COMMANDS_CPUPROFILER_CPU_PROFILE_H
#define XPROFILER_SRC_COMMANDS_CPUPROFILER_CPU_PROFILE_H

#include "nan.h"
#include "util.h"
#include "v8-profiler.h"

namespace xprofiler {
class CpuProfile {
  static void DeleteCpuProfile(const v8::CpuProfile* profile);

 public:
  using CpuProfilePtr = DeleteFnPtr<const v8::CpuProfile, DeleteCpuProfile>;
  static void Serialize(v8::Isolate* isolate, CpuProfilePtr node,
                        std::string filename);
};

}  // namespace xprofiler
#endif /* XPROFILER_SRC_COMMANDS_CPUPROFILER_CPU_PROFILE_H */
