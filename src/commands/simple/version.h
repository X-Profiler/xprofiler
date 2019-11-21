#ifndef _SRC_COMMANDS_SIMPLE_VERSION_H
#define _SRC_COMMANDS_SIMPLE_VERSION_H

#include "../../library/common.h"
#include "../../library/json.hpp"

namespace xprofiler {
using nlohmann::json;

COMMAND_CALLBACK(GetXprofilerVersion);
}  // namespace xprofiler

#endif