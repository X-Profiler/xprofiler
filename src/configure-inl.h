#ifndef XPROFILER_SRC_CONFIGURE_INL_H
#define XPROFILER_SRC_CONFIGURE_INL_H

#include "configure.h"
#include "process_data.h"

namespace xprofiler {
std::string GetLogDir() {
  return ProcessData::Get()->config_store()->GetConfig<std::string>("log_dir");
}

uint32_t GetLogInterval() {
  return ProcessData::Get()->config_store()->GetConfig<uint32_t>(
      "log_interval");
}

LOG_LEVEL GetLogLevel() {
  return ProcessData::Get()->config_store()->GetConfig<LOG_LEVEL>("log_level");
}

LOG_TYPE GetLogType() {
  return ProcessData::Get()->config_store()->GetConfig<LOG_TYPE>("log_type");
}

bool GetFormatAsAlinode() {
  return ProcessData::Get()->config_store()->GetConfig<bool>(
      "log_format_alinode");
}

bool GetPatchHttp() {
  return ProcessData::Get()->config_store()->GetConfig<bool>("patch_http");
}

uint32_t GetPatchHttpTimeout() {
  return ProcessData::Get()->config_store()->GetConfig<uint32_t>(
      "patch_http_timeout");
}

bool GetCheckThrow() {
  return ProcessData::Get()->config_store()->GetConfig<bool>("check_throw");
}

bool GetEnableLogUvHandles() {
  return ProcessData::Get()->config_store()->GetConfig<bool>(
      "enable_log_uv_handles");
}

bool GetEnableFatalErrorHook() {
  return ProcessData::Get()->config_store()->GetConfig<bool>(
      "enable_fatal_error_hook");
}

bool GetEnableFatalErrorReport() {
  return ProcessData::Get()->config_store()->GetConfig<bool>(
      "enable_fatal_error_report");
}

bool GetEnableFatalErrorCoredump() {
  return ProcessData::Get()->config_store()->GetConfig<bool>(
      "enable_fatal_error_coredump");
}

bool GetEnableHttpProfiling() {
  return ProcessData::Get()->config_store()->GetConfig<bool>(
      "enable_http_profiling");
}
}  // namespace xprofiler

#endif /* XPROFILER_SRC_CONFIGURE_INL_H */
