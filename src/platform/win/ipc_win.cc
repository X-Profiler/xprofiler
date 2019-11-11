#ifdef _WIN32
#include "../../configure.h"
#include "../../logger.h"

#include "uv.h"
#include <windows.h>

namespace xprofiler {
using std::string;
using std::wstring;

static const char module_type[] = "ipc";

#define IN_AND_OUT_BUFFER_SIZE 4096

#define TEARDOWN(message)                                                      \
  Error(module_type, message);                                                 \
  error_closed = true;                                                         \
  CloseHandle(named_pipe);

#define CREATE_NAMED_PIPE                                                      \
  named_pipe = CreateNamedPipeW(                                               \
      lp_name, PIPE_ACCESS_DUPLEX | FILE_FLAG_OVERLAPPED | WRITE_DAC,          \
      PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT,                         \
      PIPE_UNLIMITED_INSTANCES, IN_AND_OUT_BUFFER_SIZE,                        \
      IN_AND_OUT_BUFFER_SIZE, 0, NULL);

wstring String2LPCWSTR(const string &s) {
  int len;
  int slength = (int)s.length() + 1;
  len = MultiByteToWideChar(CP_ACP, 0, s.c_str(), slength, 0, 0);
  wchar_t *buf = new wchar_t[len];
  MultiByteToWideChar(CP_ACP, 0, s.c_str(), slength, buf, len);
  wstring r(buf);
  delete[] buf;
  return r;
}

void CreateIpcServer(void parsecmd(char *)) {
  HANDLE named_pipe = NULL;
  string lp_name_string = "\\\\.\\pipe\\" + GetLogDir() +
                          "\\xprofiler-named-pipe-" + std::to_string(getpid());
  wstring lp_name_ws = String2LPCWSTR(lp_name_string);
  LPCWSTR lp_name = lp_name_ws.c_str();

  Debug(module_type, "win32 named pipe path: %s.", lp_name_string.c_str());

  bool error_closed = false;

  while (1) {
    // sleep 1s when error occured
    if (error_closed) {
      Sleep(1000);
      error_closed = false;
    }

    // create named pipe
    CREATE_NAMED_PIPE

    if (named_pipe == INVALID_HANDLE_VALUE) {
      TEARDOWN("create named pipe failed.")
      continue;
    }

    Debug(module_type, "wait for client...");
    bool connected = ConnectNamedPipe(named_pipe, NULL);
    if (!connected && GetLastError() != ERROR_IO_PENDING) {
      TEARDOWN("client connected failed.")
      continue;
    }

    Debug(module_type, "client connected.");

    // check this client's data
    bool need_read = false;
    uint64_t now = uv_hrtime();
    DWORD read_bytes = 0;
    while (uv_hrtime() - now < 10e8) {
      DWORD read_bytes_tmp = 0;
      DWORD total_bytes = 0;
      char tmp[IN_AND_OUT_BUFFER_SIZE] = {0};
      bool peek = PeekNamedPipe(named_pipe, tmp, IN_AND_OUT_BUFFER_SIZE,
                                &read_bytes_tmp, &total_bytes, NULL);
      read_bytes += read_bytes_tmp;
      Debug(
          module_type,
          "check should read file: peek (%d), read_bytes (%d), total_bytes(%d)",
          peek, read_bytes, total_bytes);
      if (!peek)
        break;
      if (read_bytes != 0 && read_bytes >= total_bytes) {
        need_read = true;
        break;
      }
      Sleep(100);
    }
    if (!need_read) {
      CloseHandle(named_pipe);
      continue;
    }

    // read client data
    DWORD data_length = 0;
    char data_buffer[IN_AND_OUT_BUFFER_SIZE] = {0};
    bool readed = ReadFile(named_pipe, data_buffer, IN_AND_OUT_BUFFER_SIZE,
                           &data_length, NULL);
    if (!readed || data_length == 0) {
      TEARDOWN("read client data failed.")
      continue;
    }
    data_buffer[data_length] = '\0';

    parsecmd(data_buffer);

    FlushFileBuffers(named_pipe);
    CloseHandle(named_pipe);
  }
}

void CreateIpcClient(char *message) {
  HANDLE named_pipe_client = NULL;
  string lp_name_string =
      "\\\\.\\pipe\\" + GetLogDir() + "\\" + XPROFILER_IPC_PATH;
  wstring lp_name_ws = String2LPCWSTR(lp_name_string);
  LPCWSTR lp_name = lp_name_ws.c_str();

  // check available named pipe
  bool has_named_pipe = WaitNamedPipeW(lp_name, NMPWAIT_USE_DEFAULT_WAIT);
  if (!has_named_pipe) {
    Error(module_type, "no named pipe: %s.", lp_name_string.c_str());
    return;
  }

  // open named pipe
  named_pipe_client =
      CreateFileW(lp_name, GENERIC_READ | GENERIC_WRITE, 0, NULL, OPEN_EXISTING,
                  FILE_FLAG_OVERLAPPED, NULL);
  if (named_pipe_client == INVALID_HANDLE_VALUE) {
    Error(module_type, "create file failed.");
    return;
  }

  // send message
  DWORD send_bytes = 0;
  bool written =
      WriteFile(named_pipe_client, message, strlen(message), &send_bytes, NULL);
  if (!written || send_bytes == 0) {
    Error(module_type, "send message failed: %s.", message);
    return;
  }
  Debug(module_type, "send message succeed: %s.", message);

  CloseHandle(named_pipe_client);
}
} // namespace xprofiler

#endif