#ifndef XPROFILER_SRC_CONFIGURE_H
#define XPROFILER_SRC_CONFIGURE_H

#include "library/common.h"
#include "library/error.h"
#include "logger.h"
#include "nan.h"

namespace xprofiler {

inline std::string GetLogDir();
inline uint32_t GetLogInterval();
inline LOG_LEVEL GetLogLevel();
inline LOG_TYPE GetLogType();
inline bool GetFormatAsAlinode();
inline bool GetEnableLogUvHandles();
inline bool GetEnableFatalErrorHook();
inline bool GetPatchHttp();
inline uint32_t GetPatchHttpTimeout();
inline bool GetCheckThrow();

inline void SetLogLevel(LOG_LEVEL value);
inline void SetLogType(LOG_TYPE value);
inline void SetEnableLogUvHandles(bool value);

// javascript accessible
void Configure(const Nan::FunctionCallbackInfo<v8::Value>& info);
void GetConfig(const Nan::FunctionCallbackInfo<v8::Value>& info);

class ConfigStore {
  // TODO(legendecas): accessors.
 public:
  std::string log_dir = "/tmp";
  uint32_t log_interval = 60;
  LOG_LEVEL log_level = LOG_ERROR;
  LOG_TYPE log_type = LOG_TO_FILE;
  bool enable_log_uv_handles = true;
  bool log_format_alinode = false;
  bool enable_fatal_error_hook = true;
  bool patch_http = true;
  uint32_t patch_http_timeout = 30;
  bool check_throw = true;
};

namespace per_process {
extern ConfigStore config_store;
}

}  // namespace xprofiler

#endif /* XPROFILER_SRC_CONFIGURE_H */
