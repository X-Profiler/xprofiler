#include <time.h>

#include <string>

#include "nan.h"

namespace xprofiler {
using Nan::Get;
using Nan::GetCurrentContext;
using Nan::HandleScope;
using Nan::New;
using Nan::To;
using Nan::Utf8String;
using std::string;
using v8::Local;
using v8::Object;
using v8::String;
using v8::Value;

static time_t load_time;
static string global_node_version_string;

static void SetGlobalNodeVersion() {
  HandleScope scope;
  Local<Value> global_process =
      Get(GetCurrentContext()->Global(), New("process").ToLocalChecked())
          .ToLocalChecked();
  Local<Object> process_object = To<Object>(global_process).ToLocalChecked();
  Local<Value> node_version_value =
      Get(process_object, New("version").ToLocalChecked()).ToLocalChecked();
  Local<String> node_version_string =
      To<String>(node_version_value).ToLocalChecked();
  Utf8String node_version(node_version_string);
  global_node_version_string = (*node_version);
}

void InitGlobalVariables() {
  time(&load_time);
  SetGlobalNodeVersion();
}

unsigned long GetUptime() {
  time_t current_time;
  time(&current_time);
  return static_cast<unsigned long>(difftime(current_time, load_time));
}

string GetStartTime(string format) {
  char time_string_day[32];
  struct tm *ptm = localtime(&load_time);
  strftime(time_string_day, sizeof(time_string_day), format.c_str(), ptm);
  return (string)time_string_day;
}

string GetGlobalNodeVersion() { return global_node_version_string; }
}  // namespace xprofiler