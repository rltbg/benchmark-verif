#!/usr/bin/env bash
# Construit les quatre colonnes du benchmark : Rust (Creusot) et F* -> C / F# / OCaml.
# Enregistre empreinte disque et temps de construction dans results/footprint.json.
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p results
now() { date +%s.%N; }
record() { python3 - results/footprint.json "$1" "$2" "$3" <<'PY'
import json, sys, os
p, impl, key, val = sys.argv[1:]
d = json.load(open(p)) if os.path.exists(p) else {}
d.setdefault(impl, {})[key] = float(val)
json.dump(d, open(p, "w"), indent=1)
PY
}
echo "== rust-creusot (cargo build --release : les contrats sont effacés) =="
t0=$(now); (cd impls/rust && cargo build --release --quiet 2>&1 | { grep -E '^error' || true; })
record rust-creusot build_s "$(echo "$(now) - $t0" | bc -l)"
record rust-creusot binary_kb "$(( $(stat -c %s impls/rust/target/release/bench-rust) / 1024 ))"; record rust-creusot runtime_kb 0
echo "== F* =="
fstar/build.sh "${1:-all}"
