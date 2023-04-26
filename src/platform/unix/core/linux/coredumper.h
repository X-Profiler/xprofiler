/* Copyright (c) 2005-2008, Google Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * ---
 * Author: Markus Gutschke, Carl Crous
 *
 * Code to extract a core dump snapshot of the current process.
 */
#pragma once

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Description of external compressor programs to use for creating compressed
 * coredumps. There are a few predefined compressor descriptions; callers
 * can also define their own compressors.
 * All functions expect an array of compressors. Array entries will be tried
 * in sequence until an executable compressor has been found. An empty
 * c-string in place of the compressor name signals that no compression should
 * be performed.
 * The end of the array is signalled by an entry that is completely zero'd out.
 */
struct CoredumperCompressor {
  const char* compressor;  /* File name of compressor; e.g. gzip              */
  const char* const* args; /* execv()-style command line arguments            */
  const char* suffix;      /* Suffix that should be appended; e.g. .gz        */
};

/* Description of a elf note for use in the PT_NOTES section of the core file.
 */
struct CoredumperNote {
  const char* name;              /* The vendor name                         */
  unsigned int type;             /* A vendor specific type                  */
  unsigned int description_size; /* The size of the description field       */
  const void* description;       /* The note data                           */
};

/* Parameters used to control the core dumper. Future versions of this
 * structure must be backwards compatible so any new fields must be appended to
 * the end.
 */
struct CoreDumpParameters {
  /* The size of this structure. This is used to make sure future versions are
   * backwards compatible.
   */
  size_t size;
  /* Specific settings for the core dumper. See COREDUMPER_FLAG_*            */
  int flags;
  /* The maximum file size for the core dump.                                */
  size_t max_length;
  /* The set of compressors to choose from.                                  */
  const struct CoredumperCompressor* compressors;
  /* After dumping a compressed core, this will be set to the compressor which
   * was used to compress the core file.
   */
  struct CoredumperCompressor** selected_compressor;
  /* Extra notes to write to the core file notes section.                    */
  const struct CoredumperNote* notes;
  /* The amount of notes in the notes array.                                 */
  int note_count;
  /* Callback function */
  int (*callback_fn)(void*);
  /* Callback argument */
  void* callback_arg;
};

/* The core file is limited in size and max_length denotes the maximum size. If
 * the core file exceeds this maximum, the file will be truncated.
 */
#define COREDUMPER_FLAG_LIMITED 1

/* The core file is limited in size and max_length denotes the maximum size. If
 * the core file exceeds this maximum, the largest memory segments will be
 * reduced or removed first in order to preserve the smaller ones.
 */
#define COREDUMPER_FLAG_LIMITED_BY_PRIORITY 2

/* Try compressing with either bzip2, gzip, or compress. If all of those fail,
 * fall back on generating an uncompressed file.
 */
extern const struct CoredumperCompressor COREDUMPER_COMPRESSED[];

/* Try compressing with a specific compressor. Fail if no compressor could
 * be found.
 */
extern const struct CoredumperCompressor COREDUMPER_BZIP2_COMPRESSED[];
extern const struct CoredumperCompressor COREDUMPER_GZIP_COMPRESSED[];
extern const struct CoredumperCompressor COREDUMPER_COMPRESS_COMPRESSED[];

/* Try compressing with a specific compressor. Fall back on generating an
 * uncompressed file, if the specified compressor is unavailable.
 */
extern const struct CoredumperCompressor COREDUMPER_TRY_BZIP2_COMPRESSED[];
extern const struct CoredumperCompressor COREDUMPER_TRY_GZIP_COMPRESSED[];
extern const struct CoredumperCompressor COREDUMPER_TRY_COMPRESS_COMPRESSED[];

/* Always create an uncompressed core file.
 */
extern const struct CoredumperCompressor COREDUMPER_UNCOMPRESSED[];

/* Returns a file handle that can be read to obtain a snapshot of the
 * current state of this process. If a core file could not be
 * generated for any reason, -1 is returned and "errno" will be set
 * appropriately.
 *
 * This function momentarily suspends all threads, while creating a
 * COW (copy-on-write) copy of the process's address space.
 *
 * This function is neither reentrant nor async signal safe. Callers
 * should wrap a mutex around the invocation of this function, if necessary.
 *
 * The current implementation tries very hard to behave reasonably when
 * called from a signal handler, but no guarantees are made that this will
 * always work. Most importantly, it is the caller's responsibility to
 * make sure that there are never more than one instance of GetCoreDump()
 * or WriteCoreDump() executing concurrently.
 */
int GetCoreDump(void);

/* Gets a core dump with the given parameters. This is not compatible with any
 * core size limiting parameters.
 */
