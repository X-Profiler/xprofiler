'use strict';

const assert = require('assert');
const { execFileSync } = require('child_process');
const { appendFileSync } = require('fs');
const semver = require('semver');

function getRelease(previousVersion, version) {
  assert(semver.valid(version) === version, `Invalid release version: ${version}`);
  assert(semver.valid(previousVersion), `Invalid previous version: ${previousVersion}`);
  assert(version === previousVersion || semver.gt(version, previousVersion),
    `Release version must increase: ${previousVersion} -> ${version}`);
  return {
    changed: version !== previousVersion,
    version,
    tag: `v${version}`,
    dist_tag: semver.prerelease(version) ? 'next' : 'latest',
  };
}

async function isPublished(name, version, request = global.fetch) {
  const url = `https://registry.npmjs.org/${encodeURIComponent(name)}/${encodeURIComponent(version)}`;
  const response = await request(url, { signal: global.AbortSignal.timeout(30000) });
  if (response.status === 404) {
    return false;
  }
  assert(response.ok, `npm registry returned HTTP ${response.status}`);
  const published = await response.json();
  assert(published.version === version, 'npm registry returned an unexpected version');
  return true;
}

async function main() {
  const { name, version } = require('../package.json');
  const ref = process.argv[2];
  let outputs;
  if (ref === '--published') {
    outputs = { published: await isPublished(name, version) };
  } else {
    assert(ref && /^[a-f0-9]{40}$/.test(ref), 'Expected the previous commit SHA');
    const previous = JSON.parse(execFileSync('git', ['show', `${ref}:package.json`], { encoding: 'utf8' }));
    outputs = getRelease(previous.version, version);
  }
  const result = Object.entries(outputs).map(([key, value]) => `${key}=${value}\n`).join('');
  process.stdout.write(result);
  if (process.env.GITHUB_OUTPUT) {
    appendFileSync(process.env.GITHUB_OUTPUT, result);
  }
}

module.exports = { getRelease, isPublished };

if (require.main === module) {
  main().catch(error => {
    console.error(error);
    process.exitCode = 1;
  });
}
