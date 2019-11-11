#include "version.h"

namespace xprofiler {
void GetXprofilerVersion(json command, cb_success *success, cb_error *error) {
  char version[32];
  snprintf(version, sizeof(version), "%s", XPROFILER_VERSION);
  json data;
  data["version"] = version;
  success(data);
}
} // namespace xprofiler