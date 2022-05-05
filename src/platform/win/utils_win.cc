#ifdef _WIN32
#include <windows.h>

#include "../platform.h"
#include "uv.h"

namespace xprofiler {
void SleepCrossPlatform(int seconds) { Sleep(seconds * 1000); }

std::string GetSep() { return "\\"; }

int GetPid() { return getpid(); }
}  // namespace xprofiler

#endif
