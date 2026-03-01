#include "commands/simple/version.h"

namespace xprofiler {
using nlohmann::json;

COMMAND_CALLBACK(GetXprofilerVersion) {
  json data;
  data["version"] = format("%s", XPROFILER_VERSION);
  success(data);
}
}  // namespace xprofiler
