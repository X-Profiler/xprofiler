#include "listener.h"

#include "dump.h"
#include "logger.h"
#include "nan.h"
#include "parser.h"
#include "platform/platform.h"
#include "util.h"
#include "uv.h"

namespace xprofiler {
using Nan::False;
using Nan::ThrowTypeError;
using Nan::True;

class CommandListenerThread {
 public:
  CommandListenerThread() {
    // init log thread
    CHECK_EQ(0, uv_thread_create(&thread_, Main, nullptr));
    Info("init", "commands listener: listener thread created.");
  }

  ~CommandListenerThread() { CHECK_EQ(0, uv_thread_join(&thread_)); }

  static void Main(void* unused) { CreateIpcServer(ParseCmd); }

 private:
  uv_thread_t thread_;
};

namespace per_process {
std::unique_ptr<CommandListenerThread> command_listener_thread;
}

void RunCommandsListener(const FunctionCallbackInfo<Value>& info) {
  per_process::command_listener_thread =
      std::make_unique<CommandListenerThread>();

  // init dump action node isolate
  int rc = InitDumpAction();
  if (rc != 0) {
    ThrowTypeError("xprofiler: init dump action failed!");
    info.GetReturnValue().Set(False());
    return;
  }
  UnrefDumpActionAsyncHandle();
  Info("init", "commands listener: dump action init succeed.");

  info.GetReturnValue().Set(True());
}

void StopCommandsListener() { per_process::command_listener_thread.reset(); }
}  // namespace xprofiler
