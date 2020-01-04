#include "../configure.h"
#include "../library/utils.h"
#include "../logger.h"
#include "cpu.h"
#include "gc.h"
#include "heap.h"
#include "http.h"
#include "libuv.h"
#include "uv.h"

namespace xprofiler {
using Nan::False;
using Nan::ThrowTypeError;
using Nan::True;

static uv_thread_t uv_log_thread;

static void CreateLogThread(void *unused) {
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

      // get heap memory info
      GetMemoryInfo();

      // get libuv handles info
      GetLibuvHandles();

      // sleep 1s for executing async callback
      Sleep(1);

      // write cpu info
      WriteCpuUsageInPeriod(log_format_alinode);

      // write heap memory info
      WriteMemoryInfoToLog(log_format_alinode);

      // write gc status
      WriteGcStatusToLog(log_format_alinode);

      // write libuv handle info
      WriteLibuvHandleInfoToLog(log_format_alinode);

      // write http status
      WriteHttpStatus(log_format_alinode);
    }
  }
}

#define CHECK(fn, log)                  \
  rc = fn();                            \
  if (rc != 0) {                        \
    ThrowTypeError("xprofiler: " log);  \
    info.GetReturnValue().Set(False()); \
    return;                             \
  }

void RunLogBypass(const FunctionCallbackInfo<Value> &info) {
  int rc = 0;
  // init memory statistics callback
  CHECK(InitMemoryAsyncCallback,
        "init memory statistics async callback failed!")
  UnrefMemoryAsyncHandle();
  Info("init", "logbypass: memory statistics async callback setted.");

  // init libuv handle statistics callback
  CHECK(InitLibuvAsyncCallback,
        "init libuv handle statistics async callback failed!")
  UnrefLibuvAsyncHandle();
  Info("init", "logbypass: libuv handle statistics async callback setted.");

  // init gc hooks
  CHECK(InitGcStatusHooks, "init gc hooks failed!")
  Info("init", "logbypass: gc hooks setted.");

  // init http status
  CHECK(InitHttpStatus, "init http status failed!")
  Info("init", "logbypass: http status inited.");

  // init log thread
  rc = uv_thread_create(&uv_log_thread, CreateLogThread, nullptr);
  if (rc != 0) {
    ThrowTypeError("xprofiler: create uv log thread failed!");
    info.GetReturnValue().Set(False());
    return;
  }
  Info("init", "logbypass: log thread created.");

  info.GetReturnValue().Set(True());
}

#undef CHECK
}  // namespace xprofiler
