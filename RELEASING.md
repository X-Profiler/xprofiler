# Releasing

Releases are controlled by the version in `package.json`. Commit messages do not
select a version, and the release workflow does not write commits to `master`.

## One-time setup

Configure a GitHub Actions trusted publisher in the npm settings for `xprofiler`:

- Organization: `X-Profiler`
- Repository: `xprofiler`
- Workflow filename: `release.yml`
- Environment: leave blank
- Allow direct publishing with `npm publish`

The publisher must authorize this repository's `release.yml`, which replaces the
shared `npm-release.yml` workflow. See the
[npm trusted publishing documentation](https://docs.npmjs.com/trusted-publishers/).
No `NPM_TOKEN` or `GIT_TOKEN` secret is required by the release workflow.

Protect `master` so changes require a pull request and successful CI, including
the `Check release version` check.

## Prepare a release PR

1. Create a branch from `master` and choose the next version explicitly:

   ```sh
   npm version 4.0.0 --no-git-tag-version --package-lock=false
   ```

2. Update `CHANGELOG.md` with the version, date, and release notes.
3. Commit these changes and open a pull request into `master`.
4. Merge the PR after review and CI pass.

The push to `master` compares `package.json` with the commit before the push.
If its version increased, the workflow builds and packages the native addons.
After every build succeeds, it creates a draft GitHub release at the merged
commit and uploads all binary assets. It publishes the GitHub release and its
`v<version>` tag before publishing to npm through OIDC. This makes the binaries
available for `npm install` as soon as the npm version is available.
The release job relies on the PR's CI results. It does not change the version or
changelog.
An unchanged version does not publish. Direct pushes with a version increase also
trigger publishing, so branch protection enforces the PR requirement.

Stable versions use the `latest` npm dist-tag. Prereleases use `next` and create a
GitHub prerelease.

## Binary assets

The build matrix packages Node.js 22, 24, and 26 for these targets:

- Linux x64 and arm64, built in `manylinux_2_28` containers with glibc 2.28
- macOS x64 and arm64
- Windows x64

Version PRs also build these assets without publishing them. Each build uses
`npm run pack` to package the addon and check that the archive can load. The
workflow keeps the filenames that `node-pre-gyp` expects, such as
`xprofiler-v4.0.0-node-v127-linux-x64.tar.gz`. The publish job requires all 15
assets before it can publish the GitHub release or npm package.

## Retry a failed release

Re-run the failed workflow in GitHub Actions. It keeps assets that already
uploaded successfully and uploads missing assets before publishing the GitHub
release. It skips npm publishing if that exact version already exists. Registry
errors stop the workflow. An existing tag that points to a different commit also
stops the workflow; tags are never overwritten.
