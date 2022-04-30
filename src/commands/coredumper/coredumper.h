#ifndef XPROFILER_SRC_COMMANDS_COREDUMPER_H
#define XPROFILER_SRC_COMMANDS_COREDUMOER_H

#include <string>

namespace xprofiler {
class Coredumper {
 public:
  static void WriteCoredump(std::string filename);
};
}  // namespace xprofiler

#endif /* XPROFILER_SRC_COMMANDS_COREDUMOER_H */