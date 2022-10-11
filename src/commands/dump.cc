#include "dump.h"

#include "configure-inl.h"
#include "coredumper/coredumper.h"
#include "cpuprofiler/cpu_profiler.h"
#include "environment_data.h"
#include "gcprofiler/gc_profiler.h"
#include "heapdump/heap_profiler.h"
#include "heapprofiler/sampling_heap_profiler.h"
#include "library/utils.h"
#include "logger.h"
#include "platform/platform.h"
#include "process_data.h"
#include "report/node_report.h"
#include "uv.h"
#include "v8.h"

namespace xprofiler {
using nlohmann::json;
using std::make_pair;
using std::string;
using std::to_string;
using v8::Isolate;

const char module_type[] = "dump_action";

const ConflictMap conflict_map = {
    {START_CPU_PROFILING,
     {START_SAMPLING_HEAP_PROFILING, STOP_SAMPLING_HEAP_PROFILING}},
    {STOP_CPU_PROFILING,
     {START_SAMPLING_HEAP_PROFILING, STOP_SAMPLING_HEAP_PROFILING}},
    {HEAPDUMP, {START_SAMPLING_HEAP_PROFILING, STOP_SAMPLING_HEAP_PROFILING}},
    {START_SAMPLING_HEAP_PROFILING,
     {START_CPU_PROFILING, STOP_CPU_PROFILING, HEAPDUMP}},
    {STOP_SAMPLING_HEAP_PROFILING,
     {START_CPU_PROFILING, STOP_CPU_PROFILING, HEAPDUMP}}};

const DependentMap dependent_map = {
    {STOP_CPU_PROFILING, START_CPU_PROFILING},
    {STOP_SAMPLING_HEAP_PROFILING, START_SAMPLING_HEAP_PROFILING},
    {STOP_GC_PROFILING, START_GC_PROFILING}};

/**
 * Per-process slots. Diagnostics action can not be performed concurrently.
 */
namespace {
uv_thread_t uv_profiling_callback_thread;
ActionMap action_map;
std::string cpuprofile_filepath = "";
std::string sampling_heapprofile_filepath = "";
std::string heapsnapshot_filepath = "";
std::string gcprofile_filepath = "";
std::string node_report_filepath = "";
std::string coredump_filepath = "";

string Action2String(DumpAction action) {
  string name = "";
  switch (action) {
    case START_CPU_PROFILING:
      name = "start_cpu_profiling";
      break;
    case STOP_CPU_PROFILING:
      name = "stop_cpu_profiling";
      break;
    case HEAPDUMP:
      name = "heapdump";
      break;
    case START_SAMPLING_HEAP_PROFILING:
      name = "start_sampling_heap_profiling";
      break;
    case STOP_SAMPLING_HEAP_PROFILING:
      name = "stop_sampling_heap_profiling";
      break;
    case START_GC_PROFILING:
      name = "start_gc_profiling";
      break;
    case STOP_GC_PROFILING:
      name = "stop_gc_profiling";
      break;
    case NODE_REPORT:
      name = "node_report";
      break;
    case COREDUMP:
      name = "coredump";
      break;
    default:
      name = "unknown";
      break;
  }
  return name;
}

void ActionRunning(DumpAction action, XpfError& err) {
  if (action_map.find(action) != action_map.end()) {
    err = XpfError::Failure("%s is running.", Action2String(action).c_str());
  }
}

void ConflictActionRunning(DumpAction action, XpfError& err) {
  if (conflict_map.find(action) != conflict_map.end()) {
    for (DumpAction confilct : conflict_map.at(action)) {
      ActionRunning(confilct, err);
      if (err.Fail()) {
        err = XpfError::Failure(
            "%s conflict action %s is running, please wait for done.",
            Action2String(action).c_str(), Action2String(confilct).c_str());
        break;
      }
    }
  }
}

void DependentActionRunning(DumpAction action, XpfError& err) {
  if (dependent_map.find(action) != dependent_map.end()) {
    DumpAction dependent_action = dependent_map.at(action);
    ActionRunning(dependent_action, err);
    if (err.Success())
      err = XpfError::Failure("%s dependent action %s is not running.",
                              Action2String(action).c_str(),
                              Action2String(dependent_action).c_str());
    else
      err = XpfError::Succeed();
  }
}

template <typename T>
T* GetProfilingData(Isolate* isolate, void* data, string notify_type,
                    string unique_key) {
  EnvironmentData* env_data = EnvironmentData::GetCurrent(isolate);
  T* dump_data = static_cast<T*>(data);
  DebugT(module_type, env_data->thread_id(), "<%s> %s action start.",
         notify_type.c_str(), unique_key.c_str());
  return dump_data;
}

void AfterDumpFile(string& filepath, string notify_type, string unique_key) {
  Isolate* isolate = Isolate::GetCurrent();
  EnvironmentData* env_data = EnvironmentData::GetCurrent(isolate);
  DebugT(module_type, env_data->thread_id(), "<%s> %s dump file: %s.",
         notify_type.c_str(), unique_key.c_str(), filepath.c_str());
  filepath = "";
}

}  // namespace

#define CLEAR_DATA                                                         \
  DebugT(module_type, env_data->thread_id(), "<%s> %s dump_data cleared.", \
         notify_type.c_str(), unique_key.c_str());                         \
  delete dump_data;

#define CHECK_ERR(func)                                                   \
  func;                                                                   \
  if (err.Fail()) {                                                       \
    DebugT(module_type, env_data->thread_id(), "<%s> %s error: %s",       \
           notify_type.c_str(), unique_key.c_str(), err.GetErrMessage()); \
    CLEAR_DATA;                                                           \
    return;                                                               \
  }

void HandleAction(v8::Isolate* isolate, void* data, string notify_type) {
  BaseDumpData* dump_data = static_cast<BaseDumpData*>(data);
  string traceid = dump_data->traceid;
  DumpAction action = dump_data->action;
  EnvironmentData* env_data = EnvironmentData::GetCurrent(isolate);

  // check transaction has been done
  XpfError err;
  string unique_key = traceid + "::" + Action2String(action);

  // set action executing flag
  DebugT(module_type, env_data->thread_id(), "<%s> %s handled.",
         notify_type.c_str(), unique_key.c_str());

  // check conflict action running
  CHECK_ERR(ConflictActionRunning(action, err))

  // check dependent action running
  CHECK_ERR(DependentActionRunning(action, err))

  // start run action
  switch (action) {
    case START_CPU_PROFILING: {
      CpuProfilerDumpData* tmp = GetProfilingData<CpuProfilerDumpData>(
          isolate, data, notify_type, unique_key);
      CpuProfiler::StartProfiling(isolate, tmp->title);

      // after start cpu profiling
      dump_data->action = STOP_CPU_PROFILING;
      env_data->sampling_record_map()->insert(
          make_pair(START_CPU_PROFILING, data));
      break;
    }
    case STOP_CPU_PROFILING: {
      dump_data->run_once = true;
      CpuProfilerDumpData* tmp = static_cast<CpuProfilerDumpData*>(data);
      CpuProfiler::StopProfiling(isolate, tmp->title, cpuprofile_filepath);
      AfterDumpFile(cpuprofile_filepath, notify_type, unique_key);

      // after stop cpu profiling
      action_map.erase(START_CPU_PROFILING);
      action_map.erase(STOP_CPU_PROFILING);
      env_data->sampling_record_map()->erase(START_CPU_PROFILING);
      break;
    }
    case HEAPDUMP: {
      HeapProfiler::TakeSnapshot(isolate, heapsnapshot_filepath);
      AfterDumpFile(heapsnapshot_filepath, notify_type, unique_key);

      // after heapdump
      action_map.erase(HEAPDUMP);
      break;
    }
    case START_SAMPLING_HEAP_PROFILING: {
      GetProfilingData<SamplingHeapProfilerDumpData>(isolate, data, notify_type,
                                                     unique_key);
      SamplingHeapProfiler::StartSamplingHeapProfiling(isolate);

      // after start sampling heap profiling
      dump_data->action = STOP_SAMPLING_HEAP_PROFILING;
      env_data->sampling_record_map()->insert(
          make_pair(START_SAMPLING_HEAP_PROFILING, data));
      break;
    }
    case STOP_SAMPLING_HEAP_PROFILING: {
      dump_data->run_once = true;
      SamplingHeapProfiler::StopSamplingHeapProfiling(
          isolate, sampling_heapprofile_filepath);
      AfterDumpFile(sampling_heapprofile_filepath, notify_type, unique_key);

      // after stop sampling heap profiling
      action_map.erase(START_SAMPLING_HEAP_PROFILING);
      action_map.erase(STOP_SAMPLING_HEAP_PROFILING);
      env_data->sampling_record_map()->erase(START_SAMPLING_HEAP_PROFILING);
      break;
    }
    case START_GC_PROFILING: {
      GetProfilingData<GcProfilerDumpData>(isolate, data, notify_type,
                                           unique_key);
      GcProfiler::StartGCProfiling(isolate, gcprofile_filepath);

      // after start gc profiling
      dump_data->action = STOP_GC_PROFILING;
      env_data->sampling_record_map()->insert(
          make_pair(START_GC_PROFILING, data));
      break;
    }
    case STOP_GC_PROFILING: {
      dump_data->run_once = true;
      GcProfiler::StopGCProfiling(isolate);
      AfterDumpFile(gcprofile_filepath, notify_type, unique_key);

      // after stop gc profiling
      action_map.erase(START_GC_PROFILING);
      action_map.erase(STOP_GC_PROFILING);
      env_data->sampling_record_map()->erase(START_GC_PROFILING);
      break;
    }
    case NODE_REPORT: {
      NodeReport::GetNodeReport(isolate, node_report_filepath);
      AfterDumpFile(node_report_filepath, notify_type, unique_key);

      // after node report
      action_map.erase(NODE_REPORT);
      break;
    }
    case COREDUMP: {
      Coredumper::WriteCoredump(coredump_filepath);
      AfterDumpFile(coredump_filepath, notify_type, unique_key);

      // after coredump
      action_map.erase(COREDUMP);
      break;
    }
    default:
      ErrorT(module_type, env_data->thread_id(), "not support dump action: %d",
             action);
      break;
  }

  // clear dump_data
  if (dump_data->run_once) {
    CLEAR_DATA;
  }
  return;
}

#undef CHECK_ERR
#undef CLEAR_DATA

void FinishSampling(Isolate* isolate, const char* reason) {
  EnvironmentData* env_data = EnvironmentData::GetCurrent(isolate);

  DebugT(module_type, env_data->thread_id(), "finish sampling because: %s.",
         reason);

  for (auto itor = env_data->sampling_record_map()->begin();
       itor != env_data->sampling_record_map()->end(); itor++) {
    HandleAction(isolate, itor->second, reason);
  }
}

static void WaitForProfile(uint64_t profiling_time) {
  uint64_t start = uv_hrtime();
  while (uv_hrtime() - start < profiling_time * 10e5) {
    // release cpu
    Sleep(1);
  }
}

static void NotifyJsThread(EnvironmentData* env_data, void* data) {
  env_data->RequestInterrupt(
      [data](EnvironmentData* env_data, InterruptKind kind) {
        HandleAction(env_data->isolate(), data,
                     kind == InterruptKind::kBusy ? "v8_request_interrupt"
                                                  : "uv_async_send");
      });
}

static void ProfilingWatchDog(void* data) {
  BaseDumpData* dump_data = static_cast<BaseDumpData*>(data);
  string traceid = dump_data->traceid;

  // sleep profiling time
  WaitForProfile(dump_data->profiling_time);

  ThreadId thread_id = dump_data->thread_id;
  EnvironmentRegistry* registry = ProcessData::Get()->environment_registry();
  EnvironmentRegistry::NoExitScope scope(registry);
  EnvironmentData* env_data = registry->Get(thread_id);
  if (env_data == nullptr) {
    return;
  }

  NotifyJsThread(env_data, data);
}

static string CreateFilepath(string prefix, string ext) {
  return GetLogDir() + GetSep() + "x-" + prefix + "-" + to_string(GetPid()) +
         "-" + ConvertTime("%Y%m%d") + "-" + to_string(GetNextDiagFileId()) +
         "." + ext;
}

#define CHECK_ERR(func) \
  func;                 \
  if (err.Fail()) return result;

template <DumpAction action, bool profiling, typename T>
static json DoDumpAction(json command, string prefix, string ext, T* data,
                         XpfError& err) {
  json result;

  // get traceid
  CHECK_ERR(string traceid = GetJsonValue<string>(command, "traceid", err))
  CHECK_ERR(ThreadId thread_id =
                GetJsonValue<ThreadId>(command, "thread_id", err))

  // check action running
  CHECK_ERR(ActionRunning(action, err))

  // check conflict action running
  CHECK_ERR(ConflictActionRunning(action, err))

  // check dependent action running
  CHECK_ERR(DependentActionRunning(action, err))

  // set action running flag
  action_map.insert(make_pair(action, true));

  EnvironmentRegistry* registry = ProcessData::Get()->environment_registry();
  EnvironmentRegistry::NoExitScope scope(registry);
  EnvironmentData* env_data = registry->Get(thread_id);
  if (env_data == nullptr) {
    err = XpfError::Failure("Thread not found: %f", thread_id);
    return result;
  }

  // get file name
  switch (action) {
    case START_CPU_PROFILING:
      cpuprofile_filepath = CreateFilepath(prefix, ext);
      result["filepath"] = cpuprofile_filepath;
      break;
    case STOP_CPU_PROFILING:
      result["filepath"] = cpuprofile_filepath;
      break;
    case HEAPDUMP:
      heapsnapshot_filepath = CreateFilepath(prefix, ext);
      result["filepath"] = heapsnapshot_filepath;
      break;
    case START_SAMPLING_HEAP_PROFILING:
      sampling_heapprofile_filepath = CreateFilepath(prefix, ext);
      result["filepath"] = sampling_heapprofile_filepath;
      break;
    case STOP_SAMPLING_HEAP_PROFILING:
      result["filepath"] = sampling_heapprofile_filepath;
      break;
    case START_GC_PROFILING:
      gcprofile_filepath = CreateFilepath(prefix, ext);
      result["filepath"] = gcprofile_filepath;
      break;
    case STOP_GC_PROFILING:
      result["filepath"] = gcprofile_filepath;
      break;
    case NODE_REPORT:
      node_report_filepath = CreateFilepath(prefix, ext);
      result["filepath"] = node_report_filepath;
      break;
    case COREDUMP:
#ifdef __linux__
      coredump_filepath = CreateFilepath(prefix, ext);
      result["filepath"] = coredump_filepath;
#else
      err = XpfError::Failure("generate_coredump only support linux now.");
      action_map.erase(COREDUMP);
#endif
      break;
    default:
      break;
  }

  if (err.Fail()) return result;

  // set action callback data
  data->traceid = traceid;
  data->thread_id = thread_id;
  data->action = action;

  // send data
  NotifyJsThread(env_data, data);

  if (!profiling) return result;

  // get profiling time
  json options = command["options"];
  int profiling_time = GetJsonValue<int>(options, "profiling_time", err);
  if (err.Success()) {
    data->run_once = false;
    data->profiling_time = profiling_time;
    uv_thread_create(&uv_profiling_callback_thread, ProfilingWatchDog,
                     (void*)data);
  } else {
    err = XpfError::Succeed();
  }

  return result;
}

#define V(func, data_type, action, profiling, prefix, ext)                     \
  COMMAND_CALLBACK(func) {                                                     \
    data_type* data = new data_type;                                           \
    XpfError err;                                                              \
    json result = DoDumpAction<action, profiling, data_type>(command, #prefix, \
                                                             #ext, data, err); \
    if (err.Fail()) {                                                          \
      error(format("%s", err.GetErrMessage()));                                \
      delete data;                                                             \
      return;                                                                  \
    }                                                                          \
    success(result);                                                           \
  }

// cpu profiling
V(StartCpuProfiling, CpuProfilerDumpData, START_CPU_PROFILING, true, cpuprofile,
  cpuprofile)
V(StopCpuProfiling, CpuProfilerDumpData, STOP_CPU_PROFILING, false, cpuprofile,
  cpuprofile)

// sampling heap profiling
V(StartSamplingHeapProfiling, SamplingHeapProfilerDumpData,
  START_SAMPLING_HEAP_PROFILING, true, heapprofile, heapprofile)
V(StopSamplingHeapProfiling, SamplingHeapProfilerDumpData,
  STOP_SAMPLING_HEAP_PROFILING, false, heapprofile, heapprofile)

// gc profiling
V(StartGcProfiling, GcProfilerDumpData, START_GC_PROFILING, true, gcprofile,
  gcprofile)
V(StopGcProfiling, GcProfilerDumpData, STOP_GC_PROFILING, false, gcprofile,
  gcprofile)

// heapdump
V(Heapdump, HeapdumpDumpData, HEAPDUMP, false, heapdump, heapsnapshot)

// dynamic report
V(GetNodeReport, NodeReportDumpData, NODE_REPORT, false, diagreport, diag)

// generate coredump
V(GenerateCoredump, CoreDumpData, COREDUMP, false, coredump, core)

#undef V

#undef CHECK_ERR
}  // namespace xprofiler
