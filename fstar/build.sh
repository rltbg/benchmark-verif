#!/usr/bin/env bash
# Pipeline F* : vérification, puis extraction vers C (KaRaMeL), OCaml et F#.
#
#   fstar/build.sh verify        vérifie les 6 modules (aucune extraction)
#   fstar/build.sh c             vérifie + extrait en .krml + krml -> C + gcc  => _build/c/bench
#   fstar/build.sh ocaml         vérifie + extrait en OCaml + ocamlfind         => _build/ocaml/bench
#   fstar/build.sh fsharp        EXPÉRIMENTAL : extraction F# + runtime maison  => _build/fsharp/out/
#   fstar/build.sh all
#
# Variables : FSTAR_HOME (défaut /opt/tools/fstar/fstar), KRML_HOME (défaut ~/karamel),
#             OCAMLPATH est complété pour trouver le paquet findlib « fstar.lib ».
# Chaque étape écrit son temps dans results/footprint.json (build_s / verify_s).
set -euo pipefail
cd "$(dirname "$0")"
HERE=$PWD
FSTAR_HOME=${FSTAR_HOME:-/opt/tools/fstar/fstar}
KRML_HOME=${KRML_HOME:-$HOME/karamel}
FSTAR=${FSTAR_EXE:-$FSTAR_HOME/bin/fstar.exe}
MODS=(Common Sort Sieve Matmul Lines Hashmap)
SRC=()
for m in "${MODS[@]}"; do SRC+=("src/Bench.$m.fst"); done
B=_build
FOOT=../results/footprint.json
mkdir -p $B ../results

record() { # record <impl> <key> <value>
  python3 - "$FOOT" "$1" "$2" "$3" <<'PY'
import json, sys, os
p, impl, key, val = sys.argv[1:]
d = json.load(open(p)) if os.path.exists(p) else {}
d.setdefault(impl, {})[key] = float(val)
json.dump(d, open(p, "w"), indent=1)
PY
}
timed() { local t0=$(date +%s.%N); "$@"; ELAPSED=$(echo "$(date +%s.%N) - $t0" | bc -l); }

# Options communes : cache des .checked partagé entre les trois cibles => on ne vérifie qu'une fois.
FFLAGS=(--include src --cache_dir $B/cache --cmi --warn_error -272)

verify() {
  echo "== vérification F* =="
  timed "$FSTAR" "${FFLAGS[@]}" "${SRC[@]}"
  record fstar-c verify_s "$ELAPSED"; record fstar-ocaml verify_s "$ELAPSED"; record fstar-fsharp verify_s "$ELAPSED"
  echo "vérifié en ${ELAPSED}s"
}

target_c() {
  verify
  echo "== extraction .krml =="
  rm -rf $B/krml $B/c; mkdir -p $B/krml $B/c/dist
  "$FSTAR" "${FFLAGS[@]}" --odir $B/krml --codegen krml "${SRC[@]}"
  echo "== KaRaMeL -> C =="
  local t0=$(date +%s.%N)
  "$KRML_HOME/krml" -tmpdir $B/c/dist -skip-compilation -warn-error '+9@4' $B/krml/*.krml
  gcc -O3 -std=c11 -I $B/c/dist -I "$KRML_HOME/include" -I "$KRML_HOME/krmllib/dist/minimal" \
      $B/c/dist/Bench_*.c drivers/driver.c -o $B/c/bench
  record fstar-c build_s "$(echo "$(date +%s.%N) - $t0" | bc -l)"
  record fstar-c binary_kb "$(( $(stat -c %s $B/c/bench) / 1024 ))"
  record fstar-c runtime_kb 0
}

target_ocaml() {
  verify
  echo "== extraction OCaml =="
  rm -rf $B/ocaml; mkdir -p $B/ocaml
  "$FSTAR" "${FFLAGS[@]}" --odir $B/ocaml --codegen OCaml "${SRC[@]}"
  cp drivers/driver.ml $B/ocaml/
  # Réalisation « buffers natifs » (voir README, section « le piège de LowStar.Buffer en OCaml »).
  if [ "${FASTBUF:-0}" = 1 ]; then
    cp ocaml/fastbuf.ml $B/ocaml/Fastbuf.ml
    sed -i -E "s/LowStar_(Monotonic_)?Buffer(Ops)?\.([A-Za-z_']+)/Fastbuf.\3/g" $B/ocaml/Bench_*.ml
  fi
  local t0=$(date +%s.%N)
  ( cd $B/ocaml
    OCAMLPATH="$FSTAR_HOME/lib:${OCAMLPATH:-}" ocamlfind ocamlopt -inline 200 -w -a \
      -package fstar.lib,stdint,zarith,unix -linkpkg \
      $( [ "${FASTBUF:-0}" = 1 ] && echo Fastbuf.ml ) \
      $( [ -f Bench_Common.ml ] && echo Bench_Common.ml ) \
      Bench_Sort.ml Bench_Sieve.ml Bench_Matmul.ml Bench_Lines.ml Bench_Hashmap.ml driver.ml -o bench )
  record fstar-ocaml build_s "$(echo "$(date +%s.%N) - $t0" | bc -l)"
  record fstar-ocaml binary_kb "$(( $(stat -c %s $B/ocaml/bench) / 1024 ))"
  record fstar-ocaml runtime_kb 0
}

target_fsharp() {
  verify
  echo "== extraction F# (expérimental) =="
  rm -rf $B/fsharp; mkdir -p $B/fsharp
  "$FSTAR" "${FFLAGS[@]}" --odir $B/fsharp --codegen FSharp "${SRC[@]}"
  cp fsharp/bench-fstar-fsharp.fsproj fsharp/Runtime.fs drivers/Driver.fs $B/fsharp/
  local t0=$(date +%s.%N)
  ( cd $B/fsharp && DOTNET_CLI_TELEMETRY_OPTOUT=1 dotnet build -c Release -o out )
  record fstar-fsharp build_s "$(echo "$(date +%s.%N) - $t0" | bc -l)"
  record fstar-fsharp binary_kb "$(( $(stat -c %s $B/fsharp/out/bench-fstar-fsharp.dll) / 1024 ))"
  record fstar-fsharp runtime_kb 70000
}

case "${1:-all}" in
  verify) verify ;;
  c) target_c ;;
  ocaml) target_ocaml ;;
  fsharp) target_fsharp ;;
  all) target_c; target_ocaml; target_fsharp ;;
  *) echo "usage: $0 {verify|c|ocaml|fsharp|all}"; exit 2 ;;
esac
