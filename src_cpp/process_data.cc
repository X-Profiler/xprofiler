#include "process_data.h"

namespace xprofiler {

namespace per_process {
ProcessData process_data;
}

ProcessData* ProcessData::Get() { return &per_process::process_data; }

}  // namespace xprofiler
