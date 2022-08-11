#include "gc_profiler.h"

#include "environment_data.h"
#include "library/writer.h"
#include "logbypass/gc.h"
#include "logger.h"
#include "nan.h"

namespace xprofiler {
using std::ios;
using std::ofstream;
using v8::GCType;
using v8::HeapSpaceStatistics;

inline void write_space_data(v8::Isolate* isolate, GCType type,
                             JSONWriter* writer, const char* tag) {
  if (type == GCType::kGCTypeScavenge) {
    writer->json_keyvalue("type", "scavenge");
  } else if (type == GCType::kGCTypeMarkSweepCompact) {
    writer->json_keyvalue("type", "marksweep");
  } else if (type == GCType::kGCTypeIncrementalMarking) {
    writer->json_keyvalue("type", "marking");
  } else if (type == GCType::kGCTypeProcessWeakCallbacks) {
    writer->json_keyvalue("type", "weakcallbacks");
  } else {
    writer->json_keyvalue("type", static_cast<int>(type));
  }
  HeapSpaceStatistics s;
  size_t number_of_heap_spaces = isolate->NumberOfHeapSpaces();
  writer->json_arraystart(tag);
  for (size_t i = 0; i < number_of_heap_spaces; i++) {
    writer->json_start();
    isolate->GetHeapSpaceStatistics(&s, i);
    writer->json_keyvalue("name", s.space_name());
    writer->json_keyvalue("space_size", s.space_size());
    writer->json_keyvalue("space_used_size", s.space_used_size());
    writer->json_keyvalue("space_available_size", s.space_available_size());
    writer->json_keyvalue("physical_space_size", s.physical_space_size());
    writer->json_end();
  }
  writer->json_arrayend();
}

NAN_GC_CALLBACK(GCTracerPrologueCallback) {
  EnvironmentData* env_data = EnvironmentData::GetCurrent(isolate);
  if (env_data->gc_profiler == nullptr) {
    return;
  }
  if (env_data->gc_profiler->current_gc_type() != 0) {
    return;
  }
  env_data->gc_profiler->set_current_gc_type(type);
  JSONWriter* writer = env_data->gc_profiler->writer();
  writer->json_start();
  writer->json_keyvalue("totalSpentfromStart", TotalGcDuration());
  writer->json_keyvalue("totalTimesfromStart", TotalGcTimes());
  writer->json_keyvalue("timeFromStart", env_data->GetUptime());
  writer->json_keyvalue("start",
                        (uv_hrtime() - env_data->gc_profiler->init()) / 10e5);
  write_space_data(isolate, type, writer, "before");
}

NAN_GC_CALLBACK(GCTracerEpilogueCallback) {
  EnvironmentData* env_data = EnvironmentData::GetCurrent(isolate);
  if (env_data->gc_profiler == nullptr) {
    return;
  }
  if (env_data->gc_profiler->current_gc_type() != type) {
    return;
  }
  env_data->gc_profiler->set_current_gc_type(0);
  JSONWriter* writer = env_data->gc_profiler->writer();
  writer->json_keyvalue("end",
                        (uv_hrtime() - env_data->gc_profiler->init()) / 10e5);
  write_space_data(isolate, type, writer, "after");
  writer->json_end();
}

void GcProfiler::StartGCProfiling(v8::Isolate* isolate, std::string filename) {
  EnvironmentData* env_data = EnvironmentData::GetCurrent(isolate);
  std::unique_ptr<GcProfiler> gc_profiler =
      std::unique_ptr<GcProfiler>(new GcProfiler(isolate, filename));
  if (!gc_profiler->is_open()) {
    ErrorT("gc_profiler", env_data->thread_id(), "open file %s failed.",
           filename.c_str());
    return;
  }
  env_data->gc_profiler = std::move(gc_profiler);

  env_data->AddGCPrologueCallback(GCTracerPrologueCallback);
  env_data->AddGCEpilogueCallback(GCTracerEpilogueCallback);

  JSONWriter* writer = env_data->gc_profiler->writer();
  writer->json_start();
  writer->json_keyvalue("startTime", uv_hrtime() / kNanosecondsPerSecond);
  writer->json_arraystart("gc");
}

void GcProfiler::StopGCProfiling(v8::Isolate* isolate) {
  EnvironmentData* env_data = EnvironmentData::GetCurrent(isolate);
  env_data->RemoveGCPrologueCallback(GCTracerPrologueCallback);
  env_data->RemoveGCEpilogueCallback(GCTracerEpilogueCallback);

  if (env_data->gc_profiler == nullptr) {
    return;
  }
  JSONWriter* writer = env_data->gc_profiler->writer();
  writer->json_arrayend();
  writer->json_keyvalue("stopTime", uv_hrtime() / kNanosecondsPerSecond);
  writer->json_end();

  env_data->gc_profiler.reset();
}

GcProfiler::GcProfiler(v8::Isolate* isolate, std::string filename)
    : filename_(filename),
      outfile_(filename, ios::out | ios::binary),
      writer_(outfile_),
      init_(uv_hrtime()) {}

GcProfiler::~GcProfiler() {}

}  // namespace xprofiler
