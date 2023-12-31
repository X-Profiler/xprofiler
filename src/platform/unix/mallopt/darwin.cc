#ifdef __APPLE__

namespace xprofiler {
int AvoidRssLeak(int threshold) { return 0; };
}  // namespace xprofiler

#endif