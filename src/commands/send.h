#pragma once

#include "library/json.hpp"

namespace xprofiler {
using nlohmann::json;

void ErrorValue(std::string traceid, std::string message);
void SuccessValue(std::string traceid, json data);
}  // namespace xprofiler
