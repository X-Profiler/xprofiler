#ifndef XPROFILER_SRC_COMMANDS_REPORT_NODE_REPORT_H
#define XPROFILER_SRC_COMMANDS_REPORT_NODE_REPORT_H

#include <string>

#include "library/writer.h"
#include "nan.h"

namespace xprofiler {
class NodeReport final {
 public:
  static void GetNodeReport(v8::Isolate* isolate, std::string filepath,
                            std::string location = "Active Dump",
                            std::string message = "Active Dump",
                            bool fatal_error = false);

 private:
  NodeReport(v8::Isolate* isolate);
  void WriteNodeReport(JSONWriter* writer, std::string location,
                       std::string message, bool fatal_error);

  void SetUvStatistics(JSONWriter* writer);
  void SetSystemStatistics(JSONWriter* writer);
  void SetNativeStack(JSONWriter* writer);
  void SetJavaScriptStack(JSONWriter* writer, bool fatal_error = false);
  void SetHeapStatistics(JSONWriter* writer);

  v8::Isolate* isolate_;
};
}  // namespace xprofiler

#endif /* XPROFILER_SRC_COMMANDS_REPORT_NODE_REPORT_H */
