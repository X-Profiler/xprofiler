#ifdef _WIN32

#include <string>

namespace xprofiler {
using std::string;

string GetPcAddress(void* pc) {
  char buf[64];
  snprintf(buf, sizeof(buf), "0x%p", pc);
  return (string)buf;
}

}  // namespace xprofiler

#endif