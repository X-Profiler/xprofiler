#include <stdarg.h>

#include <fstream>

#include "uv.h"

#ifdef _WIN32
#include <time.h>
#endif

#include "configure.h"
#include "platform/platform.h"

namespace xprofiler {
using Nan::New;
using Nan::ThrowTypeError;
using Nan::To;
using Nan::Utf8String;
using std::string;
using std::to_string;
using v8::Local;
using v8::String;

static const int kMaxMessageLength = 1024;
static const int kMaxFormatLength = 1024;

// output
static std::ofstream info_stream;
static std::ofstream error_stream;
static std::ofstream debug_stream;

#define WRITET_TO_FILE(type)                   \
  type##_stream.open(filepath, std::ios::app); \
  type##_stream << log;                        \
  type##_stream.close();

void WriteToFile(const LOG_LEVEL output_level, char *log) {
  // get time of date
  char time_string_day[32];
  time_t tt = time(NULL);
  struct tm *ptm = localtime(&tt);
  strftime(time_string_day, sizeof(time_string_day), "%Y%m%d", ptm);

  // get filepath and write to file
  string log_dir = GetLogDir();
  string filepath = log_dir + GetSep();
  bool log_format_alinode = GetFormatAsAlinode();
  string file_prefix = "xprofiler-";
  if (log_format_alinode) {
    file_prefix = "node-";
  }
  switch (output_level) {
    case LOG_LEVEL::LOG_INFO:
      filepath += file_prefix + time_string_day + ".log";
      WRITET_TO_FILE(info)
      break;
    case LOG_LEVEL::LOG_ERROR:
      filepath += file_prefix + "error-" + time_string_day + ".log";
      WRITET_TO_FILE(error)
      break;
    case LOG_LEVEL::LOG_DEBUG:
      filepath += file_prefix + "debug-" + time_string_day + ".log";
      WRITET_TO_FILE(debug)
      break;
    default:
      break;
  }
}

void Log(const LOG_LEVEL output_level, const char *type, const char *format,
         va_list arglist = nullptr) {
  LOG_LEVEL level = GetLogLevel();
  if (level < output_level) {
    return;
  }

  // check if alinode
  bool log_format_alinode = GetFormatAsAlinode();

  // time of day
  char time_string_ms[64];
  char time_string_ms_alinode[64];
  time_t tt = time(NULL);
  struct tm *ptm = localtime(&tt);
  strftime(time_string_ms, sizeof(time_string_ms), "%Y-%m-%d %H:%M:%S", ptm);
  if (log_format_alinode) {
    uv_timeval64_t tv;
    uv_gettimeofday(&tv);
    snprintf(time_string_ms_alinode, sizeof(time_string_ms_alinode), "%s.%06d",
             time_string_ms, tv.tv_usec);
  }

  // log level
  string level_string = "";
  switch (output_level) {
    case LOG_LEVEL::LOG_INFO:
      level_string = "info";
      break;
    case LOG_LEVEL::LOG_ERROR:
      level_string = "error";
      break;
    case LOG_LEVEL::LOG_DEBUG:
      level_string = "debug";
      break;
    default:
      level_string = "unknown";
      break;
  }

  // get pid
  string pid = to_string(GetPid());

  // add log prefix
  char tmp_format[kMaxFormatLength];
  if (log_format_alinode) {
    snprintf(tmp_format, sizeof(tmp_format), "[%s] [%s] [%s] [%s] %s\n",
             time_string_ms_alinode, level_string.c_str(), type, pid.c_str(),
             format);
  } else {
    snprintf(tmp_format, sizeof(tmp_format), "[%s] [%s] [%s] [%s] [%s] %s\n",
             time_string_ms, level_string.c_str(), type, pid.c_str(),
             XPROFILER_VERSION, format);
  }

  // compose log
  char tmp_log[kMaxMessageLength];
  if (arglist != nullptr)
    vsnprintf(tmp_log, sizeof(tmp_log), tmp_format, arglist);
  else
    snprintf(tmp_log, sizeof(tmp_log), "%s", tmp_format);

  // get log type
  switch (GetLogType()) {
    // ttl
    case LOG_TYPE::LOG_TO_TTL:
      printf("%s", tmp_log);
      break;
    // file
    case LOG_TYPE::LOG_TO_FILE:
      WriteToFile(output_level, tmp_log);
      break;
    default:
      break;
  }
}

#define V(level)                                 \
  va_list args;                                  \
  va_start(args, format);                        \
  Log(LOG_LEVEL::level, log_type, format, args); \
  va_end(args);
void Info(const char *log_type, const char *format, ...) { V(LOG_INFO) }
void Error(const char *log_type, const char *format, ...) { V(LOG_ERROR) }
void Debug(const char *log_type, const char *format, ...) { V(LOG_DEBUG) }
#undef V

#define V(level)                                                               \
  if (!info[0]->IsString() || !info[1]->IsString()) {                          \
    ThrowTypeError(                                                            \
        New<String>("log type and content must be string!").ToLocalChecked()); \
    return;                                                                    \
  }                                                                            \
  Local<String> log_type_string = To<String>(info[0]).ToLocalChecked();        \
  Utf8String log_type(log_type_string);                                        \
  Local<String> log_content_string = To<String>(info[1]).ToLocalChecked();     \
  Utf8String log_content(log_content_string);                                  \
  Log(LOG_LEVEL::level, *log_type, *log_content);
void JsInfo(const FunctionCallbackInfo<Value> &info) { V(LOG_INFO) }
void JsError(const FunctionCallbackInfo<Value> &info) { V(LOG_ERROR) }
void JsDebug(const FunctionCallbackInfo<Value> &info) { V(LOG_DEBUG) }
#undef V

};  // namespace xprofiler
