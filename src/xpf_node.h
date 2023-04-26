#pragma once

#include "nan.h"

namespace xprofiler {

/**
 * Node.js compatibility helpers
 */

inline void AtExit(v8::Isolate* isolate, void (*cb)(void* arg), void* arg) {
  node::Environment* env =
      node::GetCurrentEnvironment(isolate->GetCurrentContext());
  node::AtExit(env, cb, arg);
}

}  // namespace xprofiler
