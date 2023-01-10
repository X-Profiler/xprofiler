#ifndef XPROFILER_SRC_HOOKS_HEAP_LIMIT_H
#define XPROFILER_SRC_HOOKS_HEAP_LIMIT_H

#include "nan.h"

namespace xprofiler {
void AutoIncreaseHeapLimit(v8::Isolate* isolate);
}  // namespace xprofiler

#endif /* XPROFILER_SRC_HOOKS_HEAP_LIMIT_H */
