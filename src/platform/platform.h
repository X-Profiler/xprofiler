#ifndef XPROFILER_SRC_PLATFORM_PLATFORM_H
#define XPROFILER_SRC_PLATFORM_PLATFORM_H
#include <string>

#include "library/writer.h"
#include "nan.h"

namespace xprofiler {

// cpu
double GetNowCpuUsage();

// ipc
void CreateIpcServer(void (*parsecmd)(char*));
void CreateIpcClient(char* message);

// utils
void SleepCrossPlatform(int seconds);
std::string GetSep();
int GetPid();
// for node-v8.x & ndoe-v10.x
#if (NODE_MODULE_VERSION < NODE_12_0_MODULE_VERSION)
typedef struct {
  int64_t tv_sec;
  int32_t tv_usec;
} uv_timeval64_t;
int uv_gettimeofday(uv_timeval64_t* tv);
#endif

// node report
std::string GetPcAddress(void* pc);
std::string GetOsVersion();
void PrintNativeStack(JSONWriter* writer);
void PrintSystemEnv(JSONWriter* writer);
void PrintResourceLimits(JSONWriter* writer);
void PrintLoadedLibraries(JSONWriter* writer);

// coredumper
void WriteCore(std::string filename);

// js binding
void CheckSocketPath(const Nan::FunctionCallbackInfo<v8::Value>& info);
}  // namespace xprofiler

#endif /* XPROFILER_SRC_PLATFORM_PLATFORM_H */
