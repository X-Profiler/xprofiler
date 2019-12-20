#include "gc_profiler.h"

#include "../../library/writer.h"
#include "../../logbypass/gc.h"
#include "../../logger.h"
#include "nan.h"

namespace xprofiler {
using Nan::AddGCEpilogueCallback;
using Nan::AddGCPrologueCallback;
using Nan::RemoveGCEpilogueCallback;
using Nan::RemoveGCPrologueCallback;
using std::ios;
using std::ofstream;
using v8::GCType;
using v8::HeapSpaceStatistics;

static string filename = "";
static ofstream outfile;
static JSONWriter *writer = nullptr;
uint64_t init = 0;

#define SPACE_DATA(tag)                                                      \
  if (type == GCType::kGCTypeScavenge) {                                     \
    writer->json_keyvalue("type", "scavenge");                               \
  } else if (type == GCType::kGCTypeMarkSweepCompact) {                      \
    writer->json_keyvalue("type", "marksweep");                              \
  } else if (type == GCType::kGCTypeIncrementalMarking) {                    \
    writer->json_keyvalue("type", "marking");                                \
  } else if (type == GCType::kGCTypeProcessWeakCallbacks) {                  \
    writer->json_keyvalue("type", "weakcallbacks");                          \
  } else {                                                                   \
    writer->json_keyvalue("type", static_cast<int>(type));                   \
  }                                                                          \
  HeapSpaceStatistics s;                                                     \
  size_t number_of_heap_spaces = isolate->NumberOfHeapSpaces();              \
  writer->json_arraystart(tag);                                              \
  for (size_t i = 0; i < number_of_heap_spaces; i++) {                       \
    writer->json_start();                                                    \
    isolate->GetHeapSpaceStatistics(&s, i);                                  \
    writer->json_keyvalue("name", s.space_name());                           \
    writer->json_keyvalue("space_size", s.space_size());                     \
    writer->json_keyvalue("space_used_size", s.space_used_size());           \
    writer->json_keyvalue("space_available_size", s.space_available_size()); \
    writer->json_keyvalue("physical_space_size", s.physical_space_size());   \
    writer->json_end();                                                      \
  }                                                                          \
  writer->json_arrayend();

GcProfiler::GcProfiler() {}
GcProfiler::~GcProfiler() {}

NAN_GC_CALLBACK(GCTracerPrologueCallback) {
  writer->json_start();
  writer->json_keyvalue("totalSpentfromStart", TotalGcDuration());
  writer->json_keyvalue("totalTimesfromStart", TotalGcTimes());
  writer->json_keyvalue("timeFromStart", GetUptime());
  writer->json_keyvalue("start", static_cast<int>((uv_hrtime() - init) / 10e5));
  SPACE_DATA("before");
}

NAN_GC_CALLBACK(GCTracerEpilogueCallback) {
  writer->json_keyvalue("end", static_cast<int>((uv_hrtime() - init) / 10e5));
  SPACE_DATA("after");
  writer->json_end();
}

void GcProfiler::StartGCProfiling(string filename_) {
  outfile.open(filename_, ios::out | ios::binary);
  if (!outfile.is_open()) {
    Error("gc_profiler", "open file %s failed.", filename_.c_str());
    outfile.close();
    return;
  }
  AddGCPrologueCallback(GCTracerPrologueCallback);
  AddGCEpilogueCallback(GCTracerEpilogueCallback);
  filename = filename_;
  writer = new JSONWriter(outfile);
  init = uv_hrtime();
  writer->json_start();
  writer->json_keyvalue("startTime",
                        static_cast<unsigned long>(uv_hrtime() / 10e8));
  writer->json_arraystart("gc");
}

void GcProfiler::StopGCProfiling() {
  RemoveGCPrologueCallback(GCTracerPrologueCallback);
  RemoveGCEpilogueCallback(GCTracerEpilogueCallback);
  if (writer != nullptr) {
    writer->json_arrayend();
    writer->json_keyvalue("stopTime",
                          static_cast<unsigned long>(uv_hrtime() / 10e8));
    writer->json_end();
    delete writer;
  }
  if (filename != "") {
    filename = "";
  }
  if (outfile.is_open()) {
    outfile.close();
  }
  init = 0;
}
}  // namespace xprofiler