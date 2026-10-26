#!/usr/bin/env bash
# Construit les implémentations de référence et enregistre empreinte disque / temps de build
# dans results/footprint.json (lu par le rapport).
set -euo pipefail
cd "$(dirname "$0")/.."
FOOT=results/footprint.json
mkdir -p results
export DOTNET_CLI_TELEMETRY_OPTOUT=1 DOTNET_NOLOGO=1

record() { python3 - "$FOOT" "$1" "$2" "$3" <<'PY'
import json, sys, os
p, impl, key, val = sys.argv[1:]
d = json.load(open(p)) if os.path.exists(p) else {}
d.setdefault(impl, {})[key] = float(val)
json.dump(d, open(p, "w"), indent=1)
PY
}
now() { date +%s.%N; }
since() { echo "$(now) - $1" | bc -l; }
kb() { echo $(( $(stat -c %s "$1") / 1024 )); }

echo "== c-ref =="
t=$(now); (cd impls/c-ref && gcc -O3 -Wall -Wextra -o bench bench.c)
record c-ref build_s "$(since $t)"; record c-ref binary_kb "$(kb impls/c-ref/bench)"; record c-ref runtime_kb 0

echo "== rust-creusot (cargo build normal : les contrats sont effacés) =="
t=$(now); (cd impls/rust && cargo build --release --quiet)
record rust-creusot build_s "$(since $t)"; record rust-creusot binary_kb "$(kb impls/rust/target/release/bench-rust)"; record rust-creusot runtime_kb 0

echo "== ocaml-ref =="
t=$(now); (cd impls/ocaml-ref && ocamlfind ocamlopt -O3 -package unix -linkpkg bench.ml -o bench 2>/dev/null && rm -f bench.cm* bench.o)
record ocaml-ref build_s "$(since $t)"; record ocaml-ref binary_kb "$(kb impls/ocaml-ref/bench)"; record ocaml-ref runtime_kb 0

echo "== fsharp-ref =="
t=$(now); (cd impls/fsharp-ref && dotnet build -c Release -o out --nologo -v q >/dev/null)
rt=$(dotnet --list-runtimes | grep 'Microsoft.NETCore.App' | tail -1 | sed -E 's/^[^ ]+ ([^ ]+) \[(.*)\]$/\2\/\1/')
record fsharp-ref build_s "$(since $t)"; record fsharp-ref binary_kb "$(kb impls/fsharp-ref/out/bench-fsharp.dll)"
record fsharp-ref runtime_kb "$(du -sk "$rt" | cut -f1)"
echo "ok -> $FOOT"
