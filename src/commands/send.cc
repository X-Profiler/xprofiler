#include <stdarg.h>

#include "../platform/platform.h"
#include "send.h"

namespace xprofiler {

void ErrorValue(const char *format, ...) {
  json result;
  result["ok"] = false;

  // compose error message
  char message[1024];
  va_list args;
  va_start(args, format);
  vsnprintf(message, sizeof(message), format, args);
  va_end(args);

  result["message"] = message;
  CreateIpcClient(const_cast<char *>(result.dump().c_str()));
}

void SuccessValue(json data) {
  json result;
  result["ok"] = true;
  result["data"] = data;
  CreateIpcClient(const_cast<char *>(result.dump().c_str()));
}
} // namespace xprofiler