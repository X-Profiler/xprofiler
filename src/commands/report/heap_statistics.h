#ifndef _SRC_COMMANDS_REPORT_HEAP_STATISTICS_H
#define _SRC_COMMANDS_REPORT_HEAP_STATISTICS_H

#include "../../library/writer.h"

namespace xprofiler {
void SetHeapStatistics(JSONWriter* writer);
}

#endif