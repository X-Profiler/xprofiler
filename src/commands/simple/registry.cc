#include "commands/simple/registry.h"

#include "environment_data.h"
#include "environment_registry.h"
#include "process_data.h"

namespace xprofiler {
using nlohmann::json;

COMMAND_CALLBACK(ListEnvironments) {
  json environment_list = json::array();
  {
    EnvironmentRegistry* registry = ProcessData::Get()->environment_registry();
    EnvironmentRegistry::NoExitScope scope(registry);
    for (auto it : *registry) {
      json env = json::object();
      env["is_main_thread"] = it->is_main_thread();
      env["thread_id"] = it->thread_id();
      env["uptime"] = it->uptime();
      environment_list.push_back(env);
    }
  }
  json data = json::object();
  data["environments"] = environment_list;
  success(data);
}
}  // namespace xprofiler
