#include "../commands/heapdump/heap_profiler.h"
#include "../configure.h"
#include "../library/utils.h"
#include "../logger.h"
#include "../platform/platform.h"
#include "nan.h"

namespace xprofiler {
using std::string;
using std::to_string;
using v8::Isolate;

static const char module_type[] = "out_of_memory";

static bool oom_flag = false;

size_t NearHeapLimitCallback(void* raw_data, size_t current_heap_limit,
                             size_t initial_heap_limit) {
  Info(module_type, "current_heap_limit is %d.", current_heap_limit);
  return initial_heap_limit + 200 * 1024 * 1024;
}

static void OnOutOfMemoryError(const char* location, bool is_heap_oom) {
  // avoid endless loop
  if (oom_flag) {
    Info(module_type, "heapdump hook before oom has been executed.");
    return;
  }
  oom_flag = true;

  Isolate* isolate = Isolate::GetCurrent();
  isolate->AddNearHeapLimitCallback(NearHeapLimitCallback, nullptr);

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
  isolate->SetOOMErrorHandler(OnOutOfMemoryError);
}
}  // namespace xprofiler