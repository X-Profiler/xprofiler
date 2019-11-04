#include "uv.h"

#include "../configure.h"
#include "../logger.h"
#include "../utils.h"
#include "cpu.h"
#include "gc.h"
#include "heap.h"

namespace xprofiler {
using Nan::False;
using Nan::ThrowTypeError;
using Nan::True;

uv_thread_t uv_log_thread;

static void CreateUvThread(void *data) {
  uint64_t last_loop_time = uv_hrtime();
  while (1) {
    // sleep 1s for releasing cpu
    Sleep(1);

    // set now cpu usage
    SetNowCpuUsage();

    // check if need to write performance logs to file
    if (uv_hrtime() - last_loop_time >= GetLogInterval() * 10e8) {
      last_loop_time = uv_hrtime();
      bool log_format_alinode = GetFormatAsAlinode();

      // write cpu info
      WriteCpuUsageInPeriod(log_format_alinode);

      // write heap memory info
      WriteMemoryInfoToLog(log_format_alinode);

      // write gc status
      WriteGcStatusToLog(log_format_alinode);
    }
  }
}

void RunLogBypass(const FunctionCallbackInfo<Value> &info) {
  int rc = 0;
  // init memory statistics callback
  rc = InitMemoryAsyncCallback();
  if (rc != 0) {
    ThrowTypeError("xprofiler: init memory statistics async callback failed!");
    info.GetReturnValue().Set(False());
    return;
  }
  UnrefAsyncHandle();
  Info("init", "xprofiler memory statistics async callback setted.");

  // init gc hooks
  rc = InitGcStatusHooks();
  if (rc != 0) {
    ThrowTypeError("xprofiler: init gc hooks failed!");
    info.GetReturnValue().Set(False());
    return;
  }
  Info("init", "gc hooks setted.");

  // init log thread
  rc = uv_thread_create(&uv_log_thread, CreateUvThread, nullptr);
  if (rc != 0) {
    ThrowTypeError("xprofiler: create uv log thread failed!");
    info.GetReturnValue().Set(False());
    return;
  }
  Info("init", "xprofiler log thread created.");

  info.GetReturnValue().Set(True());
}
} // namespace xprofiler
