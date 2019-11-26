#if defined(__APPLE__) || defined(__linux__)

#include <string>

namespace xprofiler {
using std::string;

string GetPcAddress(void* pc) {
  char buf[64];
  snprintf(buf, sizeof(buf), "%p", pc);
  return (string)buf;
}
}  // namespace xprofiler

#endif