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

std::unique_ptr<EnvironmentData> EnvironmentRegistry::Unregister(
    v8::Isolate* isolate) {
  CHECK(disallow_exit_);
  auto it = map_.find(isolate);
  CHECK_NE(it, map_.end());
  std::unique_ptr<EnvironmentData> env_data = std::move(it->second);
  map_.erase(it);
  return env_data;
}

EnvironmentData* EnvironmentRegistry::Get(v8::Isolate* isolate) {
  CHECK(disallow_exit_);
  auto it = map_.find(isolate);
  if (it == map_.end()) {
    return nullptr;
  }
  return it->second.get();
}

EnvironmentData* EnvironmentRegistry::Get(ThreadId thread_id) {
  CHECK(disallow_exit_);

  for (auto it : *this) {
    if (it->thread_id() == thread_id) {
      return it;
    }
  }

  return nullptr;
}

EnvironmentData* EnvironmentRegistry::GetMainThread() {
  CHECK(disallow_exit_);

  for (auto it : *this) {
    if (it->is_main_thread()) {
      return it;
    }
  }

  return nullptr;
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
