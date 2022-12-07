#include "commands/simple/config.h"

#include "configure-inl.h"
#include "library/json.hpp"
#include "library/utils.h"

namespace xprofiler {
using nlohmann::json;
using std::exception;

#define HANDLE_CONFIG_SETTING(key, type)                                  \
  if (options.find(key) != options.end()) {                               \
    type value;                                                           \
    XpfError err;                                                         \
    value = GetJsonValue<type>(options, key, err);                        \
    if (err.Fail()) {                                                     \
      error_message = format("%s", err.GetErrMessage());                  \
      return;                                                             \
    }                                                                     \
    ProcessData::Get()->config_store()->SetConfig<type>(key, value);      \
    setted = true;                                                        \
    data[key] = ProcessData::Get()->config_store()->GetConfig<type>(key); \
  }

COMMAND_CALLBACK(GetXprofilerConfig) {
  json data;

  ProcessData::Get()->config_store()->TraverseConfig(
      [&data](std::string key, std::string type, bool configurable) {
        if (type == "string") data[key] = GetConfig<std::string>(key);
        if (type == "number") data[key] = GetConfig<uint32_t>(key);
        if (type == "boolean") data[key] = GetConfig<bool>(key);
      });

  success(data);
}

COMMAND_CALLBACK(SetXprofilerConfig) {
  json options = command["options"];
  json data;
  std::string error_message = "";
  bool setted = false;

  ProcessData::Get()->config_store()->TraverseConfig(
      [&](std::string key, std::string type, bool configurable) {
        if (!configurable) return;
        if (type == "string") HANDLE_CONFIG_SETTING(key, std::string)
        if (type == "number") HANDLE_CONFIG_SETTING(key, uint32_t)
        if (type == "boolean") HANDLE_CONFIG_SETTING(key, bool)
      });

  if (error_message != "")
    error(error_message);
  else if (!setted)
    error(format("not support setting config %s", options.dump().c_str()));
  else
    success(data);
}
}  // namespace xprofiler
