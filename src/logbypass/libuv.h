#ifndef _SRC_LOGBYPASS_LIBUV_H
#define _SRC_LOGBYPASS_LIBUV_H

namespace xprofiler {

#define HANDLE_DEFAULT_VALUE 0

#define INIT_HANDLE(name) int active_##name##_handles = HANDLE_DEFAULT_VALUE;

#define RESET_HANDLE(name) active_##name##_handles = HANDLE_DEFAULT_VALUE;

typedef struct {
  INIT_HANDLE(file)
  INIT_HANDLE(tcp)
  INIT_HANDLE(udp)
  INIT_HANDLE(timer)

  // reset record
  void reset() {
    RESET_HANDLE(file)
    RESET_HANDLE(tcp)
    RESET_HANDLE(udp)
    RESET_HANDLE(timer)
  }
} uv_handle_statistics_t;

void WriteLibuvHandleInfoToLog(bool log_format_alinode);
} // namespace xprofiler

#endif