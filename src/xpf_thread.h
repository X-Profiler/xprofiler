#ifndef XPROFILER_SRC_XPF_THREAD_H
#define XPROFILER_SRC_XPF_THREAD_H

#include "util.h"
#include "uv.h"
#include "xpf_mutex-inl.h"

namespace xprofiler {

class XpfThread {
 public:
  XpfThread();
  virtual ~XpfThread();

  XpfThread(const XpfThread& other) = delete;

  void StartIfNeeded();
  void Join();

 protected:
  virtual void ThreadEntry(uv_loop_t* loop) = 0;
  virtual void ThreadAtExit() = 0;

 private:
  static void ThreadMain(void* arg);
  static void StopRequest(uv_async_t* handle);

  bool started_;

  // For setting up interthread communications
  Mutex thread_start_lock_;
  ConditionVariable thread_start_condition_;

  // The IO thread runs its own event loop to implement the server off
  // the main thread.
  uv_loop_t loop_;
  uv_thread_t thread_;
  uv_async_t stop_async_;
};

}  // namespace xprofiler

#endif /* XPROFILER_SRC_XPF_THREAD_H */
