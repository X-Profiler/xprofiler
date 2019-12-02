#include "system_statistics.h"

#include "../../platform/platform.h"

namespace xprofiler {
void SetSystemStatistics(JSONWriter* writer) {
  writer->json_objectstart("system");

  PrintSystemEnv(writer);
  PrintResourceLimits(writer);
  PrintLoadedLibraries(writer);

  writer->json_objectend();
}
}  // namespace xprofiler