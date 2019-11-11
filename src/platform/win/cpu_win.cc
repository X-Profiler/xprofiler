#ifdef _WIN32
#include "uv.h"
#include "windows.h"

namespace xprofiler {
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
  GET_CURRENT_TIME(lgCurTime)
  double cpu_now_ = ((lgCurTime.QuadPart - g_slgProcessTimeOld.QuadPart) * 100 /
                     ((uv_hrtime() - last_time) / 10e5));

  // update time & cpu usage
  g_slgProcessTimeOld = lgCurTime;
  last_time = uv_hrtime();

  return cpu_now_;
}
} // namespace xprofiler
#endif