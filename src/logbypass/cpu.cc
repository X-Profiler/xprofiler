#include "cpu.h"
#include "../logger.h"
#include "uv.h"

#define INIT_CPU_PERIOD(period)                                                \
  static double *cpu_##period = new double[period];                            \
  static int cpu_##period##_array_index = 0;                                   \
  static int cpu_##period##_array_length = period;

#define SET_CPU_PERIOD(period)                                                 \
  bool cpu_##period##_array_not_full =                                         \
      cpu_##period##_array_index < cpu_##period##_array_length;                \
  if (cpu_##period##_array_not_full) {                                         \
    cpu_##period[cpu_##period##_array_index++] = cpu_now;                      \
  } else {                                                                     \
    cpu_##period##_array_index = 0;                                            \
    cpu_##period[cpu_##period##_array_index++] = cpu_now;                      \
  }

#define CALAULATE_AVERAGE(total_length, period)                                \
  if (total_length == 0)                                                       \
    return;                                                                    \
  for (int i = 0; i < total_length; i++) {                                     \
    cpu_##period##_average += cpu_##period[i];                                 \
  }                                                                            \
  cpu_##period##_average = cpu_##period##_average / total_length;

#define GET_CPU_PERIOD(period)                                                 \
  bool cpu_##period##_array_not_full =                                         \
      cpu_##period##_array_index < cpu_##period##_array_length;                \
  if (cpu_##period##_array_not_full) {                                         \
    CALAULATE_AVERAGE(cpu_##period##_array_index, period);                     \
  } else {                                                                     \
    CALAULATE_AVERAGE(cpu_##period##_array_length, period);                    \
  }

namespace xprofiler {
// init cpu now
double cpu_now = 0.0;
// init cpu 15/30/60
INIT_CPU_PERIOD(15);
INIT_CPU_PERIOD(30);
INIT_CPU_PERIOD(60);

#ifndef _WIN32
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
#else
#define GET_CURRENT_TIME(curtime)                                              \
  HANDLE hProcess = OpenProcess(PROCESS_QUERY_INFORMATION, FALSE, getpid());   \
  BOOL bRetCode = FALSE;                                                       \
  FILETIME CreateTime, ExitTime, KernelTime, UserTime;                         \
  LARGE_INTEGER lgKernelTime;                                                  \
  LARGE_INTEGER lgUserTime;                                                    \
  LARGE_INTEGER lgCurTimeTmp;                                                  \
  bRetCode = GetProcessTimes(hProcess, &CreateTime, &ExitTime, &KernelTime,    \
                             &UserTime);                                       \
  if (!bRetCode) {                                                             \
    return -1;                                                                 \
  }                                                                            \
  lgKernelTime.HighPart = KernelTime.dwHighDateTime;                           \
  lgKernelTime.LowPart = KernelTime.dwLowDateTime;                             \
  lgUserTime.HighPart = UserTime.dwHighDateTime;                               \
  lgUserTime.LowPart = UserTime.dwLowDateTime;                                 \
  lgCurTimeTmp.QuadPart =                                                      \
      (lgKernelTime.QuadPart + lgUserTime.QuadPart) / 10000;                   \
  curtime = lgCurTimeTmp;

double GetNowCpuUsage() {
  static LARGE_INTEGER g_slgProcessTimeOld = {0};
  static uint64_t last_time = 0;

  // first time
  if (last_time == 0) {
    GET_CURRENT_TIME(g_slgProcessTimeOld);
    last_time = uv_hrtime();
    return -1;
  }

  LARGE_INTEGER lgCurTime = {0};
  GET_CURRENT_TIME(lgCurTime);
  double cpu_now_ = ((lgCurTime.QuadPart - g_slgProcessTimeOld.QuadPart) * 100 /
                     ((uv_hrtime() - last_time) / 10e5));

  // update time & cpu usage
  g_slgProcessTimeOld = lgCurTime;
  last_time = uv_hrtime();

  return cpu_now_;
}
#endif

void SetNowCpuUsage() {
  double cpu_now_ = GetNowCpuUsage();
  if (cpu_now_ < 0) {
    return;
  }
  cpu_now = cpu_now_;
  SET_CPU_PERIOD(15);
  SET_CPU_PERIOD(30);
  SET_CPU_PERIOD(60);
}

void WriteCpuUsageInPeriod(bool log_format_alinode) {
  double cpu_15_average = 0.0, cpu_30_average = 0.0, cpu_60_average = 0.0;
  GET_CPU_PERIOD(15);
  GET_CPU_PERIOD(30);
  GET_CPU_PERIOD(60);

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
