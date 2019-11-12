#include "config.h"
#include "../../configure.h"
#include "../../logger.h"

namespace xprofiler {
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
#define V(ret, key, func)                                                      \
  if (options.find(#key) != options.end()) {                                   \
    ret value;                                                                 \
    try {                                                                      \
      value = options[#key].get<ret>();                                        \
    } catch (exception & e) {                                                  \
      Error("set_config", "%s <" #key "> type error: %s",                      \
            options.dump().c_str(), e.what());                                 \
      error("%s <" #key "> type error: %s", options.dump().c_str(), e.what()); \
      return;                                                                  \
    }                                                                          \
    Set##func(value);                                                          \
    setted = true;                                                             \
    data[#key] = Get##func();                                                  \
  }
  V(bool, enable_log_uv_handles, EnableLogUvHandles)
  V(LOG_LEVEL, log_level, LogLevel)
  V(LOG_TYPE, log_type, LogType)
#undef V
  if (!setted)
    error("not support setting config %s", options.dump().c_str());
  else
    success(data);
}
} // namespace xprofiler