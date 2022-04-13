#include "commands/parser.h"

#include "commands/dump.h"
#include "commands/send.h"
#include "commands/simple/config.h"
#include "commands/simple/registry.h"
#include "commands/simple/version.h"
#include "library/error.h"
#include "library/json.hpp"
#include "library/utils.h"
#include "logger.h"
#include "platform/platform.h"

namespace xprofiler {
using nlohmann::json;
using std::exception;
using std::string;

#define HANDLE_COMMANDS(cmd_str, handle)                              \
  if (strcmp(cmd.c_str(), #cmd_str) == 0) {                           \
    handle(                                                           \
        parsed, FmtMessage,                                           \
        [traceid](json data) { SuccessValue(traceid, data); },        \
        [traceid](string message) { ErrorValue(traceid, message); }); \
  } else

void ParseCmd(char* command) {
  Debug("parser", "received command: %s", command);
  json parsed;
  try {
    parsed = json::parse(command);
  } catch (exception& e) {
    Error("parser", "parse command error: %s", e.what());
    return;
  }

  XpfError err;
  string cmd = GetJsonValue<string>(parsed, "cmd", err);
  if (err.Fail()) {
    ErrorValue("unknown",
               FmtMessage("cmd shoule be passed in: %s", err.GetErrMessage()));
    return;
  }
  string traceid = GetJsonValue<string>(parsed, "traceid", err);
  if (err.Fail()) {
    ErrorValue("unknown", FmtMessage("traceid shoule be passed in: %s",
                                     err.GetErrMessage()));
    return;
  }

  // get version
  HANDLE_COMMANDS(check_version, GetXprofilerVersion)

  // list environments
  HANDLE_COMMANDS(list_environments, ListEnvironments)

  // get/set config
  HANDLE_COMMANDS(get_config, GetXprofilerConfig)
  HANDLE_COMMANDS(set_config, SetXprofilerConfig)

  // cpu profiling
  HANDLE_COMMANDS(start_cpu_profiling, StartCpuProfiling)
  HANDLE_COMMANDS(stop_cpu_profiling, StopCpuProfiling)

  // heapdump
  HANDLE_COMMANDS(heapdump, Heapdump)

  // sampling heap profiling
  HANDLE_COMMANDS(start_heap_profiling, StartSamplingHeapProfiling)
  HANDLE_COMMANDS(stop_heap_profiling, StopSamplingHeapProfiling)

  // gc profiling
  HANDLE_COMMANDS(start_gc_profiling, StartGcProfiling)
  HANDLE_COMMANDS(stop_gc_profiling, StopGcProfiling)

  // node report
  HANDLE_COMMANDS(diag_report, GetNodeReport)

  // not match any commands
  /* else */ {
    ErrorValue(traceid, FmtMessage("not support command: %s", cmd.c_str()));
  }
}
}  // namespace xprofiler
