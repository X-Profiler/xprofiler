#include "node_report.h"

#include <fstream>

#include "../../library/common.h"
#include "../../library/utils.h"
#include "../../library/writer.h"
#include "../../logger.h"
#include "../../platform/platform.h"
#include "heap_statistics.h"
#include "javascript_stack.h"
#include "native_stack.h"

namespace xprofiler {
using std::ios;
using std::ofstream;

NodeReport::NodeReport() {}
NodeReport::~NodeReport() {}

static void WriteNodeReport(JSONWriter *writer) {
  writer->json_start();

  writer->json_keyvalue("pid", GetPid());
  writer->json_keyvalue("nodeVersion", GetGlobalNodeVersion());
  writer->json_keyvalue("loadTime", GetStartTime("%Y-%m-%d %H:%M:%S"));
  writer->json_keyvalue("dumpTime", ConvertTime("%Y-%m-%d %H:%M:%S"));

  SetJavaScriptStack(writer);

  SetNativeStack(writer);

  SetHeapStatistics(writer);

  writer->json_end();
}

void NodeReport::GetNodeReport(string filepath) {
  ofstream outfile;
  outfile.open(filepath, ios::out | ios::binary);
  if (!outfile.is_open()) {
    Error("node_report", "open file %s failed.", filepath.c_str());
    return;
  }
  JSONWriter writer(outfile);
  WriteNodeReport(&writer);
  outfile.close();
}
}  // namespace xprofiler