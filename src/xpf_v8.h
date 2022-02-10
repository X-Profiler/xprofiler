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
#if NODE_MODULE_VERSION > NODE_0_10_MODULE_VERSION
  inline HandleScope(v8::Isolate* isolate) : scope_(isolate) {}
  inline static int NumberOfHandles(v8::Isolate* isolate) {
    return v8::HandleScope::NumberOfHandles(isolate);
  }
#else
  inline HandleScope(v8::Isolate* isolate) : scope() {}
  inline static int NumberOfHandles(v8::Isolate* isolate) {
    return v8::HandleScope::NumberOfHandles();
  }
#endif

 private:
  // Make it hard to create heap-allocated or illegal handle scopes by
  // disallowing certain operations.
  HandleScope(const HandleScope&) = delete;
  void operator=(const HandleScope&) = delete;
  void* operator new(size_t size) = delete;
  void operator delete(void*, size_t) = delete;
};
}  // namespace xprofiler

#endif /* XPROFILER_SRC_XPF_V8_H */
