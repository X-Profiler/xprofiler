#pragma once

#include "xpf_mutex-inl.h"

namespace xprofiler {

class EnvironmentData;

constexpr uint16_t kMaxHttpStatusCode = 1000;

using HttpProfilingSamples = std::vector<std::string>;

struct HttpProfilingDetail {
  Mutex mutex;
  int64_t start_time = 0;
  int64_t end_time = 0;
  HttpProfilingSamples samples;

  void clear() {
    Mutex::ScopedLock lock(mutex);
    start_time = 0;
    end_time = 0;
    HttpProfilingSamples().swap(samples);
  }
};

struct HttpStatistics {
  Mutex mutex;
  // http server
  uint32_t live_http_request = 0;
  uint32_t http_response_close = 0;
  uint32_t http_response_sent = 0;
  uint32_t http_request_timeout = 0;
  uint32_t http_rt = 0;  // ms
  // http status code: 0 ~ 999
  uint32_t status_codes[kMaxHttpStatusCode] = {0};

  // http config
  bool config_initialized = false;
  Nan::Persistent<v8::Object> config;
};

void EnableHttpDetailProfiling(EnvironmentData* env_data);
void DisableHttpDetailProfiling(EnvironmentData* env_data);
void WriteHttpStatus(EnvironmentData* env_data, bool log_format_alinode,
                     uint32_t http_patch_timeout);
}  // namespace xprofiler
