#ifndef XPROFILER_SRC_LOGBYPASS_HEAP_H
#define XPROFILER_SRC_LOGBYPASS_HEAP_H

#include "nan.h"
#include "v8.h"

namespace xprofiler {

class EnvironmentData;

#define INIT_SPACE_INFO(name)        \
  size_t name##_space_size = 0;      \
  size_t name##_space_used = 0;      \
  size_t name##_space_available = 0; \
  size_t name##_space_committed = 0;

// heap statistics struct
struct XprofilerHeapStatistics {
 public:
  v8::HeapStatistics* handle() { return &heap_statistics_; }
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
  size_t external_memory() { return heap_statistics_.external_memory(); }

 private:
  v8::HeapStatistics heap_statistics_;
};

// heap space statistics struct
struct XprofilerHeapSpaceStatistics {
  INIT_SPACE_INFO(new)
  INIT_SPACE_INFO(old)
  INIT_SPACE_INFO(code)
  INIT_SPACE_INFO(map)
  INIT_SPACE_INFO(large_object)
  INIT_SPACE_INFO(read_only)          // needs v8 version >= 6.8
  INIT_SPACE_INFO(new_large_object)   // needs v8 version >= 6.9
  INIT_SPACE_INFO(code_large_object)  // needs v8 version >= 7.3
};

struct MemoryStatistics {
  XprofilerHeapStatistics heap_statistics;
  XprofilerHeapSpaceStatistics heap_space_statistics;
};

void CollectMemoryStatistics(EnvironmentData* env_data);
void WriteMemoryInfoToLog(EnvironmentData* env_data, bool log_format_alinode);
}  // namespace xprofiler

#endif /* XPROFILER_SRC_LOGBYPASS_HEAP_H */
