#include "http.h"

#include "../logger.h"
#include "uv.h"

namespace xprofiler {
using Nan::To;

static uv_mutex_t http_mutex;

static const char module_type[] = "http";

// http server
static unsigned int live_http_request = 0;
static unsigned int http_response_close = 0;
static unsigned int http_response_sent = 0;
static unsigned int http_rt = 0;  // ms

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

void WriteHttpStatus(bool log_format_alinode) {
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
  else
    Info("http",
         "live_http_request: %d, "
         "http_response_close: %d, "
         "http_response_sent: %d, "
         "http_rt: %.2lf",
         live_http_request, http_response_close, http_response_sent, rt);

  // reset
  live_http_request = 0;
  http_response_sent = 0;
  http_response_close = 0;
  http_rt = 0;

  uv_mutex_unlock(&http_mutex);
}
}  // namespace xprofiler