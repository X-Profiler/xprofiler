#include "platform/platform.h"

namespace xprofiler {
void Sleep(int seconds) { SleepCrossPlatform(seconds); }
}; // namespace xprofiler
