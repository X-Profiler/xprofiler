#include "commands/report/node_report.h"
#include "configure-inl.h"
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

[[noreturn]] void OnFatalError(const char* location, const char* message) {
  if (location) {
    fprintf(stderr, "xprofiler: %s %s\n", location, message);
  } else {
    fprintf(stderr, "xprofiler: %s\n", message);
  }
  fflush(stderr);

  string filepath = GetLogDir() + GetSep() + "x-fatal-error-" +
                    to_string(GetPid()) + "-" + ConvertTime("%Y%m%d") + "-" +
                    RandNum() + ".diag";

  Info(module_type, "dump report to %s.", filepath.c_str());
  Isolate* isolate = TryGetCurrentIsolate();
  NodeReport::GetNodeReport(isolate, filepath, location, message, true);
  Info(module_type, "report dumped.");

  Abort();
}

void SetFatalErrorHandler(v8::Isolate* isolate) {
  isolate->SetFatalErrorHandler(OnFatalError);
}
}  // namespace xprofiler
