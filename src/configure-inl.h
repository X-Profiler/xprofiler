#ifndef XPROFILER_SRC_CONFIGURE_INL_H
#define XPROFILER_SRC_CONFIGURE_INL_H

#include "configure.h"
#include "process_data.h"

namespace xprofiler {
std::string GetLogDir() { return ProcessData::Get()->config_store()->log_dir; }

uint32_t GetLogInterval() {
  return ProcessData::Get()->config_store()->log_interval;
}

LOG_LEVEL GetLogLevel() {
  return ProcessData::Get()->config_store()->log_level;
}

LOG_TYPE GetLogType() { return ProcessData::Get()->config_store()->log_type; }

bool GetFormatAsAlinode() {
  return ProcessData::Get()->config_store()->log_format_alinode;
}

bool GetEnableLogUvHandles() {
  return ProcessData::Get()->config_store()->enable_log_uv_handles;
}

bool GetEnableFatalErrorHook() {
  return ProcessData::Get()->config_store()->enable_fatal_error_hook;
}

bool GetEnableFatalErrorReport() {
  return ProcessData::Get()->config_store()->enable_fatal_error_report;
}

bool GetEnableFatalErrorCoredump() {
  return ProcessData::Get()->config_store()->enable_fatal_error_coredump;
}

bool GetEnableHttpProfiling() {
  return ProcessData::Get()->config_store()->enable_http_profiling;
}

bool GetPatchHttp() { return ProcessData::Get()->config_store()->patch_http; }

uint32_t GetPatchHttpTimeout() {
  return ProcessData::Get()->config_store()->patch_http_timeout;
}

bool GetCheckThrow() { return ProcessData::Get()->config_store()->check_throw; }

void SetLogLevel(LOG_LEVEL value) {
  ProcessData::Get()->config_store()->log_level = value;
}

void SetLogType(LOG_TYPE value) {
  ProcessData::Get()->config_store()->log_type = value;
}

void SetEnableLogUvHandles(bool value) {
  ProcessData::Get()->config_store()->enable_log_uv_handles = value;
}

void SetEnableFatalErrorReport(bool value) {
  ProcessData::Get()->config_store()->enable_fatal_error_report = value;
}

void SetEnableFatalErrorCoredump(bool value) {
  ProcessData::Get()->config_store()->enable_fatal_error_coredump = value;
}

void SetEnableHttpProfiling(bool value) {
  ProcessData::Get()->config_store()->enable_http_profiling = value;
}
}  // namespace xprofiler

#endif /* XPROFILER_SRC_CONFIGURE_INL_H */
