#include "config.h"
#include "../../configure.h"

namespace xprofiler {
COMMAND_CALLBACK(GetXprofilerConfig) {
  json data;
  data["log_dir"] = GetLogDir();
  data["log_interval"] = GetLogInterval();
  data["enable_log_uv_handles"] = GetEnableLogUvHandles();
  data["log_format_alinode"] = GetFormatAsAlinode();
  data["log_level"] = GetLogLevel();
  data["log_type"] = GetLogType();
  success(data);
}
} // namespace xprofiler