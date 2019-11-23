#ifndef _SRC_COMMANDS_HEAPDUMP_HEAP_SNAPSHOT_H
#define _SRC_COMMANDS_HEAPDUMP_HEAP_SNAPSHOT_H

#include "v8-profiler.h"

namespace xprofiler {
using std::string;
using v8::HeapSnapshot;

class Snapshot {
 public:
  static void Serialize(const HeapSnapshot *profile, string filename);
};
}  // namespace xprofiler
#endif  // NODE_SNAPSHOT_
