#include "heap_limit.h"

#include "configure-inl.h"
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

  int auto_incr_heap_limit_size = GetConfig<int>("auto_incr_heap_limit_size");
  size_t increased_heap =
      current_heap_limit + auto_incr_heap_limit_size * 1024 * 1024;

  EnvironmentData* env_data = static_cast<EnvironmentData*>(data);
  InfoT(module_type, env_data->thread_id(),
        "current_heap_limit is %d, initial_heap_limit is %d, "
        "auto_incr_heap_limit_size is %d, increased_heap is "
        "%d.",
        current_heap_limit, initial_heap_limit, auto_incr_heap_limit_size,
        increased_heap);

  return increased_heap;
}

void AutoIncreaseHeapLimit(v8::Isolate* isolate) {
  EnvironmentData* env_data = EnvironmentData::GetCurrent(isolate);

  InfoT(module_type, env_data->thread_id(), "auto increase heap limit hook.");
  isolate->AddNearHeapLimitCallback(NearHeapLimitCallback,
                                    static_cast<void*>(env_data));
  isolate->AutomaticallyRestoreInitialHeapLimit();
}
}  // namespace xprofiler
