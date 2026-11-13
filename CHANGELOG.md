# Changelog

## 4.0.0 (2026-11-13)

### Breaking Changes

* Support only Node.js 22, 24 and 26; drop Node.js 18 and 20.
* Remove legacy Node.js binary targets and the `pack-7u` script.

### Maintenance

* Require NAN 2.30.0 or newer within v2.
* Use npm 11.19.1 (node-gyp 12.4.0) in CI to detect Visual Studio 2026 on Windows, including Node.js 22 builds.
* Include all three supported Node.js majors in binary packaging.
* Build 15 native binaries for Linux x64/arm64, macOS x64/arm64, and Windows x64. Preserve glibc 2.17 compatibility for Node.js 22 Linux x64; other Linux binaries target glibc 2.28.
* Replace the manual release script with automated GitHub and npm publishing after version PRs merge. Upload all native binaries before publishing to npm.

## 3.1.0 (2025-10-29)

* feat: build from Node.js v24 (#253) ([10dd462](https://github.com/X-Profiler/xprofiler/commit/10dd462)), closes [#253](https://github.com/X-Profiler/xprofiler/issues/253)

## [3.0.0](https://github.com/X-Profiler/xprofiler/compare/v2.6.1...v3.0.0) (2025-03-10)


### ⚠ BREAKING CHANGES

* drop Node.js < 18.19.0 support

closes https://github.com/X-Profiler/egg-xtransit/issues/10

### Features

* monitor http server request by diagnostics_channel ([#248](https://github.com/X-Profiler/xprofiler/issues/248)) ([293abe8](https://github.com/X-Profiler/xprofiler/commit/293abe8f5e8c8316dd68f02bf0dc3f5266d568dc))
