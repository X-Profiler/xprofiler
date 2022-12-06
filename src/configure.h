#ifndef XPROFILER_SRC_CONFIGURE_H
#define XPROFILER_SRC_CONFIGURE_H

#include "library/json.hpp"
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

class ConfigStore {
  // TODO(legendecas): accessors.
 public:
  template <typename T>
  T GetConfig(std::string key) {
    return static_cast<T>(config_[key]);
  }

  template <typename T>
  void SetConfig(std::string key, T value) {
    config_[key] = value;
  }

 private:
  nlohmann::json config_;
};

}  // namespace xprofiler

#endif /* XPROFILER_SRC_CONFIGURE_H */
