#ifndef _SRC_UTILS_H
#define _SRC_UTILS_H

#include "common.h"
#include "library/json.hpp"
#include "logger.h"

namespace xprofiler {
using std::exception;
using std::string;

void Sleep(int seconds);

string FmtMessage(const char *format, ...);

template <typename T> T GetJsonValue(json data, string key, CommonError &err) {
  T result = T();
  try {
    result = data[key].get<T>();
  } catch (exception &e) {
    Error("type_value", "%s <%s> type error: %s", data.dump().c_str(),
          key.c_str(), e.what());
    err = CommonError::Failure("<%s> type error: %s", key.c_str(), e.what());
  }
  return result;
}
} // namespace xprofiler

#endif