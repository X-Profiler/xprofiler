#if defined(__APPLE__) || defined(__linux__)
#include <sys/time.h>
#include <unistd.h>

#include "platform/platform.h"

namespace xprofiler {
void SleepCrossPlatform(int seconds) { sleep(seconds); }

std::string GetSep() { return "/"; }

int GetPid() { return getpid(); }
}  // namespace xprofiler

#endif
