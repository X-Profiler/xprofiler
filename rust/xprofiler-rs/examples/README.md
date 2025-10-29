# XProfiler-RS Examples

This directory contains examples demonstrating how to use the xprofiler-rs library in different scenarios.

## Prerequisites

- Node.js 14.0.0 or higher
- The xprofiler-rs library built and available

## Available Examples

### Node.js Examples

Located in the `nodejs/` directory:

#### 1. Basic Usage (`basic_usage.js`)

Demonstrates the fundamental usage of xprofiler-rs:
- Starting and stopping the profiler
- CPU intensive tasks
- Memory allocation patterns
- HTTP request simulation
- Getting profiler statistics

**Run:**
```bash
cd nodejs
npm run basic
```

#### 2. Advanced Configuration (`advanced_config.js`)

Shows advanced features and configuration options:
- Custom profiler configuration
- Different workload simulations
- Detailed statistics reporting
- Garbage collection monitoring
- Memory pattern analysis

**Run:**
```bash
cd nodejs
npm run advanced
```

**Run with garbage collection exposed:**
```bash
cd nodejs
npm run advanced-gc
```

## Running Examples

### Quick Start

1. Navigate to the examples directory:
   ```bash
   cd examples/nodejs
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Run an example:
   ```bash
   npm run basic
   # or
   npm run advanced
   ```

### With Garbage Collection Monitoring

To enable garbage collection monitoring, run examples with the `--expose-gc` flag:

```bash
npm run basic-gc
# or
npm run advanced-gc
```

## Example Output

The examples will output:
- Real-time profiling information
- Performance statistics
- Memory usage patterns
- CPU utilization data
- HTTP request metrics (simulated)
- Libuv event loop statistics

## Configuration Options

The advanced example demonstrates various configuration options:

```javascript
const config = {
    cpu_sample_interval: 10,        // CPU sampling interval in ms
    cpu_profiling_enabled: true,    // Enable CPU profiling
    memory_profiling_enabled: true, // Enable memory profiling
    memory_sample_interval: 100,    // Memory sampling interval in ms
    gc_profiling_enabled: true,     // Enable GC profiling
    http_monitoring_enabled: true,  // Enable HTTP monitoring
    http_timeout_threshold: 5000,   // HTTP timeout threshold in ms
    libuv_monitoring_enabled: true, // Enable libuv monitoring
    log_level: 'info',              // Log level
    output_format: 'json'           // Output format
};
```

## Understanding the Output

### CPU Statistics
- `samples`: Number of CPU samples collected
- `average_cpu`: Average CPU usage percentage

### Memory Statistics
- `rss`: Resident Set Size (physical memory)
- `heap_used`: Used heap memory
- `heap_total`: Total heap memory allocated

### GC Statistics
- `collections`: Number of garbage collection cycles
- `total_time`: Total time spent in garbage collection

### HTTP Statistics
- `request_count`: Number of HTTP requests processed
- `average_response_time`: Average response time in milliseconds

### Libuv Statistics
- `active_handles`: Number of active libuv handles
- `loop_iterations`: Number of event loop iterations

## Troubleshooting

### Common Issues

1. **Module not found**: Ensure xprofiler-rs is built and the path in package.json is correct
2. **Permission errors**: Make sure you have read/write permissions in the directory
3. **Node.js version**: Ensure you're using Node.js 14.0.0 or higher

### Debug Mode

To run examples with debug output:

```bash
DEBUG=xprofiler* node basic_usage.js
```

## Contributing

When adding new examples:
1. Create descriptive filenames
2. Add comprehensive comments
3. Update this README
4. Add npm scripts in package.json
5. Test examples thoroughly

## Related Documentation

- [Main README](../../README.md) - General library documentation
- [API Documentation](../../docs/) - Detailed API reference
- [Configuration Guide](../../docs/configuration.md) - Configuration options