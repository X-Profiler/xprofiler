#include "gc.h"

#include "../library/common.h"
#include "../logger.h"

namespace xprofiler {
using Nan::AddGCEpilogueCallback;
using Nan::AddGCPrologueCallback;
using v8::GCType;

static gc_statistics_t *gc_statistics = new gc_statistics_t;
static uv_mutex_t gc_mutex;

unsigned int TotalGcTimes() {
  if (gc_statistics == nullptr) {
    return 0;
  }
  return gc_statistics->total_gc_times;
}

unsigned int TotalGcDuration() {
  if (gc_statistics == nullptr) {
    return 0;
  }
  return gc_statistics->total_gc_duration;
}

// gc prologue hook
NAN_GC_CALLBACK(GCPrologueCallback) {
  uv_mutex_lock(&gc_mutex);
  gc_statistics->start() = uv_hrtime();
  uv_mutex_unlock(&gc_mutex);
}

// gc epilogue hook
NAN_GC_CALLBACK(GCEpilogueCallback) {
  if (gc_statistics->start() == 0) {
    return;
  }
  uv_mutex_lock(&gc_mutex);
  gc_statistics->total_gc_times++;
  unsigned int duration =
      (uv_hrtime() - gc_statistics->start()) / 10e5;  // cost, ms
  gc_statistics->total_gc_duration += duration;
  gc_statistics->gc_time_during_last_record += duration;

  if (type == GCType::kGCTypeScavenge) {
    gc_statistics->total_scavange_duration += duration;
    gc_statistics->scavange_duration_last_record += duration;
  }

  if (type == GCType::kGCTypeMarkSweepCompact) {
    gc_statistics->total_marksweep_duration += duration;
    gc_statistics->marksweep_duration_last_record += duration;
  }

  if (type == GCType::kGCTypeIncrementalMarking) {
    gc_statistics->total_incremental_marking_duration += duration;
    gc_statistics->incremental_marking_duration_last_record += duration;
  }
  uv_mutex_unlock(&gc_mutex);
}

int InitGcStatusHooks() {
  int rc = uv_mutex_init(&gc_mutex);
  AddGCPrologueCallback(GCPrologueCallback);
  AddGCEpilogueCallback(GCEpilogueCallback);
  return rc;
}

void WriteGcStatusToLog(bool log_format_alinode) {
  // record gc status
  uv_mutex_lock(&gc_mutex);
  if (log_format_alinode)
    Info("gc",
         "gc_time_during_last_min: %lu, total: %lu, scavange_duration: %lu, "
         "marksweep_duration: %lu",
         gc_statistics->gc_time_during_last_record,
         gc_statistics->total_gc_duration,
         gc_statistics->scavange_duration_last_record,
         gc_statistics->marksweep_duration_last_record);
  else
    Info("gc",
         "uptime: %lu, "
         "total_gc_times: %u, "
         "total_gc_duration: %lu, "
         "total_scavange_duration: %lu, "
         "total_marksweep_duration: %lu, "
         "total_incremental_marking_duration: %lu, "
         "gc_time_during_last_record: %lu, "
         "scavange_duration_last_record: %lu, "
         "marksweep_duration_last_record: %lu, "
         "incremental_marking_duration_last_record: %lu",
         GetUptime(),  // uptime, s
         // total
         gc_statistics->total_gc_times, gc_statistics->total_gc_duration,
         gc_statistics->total_scavange_duration,
         gc_statistics->total_marksweep_duration,
         gc_statistics->total_incremental_marking_duration,
         // last record
         gc_statistics->gc_time_during_last_record,
         gc_statistics->scavange_duration_last_record,
         gc_statistics->marksweep_duration_last_record,
         gc_statistics->incremental_marking_duration_last_record);
  // reset last record
  gc_statistics->reset();
  uv_mutex_unlock(&gc_mutex);
}
}  // namespace xprofiler