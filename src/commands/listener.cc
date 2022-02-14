#include "../logger.h"
#include "../platform/platform.h"
#include "dump.h"
#include "nan.h"
#include "parser.h"
#include "uv.h"

namespace xprofiler {
using Nan::False;
using Nan::ThrowTypeError;
using Nan::True;

static uv_thread_t uv_commands_listener_thread;

static void CreateCommandsListenerThread(void* unused) {
  CreateIpcServer(ParseCmd);
}

void RunCommandsListener(const FunctionCallbackInfo<Value>& info) {
  int rc = 0;
  // init commands listener thread
  rc = uv_thread_create(&uv_commands_listener_thread,
                        CreateCommandsListenerThread, nullptr);
  if (rc != 0) {
    ThrowTypeError("xprofiler: create uv commands listener thread failed!");
    info.GetReturnValue().Set(False());
    return;
  }
  Info("init", "commands listener: listener thread created.");

  info.GetReturnValue().Set(True());
}
}  // namespace xprofiler
