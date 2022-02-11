#ifndef XPROFILER_SRC_LIBRARY_COMMON_H
#define XPROFILER_SRC_LIBRARY_COMMON_H

#include "json.hpp"

namespace xprofiler {
void InitOnceLoadTime();

// uptime
unsigned long GetUptime();
std::string GetStartTime(std::string format);

// commands
#define COMMAND_CALLBACK(cb)                                               \
  void cb(nlohmann::json command, std::string (*format)(const char*, ...), \
          std::function<void(nlohmann::json)> success,                     \
          std::function<void(std::string)> error)
}  // namespace xprofiler

#endif /* XPROFILER_SRC_LIBRARY_COMMON_H */
