#ifndef _SRC_COMMON_H
#define _SRC_COMMON_H

#include "library/json.hpp"

namespace xprofiler {
using nlohmann::json;
using std::function;
using std::string;

// xprofiler logger
enum LOG_LEVEL { LOG_INFO, LOG_ERROR, LOG_DEBUG };
enum LOG_TYPE { LOG_TO_FILE, LOG_TO_TTL };

// global variables
void InitGlobalVariables();

// uptime
unsigned long GetUptime();

// commands
#define COMMAND_CALLBACK(cb)                                                   \
  void cb(json command, string (*format)(const char *, ...),                   \
          function<void(json)> success, function<void(string)> error)

// common error
class CommonError {
public:
  CommonError() : failed_(false), msg_("") {}
  CommonError(bool failed, string msg) : failed_(failed), msg_(msg) {}
  CommonError(bool failed, const char *format, ...);
  static CommonError Failure(const char *format, ...);

  inline const char *GetErrorMessage() { return msg_.c_str(); }
  inline bool Success() const { return !Fail(); }
  inline bool Fail() const { return failed_; }

private:
  bool failed_;
  string msg_;
  static const size_t kMaxMessageLength = 256;
};
} // namespace xprofiler

#endif