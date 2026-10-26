#!/usr/bin/env bash
# Preuve Creusot du crate Rust ; enregistre la durée (sans cache) dans results/footprint.json.
set -euo pipefail
cd "$(dirname "$0")/../impls/rust"
command -v cargo-creusot >/dev/null || { echo "cargo-creusot introuvable : voir scripts/setup-creusot.sh"; exit 1; }
t0=$(date +%s.%N)
cargo creusot --no-cache -- --lib
dt=$(echo "$(date +%s.%N) - $t0" | bc -l)
mkdir -p ../../results
python3 - ../../results/footprint.json "$dt" <<'PY'
import json, sys, os
p, val = sys.argv[1:]
d = json.load(open(p)) if os.path.exists(p) else {}
d.setdefault("rust-creusot", {})["verify_s"] = float(val)
json.dump(d, open(p, "w"), indent=1)
PY
echo "preuves Creusot terminées en ${dt}s"
