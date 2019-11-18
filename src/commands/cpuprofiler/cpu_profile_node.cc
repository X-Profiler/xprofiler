#include "cpu_profile_node.h"

namespace xprofiler {
using Nan::HandleScope;
using Nan::Utf8String;

void ProfileNode::SerializeNode(const CpuProfileNode *node,
                                JSONWriter *writer) {
  HandleScope scope;
  Utf8String funcion_name(node->GetFunctionName());
  Utf8String url(node->GetScriptResourceName());

  // set parent
  writer->json_keyvalue("functionName", *funcion_name);
  writer->json_keyvalue("url", *url);
  writer->json_keyvalue("lineNumber", node->GetLineNumber());
  writer->json_keyvalue("columnNumber", node->GetColumnNumber());
  writer->json_keyvalue("bailoutReason", node->GetBailoutReason());
  writer->json_keyvalue("id", node->GetNodeId());
  writer->json_keyvalue("scriptId", node->GetScriptId());
  writer->json_keyvalue("hitCount", node->GetHitCount());

  // set children recursively
  int32_t count = node->GetChildrenCount();
  writer->json_arraystart("children");
  for (int32_t index = 0; index < count; index++) {
    writer->json_start();
    ProfileNode::SerializeNode(node->GetChild(index), writer);
    writer->json_end();
  }
  writer->json_arrayend();
}

} // namespace xprofiler
