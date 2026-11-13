#!/usr/bin/env bash
set -euo pipefail

: "${NODE_VERSION:?Set the unofficial Node.js 22 version}"
: "${NODE_SHA256:?Set the SHA-256 of the unofficial Node.js archive}"

node_dir=$(mktemp -d)
trap 'rm -rf "$node_dir"' EXIT
archive="node-v${NODE_VERSION}-linux-x64-glibc-217.tar.xz"
url="https://unofficial-builds.nodejs.org/download/release/v${NODE_VERSION}/${archive}"
curl --fail --location --show-error --retry 3 --output "$node_dir/$archive" "$url"
printf '%s  %s\n' "$NODE_SHA256" "$node_dir/$archive" | sha256sum --check -
mkdir "$node_dir/node"
tar -xJf "$node_dir/$archive" -C "$node_dir/node" --strip-components=1
export PATH="$node_dir/node/bin:$PATH"

manylinux-interpreters ensure cp312-cp312
export PYTHON=/opt/python/cp312-cp312/bin/python

npm install --global npm@11.19.1
npm install --ignore-scripts --no-package-lock
npm run build
npm run pack

binary=$(node -p 'require("@mapbox/node-pre-gyp").find(require("path").resolve("package.json"))')

# Reject dependencies newer than the glibc and C++ libraries shipped in CentOS 7.
"$PYTHON" - "$binary" <<'PYTHON'
import re
import subprocess
import sys

symbols = subprocess.check_output(["readelf", "--version-info", sys.argv[1]], text=True)
for library, maximum in [("GLIBC", (2, 17)), ("GLIBCXX", (3, 4, 19)), ("CXXABI", (1, 3, 7))]:
    versions = re.findall(r"\b" + library + r"_(\d+(?:\.\d+)+)\b", symbols)
    for version in set(versions):
        if tuple(map(int, version.split("."))) > maximum:
            sys.exit(f"Unsupported dependency: {library}_{version}")
    print(f"{library} requirements: {', '.join(sorted(set(versions))) or 'none'}")
PYTHON

# Load with the system libraries, without the compiler toolchain's library path.
env -u LD_LIBRARY_PATH -u LD_PRELOAD node -e 'require(process.argv[1])' "$binary"
