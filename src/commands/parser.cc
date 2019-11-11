#include "../common.h"
#include "../library/json.hpp"
#include "../logger.h"
#include "../platform/platform.h"
#include "./send.h"
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
#define V(cmd_str, handle)                                                     \
  if (strcmp(cmd.c_str(), #cmd_str) == 0) {                                    \
    handle(parsed, SuccessValue, ErrorValue);                                  \
    handled = true;                                                            \
  }
  // check xprofiler version
  V(check_version, GetXprofilerVersion)
#undef V

  // not match any commands
  if (!handled) {
    ErrorValue("not support command: %s", cmd.c_str());
  }
}
} // namespace xprofiler