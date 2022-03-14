#include "environment_registry.h"

namespace xprofiler {

EnvironmentRegistry::NoExitScope::NoExitScope(EnvironmentRegistry* registry)
    : registry_(registry), lock_(registry->mutex_) {
  registry_->disallow_exit_ = true;
}

EnvironmentRegistry::NoExitScope::~NoExitScope() {
  registry_->disallow_exit_ = false;
}

void EnvironmentRegistry::Register(v8::Isolate* isolate,
                                   std::unique_ptr<EnvironmentData> env) {
  CHECK(disallow_exit_);
  map_.emplace(isolate, std::move(env));
}

void EnvironmentRegistry::Unregister(v8::Isolate* isolate) {
  CHECK(disallow_exit_);
  CHECK_NE(map_.find(isolate), map_.end());
  map_.erase(isolate);
}

EnvironmentData* EnvironmentRegistry::Get(v8::Isolate* isolate) {
  CHECK(disallow_exit_);
  auto it = map_.find(isolate);
  CHECK_NE(it, map_.end());
  return it->second.get();
}

EnvironmentRegistry::Iterator EnvironmentRegistry::begin() {
  CHECK(disallow_exit_);
  return Iterator(map_.begin());
}

EnvironmentRegistry::Iterator EnvironmentRegistry::end() {
  CHECK(disallow_exit_);
  return Iterator(map_.end());
}

}  // namespace xprofiler
