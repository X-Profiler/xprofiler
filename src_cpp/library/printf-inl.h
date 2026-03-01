#ifndef XPROFILER_SRC_LIBRARY_PRINT_INL_H
#define XPROFILER_SRC_LIBRARY_PRINT_INL_H
#include "printf.h"
#include "util.h"

namespace xprofiler {
struct ToStringHelper {
  template <typename T>
  static std::string Convert(const T& value, std::string (T::*to_string)()
                                                 const = &T::ToString) {
    return (value.*to_string)();
  }
  template <typename T,
            typename test_for_number = typename std::enable_if<
                std::is_arithmetic<T>::value, bool>::type,
            typename dummy = bool>
  static std::string Convert(const T& value) {
    return std::to_string(value);
  }
  static std::string Convert(const char* value) {
    return value != nullptr ? value : "(null)";
  }
  static std::string Convert(const std::string& value) { return value; }
  static std::string Convert(bool value) { return value ? "true" : "false"; }
  template <unsigned BASE_BITS, typename T,
            typename std::enable_if<std::is_integral<T>::value, int>::type = 0>
  static std::string BaseConvert(const T& value) {
    auto v = static_cast<uint64_t>(value);
    char ret[3 * sizeof(T)];
    char* ptr = ret + 3 * sizeof(T) - 1;
    *ptr = '\0';
    const char* digits = "0123456789abcdef";
    do {
      unsigned digit = v & ((1 << BASE_BITS) - 1);
      *--ptr = (BASE_BITS < 4 ? static_cast<char>('0' + digit) : digits[digit]);
    } while ((v >>= BASE_BITS) != 0);
    return ptr;
  }
  template <unsigned BASE_BITS, typename T,
            typename std::enable_if<!std::is_integral<T>::value, int>::type = 0>
  static std::string BaseConvert(T value) {
    return Convert(std::forward<T>(value));
  }
};

template <typename T>
std::string ToString(const T& value) {
  return ToStringHelper::Convert(value);
}

template <unsigned BASE_BITS, typename T>
std::string ToBaseString(const T& value) {
  return ToStringHelper::BaseConvert<BASE_BITS>(value);
}

inline std::string SPrintFImpl(const char* format) {
  const char* p = strchr(format, '%');
  if (LIKELY(p == nullptr)) return format;
  CHECK_EQ(p[1], '%');  // Only '%%' allowed when there are no arguments.

  return std::string(format, p + 1) + SPrintFImpl(p + 2);
}

template <typename Arg, typename... Args>
std::string SPrintFImpl(  // NOLINT(runtime/string)
    const char* format, Arg&& arg, Args&&... args) {
  const char* p = strchr(format, '%');
  if (p == nullptr) return "";
  std::string ret(format, p);
  // Ignore long / size_t modifiers
  while (strchr("lz", *++p) != nullptr) {
  }
  switch (*p) {
    case '%': {
      return ret + '%' +
             SPrintFImpl(p + 1, std::forward<Arg>(arg),
                         std::forward<Args>(args)...);
    }
    default: {
      return ret + '%' +
             SPrintFImpl(p, std::forward<Arg>(arg),
                         std::forward<Args>(args)...);
    }
    case 'f':
    case 'd':
    case 'i':
    case 'u':
    case 's':
      ret += ToString(arg);
      break;
    case 'o':
      ret += ToBaseString<3>(arg);
      break;
    case 'x':
      ret += ToBaseString<4>(arg);
      break;
    case 'p': {
      CHECK(std::is_pointer<typename std::remove_reference<Arg>::type>::value);
      char out[20];
      int n = snprintf(out, sizeof(out), "%p",
                       *reinterpret_cast<const void* const*>(&arg));
      CHECK_GE(n, 0);
      ret += out;
      break;
    }
  }
  return ret + SPrintFImpl(p + 1, std::forward<Args>(args)...);
}

template <typename... Args>
std::string SPrintF(  // NOLINT(runtime/string)
    const char* format, Args&&... args) {
  return SPrintFImpl(format, std::forward<Args>(args)...);
}
}  // namespace xprofiler
#endif