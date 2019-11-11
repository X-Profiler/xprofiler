#include "version.h"

namespace xprofiler {
COMMAND_CALLBACK(GetXprofilerVersion) {
  char version[32];
  snprintf(version, sizeof(version), "%s", XPROFILER_VERSION);
  json data;
  data["version"] = version;
  success(data);
}
} // namespace xprofiler