#if defined(__APPLE__) || defined(__linux__)
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

  // calculate cpu usage
  double cpu_now_ = 100 * (clock() - last_cpu_usage) / CLOCKS_PER_SEC /
                    ((uv_hrtime() - last_time) / 10e8);

  // update time & cpu usage
  last_time = uv_hrtime();
  last_cpu_usage = clock();

  return cpu_now_;
}
} // namespace xprofiler
#endif