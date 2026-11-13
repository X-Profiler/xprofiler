---
name: release
description: Prepare xprofiler version PRs, verify GitHub and npm releases, or recover a failed release through the repository's automated publishing workflow. Use when a maintainer requests a release, version bump, release status check, or release retry.
---

# Release xprofiler

Use a version PR into `master` to release `X-Profiler/xprofiler`. By default,
prepare the PR. Merge only when the user's request authorizes it; merging a
version increase starts publishing. A status request is read-only.

Read [RELEASING.md](../../../RELEASING.md) for setup and recovery details.
Inspect [release.yml](../../../.github/workflows/release.yml) and
[build-binaries.yml](../../../.github/workflows/build-binaries.yml) for the
current release gates and asset matrix. Run commands from the repository root.

## Prepare the version PR

1. Check the working tree, repository instructions, remotes, and existing release
   PRs. Fetch the upstream `master` and tags. Reuse the task worktree and release
   branch when appropriate; otherwise branch from the current upstream `master`.
   Keep unrelated local changes out of the release PR.
2. Read `package.json`, `CHANGELOG.md`, and changes since the previous release.
   Use the version requested by the maintainer. If none was supplied, recommend
   a version based on the actual changes and ask the maintainer to choose it.
   Continue reviewing changes while waiting. Node.js support removals are
   breaking changes. Commit message prefixes do not select the version.
3. Require a canonical SemVer version greater than the version on `master`.
   Check the exact npm version and GitHub tag `v<version>` for an existing
   publication. Treat network or authorization errors as unknown status, not
   absence. If the target already exists, inspect it and use the recovery path
   when appropriate instead of attempting to replace it.
4. Set `release_version` to the chosen version. Update `package.json` without
   automatically creating a Git commit, tag, or lockfile:

   ```sh
   npm version "$release_version" --no-git-tag-version --package-lock=false
   ```

   Move the applicable `Unreleased` notes into `## <version> (YYYY-MM-DD)` in
   `CHANGELOG.md`. Use the current date. Include relevant merged changes and
   breaking changes, preserve previous entries, and leave future notes under
   `Unreleased`. Keep `package.json` and the changelog version consistent.
5. Resolve the fetched upstream `master` to its full commit SHA as
   `release_base`. Install dependencies with
   `npm install --ignore-scripts --no-package-lock` only if needed. Run the
   existing version check and inspect the final diff:

   ```sh
   node scripts/check-release.js "$release_base"
   git diff --check
   git diff -- package.json CHANGELOG.md
   ```

   Require `changed=true` and the chosen version. Stable versions use `latest`;
   prereleases use `next`. Do not add a local test suite or rebuild every target
   for a version-only change: PR CI performs the runtime checks and native builds.
   The packaging matrix runs only when this version check reports a change in a
   PR into `master`.
6. Commit the release files, push the branch, and create or update its PR into
   `master`. Use a title such as `chore: release v<version>`. Describe the version,
   material changes, and npm dist-tag in short prose. Follow repository PR rules.
   Report the PR URL and current checks. Before an authorized merge, require
   successful checks on the latest PR commit, including `Check release version`,
   `Check binary builds`, runtime checks, and all native builds. Respect required
   reviews and branch protection. If `master` changes, recheck the version
   comparison and PR status.

## Follow the automated release

When asked to complete or verify publishing, find the `Release` workflow run
for the merged commit on `master`. Do not substitute a branch CI run.

`nodejs.yml` and `release.yml` run independently. Both call `build-binaries.yml`.
The release run builds its own assets, uploads them to a GitHub release, and
publishes that release before npm trusted publishing. PRs never publish.
Leave tag creation, asset uploads, and `npm publish` to this workflow.

Check these results before reporting a release as complete:

- The `Release` run succeeds for the merged commit.
- The `v<version>` tag resolves to that commit and its GitHub release is public.
  Check asset names and uploaded states against the current matrix, not just the
  count. See the binary asset matrix in `RELEASING.md`.
- npm serves the exact package version. Check its `latest` or `next` dist-tag;
  account for any newer release before treating a moved dist-tag as a failure.

Preserve the platform filenames consumed by `node-pre-gyp`. Node.js 22 Linux x64
uses glibc 2.17; other Linux assets use glibc 2.28. A routine version PR does not
change this matrix or the pinned unofficial Node.js runtime.

Report the version, dist-tag, workflow URL, GitHub release URL, npm package URL,
and any incomplete step. A merged PR alone does not prove publication.

## Recover a failed release

Inspect the failed run and logs before retrying. For setup or OIDC failures,
follow the one-time setup in `RELEASING.md`: the npm trusted publisher must use
`release.yml`. Do not introduce publishing tokens as a workaround.

When the maintainer requests recovery, rerun failed jobs in the original release
run if its artifacts remain available. Rerun all jobs if those artifacts expired.
The workflow preserves uploaded assets and skips an npm version that already
exists. Verify both services after the retry.

Stop retrying if the same failure recurs without a corrective change. Report the
failing step and required fix. A tag that points to another commit requires
maintainer resolution; do not move the tag or replace an existing npm version.
If a fix needs a new commit after a partial publication, explain the existing
tag and npm state before selecting a new version through another PR.
