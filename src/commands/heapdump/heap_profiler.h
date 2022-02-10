#ifndef XPROFILER_SRC_COMMANDS_HEAPDUMP_HEAP_PROFILER_H
#define XPROFILER_SRC_COMMANDS_HEAPDUMP_HEAP_PROFILER_H

#include "nan.h"
#include "util.h"
#include "v8-profiler.h"

namespace xprofiler {
void DeleteHeapSnapshot(const v8::HeapSnapshot* snapshot);

using HeapSnapshotPointer =
    DeleteFnPtr<const v8::HeapSnapshot, DeleteHeapSnapshot>;

class HeapProfiler {
 public:
  static void TakeSnapshot(v8::Isolate* isolate, std::string filename);
};
}  // namespace xprofiler

#endif /* XPROFILER_SRC_COMMANDS_HEAPDUMP_HEAP_PROFILER_H */
