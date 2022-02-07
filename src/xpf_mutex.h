#ifndef XPROFILER_SRC_MUTEX_H
#define XPROFILER_SRC_MUTEX_H

#include "nan.h"

namespace xprofiler {

class Mutex {
 public:
  inline Mutex();
  inline ~Mutex();
  inline void Lock();
  inline void Unlock();

  Mutex(const Mutex&) = delete;
  Mutex& operator=(const Mutex&) = delete;

  class ScopedLock;
  class ScopedUnlock;

  class ScopedLock {
   public:
    inline explicit ScopedLock(const Mutex& mutex);
    inline explicit ScopedLock(const ScopedUnlock& scoped_unlock);
    inline ~ScopedLock();

    ScopedLock(const ScopedLock&) = delete;
    ScopedLock& operator=(const ScopedLock&) = delete;

   private:
    const Mutex& mutex_;
  };

 private:
  mutable uv_mutex_t mutex_;
};

}  // namespace xprofiler

#endif /* XPROFILER_SRC_MUTEX_H */
