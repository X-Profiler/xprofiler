#include "sampling_heap_profiler.h"

#include "environment_data.h"
#include "library/writer.h"
#include "logger.h"
#include "xpf_v8.h"

namespace xprofiler {
using Nan::Utf8String;
using std::ofstream;
using v8::AllocationProfile;
using v8::Isolate;

void TranslateAllocationProfile(Isolate* isolate, AllocationProfile::Node* node,
                                JSONWriter* writer) {
  HandleScope scope(isolate);
  writer->json_objectstart("callFrame");
  Utf8String function_name(node->name);
  Utf8String url(node->script_name);
  writer->json_keyvalue("functionName", *function_name);
  writer->json_keyvalue("scriptId", node->script_id);
  writer->json_keyvalue("url", *url);
  writer->json_keyvalue("lineNumber", node->line_number);
  writer->json_keyvalue("columnNumber", node->column_number);
  writer->json_objectend();

  // add self size
  size_t selfSize = 0;
  for (size_t i = 0; i < node->allocations.size(); i++) {
    AllocationProfile::Allocation alloc = node->allocations[i];
    selfSize += alloc.size * alloc.count;
  }
  writer->json_keyvalue("selfSize", selfSize);

  // add children
  writer->json_arraystart("children");
  for (size_t i = 0; i < node->children.size(); i++) {
    writer->json_start();
    TranslateAllocationProfile(isolate, node->children[i], writer);
    writer->json_end();
  }
  writer->json_arrayend();
}

void SamplingHeapProfiler::StartSamplingHeapProfiling(v8::Isolate* isolate) {
  isolate->GetHeapProfiler()->StartSamplingHeapProfiler();
}

void SamplingHeapProfiler::StopSamplingHeapProfiling(v8::Isolate* isolate,
                                                     std::string filename) {
  EnvironmentData* env_data = EnvironmentData::GetCurrent(isolate);
  ofstream outfile(filename, std::ios::out | std::ios::binary);
  if (!outfile.is_open()) {
    ErrorT("sampling_heap_profiler", env_data->thread_id(),
           "open file %s failed.", filename.c_str());
    return;
  }
  HandleScope scope(isolate);
  // get allocationProfile
  std::unique_ptr<AllocationProfile> profile =
      std::unique_ptr<AllocationProfile>(
          isolate->GetHeapProfiler()->GetAllocationProfile());
  // stop sampling heap profile
  isolate->GetHeapProfiler()->StopSamplingHeapProfiler();

  AllocationProfile::Node* root = profile->GetRootNode();
  JSONWriter writer(outfile);
  writer.json_start();
  writer.json_objectstart("head");
  TranslateAllocationProfile(isolate, root, &writer);
  writer.json_objectend();
  writer.json_end();
}
}  // namespace xprofiler
