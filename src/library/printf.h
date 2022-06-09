#ifndef XPROFILER_SRC_LIBRARY_PRINT_H
#define XPROFILER_SRC_LIBRARY_PRINT_H
#include <string>

namespace xprofiler {
template <typename... Args>
inline std::string SPrintF(const char* format, Args&&... args);
}  // namespace xprofiler
#endif