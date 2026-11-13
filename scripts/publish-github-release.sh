#!/usr/bin/env bash
set -eo pipefail

shopt -s nullglob
files=(release/*.tar.gz)
# Five platform/architecture targets for each of three Node.js majors.
if [ "${#files[@]}" -ne 15 ]; then
  echo "::error::Expected 15 native addon assets, found ${#files[@]}."
  exit 1
fi

if ! gh release view "$RELEASE_TAG" > /dev/null 2>&1; then
  args=(--target "$GITHUB_SHA" --title "$RELEASE_TAG" --generate-notes --draft)
  if [ "$NPM_DIST_TAG" != latest ]; then
    args+=(--prerelease --latest=false)
  fi
  gh release create "$RELEASE_TAG" "${args[@]}"
fi

uploaded=$(gh release view "$RELEASE_TAG" --json assets --jq '.assets[] | select(.state == "uploaded") | .name')
for file in "${files[@]}"; do
  if ! grep -Fxq "${file##*/}" <<< "$uploaded"; then
    gh release upload "$RELEASE_TAG" "$file" --clobber
  fi
done

if [ "$(gh release view "$RELEASE_TAG" --json isDraft --jq .isDraft)" = true ]; then
  gh release edit "$RELEASE_TAG" --draft=false
fi
