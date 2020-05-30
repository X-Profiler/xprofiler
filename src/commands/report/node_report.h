#ifndef _SRC_COMMANDS_REPORT_NODE_REPORT_H
#define _SRC_COMMANDS_REPORT_NODE_REPORT_H

#include <string>

namespace xprofiler {
using std::string;
class NodeReport {
 public:
  NodeReport();
  virtual ~NodeReport();
  static void GetNodeReport(string filepath, string location = "Active Dump",
                            string message = "Active Dump",
                            bool fatal_error = false);
};
}  // namespace xprofiler

#endif