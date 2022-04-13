#ifndef XPROFILER_SRC_LIBRARY_COMMON_H
#define XPROFILER_SRC_LIBRARY_COMMON_H

#include <string>

namespace xprofiler {
constexpr uint64_t kNanosecondsPerSecond = 1e9;

void InitOnceLoadTime();

// uptime
unsigned long GetUptime();
std::string GetStartTime(std::string format);
size_t GetNextDiagFileId();

/**
 * Update the type when we can get integer thread_id from Node.js
 */
using ThreadId = double;

}  // namespace xprofiler

#endif /* XPROFILER_SRC_LIBRARY_COMMON_H */
