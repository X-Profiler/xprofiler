#ifndef XPROFILER_SRC_PLATFORM_PLATFORM_H
#define XPROFILER_SRC_PLATFORM_PLATFORM_H
#include <string>

#include "library/writer.h"

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

// node report
std::string GetPcAddress(void* pc);
std::string GetOsVersion();
void PrintNativeStack(JSONWriter* writer);
void PrintSystemEnv(JSONWriter* writer);
void PrintResourceLimits(JSONWriter* writer);
void PrintLoadedLibraries(JSONWriter* writer);

// coredumper
void WriteCore(std::string filename);
}  // namespace xprofiler

#endif /* XPROFILER_SRC_PLATFORM_PLATFORM_H */
