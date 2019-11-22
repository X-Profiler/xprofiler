#ifndef _SRC_COMMANDS_HEAPDUMP_HEAP_PROFILER_H
#define _SRC_COMMANDS_HEAPDUMP_HEAP_PROFILER_H

#include "nan.h"
#include "v8-profiler.h"

namespace xprofiler {
using std::string;

class HeapProfiler {
 public:
  HeapProfiler();
  virtual ~HeapProfiler();
  static void TakeSnapshot(string filename);
};
}  // namespace xprofiler

#endif  // NODE_HEAP_PROFILER_H
