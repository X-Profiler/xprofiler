# Releasing

Releases are controlled by the version in `package.json`. Commit messages do not
select a version, and the release workflow does not write commits to `master`.

## One-time setup

Configure a GitHub Actions trusted publisher in the npm settings for `xprofiler`:

- Organization: `X-Profiler`
- Repository: `xprofiler`
- Workflow filename: `nodejs.yml`
- Environment: leave blank
- Allow direct publishing with `npm publish`

The publisher must authorize `nodejs.yml`, the CI workflow that calls this
repository's reusable `release.yml`. npm checks the calling workflow's filename.
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

The normal CI workflow builds and packages the native addons on every push and
pull request. It also checks version changes in PRs into `master` and pushes to
`master`.

After a push to `master`, CI compares `package.json` with the commit before the
push. If its version increased and all CI jobs succeeded, CI calls `release.yml`
with the checked version. The release job downloads the binary artifacts from
that same CI run, creates a draft GitHub release at the merged commit, and
uploads all binary assets. It publishes the GitHub release and its
`v<version>` tag before publishing to npm through OIDC. This makes the binaries
available for `npm install` as soon as the npm version is available.
The release job uses the existing CI results and artifacts. It does not rebuild
the binaries, run the test suite, or change the version or changelog.
An unchanged version does not publish. Direct pushes with a version increase also
trigger publishing, so branch protection enforces the PR requirement.

Stable versions use the `latest` npm dist-tag. Prereleases use `next` and create a
GitHub prerelease.

## Binary assets

The build matrix packages Node.js 22, 24, and 26 for these targets:

- Linux x64 and arm64, built in `manylinux_2_28` containers with glibc 2.28
- macOS x64 and arm64
- Windows x64

Every PR, including a version PR, builds these assets without publishing them.
Each build uses `npm run pack` to package the addon and check that the archive can load. The
workflow keeps the filenames that `node-pre-gyp` expects, such as
`xprofiler-v4.0.0-node-v127-linux-x64.tar.gz`. The publish job requires all 15
assets before it can publish the GitHub release or npm package.

## Retry a failed release

Re-run the failed jobs in the CI workflow in GitHub Actions. The release job uses
the binary artifacts from that CI run. If those artifacts expired, re-run all CI
jobs to rebuild them. The release job keeps assets that already uploaded
successfully and uploads missing assets before publishing the GitHub
release. It skips npm publishing if that exact version already exists. Registry
errors stop the workflow. An existing tag that points to a different commit also
stops the workflow; tags are never overwritten.
