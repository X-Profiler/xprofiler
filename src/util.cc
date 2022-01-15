#include "util.h"

#include <cstdio>
#include <cstdlib>

namespace xprofiler {
[[noreturn]] void Abort() {
  std::fflush(stderr);
  std::abort();
}

[[noreturn]] void Assert(const AssertionInfo& info) {
  fprintf(stderr, "xprofiler: %s:%s%s Assertion `%s' failed.\n", info.file_line,
          info.function, *info.function ? ":" : "", info.message);
  fflush(stderr);

  Abort();
}

}  // namespace xprofiler
