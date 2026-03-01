#ifndef XPROFILER_SRC_XPF_V8_H
#define XPROFILER_SRC_XPF_V8_H

#include "nan.h"
#include "util.h"

namespace xprofiler {
/**
 * A compatible class for v8::HandleScope. Not using Nan::HandleScope because of
 * its implicit v8::Isolate::GetCurrent.
 */
class HandleScope {
  v8::HandleScope scope_;

 public:
  inline HandleScope(v8::Isolate* isolate) : scope_(isolate) {}
  inline static int NumberOfHandles(v8::Isolate* isolate) {
    return v8::HandleScope::NumberOfHandles(isolate);
  }

 private:
  // Make it hard to create heap-allocated or illegal handle scopes by
  // disallowing certain operations.
  HandleScope(const HandleScope&) = delete;
  void operator=(const HandleScope&) = delete;
  void* operator new(size_t size) = delete;
  void operator delete(void*, size_t) = delete;
};

inline v8::Isolate* TryGetCurrentIsolate() {
#if NODE_MODULE_VERSION >= NODE_16_0_MODULE_VERSION
  return v8::Isolate::TryGetCurrent();
#else
  return v8::Isolate::GetCurrent();
#endif
}

}  // namespace xprofiler

#endif /* XPROFILER_SRC_XPF_V8_H */
