#include "../library/error.h"
#include "../library/json.hpp"
#include "../library/utils.h"
#include "../logger.h"
#include "../platform/platform.h"
#include "./dump.h"
#include "./send.h"
#include "./simple/config.h"
#include "./simple/version.h"

namespace xprofiler {
using nlohmann::json;
using std::exception;
using std::string;

void ParseCmd(char *command) {
  Debug("parser", "received command: %s", command);
  json parsed;
  try {
    parsed = json::parse(command);
  } catch (exception &e) {
    Error("parser", "parse command error: %s", e.what());
    return;
  }

  // handle cmd
  bool handled = false;
  string cmd = parsed["cmd"];

  // get traceid
  XpfError err;
  string traceid = GetJsonValue<string>(parsed, "traceid", err);
  if (err.Fail()) {
    ErrorValue("unknown", FmtMessage("traceid shoule be passed in: %s",
                                     err.GetErrMessage()));
    return;
  }

#define V(cmd_str, handle)                                            \
  if (strcmp(cmd.c_str(), #cmd_str) == 0) {                           \
    handle(                                                           \
        parsed, FmtMessage,                                           \
        [traceid](json data) { SuccessValue(traceid, data); },        \
        [traceid](string message) { ErrorValue(traceid, message); }); \
    handled = true;                                                   \
  }
  // get version
  V(check_version, GetXprofilerVersion)
  // get/set config
  V(get_config, GetXprofilerConfig)
  V(set_config, SetXprofilerConfig)
  // cpu profiling
  V(start_cpu_profiling, StartCpuProfiling)
  V(stop_cpu_profiling, StopCpuProfiling)
  // heapdump
  V(heapdump, Heapdump)
  // sampling heap profiling
  V(start_heap_profiling, StartSamplingHeapProfiling)
  V(stop_heap_profiling, StopSamplingHeapProfiling)
  // gc profiling
  V(start_gc_profiling, StartGcProfiling)
  V(stop_gc_profiling, StopGcProfiling)
#undef V

  // not match any commands
  if (!handled) {
    ErrorValue(traceid, FmtMessage("not support command: %s", cmd.c_str()));
  }
}
}  // namespace xprofiler