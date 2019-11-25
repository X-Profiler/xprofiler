#ifndef _SRC_COMMANDS_GCPROFILER_GC_PROFILER_H
#define _SRC_COMMANDS_GCPROFILER_GC_PROFILER_H

#include <string>

namespace xprofiler {
using std::string;

class GcProfiler {
 public:
  GcProfiler();
  virtual ~GcProfiler();
  static void StartGCProfiling(string filename);
  static void StopGCProfiling();
};

}  // namespace xprofiler
#endif
