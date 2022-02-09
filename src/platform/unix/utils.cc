#if defined(__APPLE__) || defined(__linux__)
#include <sys/time.h>
#include <unistd.h>

#include "../platform.h"
#include "uv.h"

namespace xprofiler {
void SleepCrossPlatform(int seconds) { sleep(seconds); }

string GetSep() { return "/"; }

int GetPid() { return getpid(); }

#if (NODE_MODULE_VERSION < NODE_12_0_MODULE_VERSION)
// from libuv: uv/src/unix
int uv_gettimeofday(uv_timeval64_t *tv) {
  struct timeval time;

  if (tv == NULL) return UV_EINVAL;

  if (gettimeofday(&time, NULL) != 0) return -1;

  tv->tv_sec = (int64_t)time.tv_sec;
  tv->tv_usec = (int32_t)time.tv_usec;
  return 0;
}
#endif
}  // namespace xprofiler

#endif
