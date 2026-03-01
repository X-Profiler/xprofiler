#ifndef XPROFILER_SRC_LIBRARY_UTILS_H
#define XPROFILER_SRC_LIBRARY_UTILS_H

#include "error.h"
#include "json.hpp"
#include "logger.h"

namespace xprofiler {
void Sleep(int seconds);

std::string FmtMessage(const char* format, ...);

std::string ConvertTime(std::string format);

template <typename T>
T GetJsonValue(nlohmann::json data, std::string key, XpfError& err) {
  T result = T();
  try {
    result = data[key].get<T>();
  } catch (nlohmann::json::exception& e) {
    // format error message
    // ref: https://en.cppreference.com/w/cpp/error/exception/what
    char error_message[256];
    snprintf(error_message, sizeof(error_message), "%s", e.what());

    // log error message
    Error("type_value", "%s <%s> type error: %s", data.dump().c_str(),
          key.c_str(), error_message);
    err = XpfError::Failure("<%s> type error: %s", key.c_str(), error_message);
  }
  return result;
}
}  // namespace xprofiler

#endif /* XPROFILER_SRC_LIBRARY_UTILS_H */
