#include "nan.h"
#include "uv.h"

#include "../logger.h"
#include "../utils.h"
#include "heap.h"

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
  if (err != 0)
    rss = 0;
}

void SetHeapStatistics() {
  GetHeapStatistics(heap_statistics->handle());
#if (NODE_MODULE_VERSION < 72)
  Isolate *isolate = Isolate::GetCurrent();
  heap_statistics->external_memory() =
      isolate->AdjustAmountOfExternalAllocatedMemory(0);
#endif
}

#define SET_SPACE_STATISTICS(name)                                             \
  if (strcmp(s.space_name(), #name) == 0) {                                    \
    heap_space_statistics->name##_size = s.space_size();                       \
    heap_space_statistics->name##_used = s.space_used_size();                  \
    heap_space_statistics->name##_available = s.space_used_size();             \
    heap_space_statistics->name##_committed = s.physical_space_size();         \
  }

void SetHeapSpaceStatistics() {
  Isolate *isolate = Isolate::GetCurrent();
  HeapSpaceStatistics s;
  size_t number_of_heap_spaces = isolate->NumberOfHeapSpaces();
  for (size_t i = 0; i < number_of_heap_spaces; i++) {
    isolate->GetHeapSpaceStatistics(&s, i);
    // todo: read_only_space, large_object_space, code_large_object_space
    // new space
    SET_SPACE_STATISTICS(new_space);
    // old space
    SET_SPACE_STATISTICS(old_space);
    // code space
    SET_SPACE_STATISTICS(code_space);
    // map space
    SET_SPACE_STATISTICS(map_space);
    // large object space
    SET_SPACE_STATISTICS(large_object_space);
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

void UnrefAsyncHandle() {
  uv_unref(reinterpret_cast<uv_handle_t *>(&memory_statistics_trigger));
}

#define LOG_SPACE_STATISTICS(name)                                             \
  heap_space_statistics->name##_space_size,                                    \
      heap_space_statistics->name##_space_used,                                \
      heap_space_statistics->name##_space_available,                           \
      heap_space_statistics->name##_space_committed

void WriteMemoryInfoToLog(bool log_format_alinode) {
  uv_async_send(&memory_statistics_trigger);
  // sleep 1s for executing async callback
  Sleep(1);

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
         "lo_space_committed: %zu",
         // rss
         rss,
         // heap statistics
         heap_statistics->used_heap_size(),
         heap_statistics->total_available_size(),
         heap_statistics->total_heap_size(), heap_statistics->heap_size_limit(),
         heap_statistics->total_heap_size_executable(),
         heap_statistics->total_physical_size(),
         heap_statistics->malloced_memory(), heap_statistics->external_memory(),
         // new space
         LOG_SPACE_STATISTICS(new),
         // old space
         LOG_SPACE_STATISTICS(old),
         // code space
         LOG_SPACE_STATISTICS(code),
         // map space
         LOG_SPACE_STATISTICS(map),
         // large object space
         LOG_SPACE_STATISTICS(large_object));
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
         "lo_space_committed: %zu",
         // rss
         rss,
         // heap statistics
         heap_statistics->used_heap_size(),
         heap_statistics->total_available_size(),
         heap_statistics->total_heap_size(), heap_statistics->heap_size_limit(),
         heap_statistics->total_heap_size_executable(),
         heap_statistics->total_physical_size(),
         heap_statistics->malloced_memory(), heap_statistics->external_memory(),
         // new space
         LOG_SPACE_STATISTICS(new),
         // old space
         LOG_SPACE_STATISTICS(old),
         // code space
         LOG_SPACE_STATISTICS(code),
         // map space
         LOG_SPACE_STATISTICS(map),
         // large object space
         LOG_SPACE_STATISTICS(large_object));
  }
}
} // namespace xprofiler
