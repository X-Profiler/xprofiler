#ifndef XPROFILER_SRC_JSAPI_UTILS_H
#define XPROFILER_SRC_JSAPI_UTILS_H

#include "nan.h"

namespace xprofiler {
void CheckSocketPath(const Nan::FunctionCallbackInfo<v8::Value>& info);
}

#endif /* XPROFILER_SRC_JSAPI_UTILS_H */
