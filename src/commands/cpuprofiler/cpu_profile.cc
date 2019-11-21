#include "cpu_profile.h"

#include "../../library/writer.h"
#include "../../logger.h"
#include "cpu_profile_node.h"

namespace xprofiler {
using Nan::HandleScope;
using Nan::Utf8String;
using std::ofstream;

void Profile::Serialize(const CpuProfile *node, std::string filename) {
  HandleScope scope;
  ofstream outfile;
  outfile.open(filename, std::ios::out | std::ios::binary);
  if (!outfile.is_open()) {
    Error("cpu_profile", "open file %s failed.", filename.c_str());
    return;
  }

  // record cpu profile
  JSONWriter writer(outfile);
  writer.json_start();
  writer.json_keyvalue("typeId", "xprofiler-cpu-profile");

  // set title
  Utf8String title(node->GetTitle());
  writer.json_keyvalue("title", *title);

  // set head
  writer.json_objectstart("head");
  ProfileNode::SerializeNode(node->GetTopDownRoot(), &writer);
  writer.json_objectend();

  // set start/end time
  writer.json_keyvalue("startTime", node->GetStartTime() / 1000000);
  writer.json_keyvalue("endTime", node->GetEndTime() / 1000000);

  // set samples
  uint32_t count = node->GetSamplesCount();
  writer.json_arraystart("samples");
  for (uint32_t index = 0; index < count; ++index) {
    writer.json_element(node->GetSample(index)->GetNodeId());
  }
  writer.json_arrayend();

  // set timestamps
  writer.json_arraystart("timestamps");
  for (uint32_t index = 0; index < count; ++index) {
    writer.json_element(node->GetSampleTimestamp(index));
  }
  writer.json_arrayend();

  // write to file
  writer.json_end();
  outfile.close();
}
}  // namespace xprofiler
