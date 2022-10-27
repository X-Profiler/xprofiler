#include "library/json.hpp"
#include "platform/platform.h"

namespace xprofiler {
using nlohmann::json;
using std::string;

#define SEND_VALUE(ok, res)    \
  result["ok"] = ok;           \
  result["traceid"] = traceid; \
  result[#res] = res;          \
  CreateIpcClient(const_cast<char*>(result.dump().c_str()));

void ErrorValue(string traceid, string message) {
  json result;
  SEND_VALUE(false, message);
}

void SuccessValue(string traceid, json data) {
  json result;
  SEND_VALUE(true, data);
}
}  // namespace xprofiler