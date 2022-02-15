#include <time.h>

#include <atomic>
#include <string>

namespace xprofiler {
namespace per_process {
time_t load_time;
std::atomic_size_t next_file_id(0);
}  // namespace per_process

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

size_t GetNextDiagFileId() { return per_process::next_file_id++; }
}  // namespace xprofiler
