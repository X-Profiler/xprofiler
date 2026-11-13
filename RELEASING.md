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
If its version increased, the workflow builds the native addon, runs the tests,
publishes that version to npm through OIDC, and creates a `v<version>` tag and a
GitHub release at the merged commit. It does not change the version or changelog.
An unchanged version does not publish. Direct pushes with a version increase also
trigger publishing, so branch protection enforces the PR requirement.

Stable versions use the `latest` npm dist-tag. Prereleases use `next` and create a
GitHub prerelease. The workflow does not build or upload prebuilt binary assets.

## Retry a failed release

Re-run the failed workflow in GitHub Actions. It checks the exact version on npm
and skips publishing if that version already exists, then creates the GitHub
release if needed. Registry errors stop the workflow. An existing tag that points
to a different commit also stops the workflow; tags are never overwritten.
