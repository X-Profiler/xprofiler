#include <time.h>

#include <string>

namespace xprofiler {
using std::string;

static time_t load_time;

void InitGlobalVariables() { time(&load_time); }

unsigned long GetUptime() {
  time_t current_time;
  time(&current_time);
  return static_cast<unsigned long>(difftime(current_time, load_time));
}

string GetStartTime(string format) {
  char time_string_day[32];
  struct tm *ptm = localtime(&load_time);
  strftime(time_string_day, sizeof(time_string_day), format.c_str(), ptm);
  return (string)time_string_day;
}
}  // namespace xprofiler