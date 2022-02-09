#include "dump.h"

#include "../logger.h"
#include "../platform/platform.h"
#include "configure-inl.h"
#include "cpuprofiler/cpu_profiler.h"
#include "gcprofiler/gc_profiler.h"
#include "heapdump/heap_profiler.h"
#include "heapprofiler/sampling_heap_profiler.h"
#include "report/node_report.h"
#include "uv.h"
#include "v8.h"

namespace xprofiler {
using std::make_pair;
using std::to_string;
using v8::Isolate;

static const char module_type[] = "dump_action";

static Isolate *node_isolate;
static uv_mutex_t node_isolate_mutex;
static uv_mutex_t async_data_mutex;
static uv_async_t async_send_callback;
static uv_thread_t uv_profiling_callback_thread;

static ActionMap action_map;
static RequestMap request_map;

static ConflictMap conflict_map = {
    {START_CPU_PROFILING,
     {START_SAMPLING_HEAP_PROFILING, STOP_SAMPLING_HEAP_PROFILING}},
    {STOP_CPU_PROFILING,
     {START_SAMPLING_HEAP_PROFILING, STOP_SAMPLING_HEAP_PROFILING}},
    {HEAPDUMP, {START_SAMPLING_HEAP_PROFILING, STOP_SAMPLING_HEAP_PROFILING}},
    {START_SAMPLING_HEAP_PROFILING,
     {START_CPU_PROFILING, STOP_CPU_PROFILING, HEAPDUMP}},
    {STOP_SAMPLING_HEAP_PROFILING,
     {START_CPU_PROFILING, STOP_CPU_PROFILING, HEAPDUMP}}};

static DependentMap dependent_map = {
    {STOP_CPU_PROFILING, START_CPU_PROFILING},
    {STOP_SAMPLING_HEAP_PROFILING, START_SAMPLING_HEAP_PROFILING},
    {STOP_GC_PROFILING, START_GC_PROFILING}};

static string cpuprofile_filepath = "";
static string sampling_heapprofile_filepath = "";
static string heapsnapshot_filepath = "";
static string gcprofile_filepath = "";
static string node_report_filepath = "";

static string Action2String(DumpAction action) {
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
    default:
      name = "unknown";
      break;
  }
  return name;
}

static void ActionRunning(DumpAction action, XpfError &err) {
  if (action_map.find(action) != action_map.end()) {
    err = XpfError::Failure("%s is running.", Action2String(action).c_str());
  }
}

