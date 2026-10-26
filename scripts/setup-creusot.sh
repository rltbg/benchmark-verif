#!/usr/bin/env bash
# Installe Creusot (testé avec le dépôt au 2026-10-24, toolchain nightly-2026-08-03).
# Guide : https://creusot-rs.github.io/creusot/guide/installation.html
set -euo pipefail
sudo apt-get install -y opam bubblewrap m4 pkg-config libgmp-dev libgtk-3-dev libgtksourceview-3.0-dev cmake
opam init --bare --disable-sandboxing -y || true
opam option depext=false
[ -d "${CREUSOT_SRC:-/opt/tools/creusot}" ] || git clone https://github.com/creusot-rs/creusot "${CREUSOT_SRC:-/opt/tools/creusot}"
cd "${CREUSOT_SRC:-/opt/tools/creusot}"
./INSTALL       # why3, why3find, alt-ergo, z3, cvc4, cvc5, cargo-creusot ; écrit ~/.cargo/config.toml (patch creusot-std)
