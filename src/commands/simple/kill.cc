#include "commands/simple/kill.h"

#ifdef _WIN32
#include <windows.h>
#else
#include <sys/types.h>
#include <unistd.h>

#include <cerrno>
#include <csignal>
#include <cstring>
#endif

#include "library/utils.h"

namespace xprofiler {
using nlohmann::json;

COMMAND_CALLBACK(KillProcess) {
  XpfError err;
  int target_pid = GetJsonValue<int>(command["options"], "target_pid", err);
  if (err.Fail()) {
    error(format("%s", err.GetErrMessage()));
    return;
  }

  if (target_pid <= 0) {
    error(format("target_pid must be a positive integer"));
    return;
  }

  if (target_pid == 1) {
    error(format("cannot kill init process (pid 1)"));
    return;
  }

#ifdef _WIN32
  if (static_cast<DWORD>(target_pid) == GetCurrentProcessId()) {
    error(format("cannot kill self process"));
    return;
  }

  // Check permission and kill on Windows
  HANDLE hProcess =
      OpenProcess(PROCESS_TERMINATE, FALSE, static_cast<DWORD>(target_pid));
  if (hProcess == NULL) {
    DWORD win_err = GetLastError();
    if (win_err == ERROR_ACCESS_DENIED) {
      error(
          format("permission denied: cannot terminate process %d", target_pid));
    } else {
      error(
          format("process %d does not exist or cannot be opened", target_pid));
    }
    return;
  }
  BOOL result = TerminateProcess(hProcess, 1);
  CloseHandle(hProcess);
  if (!result) {
    error(format("failed to terminate process %d", target_pid));
    return;
  }
#else
  if (static_cast<pid_t>(target_pid) == getpid()) {
    error(format("cannot kill self process"));
    return;
  }

  // Check permission using kill(pid, 0)
  if (kill(static_cast<pid_t>(target_pid), 0) == -1) {
    if (errno == EPERM) {
      error(format("permission denied: cannot send signal to process %d",
                   target_pid));
    } else if (errno == ESRCH) {
      error(format("process %d does not exist", target_pid));
    } else {
      error(format("failed to check process %d: %s", target_pid,
                   strerror(errno)));
    }
    return;
  }

  // Send SIGKILL
  if (kill(static_cast<pid_t>(target_pid), SIGKILL) == -1) {
    error(format("failed to kill process %d: %s", target_pid, strerror(errno)));
    return;
  }
#endif

  json data;
  data["target_pid"] = target_pid;
  data["message"] = format("process %d killed successfully", target_pid);
  success(data);
}
}  // namespace xprofiler
