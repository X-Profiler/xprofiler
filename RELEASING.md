# Releasing

Releases are controlled by the version in `package.json`. Commit messages do not
select a version, and the release workflow does not write commits to `master`.

## Release skill

Use the repository's [release skill](.agents/skills/release/SKILL.md) to prepare
the version PR and follow the publishing workflow. For example, ask Codex:
`Use $release to prepare a release PR for version 4.0.0.` Replace the example
version with the intended version. The skill also handles release status checks
and retries. Creating a release PR does not merge it or publish the package.

## One-time setup

Configure a GitHub Actions trusted publisher in the npm settings for `xprofiler`:

- Organization: `X-Profiler`
- Repository: `xprofiler`
- Workflow filename: `release.yml`
- Environment: leave blank
- Allow direct publishing with `npm publish`

The publisher must authorize `release.yml`, the standalone workflow that runs
`npm publish`.
See the [npm trusted publishing documentation](https://docs.npmjs.com/trusted-publishers/).
No `NPM_TOKEN` or `GIT_TOKEN` secret is required by the release workflow.

Protect `master` so changes require a pull request and successful CI, including
the `Check release version` check and all native addon build checks.

## Prepare a release PR

1. Create a branch from `master` and choose the next version explicitly:

   ```sh
   npm version 4.0.0 --no-git-tag-version --package-lock=false
   ```

2. Update `CHANGELOG.md` with the version, date, and release notes.
3. Commit these changes and open a pull request into `master`.
4. Merge the PR after review and CI pass.

`nodejs.yml` and `release.yml` run independently. Both call the reusable
`build-binaries.yml` workflow, which owns the complete build matrix and uploads
the binary artifacts into the caller's run.

Normal CI checks the native builds on every push and pull request. The release
workflow checks version changes in PRs into `master`. PRs do not publish.
New commits cancel older CI runs for the same PR or branch, including their
native builds. They also cancel older release version checks for the same PR.
Publishing runs on `master` are independent and are not canceled by newer commits.

After a push that changes `package.json` on `master`, `release.yml` compares the
version with the commit before the push. If the version increased, it calls the
shared build workflow to build assets for the merged commit. After all builds
succeed, the publish job downloads the artifacts from its own release run,
creates a draft GitHub release, and uploads all binary assets.
It publishes the GitHub release and its `v<version>` tag before publishing to
npm through OIDC. This makes the binaries
available for `npm install` as soon as the npm version is available.
The release workflow relies on successful PR checks enforced by branch protection.
It does not run the test suite or change the version or changelog. CI and release
runs each build their own assets with the same shared build definition.
An unchanged version does not publish. Direct pushes with a version increase also
trigger publishing, so branch protection enforces the PR requirement.

Stable versions use the `latest` npm dist-tag. Prereleases use `next` and create a
GitHub prerelease.

## Binary assets

The build matrix packages Node.js 22, 24, and 26 for these targets:

- Node.js 22 Linux x64, built in `manylinux2014_x86_64` with glibc 2.17
- Other Linux builds, using `manylinux_2_28` containers with glibc 2.28
- macOS x64 and arm64
- Windows x64

Every PR, including a version PR, builds these assets without publishing them.
Each build uses `npm run pack` to package the addon and check that the archive can load. The
workflow keeps the filenames that `node-pre-gyp` expects, such as
`xprofiler-v4.0.0-node-v127-linux-x64.tar.gz`. The publish job requires all 15
assets before it can publish the GitHub release or npm package.

The glibc 2.17 build uses the unofficial Node.js 22 runtime because official
Node.js binaries require glibc 2.28. The workflow pins its version and archive
SHA-256. Update both together when selecting a newer unofficial Node.js 22 build.
See [Node.js unofficial builds](https://github.com/nodejs/unofficial-builds).

This build checks the addon's glibc and C++ library requirements and loads it
with the container's system libraries. It also checks loading under official
Node.js 22 on the Ubuntu runner. The archive keeps the standard
`node-v127-linux-x64` filename, so the same asset serves older and newer glibc
systems. Users on older glibc systems also need a compatible Node.js runtime;
the addon does not lower the requirements of the official Node.js executable.

## Retry a failed release

Re-run the failed jobs in the Release workflow in GitHub Actions. The publish job
uses the binary artifacts from that release run. If those artifacts expired,
re-run all release jobs to rebuild them. The publish job keeps assets that
already uploaded successfully and uploads missing assets before publishing the GitHub
release. It skips npm publishing if that exact version already exists. Registry
errors stop the workflow. An existing tag that points to a different commit also
stops the workflow; tags are never overwritten.
