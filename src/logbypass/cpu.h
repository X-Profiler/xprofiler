#ifndef _SRC_LOGBYPASS_CPU_H
#define _SRC_LOGBYPASS_CPU_H

namespace xprofiler {
void SetNowCpuUsage();
void WriteCpuUsageInPeriod(bool log_format_alinode);
}  // namespace xprofiler

#endif