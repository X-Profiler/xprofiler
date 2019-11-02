#ifdef _WIN32
#include "../platform.h"
#include <windows.h>

namespace xprofiler {
void SleepCrossPlatform(int seconds) { Sleep(seconds * 1000); }

string GetSep() { return "\\"; }

int GetPid() { return getpid(); }

#if (NODE_MODULE_VERSION < 72)
// from libuv: uv/src/unix
int uv_gettimeofday(uv_timeval64_t *tv) {
  const uint64_t epoch = (uint64_t)116444736000000000ULL;
  FILETIME file_time;
  ULARGE_INTEGER ularge;

  if (tv == NULL)
    return UV_EINVAL;

  GetSystemTimeAsFileTime(&file_time);
  ularge.LowPart = file_time.dwLowDateTime;
  ularge.HighPart = file_time.dwHighDateTime;
  tv->tv_sec = (int64_t)((ularge.QuadPart - epoch) / 10000000L);
  tv->tv_usec = (int32_t)(((ularge.QuadPart - epoch) % 10000000L) / 10);
  return 0;
}
#endif
} // namespace xprofiler

#endif