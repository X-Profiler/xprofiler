#ifndef _SRC_LOGBYPASS_LIBUV_H
#define _SRC_LOGBYPASS_LIBUV_H

namespace xprofiler {

#define HANDLE_DEFAULT_VALUE 0

typedef struct {
#define V(name) int active_##name##_handles = HANDLE_DEFAULT_VALUE;
  V(file)
  V(tcp)
  V(udp)
  V(timer)
#undef V

  // reset record
  void reset() {
#define V(name) active_##name##_handles = HANDLE_DEFAULT_VALUE;
    V(file)
    V(tcp)
    V(udp)
    V(timer)
#undef V
  }
} uv_handle_statistics_t;

void WriteLibuvHandleInfoToLog(bool log_format_alinode);
} // namespace xprofiler

#endif