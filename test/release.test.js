'use strict';

const assert = require('assert');
const { getRelease, isPublished } = require('../scripts/check-release');

describe('release checks', () => {
  it('skips changes that do not increase the package version', () => {
    assert.strictEqual(getRelease('3.1.0', '3.1.0').changed, false);
  });

  it('publishes the reviewed version with the stable dist-tag', () => {
    assert.deepStrictEqual(getRelease('3.1.0', '4.0.0'), {
      changed: true,
      version: '4.0.0',
      tag: 'v4.0.0',
      dist_tag: 'latest',
    });
  });

  it('compares versions numerically', () => {
    assert.strictEqual(getRelease('3.9.0', '3.10.0').changed, true);
  });

  it('publishes prereleases to next and stable releases to latest', () => {
    assert.strictEqual(getRelease('3.1.0', '4.0.0-beta.1').dist_tag, 'next');
    assert.strictEqual(getRelease('4.0.0-beta.1', '4.0.0').dist_tag, 'latest');
  });

  it('rejects invalid versions and downgrades', () => {
    for (const version of ['3.0.0', '3.1.0-beta.1', 'v4.0.0', '4.0', 'invalid', null, undefined]) {
      assert.throws(() => getRelease('3.1.0', version));
    }
  });

  it('skips npm publishing when the exact version already exists', async () => {
    const request = async url => {
      assert.strictEqual(url, 'https://registry.npmjs.org/xprofiler/4.0.0');
      return { ok: true, status: 200, json: async () => ({ version: '4.0.0' }) };
    };
    assert.strictEqual(await isPublished('xprofiler', '4.0.0', request), true);
  });

  it('publishes only when the registry returns 404', async () => {
    const request = async () => ({ ok: false, status: 404 });
    assert.strictEqual(await isPublished('xprofiler', '4.0.0', request), false);
  });

  it('fails on registry errors instead of treating the version as unpublished', async () => {
    for (const status of [401, 403, 429, 500]) {
      await assert.rejects(isPublished('xprofiler', '4.0.0', async () => ({ ok: false, status })));
    }
    await assert.rejects(isPublished('xprofiler', '4.0.0', async () => {
      throw new Error('Connection failed');
    }));
    await assert.rejects(isPublished('xprofiler', '4.0.0', async () => ({
      ok: true, json: async () => ({ version: '3.1.0' }),
    })));
  });
});
