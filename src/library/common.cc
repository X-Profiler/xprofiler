#include <time.h>

namespace xprofiler {
static time_t load_time;

void InitGlobalVariables() { time(&load_time); }

unsigned long GetUptime() {
  time_t current_time;
  time(&current_time);
  return static_cast<unsigned long>(difftime(current_time, load_time));
}
} // namespace xprofiler