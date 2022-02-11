#ifndef XPROFILER_SRC_COMMANDS_HEAPDUMP_HEAP_SNAPSHOT_H
#define XPROFILER_SRC_COMMANDS_HEAPDUMP_HEAP_SNAPSHOT_H

#include "heap_profiler.h"
#include "v8-profiler.h"

namespace xprofiler {
class HeapSnapshot {
 public:
  static void Serialize(HeapSnapshotPointer profile, std::string filename);
};
}  // namespace xprofiler

#endif /* XPROFILER_SRC_COMMANDS_HEAPDUMP_HEAP_SNAPSHOT_H */
