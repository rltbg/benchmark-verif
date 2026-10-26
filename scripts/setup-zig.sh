#!/usr/bin/env bash
# Installe Zig 0.17.0 (obligatoire pour la colonne Zig) dans /opt/tools/zig.
set -euo pipefail
V=0.17.0
SHA=1cbe9df9f27e6b78d14ccbca43b6703a404ef79ef1c463de901d7f088d4e2026
DEST=${ZIG_PREFIX:-/opt/tools/zig}
mkdir -p "$DEST" && cd "$DEST"
if [ ! -x "zig-x86_64-linux-$V/zig" ]; then
  curl -fLO "https://ziglang.org/download/$V/zig-x86_64-linux-$V.tar.xz"
  echo "$SHA  zig-x86_64-linux-$V.tar.xz" | sha256sum -c -
  tar xf "zig-x86_64-linux-$V.tar.xz"
fi
"$DEST/zig-x86_64-linux-$V/zig" version
