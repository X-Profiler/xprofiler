#ifndef _SRC_LIBRARY_ERROR_H
#define _SRC_LIBRARY_ERROR_H

#include <string>

namespace xprofiler {
using std::string;
// common error
class XpfError {
 public:
  XpfError() : failed_(false), msg_("") {}
  XpfError(bool failed, string msg) : failed_(failed), msg_(msg) {}
  static XpfError Failure(const char *format, ...);
  static XpfError Succeed() { return XpfError(); };

  inline const char *GetErrMessage() { return msg_.c_str(); }
  inline bool Success() const { return !Fail(); }
  inline bool Fail() const { return failed_; }

 private:
  bool failed_;
  string msg_;
  static const size_t kMaxMessageLength = 256;
};
}  // namespace xprofiler

#endif