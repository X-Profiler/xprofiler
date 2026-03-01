#ifndef XPROFILER_SRC_COMMANDS_CPUPROFILER_CPU_PROFILER_H
#define XPROFILER_SRC_COMMANDS_CPUPROFILER_CPU_PROFILER_H

#include "node.h"
#include "util.h"
#include "v8-profiler.h"

namespace xprofiler {

class CpuProfiler final {
  static void DeleteCpuProfiler(v8::CpuProfiler* profiler);

  using CpuProfilerPtr = DeleteFnPtr<v8::CpuProfiler, DeleteCpuProfiler>;

 public:
  static void StartProfiling(v8::Isolate* isolate, std::string title);
  static void StopProfiling(v8::Isolate* isolate, std::string title,
                            std::string filename);
  ~CpuProfiler();

  int started_profiles_count() { return started_profiles_count_; }

 private:
  CpuProfiler(v8::Isolate* isolate);

  void StartProfiling(std::string title);
  void StopProfiling(std::string title, std::string filename);

  void SetSamplingInterval(uint32_t sample);

  v8::Isolate* isolate_;
  int started_profiles_count_ = 0;
  CpuProfilerPtr cpu_profiler_ = nullptr;
};
}  // namespace xprofiler

#endif /* XPROFILER_SRC_COMMANDS_CPUPROFILER_CPU_PROFILER_H */
