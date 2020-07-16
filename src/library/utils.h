#ifndef _SRC_LIBRARY_UTILS_H
#define _SRC_LIBRARY_UTILS_H

#include "../logger.h"
#include "error.h"
#include "json.hpp"

namespace xprofiler {
using nlohmann::json;
using std::string;

void Sleep(int seconds);

string FmtMessage(const char *format, ...);

string RandNum();

string ConvertTime(string format);

template <typename T>
T GetJsonValue(json data, string key, XpfError &err) {
  T result = T();
  try {
    result = data[key].get<T>();
  } catch (json::exception &e) {
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

#endif