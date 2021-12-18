#include "cpu_profile_node.h"

namespace xprofiler {
using Nan::HandleScope;
using Nan::Utf8String;

void ProfileNode::SerializeNode(const CpuProfileNode *node,
                                JSONWriter *writer) {
  HandleScope scope;
  Utf8String funcion_name(node->GetFunctionName());
  Utf8String url(node->GetScriptResourceName());

  // set node
  writer->json_start();
  writer->json_keyvalue("id", node->GetNodeId());
  writer->json_keyvalue("hitCount", node->GetHitCount());
  // set call frame
  writer->json_objectstart("callFrame");
  writer->json_keyvalue("functionName", *funcion_name);
  writer->json_keyvalue("scriptId", node->GetScriptId());
  writer->json_keyvalue("bailoutReason", node->GetBailoutReason());
  writer->json_keyvalue("url", *url);
  writer->json_keyvalue("lineNumber", node->GetLineNumber());
  writer->json_keyvalue("columnNumber", node->GetColumnNumber());
  writer->json_objectend();

  // set children
  int count = node->GetChildrenCount();
  writer->json_arraystart("children");
  for (int index = 0; index < count; index++) {
    writer->json_element(node->GetChild(index)->GetNodeId());
  }
  writer->json_arrayend();
  writer->json_end();

  for (int index = 0; index < count; index++) {
    ProfileNode::SerializeNode(node->GetChild(index), writer);
  }
}

}  // namespace xprofiler
