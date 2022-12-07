#include "libuv.h"

#include "configure-inl.h"
#include "environment_data.h"
#include "logger.h"
#include "uv.h"

namespace xprofiler {

#define UV_ADD(name)                                                    \
  if (uv_is_active(h)) uv_handle_statistics->active_##name##_handles++; \
  if (uv_is_active(h) && uv_has_ref(h))                                 \
    uv_handle_statistics->active_and_ref_##name##_handles++;

void LibuvWalkHandle(uv_handle_t* h, void* arg) {
  UvHandleStatistics* uv_handle_statistics =
      reinterpret_cast<UvHandleStatistics*>(arg);

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

void CollectLibuvHandleStatistics(EnvironmentData* env_data) {
  uv_loop_t* loop = node::GetCurrentEventLoop(env_data->isolate());
  UvHandleStatistics* uv_handle_statistics = env_data->uv_handle_statistics();

  if (GetConfig<bool>("enable_log_uv_handles")) {
    uv_handle_statistics->reset();
    uv_walk(loop, LibuvWalkHandle, uv_handle_statistics);
  }
  uv_handle_statistics->active_handles = loop->active_handles;
}

void WriteLibuvHandleInfoToLog(EnvironmentData* env_data,
                               bool log_format_alinode) {
  UvHandleStatistics* uv_handle_statistics = env_data->uv_handle_statistics();

  if (log_format_alinode) {
    Info("timer", "total_timer: %d, active_handles: %d",
         uv_handle_statistics->active_timer_handles,
         uv_handle_statistics->active_handles);
  } else if (GetConfig<bool>("enable_log_uv_handles")) {
    InfoT("uv", env_data->thread_id(),
          "active_handles: %ld, "
          "active_file_handles: %d, "
          "active_and_ref_file_handles: %d, "
          "active_tcp_handles: %d, "
          "active_and_ref_tcp_handles: %d, "
          "active_udp_handles: %d, "
          "active_and_ref_udp_handles: %d, "
          "active_timer_handles: %d, "
          "active_and_ref_timer_handles: %d",
          uv_handle_statistics->active_handles,
          uv_handle_statistics->active_file_handles,
          uv_handle_statistics->active_and_ref_file_handles,
          uv_handle_statistics->active_tcp_handles,
          uv_handle_statistics->active_and_ref_tcp_handles,
          uv_handle_statistics->active_udp_handles,
          uv_handle_statistics->active_and_ref_udp_handles,
          uv_handle_statistics->active_timer_handles,
          uv_handle_statistics->active_and_ref_timer_handles);
  } else {
    InfoT("uv", env_data->thread_id(), "active_handles: %ld",
          uv_handle_statistics->active_handles);
  }
}
}  // namespace xprofiler
