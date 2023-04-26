#pragma once

#include <string>

namespace xprofiler {
class Coredumper {
 public:
  static void WriteCoredump(std::string filename);
};
}  // namespace xprofiler
