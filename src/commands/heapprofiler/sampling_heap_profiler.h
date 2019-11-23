#ifndef _SRC_COMMANDS_HEAPPROFILER_SAMPLING_HEAP_PROFILER_H
#define _SRC_COMMANDS_HEAPPROFILER_SAMPLING_HEAP_PROFILER_H

#include "nan.h"
#include "v8-profiler.h"

namespace xprofiler {
using std::string;

class SamplingHeapProfile {
 public:
  SamplingHeapProfile();
  virtual ~SamplingHeapProfile();
  static void StartSamplingHeapProfiling();
  static void StopSamplingHeapProfiling(string filename);
};

}  // namespace xprofiler
#endif
