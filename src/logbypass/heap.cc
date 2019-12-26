#include "heap.h"

#include "../library/utils.h"
#include "../logger.h"
#include "nan.h"
#include "uv.h"

namespace xprofiler {
using Nan::GetHeapStatistics;
using v8::HeapSpaceStatistics;
using v8::Isolate;

static uv_async_t memory_statistics_trigger;
// memory statistics
static size_t rss = 0;
static heap_statistics_t *heap_statistics = new heap_statistics_t;
static heap_space_statistics_t *heap_space_statistics =
    new heap_space_statistics_t;

void SetRss() {
  int err = uv_resident_set_memory(&rss);
  if (err != 0) rss = 0;
}

void SetHeapStatistics() {
  GetHeapStatistics(heap_statistics->handle());
#if (NODE_MODULE_VERSION < 72)
  Isolate *isolate = Isolate::GetCurrent();
  heap_statistics->external_memory() =
      isolate->AdjustAmountOfExternalAllocatedMemory(0);
#endif
}

void SetHeapSpaceStatistics() {
  Isolate *isolate = Isolate::GetCurrent();
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

void GetMemoryStatistics(uv_async_t *handle) {
  SetRss();
  SetHeapStatistics();
  SetHeapSpaceStatistics();
}

int InitMemoryAsyncCallback() {
  int rc = uv_async_init(uv_default_loop(), &memory_statistics_trigger,
                         GetMemoryStatistics);
  return rc;
}

void UnrefMemoryAsyncHandle() {
  uv_unref(reinterpret_cast<uv_handle_t *>(&memory_statistics_trigger));
}

void GetMemoryInfo() { uv_async_send(&memory_statistics_trigger); }

void WriteMemoryInfoToLog(bool log_format_alinode) {
  if (log_format_alinode) {
    Info("heap",
         "rss: %zu, "
         "heap_used: %zu, "
         "heap_available: %zu, "
         "heap_total: %zu, "
         "heap_limit: %zu, "
         "heap_executeable: %zu, "
         "total_physical_size: %zu, "
         "malloced_memory: %zu, "
         "amount_of_external_allocated_memory: %zu, "
         "new_space_size: %zu, "
         "new_space_used: %zu, "
         "new_space_available: %zu, "
         "new_space_committed: %zu, "
         "old_space_size: %zu, "
         "old_space_used: %zu, "
         "old_space_available: %zu, "
         "old_space_committed: %zu, "
         "code_space_size: %zu, "
         "code_space_used: %zu, "
         "code_space_available: %zu, "
         "code_space_committed: %zu, "
         "map_space_size: %zu, "
         "map_space_used: %zu, "
         "map_space_available: %zu, "
         "map_space_committed: %zu, "
         "lo_space_size: %zu, "
         "lo_space_used: %zu, "
         "lo_space_available: %zu, "
         "lo_space_committed: %zu, "
         "read_only_space_size: %zu, "
         "read_only_space_used: %zu, "
         "read_only_space_available: %zu, "
         "read_only_space_committed: %zu, "
         "new_lo_space_size: %zu, "
         "new_lo_space_used: %zu, "
         "new_lo_space_available: %zu, "
         "new_lo_space_committed: %zu, "
         "code_lo_space_size: %zu, "
         "code_lo_space_used: %zu, "
         "code_lo_space_available: %zu, "
         "code_lo_space_committed: %zu",
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
         "memory_usage(byte) "
         "rss: %zu, "
         "heap_used: %zu, "
         "heap_available: %zu, "
         "heap_total: %zu, "
         "heap_limit: %zu, "
         "heap_executeable: %zu, "
         "total_physical_size: %zu, "
         "malloced_memory: %zu, "
         "amount_of_external_allocated_memory: %zu, "
         "new_space_size: %zu, "
         "new_space_used: %zu, "
         "new_space_available: %zu, "
         "new_space_committed: %zu, "
         "old_space_size: %zu, "
         "old_space_used: %zu, "
         "old_space_available: %zu, "
         "old_space_committed: %zu, "
         "code_space_size: %zu, "
         "code_space_used: %zu, "
         "code_space_available: %zu, "
         "code_space_committed: %zu, "
         "map_space_size: %zu, "
         "map_space_used: %zu, "
         "map_space_available: %zu, "
         "map_space_committed: %zu, "
         "lo_space_size: %zu, "
         "lo_space_used: %zu, "
         "lo_space_available: %zu, "
         "lo_space_committed: %zu, "
         "read_only_space_size: %zu, "
         "read_only_space_used: %zu, "
         "read_only_space_available: %zu, "
         "read_only_space_committed: %zu, "
         "new_lo_space_size: %zu, "
         "new_lo_space_used: %zu, "
         "new_lo_space_available: %zu, "
         "new_lo_space_committed: %zu, "
         "code_lo_space_size: %zu, "
         "code_lo_space_used: %zu, "
         "code_lo_space_available: %zu, "
         "code_lo_space_committed: %zu",
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
