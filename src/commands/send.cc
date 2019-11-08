#include "../library/json.hpp"

#include "../platform/platform.h"

namespace xprofiler {
using nlohmann::json;

void SendMessageToAgent(char *message) {
  json result;
  result["ok"] = true;
  result["data"] = message;
  CreateIpcClient(const_cast<char *>(result.dump().c_str()));
}
} // namespace xprofiler