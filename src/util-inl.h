#pragma once

#include <nan.h>

#include "util.h"

namespace xprofiler {

inline v8::Local<v8::String> OneByteString(v8::Isolate* isolate,
                                           const char* data, int length) {
  // Nan get implicit isolate from Isolate::GetCurrent.
  DCHECK_EQ(isolate, v8::Isolate::GetCurrent());
  return Nan::NewOneByteString(reinterpret_cast<const uint8_t*>(data), length)
      .ToLocalChecked();
}

}  // namespace xprofiler
