#ifndef _SRC_COMMANDS_SEND_H
#define _SRC_COMMANDS_SEND_H

#include "../library/json.hpp"

namespace xprofiler {
using nlohmann::json;
using std::string;

void ErrorValue(string traceid, string message);
void SuccessValue(string traceid, json data);
} // namespace xprofiler

#endif