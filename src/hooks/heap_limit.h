#pragma once

#include "nan.h"

namespace xprofiler {
void AutoIncreaseHeapLimit(v8::Isolate* isolate);
}  // namespace xprofiler
