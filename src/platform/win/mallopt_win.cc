#ifdef _WIN32

namespace xprofiler {
int AvoidRssLeak(int threshold) { return 0; };
}  // namespace xprofiler

#endif