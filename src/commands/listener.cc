#include "commands/dump.h"
#include "commands/parser.h"
#include "logger.h"
#include "nan.h"
#include "platform/platform.h"
#include "uv.h"
#include "xpf_mutex-inl.h"

namespace xprofiler {
using Nan::False;
using Nan::FunctionCallbackInfo;
using Nan::ThrowTypeError;
using Nan::True;
using v8::Value;

namespace per_process {
Mutex command_listener_mutex;
bool command_listener_thread_created;
uv_thread_t uv_commands_listener_thread;
}  // namespace per_process

static void CreateCommandsListenerThread(void* unused) {
  CreateIpcServer(ParseCmd);
}

void RunCommandsListener(const FunctionCallbackInfo<Value>& info) {
  Mutex::ScopedLock lock(per_process::command_listener_mutex);
  if (per_process::command_listener_thread_created) {
    info.GetReturnValue().Set(True());
    return;
  }
  int rc = 0;
  // init commands listener thread
  rc = uv_thread_create(&per_process::uv_commands_listener_thread,
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
