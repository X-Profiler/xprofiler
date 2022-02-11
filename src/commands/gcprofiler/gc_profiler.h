#ifndef XPROFILER_SRC_COMMANDS_GCPROFILER_GC_PROFILER_H
#define XPROFILER_SRC_COMMANDS_GCPROFILER_GC_PROFILER_H

#include <string>

#include "library/writer.h"
#include "nan.h"

namespace xprofiler {

class GcProfiler {
 public:
  static void StartGCProfiling(v8::Isolate* isolate, std::string filename);
  static void StopGCProfiling(v8::Isolate* isolate);

  ~GcProfiler();

  bool is_open() { return outfile_.is_open(); }
  JSONWriter* writer() { return &writer_; }
  uint64_t init() { return init_; }

 private:
  GcProfiler(v8::Isolate* isolate, std::string filename);

  std::string filename_ = "";
  std::ofstream outfile_;
  JSONWriter writer_;
  uint64_t init_ = 0;
};

}  // namespace xprofiler

#endif /* XPROFILER_SRC_COMMANDS_GCPROFILER_GC_PROFILER_H */
