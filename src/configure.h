#ifndef XPROFILER_SRC_CONFIGURE_H
#define XPROFILER_SRC_CONFIGURE_H

#include "logger.h"

namespace xprofiler {

inline std::string GetLogDir();
inline uint32_t GetLogInterval();
inline LOG_LEVEL GetLogLevel();
inline LOG_TYPE GetLogType();
inline bool GetFormatAsAlinode();
inline bool GetEnableLogUvHandles();
inline bool GetPatchHttp();
inline uint32_t GetPatchHttpTimeout();
inline bool GetCheckThrow();
inline bool GetEnableFatalErrorHook();
inline bool GetEnableFatalErrorReport();
inline bool GetEnableFatalErrorCoredump();
inline bool GetEnableHttpProfiling();

inline void SetLogLevel(LOG_LEVEL value);
inline void SetLogType(LOG_TYPE value);
inline void SetEnableLogUvHandles(bool value);
inline void SetEnableFatalErrorReport(bool value);
inline void SetEnableFatalErrorCoredump(bool value);
inline void SetEnableHttpProfiling(bool value);

class ConfigStore {
  // TODO(legendecas): accessors.
 public:
  std::string log_dir = "/tmp";
  uint32_t log_interval = 60;
  LOG_LEVEL log_level = LOG_ERROR;
  LOG_TYPE log_type = LOG_TO_FILE;
  bool enable_log_uv_handles = true;
  bool log_format_alinode = false;
  bool patch_http = true;
  uint32_t patch_http_timeout = 30;
  bool check_throw = true;
  bool enable_fatal_error_hook = true;
  bool enable_fatal_error_report = true;
  bool enable_fatal_error_coredump = false;
  bool enable_http_profiling = false;
};

}  // namespace xprofiler

#endif /* XPROFILER_SRC_CONFIGURE_H */
