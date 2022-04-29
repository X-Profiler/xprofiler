#include "coredumper.h"

#include "platform/platform.h"

namespace xprofiler {
int Coredumper::WriteCoredump(std::string filename) {
  return WriteCore(filename);
}
}  // namespace xprofiler