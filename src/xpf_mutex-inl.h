#pragma once

#include "util.h"
#include "xpf_mutex.h"

namespace xprofiler {

Mutex::Mutex() { CHECK_EQ(0, uv_mutex_init(&mutex_)); }

Mutex::~Mutex() { uv_mutex_destroy(&mutex_); }

void Mutex::Lock() { uv_mutex_lock(&mutex_); }

void Mutex::Unlock() { uv_mutex_unlock(&mutex_); }

Mutex::ScopedLock::ScopedLock(const Mutex& mutex) : mutex_(mutex) {
  uv_mutex_lock(&mutex_.mutex_);
}

Mutex::ScopedLock::~ScopedLock() { uv_mutex_unlock(&mutex_.mutex_); }

ConditionVariable::ConditionVariable() { CHECK_EQ(0, uv_cond_init(&cond_)); }

ConditionVariable::~ConditionVariable() { uv_cond_destroy(&cond_); }

void ConditionVariable::Broadcast(const ScopedLock&) {
  uv_cond_broadcast(&cond_);
}

void ConditionVariable::Signal(const ScopedLock&) { uv_cond_signal(&cond_); }

void ConditionVariable::Wait(const ScopedLock& scoped_lock) {
  uv_cond_wait(&cond_, &scoped_lock.mutex_.mutex_);
}

}  // namespace xprofiler
