#ifndef XPROFILER_SRC_XPF_NODE_H
#define XPROFILER_SRC_XPF_NODE_H

#include "nan.h"

namespace xprofiler {

/**
 * Node.js compatibility helpers
 */

inline void AtExit(v8::Isolate* isolate, void (*cb)(void* arg), void* arg) {
#if NODE_MODULE_VERSION >= NODE_10_0_MODULE_VERSION
  // node::GetCurrentEnvironment is available since v10.x.
  // We don't need to support multiple environments before v10.x.
  node::Environment* env =
      node::GetCurrentEnvironment(isolate->GetCurrentContext());
  node::AtExit(env, cb, arg);
#else
  node::AtExit(cb, arg);
#endif
}

}  // namespace xprofiler

#endif /* XPROFILER_SRC_XPF_NODE_H */
