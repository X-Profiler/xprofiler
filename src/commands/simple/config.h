#ifndef _SRC_COMMANDS_SIMPLE_CONFIG_H
#define _SRC_COMMANDS_SIMPLE_CONFIG_H

#include "../../common.h"
#include "../../library/json.hpp"

namespace xprofiler {
using nlohmann::json;

COMMAND_CALLBACK(GetXprofilerConfig);
} // namespace xprofiler

#endif