#!/usr/bin/env bash
# Installe Creusot (vérification déductive de Rust) d'après la documentation amont :
#   https://creusot-rs.github.io/creusot/guide/
# Les commandes exactes évoluent d'une version à l'autre : en cas d'échec, suivre le guide.
set -euo pipefail
command -v opam >/dev/null || { echo "installez opam d'abord"; exit 1; }
[ -d "$HOME/creusot" ] || git clone https://github.com/creusot-rs/creusot.git "$HOME/creusot"
cd "$HOME/creusot"
./INSTALL           # installe why3 + solveurs (alt-ergo, z3, cvc5...) dans un switch opam dédié et cargo-creusot
echo "Creusot installé. Lancer : make creusot"
