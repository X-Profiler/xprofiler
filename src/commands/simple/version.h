#ifndef _SRC_COMMANDS_SIMPLE_H
#define _SRC_COMMANDS_SIMPLE_H

#include "../../common.h"
#include "../../library/json.hpp"

namespace xprofiler {
using nlohmann::json;

void GetXprofilerVersion(json command, cb_success *success, cb_error *error);
} // namespace xprofiler

#endif