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
#include "system_statistics.h"
#include "uv_statistics.h"

namespace xprofiler {
using std::ios;
using std::ofstream;

NodeReport::NodeReport() {}
NodeReport::~NodeReport() {}

static void WriteNodeReport(JSONWriter *writer, string location, string message,
                            bool fatal_error) {
  writer->json_start();

  writer->json_keyvalue("pid", GetPid());
  writer->json_keyvalue("location", location);
  writer->json_keyvalue("message", message);
  writer->json_keyvalue("nodeVersion", GetGlobalNodeVersion());
  writer->json_keyvalue("osVersion", GetOsVersion());
  writer->json_keyvalue("loadTime", GetStartTime("%Y-%m-%d %H:%M:%S"));
  writer->json_keyvalue("dumpTime", ConvertTime("%Y-%m-%d %H:%M:%S"));

  SetJavaScriptStack(writer, fatal_error);
  SetNativeStack(writer);
  SetHeapStatistics(writer);
  SetUvStatistics(writer);
  SetSystemStatistics(writer);

  writer->json_end();
}

void NodeReport::GetNodeReport(string filepath, string location, string message,
                               bool fatal_error) {
  ofstream outfile;
  outfile.open(filepath, ios::out | ios::binary);
  if (!outfile.is_open()) {
    Error("node_report", "open file %s failed.", filepath.c_str());
    outfile.close();
    return;
  }
  JSONWriter writer(outfile);
  WriteNodeReport(&writer, location, message, fatal_error);
  outfile.close();
}
}  // namespace xprofiler