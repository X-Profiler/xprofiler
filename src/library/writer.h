#pragma once

#include <fstream>
#include <limits>
#include <string>

namespace xprofiler {
// JSON compiler definitions.
// from node-v12.13.0 src/node_report.h
std::string EscapeJsonChars(const std::string& str);

class JSONWriter {
 public:
  explicit JSONWriter(std::ostream& out) : out_(out) {}

  inline void indent() { indent_ += 2; }
  inline void deindent() { indent_ -= 2; }
  inline void advance() {
    for (int i = 0; i < indent_; i++) out_ << ' ';
  }

  inline void json_start() {
    if (state_ == kAfterValue) out_ << ',';
    out_ << '\n';
    advance();
    out_ << '{';
    indent();
    state_ = kObjectStart;
  }

  inline void json_end() {
    out_ << '\n';
    deindent();
    advance();
    out_ << '}';
    state_ = kAfterValue;
  }
  template <typename T>
  inline void json_objectstart(T key) {
    if (state_ == kAfterValue) out_ << ',';
    out_ << '\n';
    advance();
    write_string(key);
    out_ << ": {";
    indent();
    state_ = kObjectStart;
  }

  template <typename T>
  inline void json_arraystart(T key) {
    if (state_ == kAfterValue) out_ << ',';
    out_ << '\n';
    advance();
    write_string(key);
    out_ << ": [";
    indent();
    state_ = kObjectStart;
  }
  inline void json_objectend() {
    out_ << '\n';
    deindent();
    advance();
    out_ << '}';
    state_ = kAfterValue;
  }

  inline void json_arrayend() {
    out_ << '\n';
    deindent();
    advance();
    out_ << ']';
    state_ = kAfterValue;
  }
  template <typename T, typename U>
  inline void json_keyvalue(const T& key, const U& value) {
    if (state_ == kAfterValue) out_ << ',';
    out_ << '\n';
    advance();
    write_string(key);
    out_ << ": ";
    write_value(value);
    state_ = kAfterValue;
  }

  template <typename U>
  inline void json_element(const U& value) {
    if (state_ == kAfterValue) out_ << ',';
    out_ << '\n';
    advance();
    write_value(value);
    state_ = kAfterValue;
  }

  struct Null {};  // Usable as a JSON value.

 private:
  template <typename T, typename test_for_number = typename std::enable_if<
                            std::numeric_limits<T>::is_specialized, bool>::type>
  inline void write_value(T number) {
    if (std::is_same<T, bool>::value)
      out_ << (number ? "true" : "false");
    else
      out_ << number;
  }

  inline void write_value(Null null) { out_ << "null"; }
  inline void write_value(const char* str) { write_string(str); }
  inline void write_value(const std::string& str) { write_string(str); }

  inline void write_string(const std::string& str) {
    out_ << '"' << EscapeJsonChars(str) << '"';
  }
  inline void write_string(const char* str) { write_string(std::string(str)); }

  enum JSONState { kObjectStart, kAfterValue };
  std::ostream& out_;
  int indent_ = 0;
  int state_ = kObjectStart;
};
}  // namespace xprofiler
