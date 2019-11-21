#ifndef _SRC_COMMANDS_CPUPROFILER_CPU_PROFILER_H
#define _SRC_COMMANDS_CPUPROFILER_CPU_PROFILER_H

#include "node.h"
#include "v8-profiler.h"

namespace xprofiler {
using std::string;

class Profiler {
 public:
  Profiler();
  virtual ~Profiler();
  static void StartProfiling(string title);
  static void StopProfiling(string title, string filename);
  static void SetSamplingInterval(uint32_t sample);
};
}  // namespace xprofiler

#endif
