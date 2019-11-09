#include "../library/json.hpp"
#include <stdarg.h>

#include "../platform/platform.h"

namespace xprofiler {
using nlohmann::json;

void SendMessageToAgent(bool ok, const char *format, ...) {
  json result;
  result["ok"] = ok;

  // message
  char message[1024];
  va_list args;
  va_start(args, format);
  vsnprintf(message, sizeof(message), format, args);
  va_end(args);

  if (ok)
    result["data"] = message;
  else
    result["message"] = message;
  CreateIpcClient(const_cast<char *>(result.dump().c_str()));
}
} // namespace xprofiler