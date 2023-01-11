#include "dump.h"

#include <thread>

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
    /**{START_CPU_PROFILING,
     {START_SAMPLING_HEAP_PROFILING, STOP_SAMPLING_HEAP_PROFILING}},
    {STOP_CPU_PROFILING,
     {START_SAMPLING_HEAP_PROFILING, STOP_SAMPLING_HEAP_PROFILING}},
    {HEAPDUMP, {START_SAMPLING_HEAP_PROFILING, STOP_SAMPLING_HEAP_PROFILING}},
    {START_SAMPLING_HEAP_PROFILING,
     {START_CPU_PROFILING, STOP_CPU_PROFILING, HEAPDUMP}},
    {STOP_SAMPLING_HEAP_PROFILING,
     {START_CPU_PROFILING, STOP_CPU_PROFILING, HEAPDUMP}}**/};

const DependentMap dependent_map = {
    {STOP_CPU_PROFILING, START_CPU_PROFILING},
    {STOP_SAMPLING_HEAP_PROFILING, START_SAMPLING_HEAP_PROFILING},
    {STOP_GC_PROFILING, START_GC_PROFILING}};

constexpr const char* Action2String(DumpAction action) {
  const char* name = "";
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

void ActionRunning(ActionMap* action_map, DumpAction action, XpfError& err) {
  if (action_map->find(action) != action_map->end()) {
    err = XpfError::Failure("%s is running.", Action2String(action));
  }
}

void ConflictActionRunning(ActionMap* action_map, DumpAction action,
                           XpfError& err) {
  if (conflict_map.find(action) != conflict_map.end()) {
    for (DumpAction confilct : conflict_map.at(action)) {
      ActionRunning(action_map, confilct, err);
      if (err.Fail()) {
        err = XpfError::Failure(
            "%s conflict action %s is running, please wait for done.",
            Action2String(action), Action2String(confilct));
        break;
      }
    }
  }
}

void DependentActionRunning(ActionMap* action_map, DumpAction action,
                            XpfError& err) {
  if (dependent_map.find(action) != dependent_map.end()) {
    DumpAction dependent_action = dependent_map.at(action);
    ActionRunning(action_map, dependent_action, err);
    if (err.Success())
      err = XpfError::Failure("%s dependent action %s is not running.",
                              Action2String(action),
                              Action2String(dependent_action));
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

void AfterDumpFile(Isolate* isolate, string& filepath, string notify_type,
                   string unique_key) {
  EnvironmentData* env_data = EnvironmentData::GetCurrent(isolate);
  DebugT(module_type, env_data->thread_id(), "<%s> %s dump file: %s.",
         notify_type.c_str(), unique_key.c_str(), filepath.c_str());
  filepath = "";
}

#define CHECK_ERR(func)                                                     \
  if (need_check) {                                                         \
    func;                                                                   \
    if (err.Fail()) {                                                       \
      DebugT(module_type, env_data->thread_id(), "<%s> %s error: %s",       \
             notify_type.c_str(), unique_key.c_str(), err.GetErrMessage()); \
      return;                                                               \
    }                                                                       \
  }

void HandleAction(v8::Isolate* isolate, std::unique_ptr<DumpData> data,
                  const string& notify_type, bool need_check = true) {
  const string& traceid = data->traceid;
  DumpAction action = data->action;

  EnvironmentData* env_data = EnvironmentData::GetCurrent(isolate);
  if (env_data == nullptr) {
    return;
  }

  // check transaction has been done
  XpfError err;
  string unique_key = traceid + "::" + Action2String(action);

  // set action executing flag
  DebugT(module_type, env_data->thread_id(), "<%s> %s handled.",
         notify_type.c_str(), unique_key.c_str());

  // check conflict action running
  CHECK_ERR(ConflictActionRunning(env_data->action_map(), action, err))

  // check dependent action running
  CHECK_ERR(DependentActionRunning(env_data->action_map(), action, err))

  // start run action
  switch (action) {
    case START_CPU_PROFILING: {
      CpuProfiler::StartProfiling(isolate, "xprofiler");
      break;
    }
    case STOP_CPU_PROFILING: {
      CpuProfiler::StopProfiling(isolate, "xprofiler",
                                 env_data->cpuprofile_filepath);
      AfterDumpFile(isolate, env_data->cpuprofile_filepath, notify_type,
                    unique_key);
      env_data->action_map()->erase(START_CPU_PROFILING);
      env_data->action_map()->erase(STOP_CPU_PROFILING);
      break;
    }
    case HEAPDUMP: {
      HeapProfiler::TakeSnapshot(isolate, env_data->heapsnapshot_filepath);
      AfterDumpFile(isolate, env_data->heapsnapshot_filepath, notify_type,
                    unique_key);
      env_data->action_map()->erase(HEAPDUMP);
      break;
    }
    case START_SAMPLING_HEAP_PROFILING: {
      SamplingHeapProfiler::StartSamplingHeapProfiling(isolate);
      break;
    }
    case STOP_SAMPLING_HEAP_PROFILING: {
      SamplingHeapProfiler::StopSamplingHeapProfiling(
          isolate, env_data->sampling_heapprofile_filepath);
      AfterDumpFile(isolate, env_data->sampling_heapprofile_filepath,
                    notify_type, unique_key);
      env_data->action_map()->erase(START_SAMPLING_HEAP_PROFILING);
      env_data->action_map()->erase(STOP_SAMPLING_HEAP_PROFILING);
      break;
    }
    case START_GC_PROFILING: {
      GcProfiler::StartGCProfiling(isolate, env_data->gcprofile_filepath);
      break;
    }
    case STOP_GC_PROFILING: {
      GcProfiler::StopGCProfiling(isolate);
      AfterDumpFile(isolate, env_data->gcprofile_filepath, notify_type,
                    unique_key);
      env_data->action_map()->erase(START_GC_PROFILING);
      env_data->action_map()->erase(STOP_GC_PROFILING);
      break;
    }
    case NODE_REPORT: {
      NodeReport::GetNodeReport(isolate, env_data->node_report_filepath);
      AfterDumpFile(isolate, env_data->node_report_filepath, notify_type,
                    unique_key);
      env_data->action_map()->erase(NODE_REPORT);
      break;
    }
    case COREDUMP: {
      Coredumper::WriteCoredump(env_data->coredump_filepath);
      AfterDumpFile(isolate, env_data->coredump_filepath, notify_type,
                    unique_key);
      env_data->action_map()->erase(COREDUMP);
      break;
    }
    default:
      ErrorT(module_type, env_data->thread_id(), "not support dump action: %d",
             action);
      break;
  }
}

#undef CHECK_ERR

template <DumpAction action>
std::unique_ptr<DumpData> CreateFinishDumpData(EnvironmentData* env_data) {
  std::unique_ptr<DumpData> data = std::make_unique<DumpData>();
  data->traceid = "finish";
  data->thread_id = env_data->thread_id();
  data->action = action;
  return data;
}

void FinishSampling(Isolate* isolate, const char* reason) {
  EnvironmentData* env_data = EnvironmentData::GetCurrent(isolate);

  DebugT(module_type, env_data->thread_id(), "finish sampling because: %s.",
         reason);

  ActionMap current;
  current.swap(*env_data->action_map());

  std::unique_ptr<DumpData> data;

  for (auto itor : current) {
    switch (itor.first) {
      case START_CPU_PROFILING:
        data = CreateFinishDumpData<STOP_CPU_PROFILING>(env_data);
        break;
      case START_SAMPLING_HEAP_PROFILING:
        data = CreateFinishDumpData<STOP_SAMPLING_HEAP_PROFILING>(env_data);
        break;
      case START_GC_PROFILING:
        data = CreateFinishDumpData<STOP_GC_PROFILING>(env_data);
        break;
      default:
        break;
    }

    if (data == nullptr) {
      continue;
    }

    HandleAction(isolate, std::move(data), reason, false);
  }
}

static void WaitForProfile(uint64_t profiling_time) {
  uint64_t start = uv_hrtime();
  while (uv_hrtime() - start < profiling_time * 10e5) {
    // release cpu
    Sleep(1);
  }
}

constexpr const char* GetNotifyType(InterruptKind kind) {
  return kind == InterruptKind::kBusy ? "v8_request_interrupt"
                                      : "uv_async_send";
}

static void NotifyJsThread(EnvironmentData* env_data,
                           std::unique_ptr<DumpData> data) {
  env_data->RequestInterrupt(
      [data = std::move(data)](EnvironmentData* env_data,
                               InterruptKind kind) mutable {
        HandleAction(env_data->isolate(), std::move(data), GetNotifyType(kind));
      });
}

class ProfilingWatchdog {
 public:
  static void Run(std::unique_ptr<DumpData> data) {
    // self-destructive.
    new ProfilingWatchdog(std::move(data));
  }

 private:
  static void ThreadMain(void* data) {
    ProfilingWatchdog* watchdog = static_cast<ProfilingWatchdog*>(data);
    watchdog->ThreadEntry();
  }

  ProfilingWatchdog(std::unique_ptr<DumpData> data)
      : data_(std::move(data)), thread_(ThreadMain, this) {
    thread_.detach();
  }

  void ThreadEntry() {
    // sleep for profiling time
    WaitForProfile(data_->profiling_time);

    // get environment
    ThreadId thread_id = data_->thread_id;
    EnvironmentRegistry* registry = ProcessData::Get()->environment_registry();
    EnvironmentRegistry::NoExitScope scope(registry);
    EnvironmentData* env_data = registry->Get(thread_id);
    if (env_data == nullptr) {
      delete this;
      return;
    }

    env_data->RequestInterrupt([this](EnvironmentData* env_data,
                                      InterruptKind kind) mutable {
      HandleAction(env_data->isolate(), std::move(data_), GetNotifyType(kind));

      delete this;
    });
  }

  std::unique_ptr<DumpData> data_;
  std::thread thread_;
};

static string CreateFilepath(string prefix, string ext) {
  return GetConfig<string>("log_dir") + GetSep() + "x-" + prefix + "-" +
         to_string(GetPid()) + "-" + ConvertTime("%Y%m%d") + "-" +
         to_string(GetNextDiagFileId()) + "." + ext;
}

#define CHECK_ERR(func) \
  func;                 \
  if (err.Fail()) return result;

constexpr DumpAction GetStopAction(DumpAction action) {
  switch (action) {
    case START_CPU_PROFILING: {
      return STOP_CPU_PROFILING;
    }
    case START_SAMPLING_HEAP_PROFILING: {
      return STOP_SAMPLING_HEAP_PROFILING;
    }
    case START_GC_PROFILING: {
      return STOP_GC_PROFILING;
    }
    default:
      abort();
  }
}

template <DumpAction action, bool profiling>
static json DoDumpAction(json command, string prefix, string ext,
                         XpfError& err) {
  json result;

  // get traceid
  CHECK_ERR(string traceid = GetJsonValue<string>(command, "traceid", err))
  CHECK_ERR(ThreadId thread_id =
                GetJsonValue<ThreadId>(command, "thread_id", err))

  // get environment
  EnvironmentRegistry* registry = ProcessData::Get()->environment_registry();
  EnvironmentRegistry::NoExitScope scope(registry);
  EnvironmentData* env_data = registry->Get(thread_id);
  if (env_data == nullptr) {
    err = XpfError::Failure("Thread not found: %f", thread_id);
    return result;
  }

  // check action running
  CHECK_ERR(ActionRunning(env_data->action_map(), action, err))

  // check conflict action running
  CHECK_ERR(ConflictActionRunning(env_data->action_map(), action, err))

  // check dependent action running
  CHECK_ERR(DependentActionRunning(env_data->action_map(), action, err))

  // set action running flag
  env_data->action_map()->insert(make_pair(action, true));

  // get file name
  switch (action) {
    case START_CPU_PROFILING:
      env_data->cpuprofile_filepath = CreateFilepath(prefix, ext);
      result["filepath"] = env_data->cpuprofile_filepath;
      break;
    case STOP_CPU_PROFILING:
      result["filepath"] = env_data->cpuprofile_filepath;
      break;
    case HEAPDUMP:
      env_data->heapsnapshot_filepath = CreateFilepath(prefix, ext);
      result["filepath"] = env_data->heapsnapshot_filepath;
      break;
    case START_SAMPLING_HEAP_PROFILING:
      env_data->sampling_heapprofile_filepath = CreateFilepath(prefix, ext);
      result["filepath"] = env_data->sampling_heapprofile_filepath;
      break;
    case STOP_SAMPLING_HEAP_PROFILING:
      result["filepath"] = env_data->sampling_heapprofile_filepath;
      break;
    case START_GC_PROFILING:
      env_data->gcprofile_filepath = CreateFilepath(prefix, ext);
      result["filepath"] = env_data->gcprofile_filepath;
      break;
    case STOP_GC_PROFILING:
      result["filepath"] = env_data->gcprofile_filepath;
      break;
    case NODE_REPORT:
      env_data->node_report_filepath = CreateFilepath(prefix, ext);
      result["filepath"] = env_data->node_report_filepath;
      break;
    case COREDUMP:
#ifdef __linux__
      env_data->coredump_filepath = CreateFilepath(prefix, ext);
      result["filepath"] = env_data->coredump_filepath;
#else
      err = XpfError::Failure("generate_coredump only support linux now.");
      env_data->action_map()->erase(COREDUMP);
#endif
      break;
    default:
      break;
  }

  if (err.Fail()) return result;

  // set action callback data
  std::unique_ptr<DumpData> data = std::make_unique<DumpData>();
  data->traceid = traceid;
  data->thread_id = thread_id;
  data->action = action;

  // send (copied) data
  NotifyJsThread(env_data, std::make_unique<DumpData>(*data));

  if (!profiling) return result;

  // get profiling time
  json options = command["options"];
  int profiling_time = GetJsonValue<int>(options, "profiling_time", err);
  if (err.Success()) {
    data->action = GetStopAction(action);
    data->profiling_time = profiling_time;
    ProfilingWatchdog::Run(std::move(data));
  } else {
    err = XpfError::Succeed();
  }

  return result;
}

#define V(func, action, profiling, prefix, ext)                       \
  COMMAND_CALLBACK(func) {                                            \
    XpfError err;                                                     \
    json result =                                                     \
        DoDumpAction<action, profiling>(command, #prefix, #ext, err); \
    if (err.Fail()) {                                                 \
      error(format("%s", err.GetErrMessage()));                       \
      return;                                                         \
    }                                                                 \
    success(result);                                                  \
  }

// cpu profiling
V(StartCpuProfiling, START_CPU_PROFILING, true, cpuprofile, cpuprofile)
V(StopCpuProfiling, STOP_CPU_PROFILING, false, cpuprofile, cpuprofile)

// sampling heap profiling
V(StartSamplingHeapProfiling, START_SAMPLING_HEAP_PROFILING, true, heapprofile,
  heapprofile)
V(StopSamplingHeapProfiling, STOP_SAMPLING_HEAP_PROFILING, false, heapprofile,
  heapprofile)

// gc profiling
V(StartGcProfiling, START_GC_PROFILING, true, gcprofile, gcprofile)
V(StopGcProfiling, STOP_GC_PROFILING, false, gcprofile, gcprofile)

// heapdump
V(Heapdump, HEAPDUMP, false, heapdump, heapsnapshot)

// dynamic report
V(GetNodeReport, NODE_REPORT, false, diagreport, diag)

// generate coredump
V(GenerateCoredump, COREDUMP, false, coredump, core)

#undef V

#undef CHECK_ERR
}  // namespace xprofiler
