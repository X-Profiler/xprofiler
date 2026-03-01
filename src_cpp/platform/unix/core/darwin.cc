#ifdef __APPLE__
#include <fstream>

namespace xprofiler {
using std::endl;
using std::ios;
using std::ofstream;

void WriteCore(std::string filename) {
  ofstream out;
  out.open(filename, ios::out | ios::binary);
  out << "Generator core file is not supported on darwin now." << endl;
  out.close();
}
}  // namespace xprofiler

#endif