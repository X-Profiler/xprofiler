#include "../logger.h"
#include "../platform/platform.h"

#define CALAULATE_AVERAGE(total_length, period)                                \
  if (total_length == 0)                                                       \
    return;                                                                    \
  for (int i = 0; i < total_length; i++) {                                     \
    cpu_##period##_average += cpu_##period[i];                                 \
  }                                                                            \
  cpu_##period##_average = cpu_##period##_average / total_length;

namespace xprofiler {
// init cpu now
double cpu_now = 0.0;

// init cpu 15/30/60
#define V(period)                                                              \
  static double *cpu_##period = new double[period];                            \
  static int cpu_##period##_array_index = 0;                                   \
  static int cpu_##period##_array_length = period;
V(15)
V(30)
V(60)
#undef V

void SetNowCpuUsage() {
  double cpu_now_ = GetNowCpuUsage();
  if (cpu_now_ < 0) {
    return;
  }
  cpu_now = cpu_now_;

#define V(period)                                                              \
  bool cpu_##period##_array_not_full =                                         \
      cpu_##period##_array_index < cpu_##period##_array_length;                \
  if (cpu_##period##_array_not_full) {                                         \
    cpu_##period[cpu_##period##_array_index++] = cpu_now;                      \
  } else {                                                                     \
    cpu_##period##_array_index = 0;                                            \
    cpu_##period[cpu_##period##_array_index++] = cpu_now;                      \
  }
  V(15)
  V(30)
  V(60)
#undef V
}

void WriteCpuUsageInPeriod(bool log_format_alinode) {
  double cpu_15_average = 0.0, cpu_30_average = 0.0, cpu_60_average = 0.0;

#define V(period)                                                              \
  bool cpu_##period##_array_not_full =                                         \
      cpu_##period##_array_index < cpu_##period##_array_length;                \
  if (cpu_##period##_array_not_full) {                                         \
    CALAULATE_AVERAGE(cpu_##period##_array_index, period)                      \
  } else {                                                                     \
    CALAULATE_AVERAGE(cpu_##period##_array_length, period)                     \
  }
  V(15)
  V(30)
  V(60)
#undef V

  if (log_format_alinode)
    Info(
        "other",
        "cpu_usage(%%) now: %.2lf, cpu_15: %.2lf, cpu_30: %.2lf, cpu_60: %.2lf",
        cpu_now, cpu_15_average, cpu_30_average, cpu_60_average);
  else
    Info("cpu",
         "cpu_usage(%%) cpu_now: %lf, cpu_15: %lf, cpu_30: %lf, cpu_60: %lf",
         cpu_now, cpu_15_average, cpu_30_average, cpu_60_average);
}
} // namespace xprofiler
