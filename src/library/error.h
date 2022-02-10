#ifndef XPROFILER_SRC_LIBRARY_ERROR_H
#define XPROFILER_SRC_LIBRARY_ERROR_H

#include <string>

namespace xprofiler {
// common error
class XpfError {
 public:
  XpfError() : failed_(false), msg_("") {}
  XpfError(bool failed, std::string msg) : failed_(failed), msg_(msg) {}
  static XpfError Failure(const char *format, ...);
  static XpfError Succeed() { return XpfError(); };

  inline const char *GetErrMessage() { return msg_.c_str(); }
  inline bool Success() const { return !Fail(); }
  inline bool Fail() const { return failed_; }

 private:
  bool failed_;
  std::string msg_;
  static const size_t kMaxMessageLength = 256;
};
}  // namespace xprofiler

#endif /* XPROFILER_SRC_LIBRARY_ERROR_H */
