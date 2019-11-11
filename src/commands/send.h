#ifndef _SRC_COMMANDS_SEND_H
#define _SRC_COMMANDS_SEND_H

#include "../library/json.hpp"

namespace xprofiler {
using nlohmann::json;

void ErrorValue(const char *format, ...);
void SuccessValue(json data);
} // namespace xprofiler

#endif