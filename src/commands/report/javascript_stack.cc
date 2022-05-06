#include "nan.h"
#include "node_report.h"
#include "platform/platform.h"
#include "util.h"
#include "xpf_v8.h"

static const size_t kMaxFramesCount = 255;
static const char* v8_states[] = {
    "JS",       "GC",    "PARSER",   "BYTECODE_COMPILER",
    "COMPILER", "OTHER", "EXTERNAL", "IDLE"};

namespace xprofiler {
using Nan::Utf8String;
using v8::Isolate;
using v8::Local;
using v8::RegisterState;
using v8::SampleInfo;
using v8::StackFrame;
using v8::StackTrace;

void NodeReport::SetJavaScriptStack(JSONWriter* writer, bool fatal_error) {
  if (isolate_ == nullptr) {
    return;
  }
  HandleScope scope(isolate_);
  RegisterState state;
  SampleInfo info;

  // init state
  state.pc = nullptr;
  state.fp = &state;
  state.sp = &state;

  // instruction pointer
  void* samples[kMaxFramesCount];

  // get instruction pointer
  isolate_->GetStackSample(state, samples, kMaxFramesCount, &info);

  // set current vm state
  if (static_cast<size_t>(info.vm_state) < arraysize(v8_states)) {
    writer->json_keyvalue("vmState", v8_states[info.vm_state]);
  } else {
    writer->json_keyvalue("vmState", "unknown");
  }

  if (fatal_error) {
    writer->json_arraystart("jsStacks");
    writer->json_arrayend();
    return;
  }

  // get js stacks
  Local<StackTrace> stack = StackTrace::CurrentStackTrace(
      isolate_, kMaxFramesCount, StackTrace::kDetailed);
  writer->json_arraystart("jsStacks");
  for (int i = 0; i < stack->GetFrameCount(); i++) {
    writer->json_start();

    if (static_cast<size_t>(i) < info.frames_count)
      writer->json_keyvalue("pcAddress", GetPcAddress(samples[i]));
    else
      writer->json_keyvalue("pcAddress", "nullptr");

    // needs v8 version >= 6.8
    Local<StackFrame> frame = stack->GetFrame(isolate_, i);

    Utf8String fn_name_s(frame->GetFunctionName());
    Utf8String script_name(frame->GetScriptName());
    const int line_number = frame->GetLineNumber();
    const int column = frame->GetColumn();

    if (fn_name_s.length() == 0)
      writer->json_keyvalue("functionName", "anonymous");
    else
      writer->json_keyvalue("functionName", *fn_name_s);
    writer->json_keyvalue("scriptName", *script_name);
    writer->json_keyvalue("lineNumber", line_number);
    writer->json_keyvalue("column", column);

    if (frame->IsEval()) {
      writer->json_keyvalue("frameType", "eval");
    } else if (frame->IsConstructor()) {
      writer->json_keyvalue("frameType", "constructor");
    } else if (frame->IsWasm()) {
      writer->json_keyvalue("frameType", "wasm");
    }
#if (NODE_VERSION_AT_LEAST(12, 9, 0))
    else if (frame->IsUserJavaScript()) {
      writer->json_keyvalue("frameType", "userjs");
    }
#endif
    else {
      writer->json_keyvalue("frameType", "unknown");
    }

    writer->json_end();
  }
  writer->json_arrayend();
}
}  // namespace xprofiler
