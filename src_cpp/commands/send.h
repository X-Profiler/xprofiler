#ifndef XPROFILER_SRC_COMMANDS_SEND_H
#define XPROFILER_SRC_COMMANDS_SEND_H

#include "library/json.hpp"

namespace xprofiler {
using nlohmann::json;

void ErrorValue(std::string traceid, std::string message);
void SuccessValue(std::string traceid, json data);
}  // namespace xprofiler

#endif /* XPROFILER_SRC_COMMANDS_SEND_H */
