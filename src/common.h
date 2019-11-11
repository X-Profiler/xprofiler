#ifndef _SRC_COMMON_H
#define _SRC_COMMON_H

#include "library/json.hpp"

namespace xprofiler {
using nlohmann::json;

// xprofiler logger
enum LOG_LEVEL { LOG_INFO, LOG_ERROR, LOG_DEBUG };
enum LOG_TYPE { LOG_TO_FILE, LOG_TO_TTL };

// global variables
void InitGlobalVariables();

// uptime
unsigned long GetUptime();

// commands
#define COMMAND_CALLBACK(cb)                                                   \
  void cb(json command, cb_success *success, cb_error *error)
// message func
typedef void cb_success(json);
typedef void cb_error(const char *, ...);
} // namespace xprofiler

#endif