#include "heap_limit.h"

#include "environment_data.h"
#include "logger.h"

namespace xprofiler {
static const char module_type[] = "heap_limit";

size_t NearHeapLimitCallback(void* data, size_t current_heap_limit,
                             size_t initial_heap_limit) {
  // const size_t heapdump_factor = 2;
  // size_t max_limit = (std::numeric_limits<size_t>::max)() / 4;
  // size_t increased_heap =
  //     current_heap_limit +
  //     std::min(max_limit, initial_heap_limit * heapdump_factor);

  size_t increased_heap = current_heap_limit + 512 * 1024 * 1024;

  ThreadId thread_id = *static_cast<ThreadId*>(data);
  InfoT(module_type, thread_id,
        "current_heap_limit is %d, initial_heap_limit is %d, increased_heap is "
        "%d.",
        current_heap_limit, initial_heap_limit, increased_heap);

  return increased_heap;
}

void AutoIncreaseHeapLimit(v8::Isolate* isolate) {
  EnvironmentData* env_data = EnvironmentData::GetCurrent(isolate);
  ThreadId thread_id = env_data->thread_id();

  InfoT(module_type, thread_id, "auto increase heap limit hook.");
  isolate->AddNearHeapLimitCallback(NearHeapLimitCallback,
                                    static_cast<void*>(&thread_id));
}
}  // namespace xprofiler