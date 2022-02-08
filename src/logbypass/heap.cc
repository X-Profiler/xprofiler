#include "heap.h"

#include "environment_data.h"
#include "library/utils.h"
#include "logger.h"
#include "nan.h"
#include "uv.h"

namespace xprofiler {
using Nan::GetHeapStatistics;
using v8::HeapSpaceStatistics;
using v8::Isolate;

#define COMMON_INFO_FORMATTER  \
  "rss: %zu, "                 \
  "heap_used: %zu, "           \
  "heap_available: %zu, "      \
  "heap_total: %zu, "          \
  "heap_limit: %zu, "          \
  "heap_executeable: %zu, "    \
  "total_physical_size: %zu, " \
  "malloced_memory: %zu, "     \
  "amount_of_external_allocated_memory: %zu, "

#define COMMON_INFO_FORMATTERX "memory_usage(byte) " COMMON_INFO_FORMATTER

#define SPACE_INFO_FORMATTER(name)                            \
#name "_space_size: %zu, " #name "_space_used: %zu, " #name \
        "_space_available: %zu, " #name "_space_committed: %zu, "

#define LOG_SPACE_INFO(name)                         \
  heap_space_statistics->name##_space_size,          \
      heap_space_statistics->name##_space_used,      \
      heap_space_statistics->name##_space_available, \
      heap_space_statistics->name##_space_committed

#define SET_SPACE_INFO(name)                                            \
  if (strcmp(s.space_name(), #name) == 0) {                             \
    heap_space_statistics->name##_size = s.space_size();                \
    heap_space_statistics->name##_used = s.space_used_size();           \
    heap_space_statistics->name##_available = s.space_available_size(); \
    heap_space_statistics->name##_committed = s.physical_space_size();  \
  }

void GetRss(size_t* rss) {
  int err = uv_resident_set_memory(rss);
  if (err != 0) rss = 0;
}

void SetHeapStatistics(XprofilerHeapStatistics* heap_statistics) {
  GetHeapStatistics(heap_statistics->handle());
#if (NODE_MODULE_VERSION < 72)
  Isolate* isolate = Isolate::GetCurrent();
  heap_statistics->external_memory() =
      isolate->AdjustAmountOfExternalAllocatedMemory(0);
#endif
}

void SetHeapSpaceStatistics(
    XprofilerHeapSpaceStatistics* heap_space_statistics) {
  Isolate* isolate = Isolate::GetCurrent();
  HeapSpaceStatistics s;
  size_t number_of_heap_spaces = isolate->NumberOfHeapSpaces();
  for (size_t i = 0; i < number_of_heap_spaces; i++) {
    isolate->GetHeapSpaceStatistics(&s, i);

    SET_SPACE_INFO(new_space)
    SET_SPACE_INFO(old_space)
    SET_SPACE_INFO(code_space)
    SET_SPACE_INFO(map_space)
    SET_SPACE_INFO(large_object_space)
    SET_SPACE_INFO(read_only_space)          // needs v8 version >= 6.
    SET_SPACE_INFO(new_large_object_space)   // needs v8 version >= 6.9
    SET_SPACE_INFO(code_large_object_space)  // needs v8 version >= 7.3
  }
}

void CollectMemoryStatistics(EnvironmentData* env_data) {
  MemoryStatistics* memory_statistics = env_data->memory_statistics();
  SetHeapStatistics(&memory_statistics->heap_statistics);
  SetHeapSpaceStatistics(&memory_statistics->heap_space_statistics);
}

void WriteMemoryInfoToLog(EnvironmentData* env_data, bool log_format_alinode) {
  size_t rss;
  GetRss(&rss);

  MemoryStatistics* memory_statistics = env_data->memory_statistics();
  XprofilerHeapStatistics* heap_statistics =
      &memory_statistics->heap_statistics;
  XprofilerHeapSpaceStatistics* heap_space_statistics =
      &memory_statistics->heap_space_statistics;

  if (log_format_alinode) {
    Info("heap",
         COMMON_INFO_FORMATTER SPACE_INFO_FORMATTER(new) SPACE_INFO_FORMATTER(
             old) SPACE_INFO_FORMATTER(code) SPACE_INFO_FORMATTER(map)
             SPACE_INFO_FORMATTER(lo) SPACE_INFO_FORMATTER(read_only)
                 SPACE_INFO_FORMATTER(new_lo) SPACE_INFO_FORMATTER(code_lo),
         // rss
         rss,
         // heap statistics
         heap_statistics->used_heap_size(),
         heap_statistics->total_available_size(),
         heap_statistics->total_heap_size(), heap_statistics->heap_size_limit(),
         heap_statistics->total_heap_size_executable(),
         heap_statistics->total_physical_size(),
         heap_statistics->malloced_memory(), heap_statistics->external_memory(),
         // space statistics
         LOG_SPACE_INFO(new), LOG_SPACE_INFO(old), LOG_SPACE_INFO(code),
         LOG_SPACE_INFO(map), LOG_SPACE_INFO(large_object),
         LOG_SPACE_INFO(read_only), LOG_SPACE_INFO(new_large_object),
         LOG_SPACE_INFO(code_large_object));
  } else {
    Info("memory",
         COMMON_INFO_FORMATTERX SPACE_INFO_FORMATTER(new) SPACE_INFO_FORMATTER(
             old) SPACE_INFO_FORMATTER(code) SPACE_INFO_FORMATTER(map)
             SPACE_INFO_FORMATTER(lo) SPACE_INFO_FORMATTER(read_only)
                 SPACE_INFO_FORMATTER(new_lo) SPACE_INFO_FORMATTER(code_lo),
         // rss
         rss,
         // heap statistics
         heap_statistics->used_heap_size(),
         heap_statistics->total_available_size(),
         heap_statistics->total_heap_size(), heap_statistics->heap_size_limit(),
         heap_statistics->total_heap_size_executable(),
         heap_statistics->total_physical_size(),
         heap_statistics->malloced_memory(), heap_statistics->external_memory(),
         // space statistics
         LOG_SPACE_INFO(new), LOG_SPACE_INFO(old), LOG_SPACE_INFO(code),
         LOG_SPACE_INFO(map), LOG_SPACE_INFO(large_object),
         LOG_SPACE_INFO(read_only), LOG_SPACE_INFO(new_large_object),
         LOG_SPACE_INFO(code_large_object));
  }
}
}  // namespace xprofiler
