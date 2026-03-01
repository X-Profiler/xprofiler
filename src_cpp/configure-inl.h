#ifndef XPROFILER_SRC_CONFIGURE_INL_H
#define XPROFILER_SRC_CONFIGURE_INL_H

#include "configure.h"
#include "process_data.h"

namespace xprofiler {
template <typename T>
T GetConfig(std::string key) {
  T result = T();
  try {
    result = ProcessData::Get()->config_store()->GetConfig<T>(key);
  } catch (nlohmann::json::exception& e) {
  }
  return result;
}
}  // namespace xprofiler

#endif /* XPROFILER_SRC_CONFIGURE_INL_H */
