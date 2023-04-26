#pragma once

#include "nan.h"

namespace xprofiler {

class ConditionVariable;

class Mutex {
 public:
  inline Mutex();
  inline ~Mutex();
  inline void Lock();
  inline void Unlock();

  Mutex(const Mutex&) = delete;
  Mutex& operator=(const Mutex&) = delete;

  class ScopedLock {
   public:
    inline explicit ScopedLock(const Mutex& mutex);
    inline ~ScopedLock();

    ScopedLock(const ScopedLock&) = delete;
    ScopedLock& operator=(const ScopedLock&) = delete;

   private:
    friend ConditionVariable;
    const Mutex& mutex_;
  };

 private:
  friend ConditionVariable;
  mutable uv_mutex_t mutex_;
};

class ConditionVariable {
 public:
  using ScopedLock = typename Mutex::ScopedLock;
  inline ConditionVariable();
  inline ~ConditionVariable();
  inline void Broadcast(const ScopedLock&);
  inline void Signal(const ScopedLock&);
  inline void Wait(const ScopedLock& scoped_lock);

  ConditionVariable(const ConditionVariable&) = delete;
  ConditionVariable& operator=(const ConditionVariable&) = delete;

 private:
  uv_cond_t cond_;
};

}  // namespace xprofiler
