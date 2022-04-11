#ifndef XPROFILER_SRC_COMMANDS_PARSER_H
#define XPROFILER_SRC_COMMANDS_PARSER_H

#include <functional>
#include <string>

#include "library/json.hpp"

namespace xprofiler {
void ParseCmd(char* command);

// commands
#define COMMAND_CALLBACK(cb)                                               \
  void cb(nlohmann::json command, std::string (*format)(const char*, ...), \
          std::function<void(nlohmann::json)> success,                     \
          std::function<void(std::string)> error)

}  // namespace xprofiler

#endif /* XPROFILER_SRC_COMMANDS_PARSER_H */
