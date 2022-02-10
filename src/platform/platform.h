#ifndef _SRC_PLATFORM_H
#define _SRC_PLATFORM_H
#include <string>

#include "../library/writer.h"
#include "nan.h"

namespace xprofiler {
using Nan::FunctionCallbackInfo;
using std::string;
using v8::Value;

// cpu
double GetNowCpuUsage();

// ipc
void CreateIpcServer(void (*parsecmd)(char *));
void CreateIpcClient(char *message);

// utils
void SleepCrossPlatform(int seconds);
string GetSep();
int GetPid();
// for node-v8.x & ndoe-v10.x
#if (NODE_MODULE_VERSION < NODE_12_0_MODULE_VERSION)
typedef struct {
  int64_t tv_sec;
  int32_t tv_usec;
} uv_timeval64_t;
int uv_gettimeofday(uv_timeval64_t *tv);
#endif

// node report
string GetPcAddress(void *pc);
string GetOsVersion();
void PrintNativeStack(JSONWriter *writer);
void PrintSystemEnv(JSONWriter *writer);
void PrintResourceLimits(JSONWriter *writer);
void PrintLoadedLibraries(JSONWriter *writer);

// js binding
void CheckSocketPath(const FunctionCallbackInfo<Value> &info);
}  // namespace xprofiler

#endif
