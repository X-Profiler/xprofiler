#include "heap_profiler.h"

#include "heap_snapshot.h"
#include "util.h"
#include "xpf_v8.h"

namespace xprofiler {
using v8::Isolate;

void DeleteHeapSnapshot(const v8::HeapSnapshot* snapshot) {
  const_cast<v8::HeapSnapshot*>(snapshot)->Delete();
}

void HeapProfiler::TakeSnapshot(v8::Isolate* isolate, std::string filename) {
  HandleScope scope(isolate);
  HeapSnapshotPointer snap =
      HeapSnapshotPointer(isolate->GetHeapProfiler()->TakeHeapSnapshot());
  HeapSnapshot::Serialize(std::move(snap), filename);
}

}  // namespace xprofiler
