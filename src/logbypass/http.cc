#include "http.h"

#include "environment_data.h"
#include "logger.h"

namespace xprofiler {
void WriteHttpStatus(EnvironmentData* env_data, bool log_format_alinode,
                     uint32_t http_patch_timeout) {
  HttpStatistics* http_statistics = env_data->http_statistics();
  Mutex::ScopedLock lock(http_statistics->mutex);

  double rt = 0.00;
  if (http_statistics->http_response_sent != 0) {
    rt = http_statistics->http_rt * 1.00 / http_statistics->http_response_sent;
  }

  if (log_format_alinode)
    Info("http",
         "live_http_request: %d, "
         "http_request_handled: %d, "
         "http_response_sent: %d, "
         "http_rt: %lf",
         http_statistics->live_http_request,
         http_statistics->http_response_sent,
         http_statistics->http_response_sent, rt);
  else {
    std::string format = "";
    for (int i = 0; i < 1000; i++) {
      uint32_t count = http_statistics->status_codes[i];
      if (count > 0 && format.length() < 1536) {
        format += "res" XPROFILER_BLURRY_TAG + std::to_string(i) + ": " +
                  std::to_string(count) + ", ";
      }
    }

    InfoT("http", env_data->thread_id(),
          "%s"
          "live_http_request: %d, "
          "http_response_close: %d, "
          "http_response_sent: %d, "
          "http_request_timeout: %d, "
          "http_patch_timeout: %d, "
          "http_rt: %lf",
          format.c_str(), http_statistics->live_http_request,
          http_statistics->http_response_close,
          http_statistics->http_response_sent,
          http_statistics->http_request_timeout, http_patch_timeout, rt);
  }

  // reset
  http_statistics->live_http_request = 0;
  http_statistics->http_response_sent = 0;
  http_statistics->http_response_close = 0;
  http_statistics->http_request_timeout = 0;
  http_statistics->http_rt = 0;
  for (int i = 0; i < kMaxHttpStatusCode; i++) {
    http_statistics->status_codes[i] = 0;
  }
}
}  // namespace xprofiler
