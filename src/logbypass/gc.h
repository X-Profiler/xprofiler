#pragma once

#include "nan.h"
#include "xpf_mutex-inl.h"

namespace xprofiler {
class EnvironmentData;

class GcStatistics {
 public:
  // total gc times
  uint32_t total_gc_times = 0;
  // total gc duration
  uint32_t total_gc_duration = 0;
  uint32_t total_scavange_duration = 0;
  uint32_t total_marksweep_duration = 0;
  uint32_t total_incremental_marking_duration = 0;
  // last record
  uint32_t gc_time_during_last_record = 0;
  uint32_t scavange_duration_last_record = 0;
  uint32_t marksweep_duration_last_record = 0;
  uint32_t incremental_marking_duration_last_record = 0;
  uint64_t start = 0;

  Mutex mutex;

  // reset last record
  void reset() {
    start = 0;
    gc_time_during_last_record = 0;
    scavange_duration_last_record = 0;
    marksweep_duration_last_record = 0;
    incremental_marking_duration_last_record = 0;
  }
};

void InitGcStatusHooks(EnvironmentData* env_data);
void WriteGcStatusToLog(EnvironmentData* env_data, bool log_format_alinode);

uint32_t TotalGcTimes();
uint32_t TotalGcDuration();
}  // namespace xprofiler
