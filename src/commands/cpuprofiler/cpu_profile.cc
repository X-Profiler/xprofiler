#include "cpu_profile.h"

#include "configure-inl.h"
#include "cpu_profile_node.h"
#include "environment_data.h"
#include "library/writer.h"
#include "logger.h"
#include "xpf_v8.h"

namespace xprofiler {
using Nan::Utf8String;
using std::ofstream;

void CpuProfile::DeleteCpuProfile(const v8::CpuProfile* profile) {
  const_cast<v8::CpuProfile*>(profile)->Delete();
}

void CpuProfile::Serialize(v8::Isolate* isolate, CpuProfilePtr node,
                           std::string filename) {
  EnvironmentData* env_data = EnvironmentData::GetCurrent(isolate);
  HandleScope scope(isolate);
  ofstream outfile;
  outfile.open(filename, std::ios::out | std::ios::binary);
  if (!outfile.is_open()) {
    ErrorT("cpu_profile", env_data->thread_id(), "open file %s failed.",
           filename.c_str());
    return;
  }

  // record cpu profile
  JSONWriter writer(outfile);
  writer.json_start();
  writer.json_keyvalue("typeId", "xprofiler-cpu-profile");

  // set title
  Utf8String title(node->GetTitle());
  writer.json_keyvalue("title", *title);

  // set nodes
  writer.json_arraystart("nodes");
  CpuProfileNode::SerializeNode(isolate, node->GetTopDownRoot(), &writer);
  writer.json_arrayend();

  // set start/end time
  writer.json_keyvalue("startTime", node->GetStartTime());
  writer.json_keyvalue("endTime", node->GetEndTime());

  // set samples
  int count = node->GetSamplesCount();
  writer.json_arraystart("samples");
  for (int index = 0; index < count; ++index) {
    writer.json_element(node->GetSample(index)->GetNodeId());
  }
  writer.json_arrayend();

  // set timestamps
  writer.json_arraystart("timeDeltas");
  for (int index = 0; index < count; ++index) {
    int64_t prev =
        index == 0 ? node->GetStartTime() : node->GetSampleTimestamp(index - 1);
    int64_t delta = node->GetSampleTimestamp(index) - prev;
    writer.json_element(delta);
  }
  writer.json_arrayend();

  // http profiling detail
  if (GetConfig<bool>("enable_http_profiling")) {
    writer.json_arraystart("httpDetail");
    HttpProfilingDetail* http_profiling_detail =
        env_data->http_profiling_detail();
    for (size_t i = 0; i < http_profiling_detail->samples.size(); i++) {
      std::string detail = http_profiling_detail->samples.at(i);
      writer.json_element(http_profiling_detail->samples.at(i));
    }
    http_profiling_detail->clear();
    writer.json_arrayend();
  } else {
    writer.json_arraystart("httpDetail");
    writer.json_arrayend();
  }

  // write to file
  writer.json_end();
}
}  // namespace xprofiler
