#include "uv.h"

#include "../configure.h"
#include "../logger.h"
#include "../utils.h"
#include "cpu.h"
#include "log.h"

namespace xprofiler {

uv_thread_t uv_log_thread;

static void CreateUvThread(void *data) {
  uint64_t last_loop_time = uv_hrtime();
  while (1) {
    // sleep 1s for releasing cpu
    SleepSeconds(1);

    // set now cpu usage
    SetNowCpuUsage();

    // check if need to write performance logs to file
    if (uv_hrtime() - last_loop_time >= GetLogInterval() * 10e8) {
      last_loop_time = uv_hrtime();
      bool log_format_alinode = GetFormatAsAlinode();
      // write cpu info
      WriteCpuUsageInPeriod(log_format_alinode);
    }
  }
}

void RunLogBypass(const FunctionCallbackInfo<Value> &info) {
  int rc = uv_thread_create(&uv_log_thread, CreateUvThread, nullptr);
  if (rc != 0) {
    Nan::ThrowError("xprofiler: create uv log thread failed!");
    info.GetReturnValue().Set(Nan::False());
    return;
  }
  Info("init", "xprofiler log thread created.");
  info.GetReturnValue().Set(Nan::True());
}
} // namespace xprofiler
