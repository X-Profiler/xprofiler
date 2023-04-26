#pragma once

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
