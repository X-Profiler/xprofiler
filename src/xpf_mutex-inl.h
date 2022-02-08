#ifndef XPROFILER_SRC_MUTEX_INL_H
#define XPROFILER_SRC_MUTEX_INL_H

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

}  // namespace xprofiler

#endif /* XPROFILER_SRC_MUTEX_INL_H */
