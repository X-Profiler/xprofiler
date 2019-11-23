#include "heap_snapshot.h"

#include "../../library/writer.h"
#include "../../logger.h"

namespace xprofiler {
using v8::OutputStream;

class FileOutputStream : public OutputStream {
 public:
  FileOutputStream(FILE *stream) : stream_(stream) {}

  virtual int GetChunkSize() {
    return 65536;  // big chunks == faster
  }

  virtual void EndOfStream() {}

  virtual WriteResult WriteAsciiChunk(char *data, int size) {
    const size_t len = static_cast<size_t>(size);
    size_t off = 0;

    while (off < len && !feof(stream_) && !ferror(stream_))
      off += fwrite(data + off, 1, len - off, stream_);

    return off == len ? kContinue : kAbort;
  }

 private:
  FILE *stream_;
};

void Snapshot::Serialize(const HeapSnapshot *profile, string filename) {
  FILE *fp = fopen(filename.c_str(), "w");
  if (fp == NULL) {
    Error("heapdump", "open file %s failed.", filename.c_str());
    return;
  }
  FileOutputStream stream(fp);
  profile->Serialize(&stream, HeapSnapshot::kJSON);
  fclose(fp);
  const_cast<HeapSnapshot *>(profile)->Delete();
}
}  // namespace xprofiler
