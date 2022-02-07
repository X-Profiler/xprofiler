#ifndef XPROFILER_SRC_LOGBYPASS_HTTP_H
#define XPROFILER_SRC_LOGBYPASS_HTTP_H

#include "nan.h"
#include "xpf_mutex-inl.h"

namespace xprofiler {

class EnvironmentData;

constexpr uint16_t kMaxHttpStatusCode = 1000;

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
};

void WriteHttpStatus(EnvironmentData* env_data, bool log_format_alinode,
                     uint32_t http_patch_timeout);

// javascript-accessible
void AddLiveRequest(const Nan::FunctionCallbackInfo<v8::Value>& info);
void AddCloseRequest(const Nan::FunctionCallbackInfo<v8::Value>& info);
void AddSentRequest(const Nan::FunctionCallbackInfo<v8::Value>& info);
void AddRequestTimeout(const Nan::FunctionCallbackInfo<v8::Value>& info);
void AddHttpStatusCode(const Nan::FunctionCallbackInfo<v8::Value>& info);
}  // namespace xprofiler

#endif /* XPROFILER_SRC_LOGBYPASS_HTTP_H */
