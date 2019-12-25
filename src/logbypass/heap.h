#ifndef _SRC_LOGBYPASS_HEAP_H
#define _SRC_LOGBYPASS_HEAP_H

#include "node_version.h"
#include "v8.h"

namespace xprofiler {
using v8::HeapStatistics;

// heap statistics struct
typedef struct {
 public:
  HeapStatistics *handle() { return &heap_statistics_; }
  size_t total_heap_size() { return heap_statistics_.total_heap_size(); }
  size_t used_heap_size() { return heap_statistics_.used_heap_size(); }
  size_t total_available_size() {
    return heap_statistics_.total_available_size();
  }
  size_t heap_size_limit() { return heap_statistics_.heap_size_limit(); }
  size_t total_heap_size_executable() {
    return heap_statistics_.total_heap_size_executable();
  }
  size_t total_physical_size() {
    return heap_statistics_.total_physical_size();
  }
  size_t malloced_memory() { return heap_statistics_.malloced_memory(); }
#if (NODE_MODULE_VERSION >= 72)
  size_t external_memory() { return heap_statistics_.external_memory(); }
#else
  size_t &external_memory() { return external_memory_; }
#endif

 private:
#if (NODE_MODULE_VERSION < 72)
  // external memory
  size_t external_memory_ = 0;
#endif
  HeapStatistics heap_statistics_;
} heap_statistics_t;

// heap space statistics struct
typedef struct {
#define V(name)                      \
  size_t name##_space_size = 0;      \
  size_t name##_space_used = 0;      \
  size_t name##_space_available = 0; \
  size_t name##_space_committed = 0;
  // new space
  V(new)
  // old space
  V(old)
  // code space
  V(code)
  // map space
  V(map)
  // large object space
  V(large_object)
  // read only space
  // needs v8 version >= 6.8
  V(read_only)
  // new large object space
  // needs v8 version >= 6.9
  V(new_large_object)
  // code large object space
  // needs v8 version >= 7.3
  V(code_large_object)
#undef V
} heap_space_statistics_t;

int InitMemoryAsyncCallback();
void UnrefMemoryAsyncHandle();
void GetMemoryInfo();
void WriteMemoryInfoToLog(bool log_format_alinode);
}  // namespace xprofiler

#endif