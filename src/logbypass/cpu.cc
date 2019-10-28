#include "cpu.h"
#include "uv.h"

#define INIT_CPU_PERIOD(period)                                                \
  static double *cpu_##period = new double[period];                            \
  static int cpu_##period##_array_index = 0;                                   \
  static int cpu_##period##_array_lenhth = period;

#define SET_CPU_PERIOD(period, cpu_now)                                        \
  bool cpu_##period##_array_not_full =                                         \
      cpu_##period##_array_index < cpu_##period##_array_lenhth;                \
  if (cpu_##period##_array_not_full) {                                         \
    cpu_##period[cpu_##period##_array_index++] = cpu_now;                      \
  } else {                                                                     \
    cpu_##period##_array_index = 0;                                            \
    cpu_##period[cpu_##period##_array_index++] = cpu_now;                      \
  }

namespace xprofiler {
// init cpu 15/30/60
INIT_CPU_PERIOD(15);
INIT_CPU_PERIOD(30);
INIT_CPU_PERIOD(60);

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
  double cpu_now = 100 * (clock() - last_cpu_usage) / CLOCKS_PER_SEC /
                   ((uv_hrtime() - last_time) / 10e8);

  // update time & cpu usage
  last_time = uv_hrtime();
  last_cpu_usage = clock();

  return cpu_now;
}

void SetNowCpuUsage() {
  double cpu_now = GetNowCpuUsage();
  if (cpu_now < 0) {
    return;
  }
  SET_CPU_PERIOD(15, cpu_now);
  SET_CPU_PERIOD(30, cpu_now);
  SET_CPU_PERIOD(60, cpu_now);
}
} // namespace xprofiler
