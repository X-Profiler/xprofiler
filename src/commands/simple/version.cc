#include "version.h"

namespace xprofiler {
COMMAND_CALLBACK(GetXprofilerVersion) {
  json data;
  data["version"] = format("%s", XPROFILER_VERSION);
  success(data);
}
} // namespace xprofiler