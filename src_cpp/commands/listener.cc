#include "commands/dump.h"
#include "commands/parser.h"
#include "environment_data.h"
#include "logger.h"
#include "platform/platform.h"
#include "uv.h"
#include "xpf_mutex-inl.h"

namespace xprofiler {

namespace per_process {
Mutex command_listener_mutex;
bool command_listener_thread_created;
uv_thread_t uv_commands_listener_thread;
}  // namespace per_process

static void CreateCommandsListenerThread(void* unused) {
  CreateIpcServer(ParseCmd);
}

int StartCommandsListener(EnvironmentData* env_data) {
  Mutex::ScopedLock lock(per_process::command_listener_mutex);
  if (per_process::command_listener_thread_created) {
    return 0;
  }

  // init commands listener thread
  int rc = uv_thread_create(&per_process::uv_commands_listener_thread,
                            CreateCommandsListenerThread, nullptr);
  if (rc == 0) {
    InfoT("init", env_data->thread_id(),
          "commands listener: listener thread created.");
  }

  return rc;
}
}  // namespace xprofiler