static void ConflictActionRunning(DumpAction action, XpfError &err) {
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

static void DependentActionRunning(DumpAction action, XpfError &err) {
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

static void TransactionDone(string thread_name, string unique_key,
                            XpfError &err) {
  if (request_map.find(unique_key) != request_map.end()) {
    err = XpfError::Failure("<%s> %s has been executed by other thread.",
                            thread_name.c_str(), unique_key.c_str());
  }
}

template <typename T>
static T *GetProfilingData(void *data, string notify_type, string unique_key) {
  T *dump_data = static_cast<T *>(data);
  Debug(module_type, "<%s> %s action start.", notify_type.c_str(),
        unique_key.c_str());
  return dump_data;
}

template <typename T>
static T *GetDumpData(void *data) {
  T *dump_data = static_cast<T *>(data);
  if (!dump_data->run_once) dump_data->run_once = true;
  return dump_data;
}

static void AfterDumpFile(string &filepath, string notify_type,
                          string unique_key) {
  Debug(module_type, "<%s> %s dump file: %s.", notify_type.c_str(),
        unique_key.c_str(), filepath.c_str());
  filepath = "";
}

#define CHECK(func)                                              \
  func;                                                          \
  if (err.Fail()) {                                              \
    Debug(module_type, "<%s> %s error: %s", notify_type.c_str(), \
          unique_key.c_str(), err.GetErrMessage());              \
    return;                                                      \
  }

void HandleAction(void *data, string notify_type) {
  dump_data_t *dump_data = static_cast<dump_data_t *>(data);
  string traceid = dump_data->traceid;
  DumpAction action = dump_data->action;

  // check transaction has been done
  XpfError err;
  string unique_key = traceid + "::" + Action2String(action);
  TransactionDone(notify_type, unique_key, err);
  if (err.Fail()) {
    Debug(module_type, "%s", err.GetErrMessage());
    request_map.erase(unique_key);
    // clear dump_data
    if (dump_data->run_once) {
      Debug(module_type, "<%s> %s dump_data cleared.", notify_type.c_str(),
            unique_key.c_str());
      delete dump_data;
    }
    return;
  }

  // set action executing flag
  request_map.insert(make_pair(unique_key, true));
  Debug(module_type, "<%s> %s handled.", notify_type.c_str(),
        unique_key.c_str());

  // check conflict action running
  CHECK(ConflictActionRunning(action, err))

  // check dependent action running
  CHECK(DependentActionRunning(action, err))

  // start run action
  switch (action) {
    case START_CPU_PROFILING: {
      cpuprofile_dump_data_t *tmp = GetProfilingData<cpuprofile_dump_data_t>(
          data, notify_type, unique_key);
      Profiler::StartProfiling(tmp->title);
      break;
    }
    case STOP_CPU_PROFILING: {
      cpuprofile_dump_data_t *tmp = GetDumpData<cpuprofile_dump_data_t>(data);
      Profiler::StopProfiling(tmp->title, cpuprofile_filepath);
      AfterDumpFile(cpuprofile_filepath, notify_type, unique_key);
      action_map.erase(START_CPU_PROFILING);
      action_map.erase(STOP_CPU_PROFILING);
      break;
    }
    case HEAPDUMP: {
      HeapProfiler::TakeSnapshot(heapsnapshot_filepath);
      AfterDumpFile(heapsnapshot_filepath, notify_type, unique_key);
      action_map.erase(HEAPDUMP);
      break;
    }
    case START_SAMPLING_HEAP_PROFILING: {
      SamplingHeapProfile::StartSamplingHeapProfiling();
      break;
    }
    case STOP_SAMPLING_HEAP_PROFILING: {
      SamplingHeapProfile::StopSamplingHeapProfiling(
          sampling_heapprofile_filepath);
      AfterDumpFile(sampling_heapprofile_filepath, notify_type, unique_key);
      action_map.erase(START_SAMPLING_HEAP_PROFILING);
      action_map.erase(STOP_SAMPLING_HEAP_PROFILING);
      break;
    }
    case START_GC_PROFILING: {
      GcProfiler::StartGCProfiling(node_isolate, gcprofile_filepath);
      break;
    }
    case STOP_GC_PROFILING: {
      GcProfiler::StopGCProfiling(node_isolate);
      AfterDumpFile(gcprofile_filepath, notify_type, unique_key);
      action_map.erase(START_GC_PROFILING);
      action_map.erase(STOP_GC_PROFILING);
      break;
    }
    case NODE_REPORT: {
      NodeReport::GetNodeReport(node_isolate, node_report_filepath);
      AfterDumpFile(node_report_filepath, notify_type, unique_key);
      action_map.erase(NODE_REPORT);
      break;
    }
    default:
      Error(module_type, "not support dump action: %d", action);
      break;
  }
}

#undef CHECK

static void RequestInterruptCallback(Isolate *isolate, void *data) {
  HandleAction(data, "v8_request_interrupt");
}

static void AsyncSendCallback(uv_async_t *handle) {
  // get data from async handle
  uv_mutex_lock(&async_data_mutex);
  void *data = handle->data;
  handle->data = nullptr;
  uv_mutex_unlock(&async_data_mutex);

  HandleAction(data, "uv_async_send");
}

static void ProfilingTime(uint64_t profiling_time) {
  uint64_t start = uv_hrtime();
  while (uv_hrtime() - start < profiling_time * 10e5) {
    // release cpu
    Sleep(1);
  }
}

static void NoticeMainJsThread(void *data) {
  uv_mutex_lock(&node_isolate_mutex);
  node_isolate->RequestInterrupt(RequestInterruptCallback, data);
  uv_mutex_unlock(&node_isolate_mutex);

  uv_mutex_lock(&async_data_mutex);
  async_send_callback.data = data;
  uv_async_send(&async_send_callback);
  uv_mutex_unlock(&async_data_mutex);
}

template <typename T>
void StopProfiling(void *data, DumpAction stop_action) {
  T *dump_data = static_cast<T *>(data);
  ProfilingTime(dump_data->profiling_time);
  dump_data->action = stop_action;
  NoticeMainJsThread(data);
}

static void ProfilingWatchDog(void *data) {
  dump_data_t *dump_data = static_cast<dump_data_t *>(data);
  string traceid = dump_data->traceid;
  DumpAction action = dump_data->action;
  switch (action) {
    case START_CPU_PROFILING:
      StopProfiling<cpuprofile_dump_data_t>(data, STOP_CPU_PROFILING);
      break;
    case START_SAMPLING_HEAP_PROFILING:
      StopProfiling<sampling_heapprofiler_dump_data_t>(
          data, STOP_SAMPLING_HEAP_PROFILING);
      break;
    case START_GC_PROFILING:
      StopProfiling<gcprofiler_dump_data_t>(data, STOP_GC_PROFILING);
      break;
    default:
      Error(module_type, "watch dog not support dump action: %s", action);
      break;
  }
}

static string CreateFilepath(string prefix, string ext) {
  return GetLogDir() + GetSep() + "x-" + prefix + "-" + to_string(GetPid()) +
         "-" + ConvertTime("%Y%m%d") + "-" + RandNum() + "." + ext;
}

int InitDumpAction() {
  // init global node isolate
  node_isolate = Isolate::GetCurrent();

  // init async send
  int rc =
      uv_async_init(uv_default_loop(), &async_send_callback, AsyncSendCallback);
  if (rc != 0) return rc;

  // init async data mutex
  rc = uv_mutex_init(&async_data_mutex);
  if (rc != 0) return rc;

  // init isolate mutex
  rc = uv_mutex_init(&node_isolate_mutex);

  return rc;
}

void UnrefDumpActionAsyncHandle() {
  uv_unref(reinterpret_cast<uv_handle_t *>(&async_send_callback));
}

#define CHECK(func) \
  func;             \
  if (err.Fail()) return result;

template <typename T>
static json DoDumpAction(json command, DumpAction action, string prefix,
                         string ext, T *data, bool profiling, XpfError &err) {
  json result;

  // get traceid
  CHECK(string traceid = GetJsonValue<string>(command, "traceid", err))

  // check action running
  CHECK(ActionRunning(action, err))

  // check conflict action running
  CHECK(ConflictActionRunning(action, err))

  // check dependent action running
  CHECK(DependentActionRunning(action, err))

  // set action running flag
  action_map.insert(make_pair(action, true));

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
    default:
      break;
  }

  // set action callback data
  data->traceid = traceid;
  data->action = action;

  // send data
  NoticeMainJsThread(data);

  if (!profiling) return result;

  // get profiling time
  json options = command["options"];
  int profiling_time = GetJsonValue<int>(options, "profiling_time", err);
  if (err.Success()) {
    data->run_once = false;
    data->profiling_time = profiling_time;
    uv_thread_create(&uv_profiling_callback_thread, ProfilingWatchDog,
                     (void *)data);
  } else {
    err = XpfError::Succeed();
  }

  return result;
}

#define ACTION_HANDLE(action, data_type, profiling, prefix, ext) \
  XpfError err;                                                  \
  json result = DoDumpAction<data_type##dump_data_t>(            \
      command, action, #prefix, #ext, data, profiling, err);     \
  if (err.Fail()) {                                              \
    error(format("%s", err.GetErrMessage()));                    \
    return;                                                      \
  }                                                              \
  success(result);

#define V(func, data_type, action, profiling, prefix, ext)     \
  COMMAND_CALLBACK(func) {                                     \
    data_type##dump_data_t *data = new data_type##dump_data_t; \
    ACTION_HANDLE(action, data_type, profiling, prefix, ext)   \
  }

// cpu profiling
V(StartCpuProfiling, cpuprofile_, START_CPU_PROFILING, true, cpuprofile,
  cpuprofile)
V(StopCpuProfiling, cpuprofile_, STOP_CPU_PROFILING, false, cpuprofile,
  cpuprofile)

// sampling heap profiling
V(StartSamplingHeapProfiling, sampling_heapprofiler_,
  START_SAMPLING_HEAP_PROFILING, true, heapprofile, heapprofile)
V(StopSamplingHeapProfiling, sampling_heapprofiler_,
  STOP_SAMPLING_HEAP_PROFILING, false, heapprofile, heapprofile)

// gc profiling
V(StartGcProfiling, gcprofiler_, START_GC_PROFILING, true, gcprofile, gcprofile)
V(StopGcProfiling, gcprofiler_, STOP_GC_PROFILING, false, gcprofile, gcprofile)

// heapdump
V(Heapdump, heap, HEAPDUMP, false, heapdump, heapsnapshot)

// dynamic report
V(GetNodeReport, node_report_, NODE_REPORT, false, diagreport, diag)

#undef V

#undef ACTION_HANDLE

#undef CHECK
}  // namespace xprofiler
