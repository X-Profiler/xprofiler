#ifndef XPROFILER_SRC_CONFIGURE_INL_H
#define XPROFILER_SRC_CONFIGURE_INL_H

#include "configure.h"
#include "process_data.h"

namespace xprofiler {
template <typename T>
T GetConfig(std::string key) {
  return ProcessData::Get()->config_store()->GetConfig<T>(key);
}
}  // namespace xprofiler

#endif /* XPROFILER_SRC_CONFIGURE_INL_H */
