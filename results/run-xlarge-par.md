# Résultats — preset `xlarge`

_2026-10-27 07:31:04 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v165 · médiane de 1 exécutions_

## parmatmul (n = 8 192 — 4 threads)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (768 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 9.30 G/s | 59.083 s | ×1.00 | 227.38 s | 385 % | 771 Mo | ×1.00 | 196970 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 8.45 G/s | 65.087 s | ×1.10 | 253.05 s | 389 % | 770 Mo | ×1.00 | 196716 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 8.12 G/s | 67.664 s | ×1.15 | 262.17 s | 387 % | 770 Mo | ×1.00 | 262249 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 7.99 G/s | 68.786 s | ×1.16 | 267.47 s | 389 % | 770 Mo | ×1.00 | 254075 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 7.73 G/s | 71.157 s | ×1.20 | 276.73 s | 389 % | 770 Mo | ×1.00 | 262261 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 3.24 G/s | 169.600 s | ×2.87 | 656.94 s | 387 % | 796 Mo | ×1.04 | 198574 / 6 | 584 / 72 | ✅ |

## parsha (n = 25 600 000 000 — 4 threads)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (2 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 862.89 M/s | 29.668 s | ×1.00 | 115.51 s | 389 % | 2 Mo | ×0.95 | 101 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 762.89 M/s | 33.556 s | ×1.13 | 129.72 s | 387 % | 2 Mo | ×0.96 | 105 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 570.25 M/s | 44.893 s | ×1.51 | 174.57 s | 389 % | 2 Mo | ×1.10 | 116 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 503.86 M/s | 50.807 s | ×1.71 | 196.58 s | 387 % | 3 Mo | ×1.53 | 360 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 449.62 M/s | 56.937 s | ×1.92 | 223.21 s | 392 % | 2 Mo | ×1.10 | 116 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 400.03 M/s | 63.996 s | ×2.16 | 246.40 s | 385 % | 28 Mo | ×14.04 | 1975 / 0 | 0 / 72 | ✅ |

## parsort (n = 400 000 000 — 4 threads)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (3052 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 17.61 M/s | 22.715 s | ×1.00 | 72.69 s | 320 % | 3055 Mo | ×1.00 | 781611 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 15.76 M/s | 25.388 s | ×1.12 | 81.22 s | 320 % | 3054 Mo | ×1.00 | 781355 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 15.63 M/s | 25.597 s | ×1.13 | 83.99 s | 328 % | 3054 Mo | ×1.00 | 781369 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 15.23 M/s | 26.263 s | ×1.16 | 82.88 s | 316 % | 3054 Mo | ×1.00 | 781353 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 14.85 M/s | 26.928 s | ×1.19 | 87.10 s | 323 % | 3054 Mo | ×1.00 | 781367 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 10.45 M/s | 38.263 s | ×1.68 | 120.65 s | 313 % | 3080 Mo | ×1.01 | 783252 / 1 | 272 / 80 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| C natif (à la main) | ×1.09 | ×1.09 | ×1.00 | 3 |
| F* (Pulse) → C | ×1.13 | ×1.13 | ×1.00 | 3 |
| Zig 0.17 (à la main) | ×1.20 | ×1.19 | ×1.17 | 3 |
| Rust (sans Creusot) | ×1.29 | ×1.30 | ×1.05 | 3 |
| Rust + Creusot | ×1.36 | ×1.38 | ×1.05 | 3 |
| F* (Pulse) → F# | ×2.18 | ×2.17 | ×2.49 | 3 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 5 ms | 2.0 Mo |
| F* (Pulse) → C | 4 ms | 1.7 Mo |
| F* (Pulse) → F# | 90 ms | 27.8 Mo |
| Rust (sans Creusot) | 4 ms | 2.0 Mo |
| C natif (à la main) | 4 ms | 1.7 Mo |
| Zig 0.17 (à la main) | 4 ms | 2.1 Mo |

## Empreinte disque et coût de construction

| implémentation | exécutable | runtime/deps requis | compilation | vérification |
|---|--:|--:|--:|--:|
| Rust + Creusot | 0.46 Mo | 0.0 Mo | 0.2 s | 37.6 s |
| F* (Pulse) → C | 0.05 Mo | 0.0 Mo | 3.5 s | 428.5 s |
| F* (Pulse) → OCaml (runtime livré) | 5.83 Mo | 0.0 Mo | 5.3 s | 428.5 s |
| F* (Pulse) → F# | 0.09 Mo | 72.4 Mo | 9.6 s | 428.5 s |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 5.74 Mo | 0.0 Mo | 7.9 s | 428.5 s |
| Rust (sans Creusot) | 0.46 Mo | 0.0 Mo | 0.1 s | — |
| C natif (à la main) | 0.04 Mo | 0.0 Mo | 0.8 s | — |
| Zig 0.17 (à la main) | 3.71 Mo | 0.0 Mo | 18.1 s | — |
