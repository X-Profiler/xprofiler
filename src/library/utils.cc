#include <stdarg.h>

#include "../platform/platform.h"

#ifdef _WIN32
#include <time.h>
#endif

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

string RandNum() {
  srand(time(0));
  return std::to_string(rand() % 1000000);
}

string GetDate() {
  char time_string_day[32];
  time_t tt = time(NULL);
  struct tm *ptm = localtime(&tt);
  strftime(time_string_day, sizeof(time_string_day), "%Y%m%d", ptm);
  return (string)time_string_day;
}

};  // namespace xprofiler
