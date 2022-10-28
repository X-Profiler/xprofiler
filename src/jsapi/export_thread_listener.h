#ifndef XPROFILER_SRC_JSAPI_THREAD_LISTENER_H
#define XPROFILER_SRC_JSAPI_THREAD_LISTENER_H

#include "nan.h"

namespace xprofiler {
void RunCommandsListener(const Nan::FunctionCallbackInfo<v8::Value>& info);
}

#endif /* XPROFILER_SRC_JSAPI_THREAD_LISTENER_H */
