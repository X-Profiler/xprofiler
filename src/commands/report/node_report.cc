#include "node_report.h"

#include <fstream>

#include "environment_data.h"
#include "environment_registry.h"
#include "library/common.h"
#include "library/utils.h"
#include "library/writer.h"
#include "logger.h"
#include "platform/platform.h"
#include "process_data.h"

namespace xprofiler {
using std::ios;
using std::ofstream;

NodeReport::NodeReport(v8::Isolate* isolate) : isolate_(isolate) {}

void NodeReport::WriteNodeReport(JSONWriter* writer, std::string location,
                                 std::string message, bool fatal_error) {
  writer->json_start();

  writer->json_keyvalue("pid", GetPid());
  {
    EnvironmentRegistry* registry = ProcessData::Get()->environment_registry();
    EnvironmentRegistry::NoExitScope no_exit(registry);
    EnvironmentData* data = registry->Get(isolate_);
    if (data != nullptr) {
      writer->json_keyvalue("thread_id", data->thread_id());
    }
  }
  writer->json_keyvalue("location", location);
  writer->json_keyvalue("message", message);
  writer->json_keyvalue("nodeVersion", NODE_VERSION);
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

void NodeReport::GetNodeReport(v8::Isolate* isolate, std::string filepath,
                               std::string location, std::string message,
                               bool fatal_error) {
  NodeReport report(isolate);
  ofstream outfile;
  outfile.open(filepath, ios::out | ios::binary);
  if (!outfile.is_open()) {
    Error("node_report", "open file %s failed.", filepath.c_str());
    outfile.close();
    return;
  }
  JSONWriter writer(outfile);
  report.WriteNodeReport(&writer, location, message, fatal_error);
  outfile.close();
}
}  // namespace xprofiler
