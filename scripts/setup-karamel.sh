#!/usr/bin/env bash
# Installe KaRaMeL (compilateur Low* -> C). Deux options :
#   1) opam :  opam install karamel         (le plus simple)
#   2) sources : voir https://github.com/FStarLang/karamel#readme  (dépend de la version de F*)
# Le script suppose l'option 2 et définit KRML_HOME=$HOME/karamel.
set -euo pipefail
command -v opam >/dev/null || { echo "installez opam d'abord"; exit 1; }
opam install -y ppx_deriving_yojson zarith pprint "menhir>=20161115" sedlex process fix "wasm>=2.0.0" visitors ctypes-foreign ctypes uucp batteries stdint
[ -d "$HOME/karamel" ] || git clone https://github.com/FStarLang/karamel.git "$HOME/karamel"
cd "$HOME/karamel"
export FSTAR_HOME=${FSTAR_HOME:-/opt/tools/fstar/fstar}
export PATH="$FSTAR_HOME/bin:$PATH"
make -j"$(nproc)"
echo "KaRaMeL prêt : export KRML_HOME=$HOME/karamel"
