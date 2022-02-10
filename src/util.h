#ifndef XPROFILER_SRC_UTIL_H
#define XPROFILER_SRC_UTIL_H

#include <v8.h>

namespace xprofiler {

// The reason that Assert() takes a struct argument instead of individual
// const char*s is to ease instruction cache pressure in calls from CHECK.
struct AssertionInfo {
  const char* file_line;  // filename:line
  const char* message;
  const char* function;
};
[[noreturn]] void Abort();
[[noreturn]] void Assert(const AssertionInfo& info);

#define ERROR_AND_ABORT(expr)                                                 \
  do {                                                                        \
    /* Make sure that this struct does not end up in inline code, but      */ \
    /* rather in a read-only data section when modifying this code.        */ \
    static const xprofiler::AssertionInfo args = {                            \
        __FILE__ ":" STRINGIFY(__LINE__), #expr, PRETTY_FUNCTION_NAME};       \
    xprofiler::Assert(args);                                                  \
  } while (0)

#ifdef __GNUC__
#define LIKELY(expr) __builtin_expect(!!(expr), 1)
#define UNLIKELY(expr) __builtin_expect(!!(expr), 0)
#define PRETTY_FUNCTION_NAME __PRETTY_FUNCTION__
#else
#define LIKELY(expr) expr
#define UNLIKELY(expr) expr
#define PRETTY_FUNCTION_NAME ""
#endif

#define STRINGIFY_(x) #x
#define STRINGIFY(x) STRINGIFY_(x)

#define CHECK(expr)          \
  do {                       \
    if (UNLIKELY(!(expr))) { \
      ERROR_AND_ABORT(expr); \
    }                        \
  } while (0)

#define CHECK_EQ(a, b) CHECK((a) == (b))
#define CHECK_GE(a, b) CHECK((a) >= (b))
#define CHECK_GT(a, b) CHECK((a) > (b))
#define CHECK_LE(a, b) CHECK((a) <= (b))
#define CHECK_LT(a, b) CHECK((a) < (b))
#define CHECK_NE(a, b) CHECK((a) != (b))
#define CHECK_NULL(val) CHECK((val) == nullptr)
#define CHECK_NOT_NULL(val) CHECK((val) != nullptr)
#define CHECK_IMPLIES(a, b) CHECK(!(a) || (b))

#ifdef DEBUG
#define DCHECK(expr) CHECK(expr)
#define DCHECK_EQ(a, b) CHECK((a) == (b))
#define DCHECK_GE(a, b) CHECK((a) >= (b))
#define DCHECK_GT(a, b) CHECK((a) > (b))
#define DCHECK_LE(a, b) CHECK((a) <= (b))
#define DCHECK_LT(a, b) CHECK((a) < (b))
#define DCHECK_NE(a, b) CHECK((a) != (b))
#define DCHECK_NULL(val) CHECK((val) == nullptr)
#define DCHECK_NOT_NULL(val) CHECK((val) != nullptr)
#define DCHECK_IMPLIES(a, b) CHECK(!(a) || (b))
#else
#define DCHECK(expr)
#define DCHECK_EQ(a, b)
#define DCHECK_GE(a, b)
#define DCHECK_GT(a, b)
#define DCHECK_LE(a, b)
#define DCHECK_LT(a, b)
#define DCHECK_NE(a, b)
#define DCHECK_NULL(val)
#define DCHECK_NOT_NULL(val)
#define DCHECK_IMPLIES(a, b)
#endif

#define UNREACHABLE(...) \
  ERROR_AND_ABORT("Unreachable code reached" __VA_OPT__(": ") __VA_ARGS__)

template <typename Inner, typename Outer>
constexpr uintptr_t OffsetOf(Inner Outer::*field) {
  return reinterpret_cast<uintptr_t>(&(static_cast<Outer*>(nullptr)->*field));
}

// The helper is for doing safe downcasts from base types to derived types.
template <typename Inner, typename Outer>
class ContainerOfHelper {
 public:
  inline ContainerOfHelper(Inner Outer::*field, Inner* pointer)
      : pointer_(reinterpret_cast<Outer*>(reinterpret_cast<uintptr_t>(pointer) -
                                          OffsetOf(field))) {}

  template <typename TypeName>
  inline operator TypeName*() const {
    return static_cast<TypeName*>(pointer_);
  }

 private:
  Outer* const pointer_;
};

template <typename Inner, typename Outer>
constexpr ContainerOfHelper<Inner, Outer> ContainerOf(Inner Outer::*field,
                                                      Inner* pointer) {
  return ContainerOfHelper<Inner, Outer>(field, pointer);
}

template <typename T, size_t N>
constexpr size_t arraysize(const T (&)[N]) {
  return N;
}

// Convenience wrapper around v8::String::NewFromOneByte
inline v8::Local<v8::String> OneByteString(v8::Isolate* isolate,
                                           const char* data, int length = -1);

}  // namespace xprofiler

#endif /* XPROFILER_SRC_UTIL_H */
