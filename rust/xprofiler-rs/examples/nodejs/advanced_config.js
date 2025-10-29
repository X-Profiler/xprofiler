const xprofiler = require('../../');

// Advanced configuration example for xprofiler-rs
console.log('Starting XProfiler advanced configuration example...');

// Configure profiler with custom settings
const config = {
    // CPU profiling settings
    cpu_sample_interval: 10, // milliseconds
    cpu_profiling_enabled: true,
    
    // Memory profiling settings
    memory_profiling_enabled: true,
    memory_sample_interval: 100, // milliseconds
    
    // GC profiling settings
    gc_profiling_enabled: true,
    
    // HTTP monitoring settings
    http_monitoring_enabled: true,
    http_timeout_threshold: 5000, // milliseconds
    
    // Libuv monitoring settings
    libuv_monitoring_enabled: true,
    
    // Output settings
    log_level: 'info',
    output_format: 'json'
};

console.log('Configuration:', JSON.stringify(config, null, 2));

// Start profiling with custom configuration
try {
    xprofiler.configure(config);
    xprofiler.start();
    console.log('Profiler started with custom configuration');
} catch (error) {
    console.error('Failed to start profiler:', error.message);
    process.exit(1);
}

// Demonstrate different types of workloads
class WorkloadSimulator {
    constructor() {
        this.requestCounter = 0;
    }
    
    // Simulate CPU-bound work
    async simulateCpuWork(duration = 1000) {
        console.log(`Starting CPU work for ${duration}ms...`);
        const start = Date.now();
        
        while (Date.now() - start < duration) {
            // CPU intensive calculation
            Math.sqrt(Math.random() * 1000000);
        }
        
        console.log(`CPU work completed in ${Date.now() - start}ms`);
    }
    
    // Simulate memory allocation patterns
    simulateMemoryPatterns() {
        console.log('Simulating various memory allocation patterns...');
        
        // Pattern 1: Large objects
        const largeObjects = [];
        for (let i = 0; i < 10; i++) {
            largeObjects.push({
                id: i,
                data: new Array(100000).fill(Math.random()),
                timestamp: Date.now()
            });
        }
        
        // Pattern 2: Many small objects
        const smallObjects = [];
        for (let i = 0; i < 10000; i++) {
            smallObjects.push({
                id: i,
                value: Math.random()
            });
        }
        
        // Pattern 3: Circular references (potential memory leaks)
        const circularRefs = [];
        for (let i = 0; i < 100; i++) {
            const obj = { id: i };
            obj.self = obj;
            circularRefs.push(obj);
        }
        
        console.log(`Created ${largeObjects.length} large objects, ${smallObjects.length} small objects, ${circularRefs.length} circular references`);
        
        return { largeObjects, smallObjects, circularRefs };
    }
    
    // Simulate HTTP request patterns
    simulateHttpTraffic() {
        console.log('Simulating HTTP traffic patterns...');
        
        const patterns = [
            { name: 'fast_requests', count: 20, delay: 10, duration: 50 },
            { name: 'slow_requests', count: 5, delay: 200, duration: 1000 },
            { name: 'burst_requests', count: 50, delay: 1, duration: 100 }
        ];
        
        patterns.forEach(pattern => {
            this.simulateRequestPattern(pattern);
        });
    }
    
    simulateRequestPattern({ name, count, delay, duration }) {
        console.log(`Starting ${name}: ${count} requests with ${delay}ms delay, ${duration}ms duration`);
        
        for (let i = 0; i < count; i++) {
            setTimeout(() => {
                const requestId = ++this.requestCounter;
                console.log(`[${name}] Request ${requestId} started`);
                
                // Simulate request processing
                setTimeout(() => {
                    console.log(`[${name}] Request ${requestId} completed`);
                }, duration + Math.random() * 100);
                
            }, i * delay);
        }
    }
    
    // Simulate garbage collection triggers
    async triggerGarbageCollection() {
        console.log('Triggering garbage collection scenarios...');
        
        // Create and release large amounts of memory
        for (let cycle = 0; cycle < 5; cycle++) {
            console.log(`GC cycle ${cycle + 1}`);
            
            // Allocate memory
            const tempData = new Array(1000000).fill(Math.random());
            
            // Use the memory briefly
            const sum = tempData.reduce((a, b) => a + b, 0);
            console.log(`Processed ${tempData.length} items, sum: ${sum.toFixed(2)}`);
            
            // Force garbage collection if available
            if (global.gc) {
                global.gc();
                console.log('Forced garbage collection');
            }
            
            // Wait before next cycle
            await new Promise(resolve => setTimeout(resolve, 500));
        }
    }
}

// Run the advanced example
async function runAdvancedExample() {
    const simulator = new WorkloadSimulator();
    
    try {
        console.log('\n=== CPU Workload Simulation ===');
        await simulator.simulateCpuWork(2000);
        
        console.log('\n=== Memory Pattern Simulation ===');
        const memoryData = simulator.simulateMemoryPatterns();
        
        console.log('\n=== HTTP Traffic Simulation ===');
        simulator.simulateHttpTraffic();
        
        console.log('\n=== Garbage Collection Simulation ===');
        await simulator.triggerGarbageCollection();
        
        // Wait for async operations to complete
        await new Promise(resolve => setTimeout(resolve, 3000));
        
        console.log('\n=== Final Profiler Statistics ===');
        try {
            const stats = xprofiler.getStats();
            console.log('Detailed profiler statistics:');
            
            // Display stats in a formatted way
            if (stats.cpu) {
                console.log('\nCPU Statistics:');
                console.log(`  Samples: ${stats.cpu.samples || 'N/A'}`);
                console.log(`  Average CPU: ${stats.cpu.average_cpu || 'N/A'}%`);
            }
            
            if (stats.memory) {
                console.log('\nMemory Statistics:');
                console.log(`  RSS: ${stats.memory.rss || 'N/A'} bytes`);
                console.log(`  Heap Used: ${stats.memory.heap_used || 'N/A'} bytes`);
                console.log(`  Heap Total: ${stats.memory.heap_total || 'N/A'} bytes`);
            }
            
            if (stats.gc) {
                console.log('\nGC Statistics:');
                console.log(`  Collections: ${stats.gc.collections || 'N/A'}`);
                console.log(`  Total Time: ${stats.gc.total_time || 'N/A'}ms`);
            }
            
            if (stats.http) {
                console.log('\nHTTP Statistics:');
                console.log(`  Requests: ${stats.http.request_count || 'N/A'}`);
                console.log(`  Average Response Time: ${stats.http.average_response_time || 'N/A'}ms`);
            }
            
            if (stats.libuv) {
                console.log('\nLibuv Statistics:');
                console.log(`  Active Handles: ${stats.libuv.active_handles || 'N/A'}`);
                console.log(`  Loop Iterations: ${stats.libuv.loop_iterations || 'N/A'}`);
            }
            
        } catch (error) {
            console.error('Error getting detailed stats:', error.message);
        }
        
    } catch (error) {
        console.error('Error during simulation:', error.message);
    } finally {
        console.log('\nStopping profiler...');
        xprofiler.stop();
        console.log('Advanced example completed!');
    }
}

// Handle process exit
process.on('exit', () => {
    try {
        xprofiler.stop();
    } catch (error) {
        // Ignore errors during cleanup
    }
});

process.on('SIGINT', () => {
    console.log('\nReceived SIGINT, cleaning up...');
    process.exit(0);
});

// Run the advanced example
runAdvancedExample().catch(error => {
    console.error('Fatal error:', error);
    process.exit(1);
});