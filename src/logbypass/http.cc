#include "http.h"

#include "../logger.h"
#include "uv.h"

namespace xprofiler {
using Nan::To;
using std::string;

static uv_mutex_t http_mutex;

static const char module_type[] = "http";

// http server
static unsigned int live_http_request = 0;
static unsigned int http_response_close = 0;
static unsigned int http_response_sent = 0;
static unsigned int http_request_timeout = 0;
static unsigned int http_rt = 0;  // ms

// http status code: 0 ~ 999
static int status_codes[1000] = {0};

int InitHttpStatus() {
  int rc = uv_mutex_init(&http_mutex);
  return rc;
}

void AddLiveRequest(const FunctionCallbackInfo<Value> &info) {
  uv_mutex_lock(&http_mutex);
  live_http_request++;
  uv_mutex_unlock(&http_mutex);
}

void AddCloseRequest(const FunctionCallbackInfo<Value> &info) {
  uv_mutex_lock(&http_mutex);
  http_response_close++;
  uv_mutex_unlock(&http_mutex);
}

void AddSentRequest(const FunctionCallbackInfo<Value> &info) {
  if (!info[0]->IsNumber()) {
    Error(module_type, "request cost must be number!");
    return;
  }

  unsigned int cost = To<uint32_t>(info[0]).ToChecked();

  uv_mutex_lock(&http_mutex);
  http_response_sent++;
  http_rt += cost;
  uv_mutex_unlock(&http_mutex);
}

void AddRequestTimeout(const FunctionCallbackInfo<Value> &info) {
  uv_mutex_lock(&http_mutex);
  http_request_timeout++;
  uv_mutex_unlock(&http_mutex);
}

void AddHttpStatusCode(const FunctionCallbackInfo<Value> &info) {
  if (!info[0]->IsNumber()) {
    Error(module_type, "request cost must be number!");
    return;
  }

  unsigned int status_code = To<uint32_t>(info[0]).ToChecked();
  if (status_code > 0 && status_code < 1000) {
    uv_mutex_lock(&http_mutex);
    status_codes[status_code]++;
    uv_mutex_unlock(&http_mutex);
  }
}

void WriteHttpStatus(bool log_format_alinode, uint32_t http_patch_timeout) {
  uv_mutex_lock(&http_mutex);

  double rt = 0.00;
  if (http_response_sent != 0) {
    rt = http_rt * 1.00 / http_response_sent;
  }

  if (log_format_alinode)
    Info("http",
         "live_http_request: %d, "
         "http_request_handled: %d, "
         "http_response_sent: %d, "
         "http_rt: %.2lf",
         live_http_request, http_response_sent, http_response_sent, rt);
  else {
    string format = "";
    for (int i = 0; i < 1000; i++) {
      int count = status_codes[i];
      if (count > 0 && format.length() < 1536) {
        format += "res" XPROFILER_BLURRY_TAG + std::to_string(i) + ": " +
                  std::to_string(count) + ", ";
      }
    }

    Info("http",
         "%s"
         "live_http_request: %d, "
         "http_response_close: %d, "
         "http_response_sent: %d, "
         "http_request_timeout: %d, "
         "http_patch_timeout: %d, "
         "http_rt: %.2lf",
         format.c_str(), live_http_request, http_response_close,
         http_response_sent, http_request_timeout, http_patch_timeout, rt);
  }

  // reset
  live_http_request = 0;
  http_response_sent = 0;
  http_response_close = 0;
  http_request_timeout = 0;
  http_rt = 0;
  for (int i = 0; i < 1000; i++) {
    status_codes[i] = 0;
  }

  uv_mutex_unlock(&http_mutex);
}
}  // namespace xprofiler