const xprofiler = require('./xprofiler.node');
console.log('Loading xprofiler.node...');
try {
  xprofiler.start();
  console.log('xprofiler.start() called successfully.');
  const config = xprofiler.getXprofilerConfig();
  console.log('xprofiler.getXprofilerConfig() returned:', config);
} catch (err) {
  console.error('Error:', err);
}
