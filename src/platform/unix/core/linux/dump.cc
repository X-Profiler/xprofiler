#ifdef __linux__
#include <fstream>

namespace xprofiler {
using std::ios;
using std::ofstream;

int WriteCore(std::string filename) {
  ofstream out;
  out.open(filename, ios::out | ios::binary);
  out << "Generator core file is not supported on linux now.";
  out.close();
  return 0;
}
}  // namespace xprofiler

#endif