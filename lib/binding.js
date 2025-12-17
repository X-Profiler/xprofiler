'use strict';

const path = require('path');
const fs = require('fs');

/**
 * Load the appropriate native binding based on environment configuration.
 *
 * Use XPROFILER_USE_RUST=true to use the Rust implementation.
 * Default is the C++ implementation via node-pre-gyp.
 */
function loadBinding() {
  const useRust = process.env.XPROFILER_USE_RUST === 'true';

  if (useRust) {
    return loadRustBinding();
  }

  return loadCppBinding();
}

/**
 * Load the C++ binding via @mapbox/node-pre-gyp
 */
function loadCppBinding() {
  const binary = require('@mapbox/node-pre-gyp');
  const bindingPath = binary.find(path.resolve(path.join(__dirname, '..', 'package.json')));
  return require(bindingPath);
}

/**
 * Load the Rust binding built by napi-rs
 */
function loadRustBinding() {
  const platform = process.platform;
  const arch = process.arch;

  // Try to find the Rust-built .node file
  // napi-rs generates files like: index.darwin-arm64.node
  const possiblePaths = [
    // Development build path (napi-rs default naming)
    path.join(__dirname, '..', 'build', 'binding', 'Release', `index.${platform}-${arch}.node`),
    path.join(__dirname, '..', 'build', 'binding', 'Debug', `index.${platform}-${arch}.node`),
    // Alternative naming
    path.join(__dirname, '..', 'build', 'binding', 'Release', `xprofiler-rs.${platform}-${arch}.node`),
    path.join(__dirname, '..', 'build', 'binding', 'Debug', `xprofiler-rs.${platform}-${arch}.node`),
    // Root directory (for published packages)
    path.join(__dirname, '..', `xprofiler-rs.${platform}-${arch}.node`),
    path.join(__dirname, '..', `index.${platform}-${arch}.node`),
    // Simple platform name variants
    path.join(__dirname, '..', 'build', 'binding', 'Release', 'xprofiler-rs.node'),
    path.join(__dirname, '..', 'build', 'binding', 'Release', 'index.node'),
  ];

  for (const bindingPath of possiblePaths) {
    if (fs.existsSync(bindingPath)) {
      return require(bindingPath);
    }
  }

  throw new Error(
    `Could not find xprofiler Rust binding. ` +
    `Searched paths:\n${possiblePaths.map(p => `  - ${p}`).join('\n')}\n` +
    `Please run 'npm run build:rs' to build the Rust binding.`
  );
}

/**
 * Check if Rust binding is being used
 */
function isUsingRust() {
  return process.env.XPROFILER_USE_RUST === 'true';
}

module.exports = {
  loadBinding,
  loadCppBinding,
  loadRustBinding,
  isUsingRust,
};
