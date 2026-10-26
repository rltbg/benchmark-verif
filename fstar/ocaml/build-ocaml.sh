#!/usr/bin/env bash
# Compile le code OCaml extrait par F* (Custard) avec le runtime OCaml de F*.
#
# Le runtime (Prims, FStar_UInt8/16/32/64, FStar_SizeT) est recompilé depuis les *sources* livrées
# dans ulib/ml : les .cmx précompilés de la distribution F* sont liés à une variante précise du
# compilateur OCaml 5.3 et sont rejetés par un OCaml standard ("inconsistent assumptions over Stdlib").
#
# Usage : build-ocaml.sh <répertoire des .ml extraits> <répertoire de sortie> [stock|fast]
#   stock (défaut) : runtime F* tel que livré.
#   fast           : mêmes sources extraites, mais `v` et `uint_to_t` des entiers machine ne passent plus par
#                    une chaîne décimale (cf. README, « le piège des conversions d'entiers »).
# Prérequis : switch opam OCaml 5.3 « fstar-ml » avec stdint zarith ppx_deriving ppx_deriving_yojson yojson
#             (voir scripts/setup-fstar.sh).
set -euo pipefail
SRC=$(realpath "$1"); OUT=$2; MODE=${3:-stock}
HERE=$(cd "$(dirname "$0")/.." && pwd)
FSTAR_HOME=${FSTAR_HOME:-/opt/tools/fstar2/fstar}
APP=$FSTAR_HOME/lib/fstar/ulib/ml/app
OPAM_SWITCH=${OPAM_SWITCH:-fstar-ml}
eval "$(opam env --switch="$OPAM_SWITCH" --set-switch 2>/dev/null)"
mkdir -p "$OUT"; OUT=$(realpath "$OUT")
find "$OUT" -mindepth 1 -delete
cp "$APP"/Prims.ml "$APP"/FStar_UInt8.ml "$APP"/FStar_SizeT.ml "$APP"/ints/FStar_Ints.ml.body "$APP"/ints/mk_int_file.sh "$OUT"/
cp "$SRC"/*.ml "$OUT"/
cp "$HERE/drivers/driver.ml" "$OUT"/driver.ml
cd "$OUT"
for w in 16 32 64; do bash mk_int_file.sh U $w > FStar_UInt$w.ml; done
if [ "$MODE" = fast ]; then
  for w in 16 32; do
    sed -i "s|let v (x:t) : Prims.int = Prims.parse_int (M.to_string x)|let v (x:t) : Prims.int = Z.of_int (M.to_int x)|; \
            s|let uint_to_t x = M.of_string (Z.to_string x)|let uint_to_t x = M.of_int (Z.to_int x)|" FStar_UInt$w.ml
  done
  sed -i "s|let v (x:t) : Prims.int = Prims.parse_int (M.to_string x)|let v (x:t) : Prims.int = Z.of_int64_unsigned (M.to_int64 x)|; \
          s|let uint_to_t x = M.of_string (Z.to_string x)|let uint_to_t x = M.of_int64 (Z.to_int64_unsigned x)|" FStar_UInt64.ml
  sed -i "s|let v (x:uint8) : Prims.int = Prims.parse_int (string_of_int x)|let v (x:uint8) : Prims.int = Z.of_int x|" FStar_UInt8.ml
fi
PKGS=stdint,zarith,ppx_deriving.show,ppx_deriving_yojson,yojson,unix
# Le build OCaml « du banc » : -O3 est ignoré sans flambda ; -unsafe retire les contrôles de bornes
# des tableaux comme le fait le code C (le code extrait reste prouvé sûr côté F*).
ocamlfind ocamlopt ${OCAMLOPT_FLAGS:--inline 200} -unsafe -w -a -package "$PKGS" -linkpkg \
  Prims.ml FStar_UInt8.ml FStar_UInt16.ml FStar_UInt32.ml FStar_UInt64.ml FStar_SizeT.ml \
  Bench_Common.ml Bench_Sort.ml Bench_Sieve.ml Bench_Matmul.ml Bench_Lines.ml Bench_Hashmap.ml driver.ml -o bench
