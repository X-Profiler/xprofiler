#include "../logger.h"
#include "../platform/platform.h"

namespace xprofiler {
#define EXTRA_SYMBOL

#define CONCAT_SYMBOL(V) V EXTRA_SYMBOL

#define PERIOD_LIST(V)  \
  CONCAT_SYMBOL(V(15))  \
  CONCAT_SYMBOL(V(30))  \
  CONCAT_SYMBOL(V(60))  \
  CONCAT_SYMBOL(V(180)) \
  CONCAT_SYMBOL(V(300)) \
  V(600)

#define INIT_CPU_PERIOD(period)                     \
  static double* cpu_##period = new double[period]; \
  static int cpu_##period##_array_index = 0;        \
  static int cpu_##period##_array_length = period;  \
  static int cpu_##period##_array_not_full = true;

#define INIT_CPU_AVERAGE(period) double cpu_##period##_average = 0.0;

#define ALINODE_LOG_KEY(period) "cpu_" #period ": %lf"

#define XPROFILER_LOG_KEY(period) "cpu_" #period ": %lf"

#define CPU_AVERAGE_VAL(period) cpu_##period##_average

#define SET_CPU_USAGE(period)                                     \
  if (cpu_##period##_array_index < cpu_##period##_array_length) { \
    cpu_##period[cpu_##period##_array_index++] = cpu_now;         \
  } else {                                                        \
    cpu_##period##_array_index = 0;                               \
    cpu_##period##_array_not_full = false;                        \
    cpu_##period[cpu_##period##_array_index++] = cpu_now;         \
  }

#define CALAULATE_AVERAGE(total_length, period) \
  for (int i = 0; i < total_length; i++) {      \
    cpu_##period##_average += cpu_##period[i];  \
  }                                             \
  if (total_length != 0)                        \
    cpu_##period##_average = cpu_##period##_average / total_length;

#define CALAULATE_CPU_USAGE_IN_PERIOD(period)              \
  if (cpu_##period##_array_not_full) {                     \
    CALAULATE_AVERAGE(cpu_##period##_array_index, period)  \
  } else {                                                 \
    CALAULATE_AVERAGE(cpu_##period##_array_length, period) \
  }

// init cpu now
double cpu_now = 0.0;

// init cpu period
PERIOD_LIST(INIT_CPU_PERIOD)

void SetNowCpuUsage() {
  double cpu_now_ = GetNowCpuUsage();
  if (cpu_now_ < 0) {
    return;
  }
  cpu_now = cpu_now_;

  PERIOD_LIST(SET_CPU_USAGE)
}

void WriteCpuUsageInPeriod(bool log_format_alinode) {
  PERIOD_LIST(INIT_CPU_AVERAGE)

  PERIOD_LIST(CALAULATE_CPU_USAGE_IN_PERIOD)

  if (log_format_alinode)
    Info("other",
#undef EXTRA_SYMBOL
#define EXTRA_SYMBOL ", "
         "cpu_usage(%%) now: %lf, " PERIOD_LIST(ALINODE_LOG_KEY),
#undef EXTRA_SYMBOL
#define EXTRA_SYMBOL ,
         cpu_now, PERIOD_LIST(CPU_AVERAGE_VAL));
  else
    Info("cpu",
#undef EXTRA_SYMBOL
#define EXTRA_SYMBOL ", "
         "cpu_usage(%%) cpu_now: %lf, " PERIOD_LIST(XPROFILER_LOG_KEY),
#undef EXTRA_SYMBOL
#define EXTRA_SYMBOL ,
         cpu_now, PERIOD_LIST(CPU_AVERAGE_VAL));
#undef EXTRA_SYMBOL
#define EXTRA_SYMBOL
}
}  // namespace xprofiler
