#ifndef XPROFILER_SRC_COMMANDS_HTTP_PROFILER_H
#define XPROFILER_SRC_COMMANDS_HTTP_PROFILER_H

#include "logbypass/http.h"
#include "nan.h"

namespace xprofiler {

class HttpProfiler final {
 public:
  static void StartProfiling(v8::Isolate* isolate);
  static void StopProfiling(v8::Isolate* isolate);
};
}  // namespace xprofiler

#endif /* XPROFILER_SRC_COMMANDS_HTTP_PROFILER_H */
