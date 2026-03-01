#ifndef XPROFILER_SRC_JSAPI_ENVIRONMENT_H
#define XPROFILER_SRC_JSAPI_ENVIRONMENT_H
#include "nan.h"

namespace xprofiler {
void JsSetupEnvironmentData(const Nan::FunctionCallbackInfo<v8::Value>& info);
}

#endif /* XPROFILER_SRC_JSAPI_ENVIRONMENT_H */