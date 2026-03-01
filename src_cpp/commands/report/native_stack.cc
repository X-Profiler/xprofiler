#include "node_report.h"
#include "platform/platform.h"

namespace xprofiler {

void NodeReport::SetNativeStack(JSONWriter* writer) {
  PrintNativeStack(writer);
}

}  // namespace xprofiler
