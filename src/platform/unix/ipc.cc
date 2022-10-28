#if defined(__APPLE__) || defined(__linux__)

#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>

#include "configure-inl.h"
#include "logger.h"

namespace xprofiler {
using Nan::FunctionCallbackInfo;
using Nan::New;
using Nan::To;
using std::string;
using v8::Boolean;
using v8::Value;

static struct sockaddr_un server_addr;
static struct sockaddr_un client_addr;

static const char module_type[] = "ipc";

#define CLIENT_BUFFER_SIZE 4096

#define TEARDOWN(message)             \
  Error(module_type, message);        \
  error_closed = true;                \
  shutdown(new_client_fd, SHUT_RDWR); \
  close(new_client_fd);

static string GetSocketPath() {
  string filename =
      GetLogDir() + "/xprofiler-uds-path-" + std::to_string(getpid()) + ".sock";
  return filename;
}

static bool CheckSocketPathIllegal(bool log_error) {
  string filename = GetSocketPath();
  bool illegal = filename.length() > sizeof(server_addr.sun_path) - 1;
  if (illegal && log_error) {
    Error(module_type,
          "the length of <%s> is larger than sizeof(server_addr.sun_path) - 1 "
          "(which is %lu).",
          filename.c_str(), sizeof(server_addr.sun_path) - 1);
  }
  return illegal;
}

void CheckSocketPath(const FunctionCallbackInfo<Value>& info) {
  bool log_error = false;
  if (info[0]->IsBoolean()) {
    log_error = To<bool>(info[0]).ToChecked();
  }
  info.GetReturnValue().Set(New<Boolean>(!CheckSocketPathIllegal(log_error)));
}

void CreateIpcServer(void (*parsecmd)(char*)) {
  // create unix domain socket
  int server_fd = socket(AF_UNIX, SOCK_STREAM, 0);
  if (server_fd == -1) {
    Error(module_type, "create server socket failed.");
    return;
  }

  // check socket path illegal
  if (CheckSocketPathIllegal(false)) return;

  // get domain socket path
  string filename = GetSocketPath();

  Debug(module_type, "unix domain socket file name: %s.", filename.c_str());

  // set server addr
  server_addr.sun_family = AF_UNIX;
  unlink(filename.c_str());
  strcpy(server_addr.sun_path, filename.c_str());

  // bind fd
  int bind_res =
      bind(server_fd, (struct sockaddr*)&server_addr, sizeof(server_addr));
  if (bind_res == -1) {
    Error(module_type, "bind fd %d failed.", server_fd);
    return;
  }

  // listen for connection
  int listen_res = listen(server_fd, 10);
  if (listen_res == -1) {
    Error(module_type, "listen fd %d for connections failed.", server_fd);
    return;
  }

  bool error_closed = false;

  while (1) {
    // sleep 1s when error occured
    if (error_closed) {
      sleep(1);
      error_closed = false;
    }

    // accept client
    Debug("ipc", "wait for client...");
    int new_client_fd = accept(server_fd, NULL, NULL);
    if (new_client_fd == -1) {
      TEARDOWN("accept wrong client.")
      continue;
    }

    // set timeout
    struct timeval tv_out;
    tv_out.tv_sec = 1;
    tv_out.tv_usec = 0;
    setsockopt(new_client_fd, SOL_SOCKET, SO_RCVTIMEO, &tv_out, sizeof(tv_out));

    // read client data
    char data_buffer[CLIENT_BUFFER_SIZE] = {0};
    ssize_t recv_res = recv(new_client_fd, data_buffer, CLIENT_BUFFER_SIZE, 0);
    if (recv_res == -1) {
      TEARDOWN("recv client data error.")
      continue;
    }

    parsecmd(data_buffer);

    shutdown(new_client_fd, SHUT_RDWR);
    close(new_client_fd);
  }
}

void CreateIpcClient(char* message) {
  // create unix domain socket
  int client_fd = socket(AF_UNIX, SOCK_STREAM, 0);
  if (client_fd == -1) {
    Error(module_type, "create client socket failed.");
    return;
  }

  // set client addr
  client_addr.sun_family = AF_UNIX;
  std::string filename = GetLogDir() + "/" + XPROFILER_IPC_PATH;
  if (filename.length() > sizeof(client_addr.sun_path) - 1) {
    Error(module_type,
          "the length of <%s> is larger than sizeof(client_addr.sun_path) - 1 "
          "(which is %lu).",
          filename.c_str(), sizeof(client_addr.sun_path) - 1);
    return;
  }

  strcpy(client_addr.sun_path, filename.c_str());

  // connect server
  int connect_res =
      connect(client_fd, (struct sockaddr*)&client_addr, sizeof(client_addr));
  if (connect_res == -1) {
    Error(module_type, "create client connect failed: %s.\n", filename.c_str());
    return;
  }

  // send message
  ssize_t send_res = send(client_fd, message, strlen(message), 0);
  if (send_res == -1) {
    Error("ipc", "send message failed: %s.", message);
    return;
  }
  Debug("ipc", "send message succeed: %s.", message);

  shutdown(client_fd, SHUT_RDWR);
  close(client_fd);
}
}  // namespace xprofiler

#endif
