#include <time.h>

#include <atomic>
#include <string>

#include "environment_data.h"
#include "v8.h"

namespace xprofiler {
namespace per_process {
time_t load_time;
std::atomic_size_t next_file_id(0);
}  // namespace per_process

void InitOnceLoadTime() { time(&per_process::load_time); }

std::string GetStartTime(std::string format) {
  char time_string_day[32];
  struct tm* ptm = localtime(&per_process::load_time);
  strftime(time_string_day, sizeof(time_string_day), format.c_str(), ptm);
  return std::string(time_string_day);
}

size_t GetNextDiagFileId() { return per_process::next_file_id++; }

std::string GetGlobalNodeVersion(v8::Isolate* isolate) {
  EnvironmentData* env_data = EnvironmentData::GetCurrent(isolate);
  return env_data->node_version();
}
}  // namespace xprofiler
