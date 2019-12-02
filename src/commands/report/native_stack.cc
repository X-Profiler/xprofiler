#include "native_stack.h"

#include "../../platform/platform.h"

namespace xprofiler {
void SetNativeStack(JSONWriter* writer) { PrintNativeStack(writer); }
}  // namespace xprofiler