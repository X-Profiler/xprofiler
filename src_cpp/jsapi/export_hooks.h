#ifndef XPROFILER_SRC_JSAPI_HOOKS_H
#define XPROFILER_SRC_JSAPI_HOOKS_H

#include "nan.h"

namespace xprofiler {
void SetHooks(const Nan::FunctionCallbackInfo<v8::Value>& info);
}

#endif /* XPROFILER_SRC_JSAPI_HOOKS_H */
