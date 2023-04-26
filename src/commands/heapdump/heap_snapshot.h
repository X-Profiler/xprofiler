#pragma once

#include "heap_profiler.h"
#include "v8-profiler.h"

namespace xprofiler {
class HeapSnapshot {
 public:
  static void Serialize(HeapSnapshotPointer profile, std::string filename);
};
}  // namespace xprofiler
