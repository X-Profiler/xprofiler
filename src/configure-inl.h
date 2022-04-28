#ifndef XPROFILER_SRC_CONFIGURE_INL_H
#define XPROFILER_SRC_CONFIGURE_INL_H

#include "configure.h"

namespace xprofiler {
std::string GetLogDir() { return per_process::config_store.log_dir; }

uint32_t GetLogInterval() { return per_process::config_store.log_interval; }

LOG_LEVEL GetLogLevel() { return per_process::config_store.log_level; }

LOG_TYPE GetLogType() { return per_process::config_store.log_type; }

bool GetFormatAsAlinode() {
  return per_process::config_store.log_format_alinode;
}

bool GetEnableLogUvHandles() {
  return per_process::config_store.enable_log_uv_handles;
}

bool GetEnableFatalErrorHook() {
  return per_process::config_store.enable_fatal_error_hook;
}

bool GetEnableFatalErrorReport() {
  return per_process::config_store.enable_fatal_error_report;
}

bool GetEnableFatalErrorCoredump() {
  return per_process::config_store.enable_fatal_error_coredump;
}

bool GetPatchHttp() { return per_process::config_store.patch_http; }

uint32_t GetPatchHttpTimeout() {
  return per_process::config_store.patch_http_timeout;
}

bool GetCheckThrow() { return per_process::config_store.check_throw; }

void SetLogLevel(LOG_LEVEL value) {
  per_process::config_store.log_level = value;
}

void SetLogType(LOG_TYPE value) { per_process::config_store.log_type = value; }

void SetEnableLogUvHandles(bool value) {
  per_process::config_store.enable_log_uv_handles = value;
}

}  // namespace xprofiler

#endif /* XPROFILER_SRC_CONFIGURE_INL_H */
