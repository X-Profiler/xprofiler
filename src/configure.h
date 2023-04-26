#pragma once

#include <unordered_map>

#include "library/json.hpp"
#include "logger.h"

namespace xprofiler {

struct Description {
  std::string type;
  bool configurable;
};

using ConfigDescription = std::unordered_map<std::string, Description*>;

template <typename T>
T GetConfig(std::string key);

class ConfigStore {
  // TODO(legendecas): accessors.
 public:
  template <typename T>
  T GetConfig(std::string key) {
    return static_cast<T>(config_[key]);
  }

  template <typename T>
  void SetConfig(std::string key, T value) {
    config_[key] = value;
  }

  void DescribeConfig(std::string key, std::string type, bool configurable) {
    Description* desc = new Description;
    desc->type = type;
    desc->configurable = configurable;
    desc_.insert(std::make_pair(key, desc));
  }

  void TraverseConfig(
      std::function<void(std::string&, std::string&, bool)> callback) {
    for (auto it = desc_.begin(); it != desc_.end(); ++it) {
      std::string key = it->first;
      Description* desc = it->second;
      callback(key, desc->type, desc->configurable);
    }
  }

 private:
  nlohmann::json config_;
  ConfigDescription desc_;
};

}  // namespace xprofiler
