#include "../commands/report/node_report.h"
#include "../configure.h"
#include "../library/utils.h"
#include "../logger.h"
#include "../platform/platform.h"
#include "nan.h"

namespace xprofiler {
using std::string;
using std::to_string;
using v8::Isolate;

static const char module_type[] = "fatal_error";

static void OnFatalError(const char* location, const char* message) {
  string filepath = GetLogDir() + GetSep() + "x-fatal-error-" +
                    to_string(GetPid()) + "-" + ConvertTime("%Y%m%d") + "-" +
                    RandNum() + ".diag";
  Info(module_type, "dump report to %s.", filepath.c_str());
  NodeReport::GetNodeReport(filepath, location, message, true);
  Info(module_type, "report dumped.");
  raise(SIGABRT);
}

void SetFatalErrorHandler() {
  Isolate* isolate = Isolate::GetCurrent();
  isolate->SetFatalErrorHandler(OnFatalError);
}
}  // namespace xprofiler