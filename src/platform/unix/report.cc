#if defined(__APPLE__) || defined(__linux__)

#include <cxxabi.h>
#include <dlfcn.h>
#include <execinfo.h>

#include <string>

#include "../../library/writer.h"

extern char** environ;

namespace xprofiler {
using std::string;

static const int kMaxFrams = 256;

string GetPcAddress(void* pc) {
  char buf[64];
  snprintf(buf, sizeof(buf), "%p", pc);
  return (string)buf;
}

#if (defined(__linux__) && !defined(__GLIBC__))
void PrintNativeStack(JSONWriter* writer) {
  writer->json_arraystart("nativeStacks");
  writer->json_arrayend();
}
#else
void PrintNativeStack(JSONWriter* writer) {
  writer->json_arraystart("nativeStacks");
  void* frames[256];
  char buf[64];

  const int size = backtrace(frames, kMaxFrams);

  for (int i = 2; i < size; i++) {
    writer->json_start();
    snprintf(buf, sizeof(buf), "%p", frames[i]);
    writer->json_keyvalue("pcAddress", buf);

    Dl_info info;
    if (dladdr(frames[i], &info)) {
      if (info.dli_sname != nullptr) {
        if (char* demangled = abi::__cxa_demangle(info.dli_sname, 0, 0, 0)) {
          writer->json_keyvalue("symbolName", demangled);
          free(demangled);
        } else
          writer->json_keyvalue("symbolName", info.dli_sname);
      }
      if (info.dli_fname != nullptr)
        writer->json_keyvalue("sharedObjectName", info.dli_fname);
    }
    writer->json_end();
  }
  writer->json_arrayend();
}
#endif

void PrintSystemEnv(JSONWriter* writer) {
  writer->json_arraystart("env");

  int index = 1;
  char* env_var = *environ;
  while (env_var != nullptr) {
    writer->json_element(env_var);
    env_var = *(environ + index++);
  }

  writer->json_arrayend();
}
}  // namespace xprofiler

#endif