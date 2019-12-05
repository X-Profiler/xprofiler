#include "uv_statistics.h"

#include <sstream>

#include "../../platform/platform.h"
#include "uv.h"

namespace xprofiler {
using std::ostringstream;
using std::string;

static const char *SignoString(int signo) {
#define SIGNO_CASE(e) \
  case e:             \
    return #e;
  switch (signo) {
#ifdef SIGHUP
    SIGNO_CASE(SIGHUP);
#endif
#ifdef SIGINT
    SIGNO_CASE(SIGINT);
#endif
#ifdef SIGQUIT
    SIGNO_CASE(SIGQUIT);
#endif
#ifdef SIGILL
    SIGNO_CASE(SIGILL);
#endif
#ifdef SIGTRAP
    SIGNO_CASE(SIGTRAP);
#endif
#ifdef SIGABRT
    SIGNO_CASE(SIGABRT);
#endif
#ifdef SIGIOT
#if SIGABRT != SIGIOT
    SIGNO_CASE(SIGIOT);
#endif
#endif
#ifdef SIGBUS
    SIGNO_CASE(SIGBUS);
#endif
#ifdef SIGFPE
    SIGNO_CASE(SIGFPE);
#endif
#ifdef SIGKILL
    SIGNO_CASE(SIGKILL);
#endif
#ifdef SIGUSR1
    SIGNO_CASE(SIGUSR1);
#endif
#ifdef SIGSEGV
    SIGNO_CASE(SIGSEGV);
#endif
#ifdef SIGUSR2
    SIGNO_CASE(SIGUSR2);
#endif
#ifdef SIGPIPE
    SIGNO_CASE(SIGPIPE);
#endif
#ifdef SIGALRM
    SIGNO_CASE(SIGALRM);
#endif
    SIGNO_CASE(SIGTERM);
#ifdef SIGCHLD
    SIGNO_CASE(SIGCHLD);
#endif
#ifdef SIGSTKFLT
    SIGNO_CASE(SIGSTKFLT);
#endif
#ifdef SIGCONT
    SIGNO_CASE(SIGCONT);
#endif
#ifdef SIGSTOP
    SIGNO_CASE(SIGSTOP);
#endif
#ifdef SIGTSTP
    SIGNO_CASE(SIGTSTP);
#endif
#ifdef SIGBREAK
    SIGNO_CASE(SIGBREAK);
#endif
#ifdef SIGTTIN
    SIGNO_CASE(SIGTTIN);
#endif
#ifdef SIGTTOU
    SIGNO_CASE(SIGTTOU);
#endif
#ifdef SIGURG
    SIGNO_CASE(SIGURG);
#endif
#ifdef SIGXCPU
    SIGNO_CASE(SIGXCPU);
#endif
#ifdef SIGXFSZ
    SIGNO_CASE(SIGXFSZ);
#endif
#ifdef SIGVTALRM
    SIGNO_CASE(SIGVTALRM);
#endif
#ifdef SIGPROF
    SIGNO_CASE(SIGPROF);
#endif
#ifdef SIGWINCH
    SIGNO_CASE(SIGWINCH);
#endif
#ifdef SIGIO
    SIGNO_CASE(SIGIO);
#endif
#ifdef SIGPOLL
#if SIGPOLL != SIGIO
    SIGNO_CASE(SIGPOLL);
#endif
#endif
#ifdef SIGLOST
#if SIGLOST != SIGABRT
    SIGNO_CASE(SIGLOST);
#endif
#endif
#ifdef SIGPWR
#if SIGPWR != SIGLOST
    SIGNO_CASE(SIGPWR);
#endif
#endif
#ifdef SIGINFO
#if !defined(SIGPWR) || SIGINFO != SIGPWR
    SIGNO_CASE(SIGINFO);
#endif
#endif
#ifdef SIGSYS
    SIGNO_CASE(SIGSYS);
#endif
    default:
      return "unknown";
  }
}

static void reportPath(uv_handle_t *h, ostringstream &out) {
  char *buffer = nullptr;
  int rc = -1;
  size_t size = 0;
  uv_any_handle *handle = (uv_any_handle *)h;
  switch (h->type) {
    case UV_FS_EVENT: {
      rc = uv_fs_event_getpath(&(handle->fs_event), buffer, &size);
      break;
    }
    case UV_FS_POLL: {
      rc = uv_fs_poll_getpath(&(handle->fs_poll), buffer, &size);
      break;
    }
    default:
      break;
  }
  if (rc == UV_ENOBUFS) {
    buffer = static_cast<char *>(malloc(size));
    switch (h->type) {
      case UV_FS_EVENT: {
        rc = uv_fs_event_getpath(&(handle->fs_event), buffer, &size);
        break;
      }
      case UV_FS_POLL: {
        rc = uv_fs_poll_getpath(&(handle->fs_poll), buffer, &size);
        break;
      }
      default:
        break;
    }
    if (rc == 0) {
      string name(buffer, size);
      out << "filename: " << name;
    }
    free(buffer);
  }
}

static void reportEndpoint(uv_handle_t *h, struct sockaddr *addr,
                           const char *prefix, ostringstream &out) {
  uv_getnameinfo_t endpoint;
  if (uv_getnameinfo(h->loop, &endpoint, nullptr, addr, NI_NUMERICSERV) == 0) {
    out << prefix << endpoint.host << ":" << endpoint.service;
  } else {
    char host[INET6_ADDRSTRLEN];
    const int family = addr->sa_family;
    const void *src =
        family == AF_INET
            ? static_cast<void *>(
                  &(reinterpret_cast<sockaddr_in *>(addr)->sin_addr))
            : static_cast<void *>(
                  &(reinterpret_cast<sockaddr_in6 *>(addr)->sin6_addr));
    if (uv_inet_ntop(family, src, host, sizeof(host)) == 0) {
      const int port =
          ntohs(family == AF_INET
                    ? reinterpret_cast<sockaddr_in *>(addr)->sin_port
                    : reinterpret_cast<sockaddr_in6 *>(addr)->sin6_port);
      out << prefix << host << ":" << port;
    }
  }
}

static void reportEndpoints(uv_handle_t *h, ostringstream &out) {
  struct sockaddr_storage addr_storage;
  struct sockaddr *addr = (sockaddr *)&addr_storage;
  uv_any_handle *handle = (uv_any_handle *)h;
  int addr_size = sizeof(addr_storage);
  int rc = -1;

  switch (h->type) {
    case UV_UDP: {
      rc = uv_udp_getsockname(&(handle->udp), addr, &addr_size);
      break;
    }
    case UV_TCP: {
      rc = uv_tcp_getsockname(&(handle->tcp), addr, &addr_size);
      break;
    }
    default:
      break;
  }
  if (rc == 0) {
    reportEndpoint(h, addr, "", out);

    if (h->type == UV_TCP) {
      rc = uv_tcp_getpeername(&(handle->tcp), addr, &addr_size);
      if (rc == 0) {
        reportEndpoint(h, addr, " connected to ", out);
      } else if (rc == UV_ENOTCONN) {
        out << " (not connected)";
      }
    }
  }
}

static void walkHandle(uv_handle_t *h, void *arg) {
  string type;
  JSONWriter *writer = reinterpret_cast<JSONWriter *>(arg);
  uv_any_handle *handle = (uv_any_handle *)h;
  ostringstream data;

  writer->json_start();

  switch (h->type) {
    case UV_UNKNOWN_HANDLE:
      type = "unknown";
      break;
    case UV_ASYNC:
      type = "async";
      break;
    case UV_CHECK:
      type = "check";
      break;
    case UV_FS_EVENT: {
      type = "fs_event";
      reportPath(h, data);
      break;
    }
    case UV_FS_POLL: {
      type = "fs_poll";
      reportPath(h, data);
      break;
    }
    case UV_HANDLE:
      type = "handle";
      break;
    case UV_IDLE:
      type = "idle";
      break;
    case UV_NAMED_PIPE:
      type = "pipe";
      break;
    case UV_POLL:
      type = "poll";
      break;
    case UV_PREPARE:
      type = "prepare";
      break;
    case UV_PROCESS:
      type = "process";
      data << "pid: " << handle->process.pid;
      break;
    case UV_STREAM:
      type = "stream";
      break;
    case UV_TCP: {
      type = "tcp";
      reportEndpoints(h, data);
      break;
    }
    case UV_TIMER: {
#if defined(_WIN32) && (UV_VERSION_HEX < ((1 << 16) | (22 << 8)))
      uint64_t due = handle->timer.due;
#else
      uint64_t due = handle->timer.timeout;
#endif
      uint64_t now = uv_now(handle->timer.loop);
      type = "timer";
      data << "repeat: " << uv_timer_get_repeat(&(handle->timer));
      if (due > now) {
        data << ", timeout in: " << (due - now) << " ms";
      } else {
        data << ", timeout expired: " << (now - due) << " ms ago";
      }
      break;
    }
    case UV_TTY: {
      int height, width, rc;
      type = "tty";
      rc = uv_tty_get_winsize(&(handle->tty), &width, &height);
      if (rc == 0) {
        data << "width: " << width << ", height: " << height;
      }
      break;
    }
    case UV_UDP: {
      type = "udp";
      reportEndpoints(h, data);
      break;
    }
    case UV_SIGNAL: {
      type = "signal";
      data << "signum: " << handle->signal.signum << " ("
           << SignoString(handle->signal.signum) << ")";
      break;
    }
    case UV_FILE:
      type = "file";
      break;
    case UV_HANDLE_TYPE_MAX:
      type = "max";
      break;
  }

  if (h->type == UV_TCP || h->type == UV_UDP
#ifndef _WIN32
      || h->type == UV_NAMED_PIPE
#endif
  ) {
    int send_size = 0;
    int recv_size = 0;
    if (h->type == UV_TCP || h->type == UV_UDP) {
      data << ", ";
    }
    uv_send_buffer_size(h, &send_size);
    uv_recv_buffer_size(h, &recv_size);
    data << "send buffer size: " << send_size
         << ", recv buffer size: " << recv_size;
  }

  if (h->type == UV_TCP || h->type == UV_NAMED_PIPE || h->type == UV_TTY ||
      h->type == UV_UDP || h->type == UV_POLL) {
    uv_os_fd_t fd_v;
    uv_os_fd_t *fd = &fd_v;
    int rc = uv_fileno(h, fd);
#ifndef _WIN32
    if (rc == 0) {
      switch (fd_v) {
        case 0:
          data << ", stdin";
          break;
        case 1:
          data << ", stdout";
          break;
        case 2:
          data << ", stderr";
          break;
        default:
          data << ", file descriptor: " << static_cast<int>(fd_v);
          break;
      }
    }
#endif
  }

  if (h->type == UV_TCP || h->type == UV_NAMED_PIPE || h->type == UV_TTY) {
    data << ", write queue size: " << handle->stream.write_queue_size;
    data << (uv_is_readable(&handle->stream) ? ", readable" : "")
         << (uv_is_writable(&handle->stream) ? ", writable" : "");
  }

  // for empty string
  data << "";

  writer->json_keyvalue("type", type);
  writer->json_keyvalue("address", GetPcAddress(static_cast<void *>(h)));
  writer->json_keyvalue("hasRed", uv_has_ref(h));
  writer->json_keyvalue("isActive", uv_is_active(h));
  writer->json_keyvalue("detail", data.str());

  writer->json_end();
}

void SetUvStatistics(JSONWriter *writer) {
  writer->json_arraystart("libuvHandles");
  uv_walk(uv_default_loop(), walkHandle, (void *)writer);
  writer->json_arrayend();
}
}  // namespace xprofiler