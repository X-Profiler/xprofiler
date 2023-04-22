#include "commands/coredumper/coredumper.h"
#include "commands/dump.h"
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
  if (location) {
    fprintf(stderr, "xprofiler: %s %s\n", location, message);
  } else {
    fprintf(stderr, "xprofiler: %s\n", message);
  }
  fflush(stderr);

  Isolate* isolate = TryGetCurrentIsolate();
  EnvironmentData* env_data = EnvironmentData::GetCurrent(isolate);
  ThreadId thread_id = env_data->thread_id();

#if (defined(__APPLE__) || (defined(__linux__) && defined(__x86_64__)))
  // finish sampling
  FinishSampling(isolate, "fatal_error");
#endif

  std::string log_dir = GetConfig<std::string>("log_dir");

  // generate report before abort
  if (GetConfig<bool>("enable_fatal_error_report")) {
    string filepath = log_dir + GetSep() + "x-fatal-error-" +
                      to_string(GetPid()) + "-" + ConvertTime("%Y%m%d") + "-" +
                      to_string(GetNextDiagFileId()) + ".diag";

    InfoT(module_type, thread_id, "dump report to %s.", filepath.c_str());
    NodeReport::GetNodeReport(isolate, filepath, location, message, true);
    InfoT(module_type, thread_id, "report dumped.");
  }

  // generator core file before abort
  if (GetConfig<bool>("enable_fatal_error_coredump")) {
    string filepath = log_dir + GetSep() + "x-fatal-error-" +
                      to_string(GetPid()) + "-" + ConvertTime("%Y%m%d") + "-" +
                      to_string(GetNextDiagFileId()) + ".core";
    InfoT(module_type, thread_id, "dump core to %s.", filepath.c_str());
    Coredumper::WriteCoredump(filepath);
    InfoT(module_type, thread_id, "core dumped.");
  }
}

#if (NODE_MODULE_VERSION > NODE_18_0_MODULE_VERSION)
[[noreturn]] void OnOOMError(const char* location,
                             const v8::OOMDetails& details) {
  const char* message =
      details.detail != nullptr ? details.detail
      : details.is_heap_oom
#else
[[noreturn]] void OnOOMError(const char* location, bool is_heap_oom) {
  const char* message =
      is_heap_oom
#endif
          ? "Allocation failed - JavaScript heap out of memory"
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
