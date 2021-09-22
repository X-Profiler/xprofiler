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

static void OnOutOfMemoryError(const char* location, bool is_heap_oom) {
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