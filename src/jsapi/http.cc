#include "include/http.h"

#include "environment_data.h"
#include "logger.h"

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
}  // namespace xprofiler