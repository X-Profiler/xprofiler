#ifndef _WIN32
#include <unistd.h>
#else
#include <windows.h>
#endif

#include "utils.h"

namespace xprofiler {
void SleepSeconds(int seconds) {
#ifndef _WIN32
  sleep(seconds);
#else
  Sleep(seconds * 1000);
#endif
}
}; // namespace xprofiler
