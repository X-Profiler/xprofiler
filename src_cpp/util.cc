#include "util.h"

#include <cstdio>
#include <cstdlib>

#include "hooks/fatal_error.h"
#include "platform/platform.h"

namespace xprofiler {
[[noreturn]] void Abort() {
  std::fflush(stderr);
  std::abort();
}

[[noreturn]] void Assert(const AssertionInfo& info) {
  std::string location =
      std::string(info.file_line) + ":" + std::string(info.function);
  std::string message =
      "Assertion `" + std::string(info.message) + "' failed.\n";
  OnFatalError(location.c_str(), message.c_str());
}

}  // namespace xprofiler
