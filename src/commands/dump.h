#ifndef XPROFILER_SRC_COMMANDS_DUMP_H
#define XPROFILER_SRC_COMMANDS_DUMP_H

#include "library/common.h"
#include "library/utils.h"
#include "unordered_map"
#include "vector"

namespace xprofiler {

enum DumpAction {
  START_CPU_PROFILING,
  STOP_CPU_PROFILING,
  HEAPDUMP,
  START_SAMPLING_HEAP_PROFILING,
  STOP_SAMPLING_HEAP_PROFILING,
  START_GC_PROFILING,
  STOP_GC_PROFILING,
  NODE_REPORT
};

using ActionMap = std::unordered_map<int, bool>;
using RequestMap = std::unordered_map<std::string, bool>;
using ConflictMap = std::unordered_map<int, std::vector<DumpAction>>;
using DependentMap = std::unordered_map<int, DumpAction>;

struct BaseDumpData {
  std::string traceid;
  DumpAction action;
  int profiling_time;
  bool run_once = true;
};

struct CpuProfilerDumpData : BaseDumpData {
  std::string title = "xprofiler";
};

struct HeapdumpDumpData : BaseDumpData {};

struct SamplingHeapProfilerDumpData : BaseDumpData {};

struct GcProfilerDumpData : BaseDumpData {};

struct NodeReportDumpData : BaseDumpData {};

COMMAND_CALLBACK(StartCpuProfiling);
COMMAND_CALLBACK(StopCpuProfiling);
COMMAND_CALLBACK(Heapdump);
COMMAND_CALLBACK(StartSamplingHeapProfiling);
COMMAND_CALLBACK(StopSamplingHeapProfiling);
COMMAND_CALLBACK(StartGcProfiling);
COMMAND_CALLBACK(StopGcProfiling);
COMMAND_CALLBACK(GetNodeReport);
}  // namespace xprofiler

#endif /* XPROFILER_SRC_COMMANDS_DUMP_H */
