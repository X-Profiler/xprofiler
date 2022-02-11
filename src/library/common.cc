#include <time.h>

#include <string>

namespace xprofiler {
namespace per_process {
time_t load_time;
}

void InitOnceLoadTime() { time(&per_process::load_time); }

unsigned long GetUptime() {
  time_t current_time;
  time(&current_time);
  return static_cast<unsigned long>(
      difftime(current_time, per_process::load_time));
}

std::string GetStartTime(std::string format) {
  char time_string_day[32];
  struct tm* ptm = localtime(&per_process::load_time);
  strftime(time_string_day, sizeof(time_string_day), format.c_str(), ptm);
  return std::string(time_string_day);
}
}  // namespace xprofiler
