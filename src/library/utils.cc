#include <stdarg.h>

#include "../platform/platform.h"

namespace xprofiler {
using std::string;
void Sleep(int seconds) { SleepCrossPlatform(seconds); }

string FmtMessage(const char *format, ...) {
  char message[1024];
  va_list args;
  va_start(args, format);
  vsnprintf(message, sizeof(message), format, args);
  va_end(args);

  return string(message);
}

};  // namespace xprofiler
