#!/usr/bin/env bash
# Lance la preuve Creusot du crate Rust et enregistre le temps dans results/footprint.json.
# Prérequis : `cargo creusot` (https://github.com/creusot-rs/creusot) installé avec why3 et ses solveurs.
set -euo pipefail
cd "$(dirname "$0")/../impls/rust"
command -v cargo-creusot >/dev/null || { echo "cargo-creusot introuvable : voir scripts/setup-creusot.sh"; exit 1; }
t0=$(date +%s.%N)
cargo creusot prove
dt=$(echo "$(date +%s.%N) - $t0" | bc -l)
python3 - "../../results/footprint.json" "$dt" <<'PY'
import json, sys, os
p, val = sys.argv[1:]
d = json.load(open(p)) if os.path.exists(p) else {}
d.setdefault("rust-creusot", {})["verify_s"] = float(val)
json.dump(d, open(p, "w"), indent=1)
PY
echo "preuve Creusot terminée en ${dt}s"
