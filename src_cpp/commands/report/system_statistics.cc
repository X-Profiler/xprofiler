#include "node_report.h"
#include "platform/platform.h"

namespace xprofiler {
void NodeReport::SetSystemStatistics(JSONWriter* writer) {
  writer->json_objectstart("system");

  PrintSystemEnv(writer);
  PrintResourceLimits(writer);
  PrintLoadedLibraries(writer);

  writer->json_objectend();
}
}  // namespace xprofiler
