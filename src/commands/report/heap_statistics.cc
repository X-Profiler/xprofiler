#include "node_report.h"
#include "v8.h"

namespace xprofiler {
using v8::HeapSpaceStatistics;
using v8::HeapStatistics;
using v8::Isolate;

void NodeReport::SetHeapStatistics(JSONWriter* writer) {
  HeapStatistics v8_heap_stats;
  isolate_->GetHeapStatistics(&v8_heap_stats);

  writer->json_objectstart("heapStatistics");
  writer->json_keyvalue("heapTotal", v8_heap_stats.total_heap_size());
  writer->json_keyvalue("heapTotalCommitted",
                        v8_heap_stats.total_physical_size());
  writer->json_keyvalue("heapTotalUsed", v8_heap_stats.used_heap_size());
  writer->json_keyvalue("heapTotalAvailable",
                        v8_heap_stats.total_available_size());
  writer->json_keyvalue("heapLimit", v8_heap_stats.heap_size_limit());
  writer->json_objectend();

  HeapSpaceStatistics v8_heap_space_stats;
  writer->json_arraystart("heapSpaceStatistics");
  for (size_t i = 0; i < isolate_->NumberOfHeapSpaces(); i++) {
    isolate_->GetHeapSpaceStatistics(&v8_heap_space_stats, i);
    writer->json_start();
    writer->json_keyvalue("name", v8_heap_space_stats.space_name());
    writer->json_keyvalue("size", v8_heap_space_stats.space_size());
    writer->json_keyvalue("committed",
                          v8_heap_space_stats.physical_space_size());
    writer->json_keyvalue("capacity",
                          v8_heap_space_stats.space_used_size() +
                              v8_heap_space_stats.space_available_size());
    writer->json_keyvalue("used", v8_heap_space_stats.space_used_size());
    writer->json_keyvalue("available",
                          v8_heap_space_stats.space_available_size());
    writer->json_end();
  }
  writer->json_arrayend();
}
}  // namespace xprofiler
