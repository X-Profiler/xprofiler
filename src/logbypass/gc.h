#ifndef _SRC_LOGBYPASS_GC_H
#define _SRC_LOGBYPASS_GC_H

#include "nan.h"

namespace xprofiler {
typedef struct {
 public:
  // total gc times
  unsigned int total_gc_times = 0;
  // total gc duration
  unsigned long total_gc_duration = 0;
  unsigned long total_scavange_duration = 0;
  unsigned long total_marksweep_duration = 0;
  unsigned long total_incremental_marking_duration = 0;
  // last record
  unsigned long gc_time_during_last_record = 0;
  unsigned long scavange_duration_last_record = 0;
  unsigned long marksweep_duration_last_record = 0;
  unsigned long incremental_marking_duration_last_record = 0;

  // record start
  uint64_t &start() { return start_; }

  // reset last record
  void reset() {
    start_ = 0;
    gc_time_during_last_record = 0;
    scavange_duration_last_record = 0;
    marksweep_duration_last_record = 0;
    incremental_marking_duration_last_record = 0;
  }

 private:
  uint64_t start_ = 0;
} gc_statistics_t;

int InitGcStatusHooks();
void WriteGcStatusToLog(bool log_format_alinode);
}  // namespace xprofiler

#endif