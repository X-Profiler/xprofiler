#include "uv.h"

#include "../configure.h"
#include "../utils.h"
#include "cpu.h"
#include "log.h"

namespace xprofiler {

uv_thread_t uv_log_thread;

static void CreateUvThread(void *data) {
  while (1) {
    // sleep 1s for releasing cpu
    Sleep(1);

    // set now cpu usage
    SetNowCpuUsage();
  }
}

void RunLogBypass(const FunctionCallbackInfo<Value> &info) {
  int rc = uv_thread_create(&uv_log_thread, CreateUvThread, nullptr);
  if (rc != 0) {
    Nan::ThrowError("xprofiler: create uv log thread failed!");
    info.GetReturnValue().Set(Nan::False());
    return;
  }
  info.GetReturnValue().Set(Nan::True());
}
} // namespace xprofiler
