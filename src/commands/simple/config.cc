#include "commands/simple/config.h"

#include "configure-inl.h"
#include "library/json.hpp"
#include "library/utils.h"

namespace xprofiler {
using nlohmann::json;
using std::exception;

#define HANDLE_CONFIG_SETTING(type, key)                                    \
  if (options.find(#key) != options.end()) {                                \
    type value;                                                             \
    XpfError err;                                                           \
    value = GetJsonValue<type>(options, #key, err);                         \
    if (err.Fail()) {                                                       \
      error(format("%s", err.GetErrMessage()));                             \
      return;                                                               \
    }                                                                       \
    ProcessData::Get()->config_store()->SetConfig<type>(#key, value);       \
    setted = true;                                                          \
    data[#key] = ProcessData::Get()->config_store()->GetConfig<type>(#key); \
  }

COMMAND_CALLBACK(GetXprofilerConfig) {
  json data;
  data["log_dir"] = GetLogDir();
  data["log_interval"] = GetLogInterval();
  data["log_level"] = GetLogLevel();
  data["log_type"] = GetLogType();
  data["log_format_alinode"] = GetFormatAsAlinode();
  data["patch_http"] = GetPatchHttp();
  data["patch_http_timeout"] = GetPatchHttpTimeout();
  data["check_throw"] = GetCheckThrow();
  data["enable_log_uv_handles"] = GetEnableLogUvHandles();
  data["enable_fatal_error_hook"] = GetEnableFatalErrorHook();
  data["enable_fatal_error_report"] = GetEnableFatalErrorReport();
  data["enable_fatal_error_coredump"] = GetEnableFatalErrorCoredump();
  data["enable_http_profiling"] = GetEnableHttpProfiling();
  success(data);
}

COMMAND_CALLBACK(SetXprofilerConfig) {
  json options = command["options"];
  bool setted = false;
  json data;

  HANDLE_CONFIG_SETTING(LOG_LEVEL, log_level)
  HANDLE_CONFIG_SETTING(LOG_TYPE, log_type)
  HANDLE_CONFIG_SETTING(bool, enable_log_uv_handles)
  HANDLE_CONFIG_SETTING(bool, enable_fatal_error_report)
  HANDLE_CONFIG_SETTING(bool, enable_fatal_error_coredump)
  HANDLE_CONFIG_SETTING(bool, enable_http_profiling)

  if (!setted)
    error(format("not support setting config %s", options.dump().c_str()));
  else
    success(data);
}
}  // namespace xprofiler
