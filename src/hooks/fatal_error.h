#pragma once

#include "nan.h"

namespace xprofiler {
[[noreturn]] void OnFatalError(const char* location, const char* message);
void SetFatalErrorHandler(v8::Isolate* isolate);
}  // namespace xprofiler
