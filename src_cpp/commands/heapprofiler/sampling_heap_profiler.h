#ifndef XPROFILER_SRC_COMMANDS_HEAPPROFILER_SAMPLING_HEAP_PROFILER_H
#define XPROFILER_SRC_COMMANDS_HEAPPROFILER_SAMPLING_HEAP_PROFILER_H

#include "nan.h"
#include "v8-profiler.h"

namespace xprofiler {
class SamplingHeapProfiler final {
 public:
  static void StartSamplingHeapProfiling(v8::Isolate* isolate);
  static void StopSamplingHeapProfiling(v8::Isolate* isolate,
                                        std::string filename);
};

}  // namespace xprofiler
#endif /* XPROFILER_SRC_COMMANDS_HEAPPROFILER_SAMPLING_HEAP_PROFILER_H */
