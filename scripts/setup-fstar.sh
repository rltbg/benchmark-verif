#!/usr/bin/env bash
# Installe F* (binaire officiel, avec Z3 embarqué) et les bibliothèques OCaml nécessaires pour
# compiler le code OCaml extrait. Testé sur Ubuntu 24.04. Variables : PREFIX (défaut /opt/tools/fstar),
# FSTAR_VERSION (défaut v2025.03.25).
set -euo pipefail
PREFIX=${PREFIX:-/opt/tools/fstar}
FSTAR_VERSION=${FSTAR_VERSION:-v2025.03.25}
sudo mkdir -p "$PREFIX" && sudo chown "$USER" "$PREFIX"
curl -fsSL "https://github.com/FStarLang/FStar/releases/download/${FSTAR_VERSION}/fstar-${FSTAR_VERSION}-Linux-x86_64.tar.gz" | tar xz -C "$PREFIX"
echo "F* installé dans $PREFIX/fstar  =>  export FSTAR_HOME=$PREFIX/fstar"
"$PREFIX/fstar/bin/fstar.exe" --version | head -3

# Bibliothèques OCaml requises par le paquet findlib « fstar.lib » (code OCaml extrait).
if command -v opam >/dev/null; then
  opam install -y batteries stdint zarith pprint ppx_deriving ppx_deriving_yojson yojson
else
  echo "opam absent : installez-le (apt install opam), puis relancez pour obtenir batteries/stdint/zarith/..."
fi
