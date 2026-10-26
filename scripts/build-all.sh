#!/usr/bin/env bash
# Construit toutes les colonnes : Rust (Creusot), Rust simple, C natif, Zig 0.17 et F* -> C / F# / OCaml.
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
ZIG=${ZIG:-/opt/tools/zig/zig-x86_64-linux-0.17.0/zig}
echo "== rust-plain =="
t0=$(now); (cd impls/rust-plain && cargo build --release --quiet 2>&1 | { grep -E '^error' || true; })
record rust-plain build_s "$(echo "$(now) - $t0" | bc -l)"
record rust-plain binary_kb "$(( $(stat -c %s impls/rust-plain/target/release/bench-rust-plain) / 1024 ))"; record rust-plain runtime_kb 0
echo "== c-native =="
t0=$(now); gcc -O3 -std=c11 -Wall -Wextra -o impls/c-native/bench impls/c-native/bench.c
record c-native build_s "$(echo "$(now) - $t0" | bc -l)"
record c-native binary_kb "$(( $(stat -c %s impls/c-native/bench) / 1024 ))"; record c-native runtime_kb 0
echo "== zig $("$ZIG" version) =="
t0=$(now); (cd impls/zig && "$ZIG" build-exe bench.zig -O ReleaseFast -lc -femit-bin=bench && rm -f bench.o)
record zig build_s "$(echo "$(now) - $t0" | bc -l)"
record zig binary_kb "$(( $(stat -c %s impls/zig/bench) / 1024 ))"; record zig runtime_kb 0
echo "== F* =="
fstar/build.sh "${1:-all}"
