#pragma once

#include <stdint.h>

namespace xprofiler {

class EnvironmentData;

#define HANDLE_DEFAULT_VALUE 0

#define INIT_UV_HANDLE(name)                          \
  int active_##name##_handles = HANDLE_DEFAULT_VALUE; \
  int active_and_ref_##name##_handles = HANDLE_DEFAULT_VALUE;

#define RESET_UV_HANDLE(name)                     \
  active_##name##_handles = HANDLE_DEFAULT_VALUE; \
  active_and_ref_##name##_handles = HANDLE_DEFAULT_VALUE;

struct UvHandleStatistics {
  uint32_t active_handles = 0;

  INIT_UV_HANDLE(file)
  INIT_UV_HANDLE(tcp)
  INIT_UV_HANDLE(udp)
  INIT_UV_HANDLE(timer)

  // reset record
  void reset() {
    active_handles = 0;
    RESET_UV_HANDLE(file)
    RESET_UV_HANDLE(tcp)
    RESET_UV_HANDLE(udp)
    RESET_UV_HANDLE(timer)
  }
};

void CollectLibuvHandleStatistics(EnvironmentData* env_data);
void WriteLibuvHandleInfoToLog(EnvironmentData* env_data,
                               bool log_format_alinode);
}  // namespace xprofiler
