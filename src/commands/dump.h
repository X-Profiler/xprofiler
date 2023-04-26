#pragma once

#include <unordered_map>
#include <vector>

#include "commands/parser.h"
#include "library/common.h"

namespace xprofiler {

enum DumpAction {
  START_CPU_PROFILING,
  STOP_CPU_PROFILING,
  HEAPDUMP,
  START_SAMPLING_HEAP_PROFILING,
  STOP_SAMPLING_HEAP_PROFILING,
  START_GC_PROFILING,
  STOP_GC_PROFILING,
  NODE_REPORT,
  COREDUMP,
};

using ActionMap = std::unordered_map<DumpAction, bool>;
using ConflictMap = std::unordered_map<DumpAction, std::vector<DumpAction>>;
using DependentMap = std::unordered_map<DumpAction, DumpAction>;

struct DumpData {
  std::string traceid;
  DumpAction action;
  ThreadId thread_id;
  int profiling_time;
};

void FinishSampling(v8::Isolate* isolate, const char* reason);

COMMAND_CALLBACK(StartCpuProfiling);
COMMAND_CALLBACK(StopCpuProfiling);
COMMAND_CALLBACK(Heapdump);
COMMAND_CALLBACK(StartSamplingHeapProfiling);
COMMAND_CALLBACK(StopSamplingHeapProfiling);
COMMAND_CALLBACK(StartGcProfiling);
COMMAND_CALLBACK(StopGcProfiling);
COMMAND_CALLBACK(GetNodeReport);
COMMAND_CALLBACK(GenerateCoredump);
}  // namespace xprofiler
