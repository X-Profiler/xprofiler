#include "sampling_heap_profiler.h"

#include "../../library/writer.h"
#include "../../logger.h"

namespace xprofiler {
using Nan::HandleScope;
using Nan::Utf8String;
using std::ofstream;
using v8::AllocationProfile;
using v8::Isolate;

SamplingHeapProfile::SamplingHeapProfile() {}
SamplingHeapProfile::~SamplingHeapProfile() {}

void TranslateAllocationProfile(AllocationProfile::Node *node,
                                JSONWriter *writer) {
  HandleScope scope;
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
  int selfSize = 0;
  for (size_t i = 0; i < node->allocations.size(); i++) {
    AllocationProfile::Allocation alloc = node->allocations[i];
    selfSize += alloc.size * alloc.count;
  }
  writer->json_keyvalue("selfSize", selfSize);

  // add children
  writer->json_arraystart("children");
  for (size_t i = 0; i < node->children.size(); i++) {
    writer->json_start();
    TranslateAllocationProfile(node->children[i], writer);
    writer->json_end();
  }
  writer->json_arrayend();
}

void SamplingHeapProfile::StartSamplingHeapProfiling() {
  Isolate::GetCurrent()->GetHeapProfiler()->StartSamplingHeapProfiler();
}

void SamplingHeapProfile::StopSamplingHeapProfiling(string filename) {
  HandleScope scope;
  ofstream outfile;
  outfile.open(filename, std::ios::out | std::ios::binary);
  if (!outfile.is_open()) {
    Error("sampling_heap_profiler", "open file %s failed.", filename.c_str());
    outfile.close();
    return;
  }
  // get allocationProfile
  AllocationProfile *profile =
      Isolate::GetCurrent()->GetHeapProfiler()->GetAllocationProfile();
  AllocationProfile::Node *root = profile->GetRootNode();
  JSONWriter writer(outfile);
  writer.json_start();
  writer.json_objectstart("head");
  TranslateAllocationProfile(root, &writer);
  writer.json_objectend();
  writer.json_end();
  outfile.close();
  free(profile);
  // stop sampling heap profile
  Isolate::GetCurrent()->GetHeapProfiler()->StopSamplingHeapProfiler();
}
}  // namespace xprofiler