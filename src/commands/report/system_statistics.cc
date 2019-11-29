#include "system_statistics.h"

#include <inttypes.h>

#include "../../platform/platform.h"

namespace xprofiler {

#if defined(_MSC_VER) && _MSC_VER < 1900
#define arraysize(a) (sizeof(a) / sizeof(*a))
#else
template <typename T, size_t N>
constexpr size_t arraysize(const T (&)[N]) {
  return N;
}
#endif

static void PrintResourceLimits(JSONWriter* writer) {
  writer->json_arraystart("resourceLimits");
#ifdef _WIN32
#else
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
        snprintf(buf, sizeof(buf), "%lld\n", limit.rlim_max);
        writer->json_keyvalue("hardLimit", buf);
#else
        snprintf(buf, sizeof(buf), "%" PRIu64 "\n", limit.rlim_max);
        writer->json_keyvalue("hardLimit", buf);
#endif
      }
    }

    writer->json_end();
  }
#endif

  writer->json_arrayend();
}

void SetSystemStatistics(JSONWriter* writer) {
  writer->json_objectstart("system");

  PrintSystemEnv(writer);
  PrintResourceLimits(writer);

  writer->json_objectend();
}
}  // namespace xprofiler