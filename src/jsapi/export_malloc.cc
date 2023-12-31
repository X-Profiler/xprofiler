#include "export_malloc.h"

#include "configure-inl.h"
#include "platform.h"

namespace xprofiler {
using Nan::FunctionCallbackInfo;
using v8::Value;

void InitMallopt(const FunctionCallbackInfo<Value>& info) {
  if (GetConfig<bool>("enable_avoid_rss_leak")) {
    int threshold = GetConfig<int>("m_mmap_threshold");
    threshold = threshold > 128 ? threshold : 128;
    AvoidRssLeak(threshold);
  }
}
}  // namespace xprofiler
