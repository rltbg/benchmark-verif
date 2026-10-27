#!/usr/bin/env bash
# Active/désactive un fichier de swap pour le mode « dépassement de RAM » : swap.sh on [Go] | off
set -euo pipefail
F=${SWAPFILE:-/var/tmp/bench.swap}
case "${1:-}" in
  on)  fallocate -l "${2:-8}G" "$F"; chmod 600 "$F"; mkswap "$F" >/dev/null; swapon "$F"; swapon --show ;;
  off) swapoff "$F" 2>/dev/null || true; rm -f "$F" ;;
  *) echo "usage: $0 on [Go] | off"; exit 2 ;;
esac
