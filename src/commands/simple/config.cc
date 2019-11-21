#include "../../configure.h"
#include "../../library/json.hpp"
#include "../../library/utils.h"

namespace xprofiler {
using nlohmann::json;
using std::exception;

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

COMMAND_CALLBACK(SetXprofilerConfig) {
  json options = command["options"];
  bool setted = false;
  json data;
#define V(ret, key, func)                          \
  if (options.find(#key) != options.end()) {       \
    ret value;                                     \
    XpfError err;                                  \
    value = GetJsonValue<ret>(options, #key, err); \
    if (err.Fail()) {                              \
      error(format("%s", err.GetErrMessage()));    \
      return;                                      \
    }                                              \
    Set##func(value);                              \
    setted = true;                                 \
    data[#key] = Get##func();                      \
  }
  V(bool, enable_log_uv_handles, EnableLogUvHandles)
  V(LOG_LEVEL, log_level, LogLevel)
  V(LOG_TYPE, log_type, LogType)
#undef V
  if (!setted)
    error(format("not support setting config %s", options.dump().c_str()));
  else
    success(data);
}
}  // namespace xprofiler