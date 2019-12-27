#include "../logger.h"
#include "../platform/platform.h"

namespace xprofiler {
#define INIT_CPU_PERIOD(period)                     \
  static double *cpu_##period = new double[period]; \
  static int cpu_##period##_array_index = 0;        \
  static int cpu_##period##_array_length = period;

#define SET_CPU_USAGE(period)                                   \
  bool cpu_##period##_array_not_full =                          \
      cpu_##period##_array_index < cpu_##period##_array_length; \
  if (cpu_##period##_array_not_full) {                          \
    cpu_##period[cpu_##period##_array_index++] = cpu_now;       \
  } else {                                                      \
    cpu_##period##_array_index = 0;                             \
    cpu_##period[cpu_##period##_array_index++] = cpu_now;       \
  }

#define CALAULATE_AVERAGE(total_length, period) \
  if (total_length == 0) return;                \
  for (int i = 0; i < total_length; i++) {      \
    cpu_##period##_average += cpu_##period[i];  \
  }                                             \
  cpu_##period##_average = cpu_##period##_average / total_length;

#define CALAULATE_CPU_USAGE_IN_PERIOD(period)                   \
  bool cpu_##period##_array_not_full =                          \
      cpu_##period##_array_index < cpu_##period##_array_length; \
  if (cpu_##period##_array_not_full) {                          \
    CALAULATE_AVERAGE(cpu_##period##_array_index, period)       \
  } else {                                                      \
    CALAULATE_AVERAGE(cpu_##period##_array_length, period)      \
  }

// init cpu now
double cpu_now = 0.0;

// init cpu 15/30/60
INIT_CPU_PERIOD(15)
INIT_CPU_PERIOD(30)
INIT_CPU_PERIOD(60)

void SetNowCpuUsage() {
  double cpu_now_ = GetNowCpuUsage();
  if (cpu_now_ < 0) {
    return;
  }
  cpu_now = cpu_now_;

  SET_CPU_USAGE(15)
  SET_CPU_USAGE(30)
  SET_CPU_USAGE(60)
}

void WriteCpuUsageInPeriod(bool log_format_alinode) {
  double cpu_15_average = 0.0, cpu_30_average = 0.0, cpu_60_average = 0.0;

  CALAULATE_CPU_USAGE_IN_PERIOD(15)
  CALAULATE_CPU_USAGE_IN_PERIOD(30)
  CALAULATE_CPU_USAGE_IN_PERIOD(60)

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
}  // namespace xprofiler
