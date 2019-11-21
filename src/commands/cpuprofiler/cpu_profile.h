#ifndef _SRC_COMMANDS_CPUPROFILER_CPU_PROFILE_H
#define _SRC_COMMANDS_CPUPROFILER_CPU_PROFILE_H

#include "nan.h"
#include "v8-profiler.h"

namespace xprofiler {
using std::string;
using v8::CpuProfile;

class Profile {
 public:
  static void Serialize(const CpuProfile *node, string filename);
};

}  // namespace xprofiler
#endif
