#!/usr/bin/env bash
# Pipeline F* : vérification des modules Pulse, puis extraction « programme entier » (Custard) vers
# C, OCaml ou F#.
#
#   fstar/build.sh verify        vérifie les 6 modules (écrit les .checked)
#   fstar/build.sh c             => _build/c/bench
#   fstar/build.sh fsharp        => _build/fsharp/out/bench-fstar-fsharp.dll
#   fstar/build.sh ocaml         => _build/ocaml/bench
#   fstar/build.sh all
#
# Variables : FSTAR_HOME (défaut /opt/tools/fstar2/fstar). Les durées sont écrites dans
# results/footprint.json (verify_s, build_s, binary_kb, runtime_kb).
set -euo pipefail
cd "$(dirname "$0")"
FSTAR_HOME=${FSTAR_HOME:-/opt/tools/fstar2/fstar}
export PATH="$FSTAR_HOME/bin:$PATH"
FOOT=../results/footprint.json
B=$PWD/_build
MODS=(Common.fst SortSpec.fsti SortSpec.fst Sort.fst Sieve.fst Matmul.fst Lines.fst Hashmap.fst
      Stream.fst
      Bfs.fst
      # @@MODS@@ (modules des workloads étendus, avant All.fst : une ligne `<Nom>.fst` ; les ancres sont retirées par la ligne suivante)
      All.fst)
ENTRIES=(--custard_entry Bench.Sort.run --custard_entry Bench.Sieve.run --custard_entry Bench.Matmul.run
         --custard_entry Bench.Lines.run --custard_entry Bench.Hashmap.run
         --custard_entry Bench.Stream.chunk --custard_entry Bench.Stream.new_buf --custard_entry Bench.Stream.new_state
         --custard_entry Bench.Stream.new_table --custard_entry Bench.Stream.finish
         --custard_entry Bench.Bfs.run
         # @@ENTRIES@@ (une ligne `--custard_entry Bench.<Nom>.run` par workload étendu)
)
export DOTNET_CLI_TELEMETRY_OPTOUT=1 DOTNET_NOLOGO=1
mkdir -p "$B" ../results

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

verify() {
  local t0 warm=0; t0=$(now)
  [ -f "$B/cache/Bench.All.fst.checked" ] && warm=1
  ( cd src
    for m in "${MODS[@]}"; do
      fstar.exe -c --cache_dir "$B/cache" "Bench.$m" 2>&1 | { grep -v '^All verification conditions' || true; }
    done )
  local dt; dt=$(since "$t0")
  # cache chaud : la durée ne mesure pas la vérification, on garde la dernière mesure à froid
  if [ "$warm" = 0 ]; then for i in fstar-c fstar-ocaml fstar-ocaml-fast fstar-fsharp; do record "$i" verify_s "$dt"; done; fi
  echo "vérifié en ${dt}s"
}

# Extraction depuis le cache (les .checked doivent exister : appeler verify avant).
extract() { # extract <C|OCaml|FSharp> <outdir> [options...]
  local cg=$1 out=$2; shift 2
  rm -rf "$out"; mkdir -p "$out"
  ( cd src && fstar.exe --cache_dir "$B/cache" Bench.All.fst --codegen "$cg" "${ENTRIES[@]}" --odir "$out" "$@" 2>&1 \
      | { grep -v '^Verified module\|^All verification\|Warning 249\|Losing precision\|fun _ ->\|Unannotated\|^$' || true; } )
}

target_c() {
  verify
  local t0; t0=$(now)
  extract C "$B/c"
  gcc -O3 -march=native -std=gnu11 -pthread -I "$B/c" "$B/c/Custard.c" drivers/driver.c -o "$B/c/bench"
  record fstar-c build_s "$(since "$t0")"
  record fstar-c binary_kb "$(( $(stat -c %s "$B/c/bench") / 1024 ))"; record fstar-c runtime_kb 0
}

target_fsharp() {
  verify
  local t0; t0=$(now)
  extract FSharp "$B/fsharp"
  cp drivers/Driver.fs "$B/fsharp/Driver.fs"
  sed -i 's|<OutputType>Library</OutputType>|<OutputType>Exe</OutputType>|; s|<Compile Include="Custard.fs" />|<Compile Include="Custard.fs" /><Compile Include="Driver.fs" />|' "$B/fsharp/Custard.fsproj"
  sed -i 's|<InvariantGlobalization>|<AssemblyName>bench-fstar-fsharp</AssemblyName><Optimize>true</Optimize><DebugType>none</DebugType><ServerGarbageCollector>false</ServerGarbageCollector><ConcurrentGarbageCollection>false</ConcurrentGarbageCollection><TieredPGO>true</TieredPGO><TieredCompilationQuickJitForLoops>false</TieredCompilationQuickJitForLoops><InvariantGlobalization>|' "$B/fsharp/Custard.fsproj"
  ( cd "$B/fsharp" && dotnet build -c Release -o out --nologo -v q 2>&1 | grep -E 'error|Warn|Error' | head )
  record fstar-fsharp build_s "$(since "$t0")"
  record fstar-fsharp binary_kb "$(( $(stat -c %s "$B/fsharp/out/bench-fstar-fsharp.dll") / 1024 ))"
  rt=$(dotnet --list-runtimes | grep 'Microsoft.NETCore.App' | tail -1 | sed -E 's/^[^ ]+ ([^ ]+) \[(.*)\]$/\2\/\1/')
  record fstar-fsharp runtime_kb "$(du -sk "$rt" | cut -f1)"
}

target_ocaml() { # voir ocaml/build-ocaml.sh (runtime F* compilé séparément)
  verify
  local t0; t0=$(now)
  extract OCaml "$B/ocaml-src" --custard_split
  ./ocaml/build-ocaml.sh "$B/ocaml-src" "$B/ocaml" stock
  record fstar-ocaml build_s "$(since "$t0")"
  record fstar-ocaml binary_kb "$(( $(stat -c %s "$B/ocaml/bench") / 1024 ))"; record fstar-ocaml runtime_kb 0
  ./ocaml/build-ocaml.sh "$B/ocaml-src" "$B/ocaml-fast" fast
  record fstar-ocaml-fast build_s "$(since "$t0")"
  record fstar-ocaml-fast binary_kb "$(( $(stat -c %s "$B/ocaml-fast/bench") / 1024 ))"; record fstar-ocaml-fast runtime_kb 0
}

case "${1:-all}" in
  verify) verify ;;
  c) target_c ;;
  fsharp) target_fsharp ;;
  ocaml) target_ocaml ;;
  all) target_c; target_fsharp; target_ocaml ;;
  *) echo "usage: $0 {verify|c|fsharp|ocaml|all}"; exit 2 ;;
esac
