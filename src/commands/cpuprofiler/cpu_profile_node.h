#pragma once

#include "library/writer.h"
#include "nan.h"
#include "v8-profiler.h"

namespace xprofiler {
class CpuProfileNode {
 public:
  static void SerializeNode(v8::Isolate* isolate,
                            const v8::CpuProfileNode* node, JSONWriter* writer);
};

}  // namespace xprofiler
