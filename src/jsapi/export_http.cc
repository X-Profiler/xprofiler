#include "export_http.h"

#include "environment_data.h"
#include "logger.h"
#include "util-inl.h"
#include "xpf_v8.h"

namespace xprofiler {
using Nan::FunctionCallbackInfo;
using v8::Value;

constexpr char module_type[] = "http";

void SetHttpConfig(const FunctionCallbackInfo<Value>& info) {
  EnvironmentData* env_data = EnvironmentData::GetCurrent(info);
  if (!env_data) {
    return;
  }
  HttpStatistics* http_statistics = env_data->http_statistics();

  v8::Isolate* isolate = info.GetIsolate();
  HandleScope scope(isolate);

  if (!info[0]->IsObject()) {
    Nan::ThrowTypeError("Argument should be an object");
    return;
  }

  v8::Local<v8::Object> config = Nan::To<v8::Object>(info[0]).ToLocalChecked();

  // init
  http_statistics->config.Reset(config);
  http_statistics->config_initialized = true;
}

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

void AddHttpProfilingDetail(const Nan::FunctionCallbackInfo<v8::Value>& info) {
  EnvironmentData* env_data = EnvironmentData::GetCurrent(info);
  if (env_data == nullptr) {
    return;
  }
  HttpProfilingDetail* http_profiling_detail =
      env_data->http_profiling_detail();

  if (!info[0]->IsString()) {
    ErrorT(module_type, env_data->thread_id(),
           "request detail must be string!");
    return;
  }

  v8::Local<v8::String> detail = Nan::To<v8::String>(info[0]).ToLocalChecked();
  Nan::Utf8String detail_s(detail);

  Mutex::ScopedLock lock(http_profiling_detail->mutex);
  http_profiling_detail->samples.emplace_back(*detail_s);
}
}  // namespace xprofiler