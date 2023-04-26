#pragma once

#include "environment_data.h"

namespace xprofiler {

template <typename Fn>
void EnvironmentData::RequestInterrupt(Fn&& callback) {
  {
    Mutex::ScopedLock lock(interrupt_mutex_);
    std::unique_ptr<InterruptCallback> interrupt_callback =
        std::make_unique<InterruptCallbackImpl<Fn>>(std::move(callback));
    interrupt_callback->next_.swap(interrupt_requests_);
    interrupt_requests_.swap(interrupt_callback);
  }
  isolate_->RequestInterrupt(InterruptBusyCallback, this);
  uv_async_send(&interrupt_async_);
}

template <typename Fn>
EnvironmentData::InterruptCallbackImpl<Fn>::InterruptCallbackImpl(Fn&& callback)
    : callback_(std::move(callback)) {}

template <typename Fn>
void EnvironmentData::InterruptCallbackImpl<Fn>::Call(EnvironmentData* env_data,
                                                      InterruptKind kind) {
  callback_(env_data, kind);
}

}  // namespace xprofiler
