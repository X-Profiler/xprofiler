#ifndef XPROFILER_SRC_LOGGER_H
#define XPROFILER_SRC_LOGGER_H

#include "library/common.h"
#include "library/printf-inl.h"

namespace xprofiler {
// xprofiler logger
enum LOG_LEVEL { LOG_INFO, LOG_ERROR, LOG_DEBUG };
enum LOG_TYPE { LOG_TO_FILE, LOG_TO_TTY };

void Log(const LOG_LEVEL output_level, const char* type, ThreadId thread_id,
         const char* message);

// normal external
#define NATIVE_LOGGERS(V) \
  V(Info, LOG_INFO)       \
  V(Error, LOG_ERROR)     \
  V(Debug, LOG_DEBUG)

#define DEFINE_LOGGER(name, level)                                            \
  template <typename... Args>                                                 \
  inline void name(const char* component, const char* format, Args... args) { \
    std::string message = SPrintF(format, std::forward<Args>(args)...);       \
    Log(LOG_LEVEL::level, component, 0, message.c_str());                     \
  }
NATIVE_LOGGERS(DEFINE_LOGGER);
#undef DEFINE_LOGGER

#define DEFINE_LOGGER(name, level)                                      \
  template <typename... Args>                                           \
  inline void name##T(const char* component, ThreadId thread_id,        \
                      const char* format, Args... args) {               \
    std::string message = SPrintF(format, std::forward<Args>(args)...); \
    Log(LOG_LEVEL::level, component, thread_id, message.c_str());       \
  }
NATIVE_LOGGERS(DEFINE_LOGGER);
#undef DEFINE_LOGGER
}  // namespace xprofiler

#endif /* XPROFILER_SRC_LOGGER_H */
