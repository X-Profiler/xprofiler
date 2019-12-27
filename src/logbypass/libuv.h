#ifndef _SRC_LOGBYPASS_LIBUV_H
#define _SRC_LOGBYPASS_LIBUV_H

namespace xprofiler {

#define HANDLE_DEFAULT_VALUE 0

#define INIT_UV_HANDLE(name)                          \
  int active_##name##_handles = HANDLE_DEFAULT_VALUE; \
  int active_and_ref_##name##_handles = HANDLE_DEFAULT_VALUE;

#define RESET_UV_HANDLE(name)                     \
  active_##name##_handles = HANDLE_DEFAULT_VALUE; \
  active_and_ref_##name##_handles = HANDLE_DEFAULT_VALUE;

typedef struct {
  INIT_UV_HANDLE(file)
  INIT_UV_HANDLE(tcp)
  INIT_UV_HANDLE(udp)
  INIT_UV_HANDLE(timer)

  // reset record
  void reset() {
    RESET_UV_HANDLE(file)
    RESET_UV_HANDLE(tcp)
    RESET_UV_HANDLE(udp)
    RESET_UV_HANDLE(timer)
  }
} uv_handle_statistics_t;

int InitLibuvAsyncCallback();
void UnrefLibuvAsyncHandle();
void GetLibuvHandles();
void WriteLibuvHandleInfoToLog(bool log_format_alinode);
}  // namespace xprofiler

#endif