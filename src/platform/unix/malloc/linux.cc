#ifdef __linux__
#include <malloc.h>

namespace xprofiler {
int AvoidRssLeak(int threshold) { 
  int rc = mallopt(M_MMAP_THRESHOLD, threshold);
  return rc;
};
}  // namespace xprofiler

#endif