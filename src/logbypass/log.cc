#include "log.h"

#include "configure-inl.h"
#include "cpu.h"
#include "environment_data.h"
#include "gc.h"
#include "heap.h"
#include "http.h"
#include "library/utils.h"
#include "libuv.h"
#include "logger.h"
#include "uv.h"

namespace xprofiler {
using Nan::False;
using Nan::ThrowTypeError;
using Nan::True;

namespace per_process {
uv_thread_t uv_log_thread;
}

static void LogThreadMain(void* unused) {
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

      EnvironmentData* env_data = EnvironmentData::GetCurrent();

      env_data->SendCollectStatistics();
      // sleep 1s for executing async callback
      Sleep(1);

      // write cpu info
      WriteCpuUsageInPeriod(log_format_alinode);

      // write heap memory info
      WriteMemoryInfoToLog(env_data, log_format_alinode);

      // write gc status
      WriteGcStatusToLog(env_data, log_format_alinode);

      // write libuv handle info
      WriteLibuvHandleInfoToLog(env_data, log_format_alinode);

      // write http status
      WriteHttpStatus(env_data, log_format_alinode, GetPatchHttpTimeout());
    }
  }
}

void RunLogBypass(const FunctionCallbackInfo<Value>& info) {
  int rc = 0;
  // init gc hooks
  InitGcStatusHooks();
  Info("init", "logbypass: gc hooks setted.");

  // init log thread
  rc = uv_thread_create(&per_process::uv_log_thread, LogThreadMain, nullptr);
  if (rc != 0) {
    ThrowTypeError("xprofiler: create uv log thread failed!");
    info.GetReturnValue().Set(False());
    return;
  }
  Info("init", "logbypass: log thread created.");

  info.GetReturnValue().Set(True());
}
}  // namespace xprofiler
