#pragma once

#include <string>

#include "v8.h"

namespace xprofiler {
constexpr uint64_t kNanosecondsPerSecond = 1e9;

void InitOnceLoadTime();

// uptime
unsigned long GetUptime();
std::string GetStartTime(std::string format);
size_t GetNextDiagFileId();
std::string GetGlobalNodeVersion(v8::Isolate* isolate);

/**
 * Update the type when we can get integer thread_id from Node.js
 */
using ThreadId = double;

}  // namespace xprofiler
