#include "dump.h"

#include "../configure.h"
#include "../logger.h"
#include "../platform/platform.h"
#include "cpuprofiler/cpu_profiler.h"
#include "uv.h"
#include "v8.h"

namespace xprofiler {
using std::make_pair;
using std::to_string;
using v8::Isolate;

static const char module_type[] = "dump_action";

static Isolate *node_isolate;
static uv_mutex_t node_isolate_mutex;
static uv_async_t async_send_callback;
static uv_thread_t uv_profiling_callback_thread;

static ActionMap action_map;
static RequestMap request_map;

static ConflictMap conflict_map = {{START_CPU_PROFILING, {}},
                                   {STOP_CPU_PROFILING, {}}};

static DependentMap dependent_map = {{STOP_CPU_PROFILING, START_CPU_PROFILING}};

static string Action2String(DumpAction action) {
  string name = "";
  switch (action) {
    case START_CPU_PROFILING:
      name = "start_cpu_profiling";
      break;
    case STOP_CPU_PROFILING:
      name = "stop_cpu_profiling";
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
    for (DumpAction confilct : conflict_map.at(action))
      ActionRunning(confilct, err);
  }
}

static void DependentActionRunning(DumpAction action, XpfError &err) {
  if (dependent_map.find(action) != dependent_map.end()) {
    DumpAction dependent_action = dependent_map.at(action);
    ActionRunning(dependent_action, err);
    if (err.Success())
      err = XpfError::Failure("%s dependent action %s not running.",
                              Action2String(action).c_str(),
                              Action2String(dependent_action).c_str());
    else
      err = XpfError::Succeed();
  }
}

static void TransactionDone(const string thread_name, string unique_key,
                            XpfError &err) {
  if (request_map.find(unique_key) != request_map.end()) {
    err = XpfError::Failure("<%s> %s has been executed by other thread.",
                            thread_name.c_str(), unique_key.c_str());
  }
}

#define PROFILING(data_type, type, func)                         \
  {                                                              \
    data_type##_dump_data_t *data_type##_dump_data =             \
        static_cast<data_type##_dump_data_t *>(dump_data);       \
    func;                                                        \
    Debug(module_type, "<" #type "> %s " #data_type " started.", \
          unique_key.c_str());                                   \
    break;                                                       \
  }

#define DUMP_FILE(data_type, type, func, action)                      \
  data_type##_dump_data_t *data_type##_dump_data =                    \
      static_cast<data_type##_dump_data_t *>(dump_data);              \
  func;                                                               \
  Debug(module_type, "<" #type "> %s " #data_type ": %s created.",    \
        unique_key.c_str(), data_type##_dump_data->filepath.c_str()); \
  if (!dump_data->run_once) dump_data->run_once = true;               \
  action_map.erase(action);

#define V(data, type)                                                    \
  dump_data_t *dump_data = static_cast<dump_data_t *>(data);             \
  string traceid = dump_data->traceid;                                   \
  DumpAction action = dump_data->action;                                 \
  XpfError err;                                                          \
  string unique_key = traceid + "::" + Action2String(action);            \
  TransactionDone(#type, unique_key, err);                               \
  if (err.Fail()) {                                                      \
    Debug(module_type, "%s", err.GetErrMessage());                       \
    request_map.erase(unique_key);                                       \
    if (dump_data->run_once) {                                           \
      Debug(module_type, "<" #type "> %s dump_data cleared.",            \
            unique_key.c_str());                                         \
      delete dump_data;                                                  \
    }                                                                    \
    return;                                                              \
  }                                                                      \
  request_map.insert(make_pair(unique_key, true));                       \
  Debug(module_type, "<" #type "> %s handled.", unique_key.c_str());     \
  DependentActionRunning(action, err);                                   \
  if (err.Fail()) {                                                      \
    Debug(module_type, "<" #type "> %s error: %s", unique_key.c_str(),   \
          err.GetErrMessage());                                          \
    return;                                                              \
  }                                                                      \
  switch (action) {                                                      \
    case START_CPU_PROFILING:                                            \
      PROFILING(cpuprofile, type,                                        \
                Profiler::StartProfiling(cpuprofile_dump_data->title))   \
    case STOP_CPU_PROFILING: {                                           \
      DUMP_FILE(cpuprofile, type,                                        \
                Profiler::StopProfiling(cpuprofile_dump_data->title,     \
                                        cpuprofile_dump_data->filepath), \
                STOP_CPU_PROFILING)                                      \
      action_map.erase(START_CPU_PROFILING);                             \
      break;                                                             \
    }                                                                    \
    default:                                                             \
      Error(module_type, "not support dump action: %d", action);         \
      break;                                                             \
  }

static void RequestInterruptCallback(Isolate *isolate, void *data) {
  V(data, v8_request_interrupt)
}

static void AsyncSendCallback(uv_async_t *handle) {
  V(handle->data, uv_async_send)
}

#undef PROFILING
#undef DUMP_FILE
#undef V

static void ProfilingTime(uint64_t profiling_time) {
  uint64_t start = uv_hrtime();
  while (uv_hrtime() - start < profiling_time * 10e5) {
    // release cpu
    Sleep(1);
  }
}

static void NoticeMainJsThread(void *data) {
  // request interrupt
  uv_mutex_lock(&node_isolate_mutex);
  node_isolate->RequestInterrupt(RequestInterruptCallback, data);
  uv_mutex_unlock(&node_isolate_mutex);

  // uv async send
  async_send_callback.data = data;
  uv_async_send(&async_send_callback);
}

#define V(data_type, action_type)                          \
  case START_##action_type: {                              \
    data_type##_dump_data_t *data_type##_dump_data =       \
        static_cast<data_type##_dump_data_t *>(dump_data); \
    ProfilingTime(data_type##_dump_data->profiling_time);  \
    data_type##_dump_data->action = STOP_##action_type;    \
    NoticeMainJsThread((void *)data_type##_dump_data);     \
    break;                                                 \
  }

static void ProfilingWatchDog(void *data) {
  dump_data_t *dump_data = static_cast<dump_data_t *>(data);
  string traceid = dump_data->traceid;
  DumpAction action = dump_data->action;
  switch (action) {
    V(cpuprofile, CPU_PROFILING)
    default:
      Error(module_type, "watch dog not support dump action: %s", action);
      break;
  }
}

#undef V

int InitDumpAction() {
  node_isolate = Isolate::GetCurrent();
  int rc =
      uv_async_init(uv_default_loop(), &async_send_callback, AsyncSendCallback);
  if (rc != 0) return rc;
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
                         string ext, T *data, XpfError &err) {
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
  string filepath = GetLogDir() + GetSep() + "x-" + prefix + "-" +
                    to_string(GetPid()) + "-" + GetDate() + "-" + RandNum() +
                    "." + ext;
  result["filepath"] = filepath;

  // set action callback data
  data->traceid = traceid;
  data->action = action;
  data->filepath = filepath;

  // send data
  NoticeMainJsThread(data);

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

#define V(func_name, action, data_type, ext, extra)              \
  COMMAND_CALLBACK(func_name) {                                  \
    XpfError err;                                                \
    data_type##_dump_data_t *data = new data_type##_dump_data_t; \
    extra;                                                       \
    json result = DoDumpAction<data_type##_dump_data_t>(         \
        command, action, #data_type, #ext, data, err);           \
    if (err.Fail()) {                                            \
      error(format("%s", err.GetErrMessage()));                  \
      return;                                                    \
    }                                                            \
    success(result);                                             \
  }

// cpu profiling
V(StartCpuProfiling, START_CPU_PROFILING, cpuprofile, cpuprofile,
  data->title = "xprofiler")
V(StopCpuProfiling, STOP_CPU_PROFILING, cpuprofile, cpuprofile, {})

#undef CHECK
}  // namespace xprofiler