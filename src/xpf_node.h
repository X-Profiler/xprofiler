#ifndef XPROFILER_SRC_XPF_NODE_H
#define XPROFILER_SRC_XPF_NODE_H

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

#endif /* XPROFILER_SRC_XPF_NODE_H */
