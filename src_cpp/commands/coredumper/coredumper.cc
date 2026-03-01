#include "coredumper.h"

#include "platform/platform.h"

namespace xprofiler {
void Coredumper::WriteCoredump(std::string filename) { WriteCore(filename); }
}  // namespace xprofiler