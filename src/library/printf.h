#pragma once

#include <string>

namespace xprofiler {
template <typename... Args>
inline std::string SPrintF(const char* format, Args&&... args);
}  // namespace xprofiler
