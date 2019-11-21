#include "error.h"

#include <stdarg.h>

namespace xprofiler {
XpfError XpfError::Failure(const char *format, ...) {
  char tmp[kMaxMessageLength];
  va_list arglist;
  va_start(arglist, format);
  vsnprintf(tmp, sizeof(tmp), format, arglist);
  va_end(arglist);
  return XpfError(true, string(tmp));
}
}  // namespace xprofiler