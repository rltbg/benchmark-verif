#!/usr/bin/env bash
# Installe F* (binaire officiel, Z3 inclus) + .NET 10 + un switch opam OCaml 5.3 pour compiler le code OCaml extrait.
# Testé sur Ubuntu 24.04. Variables : PREFIX (défaut /opt/tools/fstar2), FSTAR_VERSION (défaut v2026.10.25).
set -euo pipefail
PREFIX=${PREFIX:-/opt/tools/fstar2}
FSTAR_VERSION=${FSTAR_VERSION:-v2026.10.25}
sudo mkdir -p "$PREFIX" && sudo chown "$USER" "$PREFIX"
curl -fsSL "https://github.com/FStarLang/FStar/releases/download/${FSTAR_VERSION}/fstar-${FSTAR_VERSION}-Linux-x86_64.tar.gz" | tar xz -C "$PREFIX"
"$PREFIX/fstar/bin/fstar.exe" --version | head -3
echo "export FSTAR_HOME=$PREFIX/fstar"
sudo apt-get install -y gcc bc opam bubblewrap m4 pkg-config libgmp-dev dotnet-sdk-10.0
opam init --bare --disable-sandboxing -y || true
opam switch create fstar-ml 5.3.0 -y
opam install -y --switch=fstar-ml stdint zarith ppx_deriving ppx_deriving_yojson yojson
