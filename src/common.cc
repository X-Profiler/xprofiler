#include "common.h"
#include <stdarg.h>
#include <time.h>

namespace xprofiler {
static time_t load_time;

void InitGlobalVariables() { time(&load_time); }

unsigned long GetUptime() {
  time_t current_time;
  time(&current_time);
  return static_cast<unsigned long>(difftime(current_time, load_time));
}

CommonError::CommonError(bool failed, const char *format, ...) {
  failed_ = failed;
  char tmp[kMaxMessageLength];
  va_list arglist;
  va_start(arglist, format);
  vsnprintf(tmp, sizeof(tmp), format, arglist);
  va_end(arglist);
  msg_ = tmp;
}

CommonError CommonError::Failure(const char *format, ...) {
  char tmp[kMaxMessageLength];
  va_list arglist;
  va_start(arglist, format);
  vsnprintf(tmp, sizeof(tmp), format, arglist);
  va_end(arglist);
  return CommonError(true, string(tmp));
}
} // namespace xprofiler