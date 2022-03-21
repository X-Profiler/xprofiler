#ifndef XPROFILER_SRC_ENVIRONMENT_REGISTRY_H
#define XPROFILER_SRC_ENVIRONMENT_REGISTRY_H

#include <memory>
#include <unordered_map>

#include "environment_data.h"
#include "nan.h"
#include "xpf_mutex-inl.h"

namespace xprofiler {

class EnvironmentRegistry {
  using Map =
      std::unordered_map<v8::Isolate*, std::unique_ptr<EnvironmentData>>;

 public:
  class NoExitScope {
   public:
    explicit NoExitScope(EnvironmentRegistry* registry);
    NoExitScope(const NoExitScope& other) = delete;
    ~NoExitScope();

   private:
    EnvironmentRegistry* registry_;
    Mutex::ScopedLock lock_;
  };

  class Iterator {
   public:
    EnvironmentData* operator*() { return it_->second.get(); };
    bool operator==(const Iterator& other) { return it_ == other.it_; };
    bool operator==(Iterator& other) { return it_ == other.it_; };
    bool operator!=(const Iterator& other) { return it_ != other.it_; };
    bool operator!=(Iterator& other) { return it_ != other.it_; };

    Iterator operator++() { return Iterator(it_++); }

   private:
    friend EnvironmentRegistry;
    explicit Iterator(Map::iterator it) : it_(it){};
    Map::iterator it_;
  };

  EnvironmentRegistry(){};
  // Disallow copy
  EnvironmentRegistry(const EnvironmentRegistry& other) = delete;

  void Register(v8::Isolate* isolate, std::unique_ptr<EnvironmentData> env);
  std::unique_ptr<EnvironmentData> Unregister(v8::Isolate* isolate);
  EnvironmentData* Get(v8::Isolate* isolate);
  EnvironmentData* GetMainThread();

  Iterator begin();
  Iterator end();

 private:
  friend NoExitScope;

  bool disallow_exit_ = false;
  Mutex mutex_;
  Map map_;
};

}  // namespace xprofiler

#endif /* XPROFILER_SRC_ENVIRONMENT_REGISTRY_H */