int GetCoreDumpWith(const struct CoreDumpParameters* params);

/* Attempts to compress the core file on the fly, if a suitable compressor
 * could be located. Sets "selected_compressor" to the compressor that
 * was picked.
 */
int GetCompressedCoreDump(const struct CoredumperCompressor compressors[],
                          struct CoredumperCompressor** selected_compressor);

/* Writes the core file to disk. This is a convenience method wrapping
 * GetCoreDump(). If a core file could not be generated for any reason,
 * -1 is returned and errno is set appropriately. On success, zero is
 * returned.
 */
int WriteCoreDump(const char* file_name);

/* Writes a core dump to the given file with the given parameters.           */
int WriteCoreDumpWith(const struct CoreDumpParameters* params,
                      const char* file_name);

/* Callers might need to restrict the maximum size of the core file. This
 * convenience method provides the necessary support to emulate "ulimit -c".
 */
int WriteCoreDumpLimited(const char* file_name, size_t max_length);

/* Writes a limited size core file, however instead of truncating the file at
 * the limit, the core dumper will prioritize smaller memory segments. This
 * means that a large heap will most likely either be only partially included
 * or not included at all. If the max_length is set too small, this could cause
 * performance issues.
 */
int WriteCoreDumpLimitedByPriority(const char* file_name, size_t max_length);

/* Attempts to compress the core file on the fly, if a suitable compressor
 * could be located. Sets "selected_compressor" to the compressor that
 * was picked. The filename automatically has a suitable suffix appended
 * to it. Normally this would be ".bz2" for bzip2 compression ".gz" for
 * gzip compression, or ".Z" for compress compression. This behavior can
 * be changed by defining custom CoredumperCompressor descriptions.
 */
int WriteCompressedCoreDump(const char* file_name, size_t max_length,
                            const struct CoredumperCompressor compressors[],
                            struct CoredumperCompressor** selected_compressor);

/* A convenience definition to clear core dump parameters.
 */
#define ClearCoreDumpParameters(p) \
  ClearCoreDumpParametersInternal((p), sizeof(struct CoreDumpParameters))

/* Checks if the current version of the coredumper has a specific parameter.
 */
#define CoreDumpParametersHas(p, f) \
  ((p)->size >= offsetof(struct CoreDumpParameters, f) + sizeof((p)->f))

/* Sets a coredumper parameter to a given value. This will abort the program if
 * the given parameter doesn't exist in the parameters.
 */
#define SetCoreDumpParameter(p, f, v)   \
  do {                                  \
    if (!CoreDumpParametersHas(p, f)) { \
      abort();                          \
    }                                   \
    (p)->f = (v);                       \
  } while (0)

/* Gets a coredumper parameter. If the parameter doesn't exist, 0 is returned.
 */
#define GetCoreDumpParameter(p, f) (CoreDumpParametersHas(p, f) ? (p)->f : 0)

/* Clears the given coredumper parameters to zero, sets the size parameter and
 * the max_length parameter to SIZE_MAX.
 */
void ClearCoreDumpParametersInternal(struct CoreDumpParameters* params,
                                     size_t size);

/* Sets the coredumper parameters to provide a limited core dump. Returns
 * zero on success otherwise -1 will be returned and errno will be set.
 */
int SetCoreDumpLimited(struct CoreDumpParameters* params, size_t max_length);

/* Sets the coredumper parameters to provide a compressed core dump. Returns
 * zero on success otherwise -1 will be returned and errno will be set.
 */
int SetCoreDumpCompressed(struct CoreDumpParameters* params,
                          const struct CoredumperCompressor* compressors,
                          struct CoredumperCompressor** selected_compressor);

/* Sets the coredumper parameters to provide a prioritized limited core file.
 * Returns zero on success otherwise -1 will be returned and errno will be set.
 */
int SetCoreDumpLimitedByPriority(struct CoreDumpParameters* params,
                                 size_t max_length);

/* Sets the coredumper parameters to add extra notes to the core file.
 * Returns zero on success otherwise -1 will be returned and errno will be set.
 */
int SetCoreDumpNotes(struct CoreDumpParameters* params,
                     struct CoredumperNote* notes, int note_count);

/* Sets the coredumper parameters to provide a callback function.
 * The callback will be invoked after all threads have been suspended but
 * before any coredump operation is invoked.  The callback argument will be
 * provided.  If the return value is 0 then the coredump will continue; if the
 * value is something other than 0 then the coredump will not be generated.
 */
int SetCoreDumpCallback(struct CoreDumpParameters* params, int (*fn)(void*),
                        void* arg);

#ifdef __cplusplus
}
#endif
