#if defined(__APPLE__) || defined(__linux__)

#ifndef __STDC_FORMAT_MACROS
#define __STDC_FORMAT_MACROS
#endif

#if defined(__linux__) && !defined(__GLIBC__) || defined(__UCLIBC__) || \
    defined(_AIX)
#define HAVE_EXECINFO_H 0
#else
#define HAVE_EXECINFO_H 1
#endif

#if HAVE_EXECINFO_H
#include <cxxabi.h>
#include <execinfo.h>
#endif

#include <dlfcn.h>
#include <inttypes.h>
#include <sys/resource.h>
#include <sys/utsname.h>

#include <sstream>
#include <string>

#ifdef __linux__
#include <link.h>
#endif

#ifdef __APPLE__
#include <mach-o/dyld.h>
#endif

#include "library/writer.h"

extern char** environ;

namespace xprofiler {
using std::ostringstream;
using std::string;

static const int kMaxFrams = 256;

#if defined(_MSC_VER) && _MSC_VER < 1900
#define arraysize(a) (sizeof(a) / sizeof(*a))
#else
template <typename T, size_t N>
constexpr size_t arraysize(const T (&)[N]) {
  return N;
}
#endif

string GetPcAddress(void* pc) {
  char buf[64];
  snprintf(buf, sizeof(buf), "%p", pc);
  return (string)buf;
}

#if (HAVE_EXECINFO_H)
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
#else
void PrintNativeStack(JSONWriter* writer) {
  writer->json_arraystart("nativeStacks");
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

void PrintResourceLimits(JSONWriter* writer) {
  writer->json_arraystart("resourceLimits");

  const static struct {
    const char* description;
    int id;
  } rlimit_strings[] = {
    {"core file size (blocks)", RLIMIT_CORE},
    {"data seg size (kbytes)", RLIMIT_DATA},
    {"file size (blocks)", RLIMIT_FSIZE},
#if !(defined(_AIX) || defined(__MVS__))
    {"max locked memory (bytes)", RLIMIT_MEMLOCK},
#endif
#if !(defined(__sun) || defined(__MVS__))
    {"max memory size (kbytes)", RLIMIT_RSS},
#endif
    {"open files", RLIMIT_NOFILE},
    {"stack size (bytes)", RLIMIT_STACK},
    {"cpu time (seconds)", RLIMIT_CPU},
#if !(defined(__sun) || defined(__MVS__))
    {"max user processes", RLIMIT_NPROC},
#endif
    {"virtual memory (kbytes)", RLIMIT_AS}
  };

  struct rlimit limit;
  char buf[64];

  for (size_t i = 0; i < arraysize(rlimit_strings); i++) {
    writer->json_start();

    if (getrlimit(rlimit_strings[i].id, &limit) == 0) {
      writer->json_keyvalue("type", rlimit_strings[i].description);
      if (limit.rlim_cur == RLIM_INFINITY) {
        writer->json_keyvalue("softLimit", "unlimited");
      } else {
#if (defined(__linux__) && !defined(__GLIBC__))
        snprintf(buf, sizeof(buf), "%lld", limit.rlim_cur);
        writer->json_keyvalue("softLimit", buf);
#else
        snprintf(buf, sizeof(buf), "%" PRIu64, limit.rlim_cur);
        writer->json_keyvalue("softLimit", buf);
#endif
      }
      if (limit.rlim_max == RLIM_INFINITY) {
        writer->json_keyvalue("hardLimit", "unlimited");
      } else {
#if (defined(__linux__) && !defined(__GLIBC__))
        snprintf(buf, sizeof(buf), "%lld", limit.rlim_max);
        writer->json_keyvalue("hardLimit", buf);
#else
        snprintf(buf, sizeof(buf), "%" PRIu64, limit.rlim_max);
        writer->json_keyvalue("hardLimit", buf);
#endif
      }
    }

    writer->json_end();
  }

  writer->json_arrayend();
}

#ifdef __linux__
static int LibraryPrintCallback(struct dl_phdr_info* info, size_t size,
                                void* data) {
  JSONWriter* writer = reinterpret_cast<JSONWriter*>(data);
  if (info->dlpi_name != nullptr && *info->dlpi_name != '\0') {
    writer->json_element(info->dlpi_name);
  }
  return 0;
}
#endif

void PrintLoadedLibraries(JSONWriter* writer) {
  writer->json_arraystart("loadedLibraries");

#ifdef __linux__
  dl_iterate_phdr(LibraryPrintCallback, writer);
#elif __APPLE__
  int i = 0;
  const char* name = _dyld_get_image_name(i);
  while (name != nullptr) {
    writer->json_element(name);
    i++;
    name = _dyld_get_image_name(i);
  }
#endif

  writer->json_arrayend();
}

string GetOsVersion() {
  ostringstream data;
  struct utsname os_info;
  if (uname(&os_info) >= 0) {
// os info
#if defined(_AIX)
    data << os_info.sysname << " / " << os_info.version << "."
         << os_info.release;
#else
    data << os_info.sysname << " / " << os_info.release << " / "
         << os_info.version;
#endif

// machine info
#if defined(__MVS__)
#else
    const char* (*libc_version)();
    *(void**)(&libc_version) = dlsym(RTLD_DEFAULT, "gnu_get_libc_version");
    if (libc_version != NULL) {
      data << " (glibc: " << (*libc_version)() << ")";
    }
#if defined(_AIX)
    char hn[256];
    memset(hn, 0, sizeof(hn));
    gethostname(hn, sizeof(hn));
    data << " / " << hn << " " << os_info.nodename << " " << os_info.machine;
#else
    data << " / " << os_info.nodename << " " << os_info.machine;
#endif
#endif
  } else
    data << "unknown";

  string detail = data.str();
  return detail;
}

}  // namespace xprofiler

#endif