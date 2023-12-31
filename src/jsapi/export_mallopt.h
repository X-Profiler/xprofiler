#ifndef XPROFILER_SRC_JSAPI_MALLOC_H
#define XPROFILER_SRC_JSAPI_MALLOC_H

#include "nan.h"

namespace xprofiler {
void InitMallopt(const Nan::FunctionCallbackInfo<v8::Value>& info);
}

#endif /* XPROFILER_SRC_JSAPI_MALLOC_H */
