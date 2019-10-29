#include "uv.h"
#include <fstream>
#include <iostream>
#include <sys/time.h>

#ifndef _WIN32
#include "unistd.h"
#else
#include <windows.h>
#endif

#include "configure.h"
#include "logger.h"

namespace xprofiler {
using Nan::New;
using Nan::ThrowTypeError;
using Nan::To;
using Nan::Utf8String;
using v8::Local;
using v8::String;

static int kMaxMessageLength = 1024;
static int kMaxFormatLength = 1024;

// output
static std::ofstream info_stream;
static std::ofstream error_stream;
static std::ofstream debug_stream;

#ifndef _WIN32
std::string sep = "/";
#else
std::string sep = "\\";
#endif

#define WRITET_TO_FILE(type)                                                   \
  type##_stream.open(filepath, std::ios::app);                                 \
  type##_stream << log;                                                        \
  type##_stream.close();

void WriteToFile(const LOG_LEVEL output_level, char *log) {
  // get time of date
  char time_string_day[32];
  uv_timeval64_t tv;
  uv_gettimeofday(&tv);
  struct tm *ptm = localtime((time_t *)&tv.tv_sec);
  strftime(time_string_day, sizeof(time_string_day), "%Y%m%d", ptm);

  // get filepath and write to file
  std::string log_dir = GetLogDir();
  std::string filepath = log_dir + sep;
  bool log_format_alinode = GetFormatAsAlinode();
  switch (output_level) {
  case LOG_LEVEL::INFO:
    if (log_format_alinode) {
      filepath = filepath + "node-" + time_string_day + ".log";
    } else {
      filepath = filepath + "xprofiler-" + time_string_day + ".log";
    }
    WRITET_TO_FILE(info);
    break;
  case LOG_LEVEL::ERROR:
    if (log_format_alinode) {
      filepath = filepath + "node-error-" + time_string_day + ".log";
    } else {
      filepath = filepath + "xprofiler-error-" + time_string_day + ".log";
    }
    WRITET_TO_FILE(error);
    break;
  case LOG_LEVEL::DEBUG:
    if (log_format_alinode) {
      filepath = filepath + "node-debug-" + time_string_day + ".log";
    } else {
      filepath = filepath + "xprofiler-debug-" + time_string_day + ".log";
    }
    WRITET_TO_FILE(debug);
    break;
  default:
    break;
  }
}

void Log(const LOG_LEVEL output_level, const char *type, const char *format,
         ...) {
  LOG_LEVEL level = GetLogLevel();
  if (level < output_level) {
    return;
  }

  // check if alinode
  bool log_format_alinode = GetFormatAsAlinode();

  // time of day
  char time_string_ms[64];
  uv_timeval64_t tv;
  uv_gettimeofday(&tv);
  struct tm *ptm = localtime((time_t *)&tv.tv_sec);
  strftime(time_string_ms, sizeof(time_string_ms), "%Y-%m-%d %H:%M:%S", ptm);
  if (log_format_alinode) {
    snprintf(time_string_ms, sizeof(time_string_ms), "%s.%03d", time_string_ms,
             tv.tv_usec);
  }

  // log level
  std::string level_string = "";
  switch (output_level) {
  case LOG_LEVEL::INFO:
    level_string = "info";
    break;
  case LOG_LEVEL::ERROR:
    level_string = "error";
    break;
  case LOG_LEVEL::DEBUG:
    level_string = "debug";
    break;
  default:
    level_string = "unknown";
    break;
  }

  // pid
  std::string pid = std::to_string(getpid());

  // add log prefix
  char tmp_format[kMaxFormatLength];
  if (log_format_alinode) {
    snprintf(tmp_format, sizeof(tmp_format), "[%s] [%s] [%s] [%s] %s\n",
             time_string_ms, level_string.c_str(), type, pid.c_str(), format);
  } else {
    snprintf(tmp_format, sizeof(tmp_format), "[%s] [%s] [%s] [%s] [%s] %s\n",
             time_string_ms, XPROFILER_VERSION, level_string.c_str(), type,
             pid.c_str(), format);
  }

  // compose log
  char tmp_log[kMaxMessageLength];
  va_list arglist;
  va_start(arglist, format);
  vsnprintf(tmp_log, sizeof(tmp_log), tmp_format, arglist);
  va_end(arglist);

  WriteToFile(output_level, tmp_log);
}

void Info(const char *log_type, const char *format, ...) {
  va_list args;
  va_start(args, format);
  Log(LOG_LEVEL::INFO, log_type, format, args);
  va_end(args);
}

void Error(const char *log_type, const char *format, ...) {
  va_list args;
  va_start(args, format);
  Log(LOG_LEVEL::ERROR, log_type, format, args);
  va_end(args);
}

void Debug(const char *log_type, const char *format, ...) {
  va_list args;
  va_start(args, format);
  Log(LOG_LEVEL::DEBUG, log_type, format, args);
  va_end(args);
}

void JsLog(LOG_LEVEL output_level, const FunctionCallbackInfo<Value> &info) {
  if (!info[0]->IsString() || !info[1]->IsString()) {
    ThrowTypeError(
        New<String>("log type and content must be string!").ToLocalChecked());
    return;
  }
  Local<String> log_type_string = To<String>(info[0]).ToLocalChecked();
  Utf8String log_type(log_type_string);
  Local<String> log_content_string = To<String>(info[1]).ToLocalChecked();
  Utf8String log_content(log_content_string);
  Log(output_level, *log_type, *log_content);
}

void JsInfo(const FunctionCallbackInfo<Value> &info) {
  JsLog(LOG_LEVEL::INFO, info);
}

void JsError(const FunctionCallbackInfo<Value> &info) {
  JsLog(LOG_LEVEL::ERROR, info);
}
void JsDebug(const FunctionCallbackInfo<Value> &info) {
  JsLog(LOG_LEVEL::DEBUG, info);
}
}; // namespace xprofiler
