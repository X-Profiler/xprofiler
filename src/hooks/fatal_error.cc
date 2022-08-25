#include "commands/coredumper/coredumper.h"
#include "commands/report/node_report.h"
#include "configure-inl.h"
#include "environment_data.h"
#include "library/utils.h"
#include "logger.h"
#include "nan.h"
#include "platform/platform.h"
#include "util.h"
#include "xpf_v8.h"

namespace xprofiler {
using std::string;
using std::to_string;
using v8::Isolate;

constexpr char module_type[] = "fatal_error";

void DumpBeforeAbort(const char* location, const char* message) {
  Isolate* isolate = TryGetCurrentIsolate();
  EnvironmentData* env_data = EnvironmentData::GetCurrent(isolate);
  ThreadId thread_id = env_data->thread_id();

  // generate report before abort
  if (GetEnableFatalErrorReport()) {
    string filepath = GetLogDir() + GetSep() + "x-fatal-error-" +
                      to_string(GetPid()) + "-" + ConvertTime("%Y%m%d") + "-" +
                      to_string(GetNextDiagFileId()) + ".diag";

    InfoT(module_type, thread_id, "dump report to %s.", filepath.c_str());
    NodeReport::GetNodeReport(isolate, filepath, location, message, true);
    InfoT(module_type, thread_id, "report dumped.");
  }

  // generator core file before abort
  if (GetEnableFatalErrorCoredump()) {
    string filepath = GetLogDir() + GetSep() + "x-fatal-error-" +
                      to_string(GetPid()) + "-" + ConvertTime("%Y%m%d") + "-" +
                      to_string(GetNextDiagFileId()) + ".core";
    InfoT(module_type, thread_id, "dump core to %s.", filepath.c_str());
    Coredumper::WriteCoredump(filepath);
    InfoT(module_type, thread_id, "core dumped.");
  }

  if (location) {
    fprintf(stderr, "xprofiler: %s %s\n", location, message);
  } else {
    fprintf(stderr, "xprofiler: %s\n", message);
  }
  fflush(stderr);
}

[[noreturn]] void OnOOMError(const char* location, bool is_heap_oom) {
  const char* message =
      is_heap_oom ? "Allocation failed - JavaScript heap out of memory"
                  : "Allocation failed - process out of memory";
  DumpBeforeAbort(location, message);
  Abort();
}

[[noreturn]] void OnFatalError(const char* location, const char* message) {
  DumpBeforeAbort(location, message);
  Abort();
}

void SetFatalErrorHandler(v8::Isolate* isolate) {
  isolate->SetOOMErrorHandler(OnOOMError);
  isolate->SetFatalErrorHandler(OnFatalError);
}
}  // namespace xprofiler
