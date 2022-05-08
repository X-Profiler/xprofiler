#include "http.h"

#include "environment_data.h"
#include "logger.h"
#include "uv.h"

namespace xprofiler {
using Nan::FunctionCallbackInfo;
using v8::Value;

constexpr char module_type[] = "http";

void AddLiveRequest(const FunctionCallbackInfo<Value>& info) {
  EnvironmentData* env_data = EnvironmentData::GetCurrent(info);
  if (env_data == nullptr) {
    return;
  }
  HttpStatistics* http_statistics = env_data->http_statistics();
  Mutex::ScopedLock lock(http_statistics->mutex);
  http_statistics->live_http_request++;
}

void AddCloseRequest(const FunctionCallbackInfo<Value>& info) {
  EnvironmentData* env_data = EnvironmentData::GetCurrent(info);
  if (env_data == nullptr) {
    return;
  }
  HttpStatistics* http_statistics = env_data->http_statistics();
  Mutex::ScopedLock lock(http_statistics->mutex);
  http_statistics->http_response_close++;
}

void AddSentRequest(const FunctionCallbackInfo<Value>& info) {
  EnvironmentData* env_data = EnvironmentData::GetCurrent(info);
  if (env_data == nullptr) {
    return;
  }
  HttpStatistics* http_statistics = env_data->http_statistics();

  if (!info[0]->IsNumber()) {
    ErrorT(module_type, env_data->thread_id(), "request cost must be number!");
    return;
  }

  uint32_t cost = Nan::To<uint32_t>(info[0]).ToChecked();

  Mutex::ScopedLock lock(http_statistics->mutex);
  http_statistics->http_response_sent++;
  http_statistics->http_rt += cost;
}

void AddRequestTimeout(const FunctionCallbackInfo<Value>& info) {
  EnvironmentData* env_data = EnvironmentData::GetCurrent(info);
  if (env_data == nullptr) {
    return;
  }
  HttpStatistics* http_statistics = env_data->http_statistics();
  Mutex::ScopedLock lock(http_statistics->mutex);
  http_statistics->http_request_timeout++;
}

void AddHttpStatusCode(const FunctionCallbackInfo<Value>& info) {
  EnvironmentData* env_data = EnvironmentData::GetCurrent(info);
  if (env_data == nullptr) {
    return;
  }
  HttpStatistics* http_statistics = env_data->http_statistics();

  if (!info[0]->IsNumber()) {
    ErrorT(module_type, env_data->thread_id(), "request cost must be number!");
    return;
  }

  uint32_t status_code = Nan::To<uint32_t>(info[0]).ToChecked();
  if (status_code >= kMaxHttpStatusCode) {
    return;
  }

  Mutex::ScopedLock lock(http_statistics->mutex);
  http_statistics->status_codes[status_code]++;
}

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
         "http_rt: %.2lf",
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
          "http_rt: %.2lf",
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
