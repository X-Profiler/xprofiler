#ifndef _SRC_COMMON_H
#define _SRC_COMMON_H

namespace xprofiler {
// xprofiler logger
enum LOG_LEVEL { LOG_INFO, LOG_ERROR, LOG_DEBUG };
enum LOG_TYPE { LOG_TO_FILE, LOG_TO_TTL };

// global variables
void InitGlobalVariables();
// uptime
unsigned long GetUptime();
} // namespace xprofiler

#endif