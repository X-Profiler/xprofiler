#ifndef XPROFILER_SRC_PROCESS_DATA_H
#define XPROFILER_SRC_PROCESS_DATA_H

#include "environment_data.h"
#include "environment_registry.h"
#include "logbypass/log.h"
#include "xpf_mutex-inl.h"

namespace xprofiler {

// Slots for all non-trivially destructable per-process variables.
// Prior to Node.js v10.x (inclusive) `process.exit` doesn't invokes
// `Environment::AtExit` hooks.
// Do our best course to destruct per_process slots in an expected order.
class ProcessData {
 public:
  static ProcessData* Get();

  ProcessData(){};
  ~ProcessData() {
    if (log_by_pass != nullptr) {
      log_by_pass->Join();
    }
  };
  // Disallow copy;
  ProcessData(const ProcessData& other) = delete;

  EnvironmentRegistry* environment_registry() {
    return &environment_registry_;
  };
  std::unique_ptr<LogByPass> log_by_pass;
  Mutex log_by_pass_mutex;
  Mutex logger_mutex;

 private:
  EnvironmentRegistry environment_registry_;
};

}  // namespace xprofiler

#endif /* XPROFILER_SRC_PROCESS_DATA_H */
