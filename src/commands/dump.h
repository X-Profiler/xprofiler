#ifndef _SRC_COMMANDS_DUMP_H
#define _SRC_COMMANDS_DUMP_H

#include "../library/common.h"
#include "../library/utils.h"
#include "unordered_map"
#include "vector"

namespace xprofiler {
using std::unordered_map;
using std::vector;

enum DumpAction { START_CPU_PROFILING, STOP_CPU_PROFILING, HEAPDUMP };

typedef unordered_map<int, bool> ActionMap;
typedef unordered_map<string, bool> RequestMap;
typedef unordered_map<int, vector<DumpAction>> ConflictMap;
typedef unordered_map<int, DumpAction> DependentMap;

typedef struct BaseDumpData {
  string traceid;
  DumpAction action;
  string filepath;
  int profiling_time;
  bool run_once = true;
} dump_data_t;

typedef struct CpuProfilerDumpData : BaseDumpData {
  string title;
} cpuprofile_dump_data_t;

typedef struct HeapdumpData : BaseDumpData {
} heapdump_data_t;

int InitDumpAction();

void UnrefDumpActionAsyncHandle();

COMMAND_CALLBACK(StartCpuProfiling);
COMMAND_CALLBACK(StopCpuProfiling);
COMMAND_CALLBACK(Heapdump);
}  // namespace xprofiler

#endif