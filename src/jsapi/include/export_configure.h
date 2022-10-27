#ifndef XPROFILER_SRC_JSAPI_CONFIGURE_H
#define XPROFILER_SRC_JSAPI_CONFIGURE_H

#include "nan.h"

namespace xprofiler {
void Configure(const Nan::FunctionCallbackInfo<v8::Value>& info);
void GetConfig(const Nan::FunctionCallbackInfo<v8::Value>& info);

}  // namespace xprofiler

#endif /* XPROFILER_SRC_JSAPI_CONFIGURE_H */
