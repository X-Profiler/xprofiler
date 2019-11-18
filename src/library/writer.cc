#include <string>

namespace xprofiler {
using std::string;

// from node-v12.13.0 src/node_report_utils.cc
string EscapeJsonChars(const string &str) {
  const string control_symbols[0x20] = {
      "\\u0000", "\\u0001", "\\u0002", "\\u0003", "\\u0004", "\\u0005",
      "\\u0006", "\\u0007", "\\b",     "\\t",     "\\n",     "\\v",
      "\\f",     "\\r",     "\\u000e", "\\u000f", "\\u0010", "\\u0011",
      "\\u0012", "\\u0013", "\\u0014", "\\u0015", "\\u0016", "\\u0017",
      "\\u0018", "\\u0019", "\\u001a", "\\u001b", "\\u001c", "\\u001d",
      "\\u001e", "\\u001f"};

  string ret;
  size_t last_pos = 0;
  size_t pos = 0;
  for (; pos < str.size(); ++pos) {
    string replace;
    char ch = str[pos];
    if (ch == '\\') {
      replace = "\\\\";
    } else if (ch == '\"') {
      replace = "\\\"";
    } else {
      size_t num = static_cast<size_t>(ch);
      if (num < 0x20)
        replace = control_symbols[num];
    }
    if (!replace.empty()) {
      if (pos > last_pos) {
        ret += str.substr(last_pos, pos - last_pos);
      }
      last_pos = pos + 1;
      ret += replace;
    }
  }
  // Append any remaining symbols.
  if (last_pos < str.size()) {
    ret += str.substr(last_pos, pos - last_pos);
  }
  return ret;
}
} // namespace xprofiler