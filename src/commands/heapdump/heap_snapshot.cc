#include "heap_snapshot.h"

#include "library/writer.h"
#include "logger.h"

namespace xprofiler {
using v8::OutputStream;

class FileOutputStream final : public OutputStream {
 public:
  FileOutputStream(std::string filename) {
    stream_ = fopen(filename.c_str(), "w");
  }
  ~FileOutputStream() {
    if (stream_ != nullptr) {
      fclose(stream_);
    }
  }

  // Delete copy
  FileOutputStream(const FileOutputStream& other) = delete;
  FileOutputStream& operator=(const FileOutputStream& other) = delete;

  bool is_open() { return stream_ != nullptr; }

  int GetChunkSize() override {
    return 65536;  // big chunks == faster
  }

  void EndOfStream() override {}

  WriteResult WriteAsciiChunk(char* data, int size) override {
    const size_t len = static_cast<size_t>(size);
    size_t off = 0;

    while (off < len && !feof(stream_) && !ferror(stream_))
      off += fwrite(data + off, 1, len - off, stream_);

    return off == len ? kContinue : kAbort;
  }

 private:
  FILE* stream_ = nullptr;
};

void HeapSnapshot::Serialize(HeapSnapshotPointer profile,
                             std::string filename) {
  FileOutputStream stream(filename);
  if (!stream.is_open()) {
    Error("heapdump", "open file %s failed.", filename.c_str());
    return;
  }
  profile->Serialize(&stream, v8::HeapSnapshot::kJSON);
}
}  // namespace xprofiler
