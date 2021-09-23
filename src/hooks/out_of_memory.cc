#include "../commands/heapdump/heap_profiler.h"
#include "../configure.h"
#include "../library/utils.h"
#include "../logger.h"
#include "../platform/platform.h"
#include "nan.h"

namespace xprofiler {
using Nan::HandleScope;
using std::string;
using std::to_string;
using v8::Isolate;

static const char module_type[] = "out_of_memory";

static bool oom_flag = false;

Isolate* isolate_ = nullptr;

size_t NearHeapLimitCallback(void* raw_data, size_t current_heap_limit,
                             size_t initial_heap_limit) {
  const size_t heapdump_factor = 2;
  size_t max_limit = (std::numeric_limits<size_t>::max)() / 4;
  size_t increased_heap =
      std::min(max_limit, initial_heap_limit * heapdump_factor);
  Info(module_type,
       "current_heap_limit is %d, initial_heap_limit is %d, increased_heap is "
       "%d.",
       current_heap_limit, initial_heap_limit, increased_heap);
  return increased_heap;
}

static void OnOutOfMemoryError(const char* location, bool is_heap_oom) {
  // avoid endless loop
  if (oom_flag) {
    Info(module_type, "heapdump hook before oom has been executed.");
    return;
  }
  oom_flag = true;

#if (NODE_MODULE_VERSION >= 64)
  Info(module_type, "increase heap limit hook.");
  HandleScope scope;
  isolate_->AddNearHeapLimitCallback(NearHeapLimitCallback, nullptr);
  Sleep(2);
#endif

  // dump snapshot
  string filepath = GetLogDir() + GetSep() + "x-oom-" + to_string(GetPid()) +
                    "-" + ConvertTime("%Y%m%d") + "-" + RandNum() +
                    ".heapsnapshot";
  Info(module_type, "heapdump to %s.", filepath.c_str());
  HeapProfiler::TakeSnapshot(filepath);
  Info(module_type, "heapsnapshot dumped.");
  raise(SIGABRT);
}

void SetOOMErrorHandler() {
  Isolate* isolate = Isolate::GetCurrent();
  isolate_ = isolate;
  isolate->SetOOMErrorHandler(OnOutOfMemoryError);
}
}  // namespace xprofiler