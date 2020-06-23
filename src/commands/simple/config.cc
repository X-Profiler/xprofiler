#include "../../configure.h"
#include "../../library/json.hpp"
#include "../../library/utils.h"

namespace xprofiler {
using nlohmann::json;
using std::exception;

#define HANDLE_CONFIG_SETTING(ret, key, func)      \
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

COMMAND_CALLBACK(GetXprofilerConfig) {
  json data;
  data["log_dir"] = GetLogDir();
  data["log_interval"] = GetLogInterval();
  data["enable_log_uv_handles"] = GetEnableLogUvHandles();
  data["log_format_alinode"] = GetFormatAsAlinode();
  data["log_level"] = GetLogLevel();
  data["log_type"] = GetLogType();
  data["enable_fatal_error_hook"] = GetEnableFatalErrorHook();
  data["patch_http"] = GetPatchHttp();
  data["patch_http_timeout"] = GetPatchHttpTimeout();
  data["check_throw"] = GetCheckThrow();
  success(data);
}

COMMAND_CALLBACK(SetXprofilerConfig) {
  json options = command["options"];
  bool setted = false;
  json data;

  HANDLE_CONFIG_SETTING(LOG_LEVEL, log_level, LogLevel)
  HANDLE_CONFIG_SETTING(LOG_TYPE, log_type, LogType)
  HANDLE_CONFIG_SETTING(bool, enable_log_uv_handles, EnableLogUvHandles)
  HANDLE_CONFIG_SETTING(bool, enable_fatal_error_hook, EnableFatalErrorHook)

  if (!setted)
    error(format("not support setting config %s", options.dump().c_str()));
  else
    success(data);
}
}  // namespace xprofiler