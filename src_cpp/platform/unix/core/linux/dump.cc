#ifdef __linux__
#include <string>

#include "coredumper.h"

namespace xprofiler {

void WriteCore(std::string filename) { WriteCoreDump(filename.c_str()); }
}  // namespace xprofiler

#endif