const xprofiler = require('../../');

// Basic usage example for xprofiler-rs
console.log('Starting XProfiler basic usage example...');

// Start profiling with default configuration
xprofiler.start();

// Simulate some CPU work
function cpuIntensiveTask() {
    const start = Date.now();
    let result = 0;
    for (let i = 0; i < 1000000; i++) {
        result += Math.sqrt(i);
    }
    console.log(`CPU task completed in ${Date.now() - start}ms, result: ${result.toFixed(2)}`);
}

// Simulate memory allocation
function memoryIntensiveTask() {
    const arrays = [];
    for (let i = 0; i < 100; i++) {
        arrays.push(new Array(10000).fill(Math.random()));
    }
    console.log(`Created ${arrays.length} arrays with ${arrays[0].length} elements each`);
    return arrays;
}

// Simulate HTTP-like operations
function simulateHttpRequests() {
    console.log('Simulating HTTP requests...');
    
    for (let i = 0; i < 10; i++) {
        setTimeout(() => {
            console.log(`Processing request ${i + 1}`);
            
            // Simulate request processing
            const processingTime = Math.random() * 100;
            setTimeout(() => {
                console.log(`Request ${i + 1} completed in ${processingTime.toFixed(2)}ms`);
            }, processingTime);
        }, i * 50);
    }
}

// Run the example
async function runExample() {
    console.log('\n=== CPU Intensive Task ===');
    cpuIntensiveTask();
    
    console.log('\n=== Memory Intensive Task ===');
    const memoryData = memoryIntensiveTask();
    
    console.log('\n=== HTTP Simulation ===');
    simulateHttpRequests();
    
    // Wait a bit for async operations
    setTimeout(() => {
        console.log('\n=== Getting Profiler Stats ===');
        try {
            const stats = xprofiler.getStats();
            console.log('Profiler statistics:', JSON.stringify(stats, null, 2));
        } catch (error) {
            console.log('Error getting stats:', error.message);
        }
        
        // Stop profiling
        console.log('\nStopping profiler...');
        xprofiler.stop();
        console.log('Example completed!');
    }, 2000);
}

// Handle process exit
process.on('exit', () => {
    console.log('Process exiting, stopping profiler...');
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

// Run the example
runExample();