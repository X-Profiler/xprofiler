#include "xpf_thread.h"

namespace xprofiler {

void PrintUvHandleInformation(uv_loop_t* loop, FILE* stream) {
  struct Info {
    FILE* stream;
    size_t num_handles;
  };

  Info info{stream, 0};

  fprintf(stream, "uv loop at [%p] has open handles:\n", loop);

  uv_walk(
      loop,
      [](uv_handle_t* handle, void* arg) {
        Info* info = static_cast<Info*>(arg);
        FILE* stream = info->stream;
        info->num_handles++;

        fprintf(stream, "[%p] %s%s\n", handle,
                uv_handle_type_name(handle->type),
                uv_is_active(handle) ? " (active)" : "");

        void* close_cb = reinterpret_cast<void*>(handle->close_cb);
        fprintf(stream, "\tClose callback: %p\n", close_cb);

        fprintf(stream, "\tData: %p\n", handle->data);
      },
      &info);

  fprintf(stream, "uv loop at [%p] has %zu open handles in total\n", loop,
          info.num_handles);
}

void CheckedUvLoopClose(uv_loop_t* loop) {
  if (uv_loop_close(loop) == 0) return;

  PrintUvHandleInformation(loop, stderr);

  fflush(stderr);
  // Finally, abort.
  CHECK(0 && "uv_loop_close() while having open handles");
}

XpfThread::XpfThread() : started_(false) {}

XpfThread::~XpfThread() { CHECK_EQ(started_, false); }

void XpfThread::StartIfNeeded() {
  CHECK_EQ(started_, false);
  Mutex::ScopedLock scoped_lock(thread_start_lock_);
  CHECK_EQ(0, uv_thread_create(&thread_, ThreadMain, this));
  thread_start_condition_.Wait(scoped_lock);
  started_ = true;
}

void XpfThread::Join() {
  CHECK_EQ(started_, true);
  if (started_) {
    uv_async_send(&stop_async_);
    uv_thread_join(&thread_);
  }
  started_ = false;
}

// static
void XpfThread::ThreadMain(void* arg) {
  XpfThread* that = reinterpret_cast<XpfThread*>(arg);
  uv_loop_init(&that->loop_);

  {
    Mutex::ScopedLock scoped_lock(that->thread_start_lock_);

    uv_async_init(&that->loop_, &that->stop_async_, StopRequest);

    that->ThreadEntry(&that->loop_);

    that->thread_start_condition_.Broadcast(scoped_lock);
  }
  uv_run(&that->loop_, UV_RUN_DEFAULT);

  CheckedUvLoopClose(&that->loop_);
}

// static
void XpfThread::StopRequest(uv_async_t* handle) {
  XpfThread* that = ContainerOf(&XpfThread::stop_async_, handle);
  that->ThreadAtExit();
  uv_close(reinterpret_cast<uv_handle_t*>(&that->stop_async_), nullptr);
}

}  // namespace xprofiler
