#ifndef _SRC_COMMANDS_REPORT_NATIVE_STACK_H
#define _SRC_COMMANDS_REPORT_NATIVE_STACK_H

#include "../../library/writer.h"

namespace xprofiler {
void SetNativeStack(JSONWriter* writer);
}

#endif