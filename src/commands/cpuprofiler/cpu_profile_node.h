#ifndef _SRC_COMMANDS_CPUPROFILER_CPU_PROFILE_NODE_H
#define _SRC_COMMANDS_CPUPROFILER_CPU_PROFILE_NODE_H

#include "../../library/writer.h"
#include "nan.h"
#include "v8-profiler.h"

namespace xprofiler {
using v8::CpuProfileNode;

class ProfileNode {
 public:
  static void SerializeNode(const CpuProfileNode* node, JSONWriter* writer);
};

}  // namespace xprofiler
#endif
