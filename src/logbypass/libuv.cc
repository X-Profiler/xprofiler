#include "libuv.h"

#include "../logger.h"
#include "configure-inl.h"
#include "uv.h"

namespace xprofiler {
static uv_async_t uv_statistics_trigger;
// libuv handles statistics
unsigned int active_handles = 0;
static uv_handle_statistics_t *uv_handle_statistics =
    new uv_handle_statistics_t;

#define UV_ADD(name)                                                    \
  if (uv_is_active(h)) uv_handle_statistics->active_##name##_handles++; \
  if (uv_is_active(h) && uv_has_ref(h))                                 \
    uv_handle_statistics->active_and_ref_##name##_handles++;

void LibuvWalkHandle(uv_handle_t *h, void *unused) {
  switch (h->type) {
    case UV_UNKNOWN_HANDLE:
      break;
    case UV_ASYNC:
      break;
    case UV_CHECK:
      break;
    case UV_FS_EVENT:
    case UV_FS_POLL:
      UV_ADD(file)
      break;
    case UV_HANDLE:
      break;
    case UV_IDLE:
      break;
    case UV_NAMED_PIPE:
      break;
    case UV_POLL:
      break;
    case UV_PREPARE:
      break;
    case UV_PROCESS:
      break;
    case UV_STREAM:
      break;
    case UV_TCP:
      UV_ADD(tcp)
      break;
    case UV_TIMER:
      UV_ADD(timer)
      break;
    case UV_TTY:
      break;
    case UV_UDP:
      UV_ADD(udp)
      break;
    case UV_SIGNAL:
      break;
    case UV_FILE:
      break;
    case UV_HANDLE_TYPE_MAX:
      break;
  }
}

void GetLibuvHandles(uv_async_t *handle) {
  active_handles = uv_default_loop()->active_handles;
  bool enable_log_uv_handles = GetEnableLogUvHandles();
  if (enable_log_uv_handles) {
    uv_handle_statistics->reset();
    uv_walk(uv_default_loop(), LibuvWalkHandle, nullptr);
  }
}

int InitLibuvAsyncCallback() {
  int rc =
      uv_async_init(uv_default_loop(), &uv_statistics_trigger, GetLibuvHandles);
  return rc;
}

void UnrefLibuvAsyncHandle() {
  uv_unref(reinterpret_cast<uv_handle_t *>(&uv_statistics_trigger));
}

void GetLibuvHandles() { uv_async_send(&uv_statistics_trigger); }

void WriteLibuvHandleInfoToLog(bool log_format_alinode) {
  bool enable_log_uv_handles = GetEnableLogUvHandles();

  if (log_format_alinode)
    Info("timer", "total_timer: %d, active_handles: %d",
         uv_handle_statistics->active_timer_handles, active_handles);
  else if (enable_log_uv_handles)
    Info("uv",
         "active_handles: %d, "
         "active_file_handles: %d, "
         "active_and_ref_file_handles: %d, "
         "active_tcp_handles: %d, "
         "active_and_ref_tcp_handles: %d, "
         "active_udp_handles: %d, "
         "active_and_ref_udp_handles: %d, "
         "active_timer_handles: %d, "
         "active_and_ref_timer_handles: %d",
         active_handles, uv_handle_statistics->active_file_handles,
         uv_handle_statistics->active_and_ref_file_handles,
         uv_handle_statistics->active_tcp_handles,
         uv_handle_statistics->active_and_ref_tcp_handles,
         uv_handle_statistics->active_udp_handles,
         uv_handle_statistics->active_and_ref_udp_handles,
         uv_handle_statistics->active_timer_handles,
         uv_handle_statistics->active_and_ref_timer_handles);
  else
    Info("uv", "active_handles: %d", active_handles);
}
}  // namespace xprofiler
