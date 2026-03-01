#if defined(__APPLE__) || defined(__linux__)
#include "library/common.h"
#include "uv.h"

namespace xprofiler {
double GetNowCpuUsage() {
  static uint64_t last_time = 0;
  static clock_t last_cpu_usage = 0;

  // first time
  if (last_time == 0 || last_cpu_usage == 0) {
    last_time = uv_hrtime();
    last_cpu_usage = clock();
    return -1;
  }

  uint64_t duration = (uv_hrtime() - last_time) / kNanosecondsPerSecond;
  if (duration <= 0) {
    return -1;
  }

  // calculate cpu usage
  double cpu_now_ =
      100 * (clock() - last_cpu_usage) / CLOCKS_PER_SEC / duration;

  // update time & cpu usage
  last_time = uv_hrtime();
  last_cpu_usage = clock();

  return cpu_now_;
}
}  // namespace xprofiler
#endif
