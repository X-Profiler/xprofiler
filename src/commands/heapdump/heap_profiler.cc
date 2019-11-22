#include "heap_profiler.h"

#include "heap_snapshot.h"

namespace xprofiler {
using Nan::HandleScope;
using v8::HeapSnapshot;
using v8::Isolate;

HeapProfiler::HeapProfiler() {}
HeapProfiler::~HeapProfiler() {}

void HeapProfiler::TakeSnapshot(string filename) {
  Isolate *isolate = Isolate::GetCurrent();
  HandleScope scope;
  const HeapSnapshot *snap = isolate->GetHeapProfiler()->TakeHeapSnapshot();
  Snapshot::Serialize(snap, filename);
}
}  // namespace xprofiler
