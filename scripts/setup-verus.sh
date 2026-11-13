#!/usr/bin/env bash
# Installe Verus (version hebdomadaire épinglée, binaire officiel avec Z3) et la toolchain Rust qu'il exige.
set -euo pipefail
V=${VERUS_VERSION:-0.2026.11.02.8bf68ee}
DEST=${VERUS_PREFIX:-/opt/tools/verus}
mkdir -p "$DEST" && cd "$DEST"
if [ ! -x verus-x86-linux/verus ]; then
  curl -fL -o verus.zip "https://github.com/verus-lang/verus/releases/download/release/$V/verus-$V-x86-linux.zip"
  unzip -q -o verus.zip && rm verus.zip
fi
TC=$(python3 -c "import json;print(json.load(open('verus-x86-linux/version.json'))['verus']['toolchain'].split()[0])" 2>/dev/null || echo 1.99.1-x86_64-unknown-linux-gnu)
rustup toolchain install "$TC" --profile minimal
"$DEST/verus-x86-linux/verus" --version
echo "export PATH=$DEST/verus-x86-linux:\$PATH"
