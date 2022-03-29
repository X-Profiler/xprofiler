#ifndef XPROFILER_SRC_LOGBYPASS_LOG_H
#define XPROFILER_SRC_LOGBYPASS_LOG_H

#include "nan.h"
#include "xpf_thread.h"

namespace xprofiler {

class EnvironmentData;
class LogByPass final : public XpfThread {
 public:
  ~LogByPass() override{};

 protected:
  void ThreadEntry(uv_loop_t* loop) override;
  void ThreadAtExit() override;

 private:
  static void OnCpuInterval(uv_timer_t* handle);
  static void OnLogInterval(uv_timer_t* handle);

  void SendCollectStatistics();
  void CollectStatistics();
  void Write(EnvironmentData* env_data, bool log_format_alinode);

  uv_timer_t cpu_interval_;
  uv_timer_t log_interval_;
  bool next_log_ = false;
};

// javascript-accessible
void RunLogBypass(const Nan::FunctionCallbackInfo<v8::Value>& info);
}  // namespace xprofiler

#endif /* XPROFILER_SRC_LOGBYPASS_LOG_H */
