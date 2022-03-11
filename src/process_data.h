#ifndef XPROFILER_SRC_PROCESS_DATA_H
#define XPROFILER_SRC_PROCESS_DATA_H

#include "environment_data.h"
#include "logbypass/log.h"

namespace xprofiler {

// Slots for all non-trivially destructable per-process variables.
// Prior to Node.js v10.x (inclusive) `process.exit` doesn't invokes
// `Environment::AtExit` hooks.
// Do our best course to destruct per_process slots in an expected order.
class ProcessData {
 public:
  ProcessData(){};
  ~ProcessData() {
    if (log_by_pass != nullptr) {
      log_by_pass->Join();
    }
  };

  // Disallow copy;
  ProcessData(const ProcessData& other) = delete;

  // TODO(legendecas): environment registry.
  std::unique_ptr<EnvironmentData> environment_data;
  std::unique_ptr<LogByPass> log_by_pass;
};

namespace per_process {
extern ProcessData process_data;
}

}  // namespace xprofiler

#endif /* XPROFILER_SRC_PROCESS_DATA_H */
