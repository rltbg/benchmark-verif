# Résultats — preset `xlarge`

_2026-10-27 06:30:53 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v165 · médiane de 1 exécutions_

## alloc (n = 500 000 000)

Débit = allocations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (1043 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 5.64 M/s | 88.589 s | ×1.00 | 87.25 s | 98 % | 1152 Mo | ×1.10 | 294569 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 5.49 M/s | 91.147 s | ×1.03 | 89.85 s | 99 % | 1234 Mo | ×1.18 | 315493 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 5.45 M/s | 91.809 s | ×1.04 | 90.57 s | 99 % | 1323 Mo | ×1.27 | 338517 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 5.42 M/s | 92.290 s | ×1.04 | 90.52 s | 98 % | 1174 Mo | ×1.13 | 300322 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 5.31 M/s | 94.088 s | ×1.06 | 92.98 s | 99 % | 1293 Mo | ×1.24 | 330761 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 2.16 M/s | 231.564 s | ×2.61 | 228.98 s | 99 % | 4378 Mo | ×4.20 | 1928656 / 3 | 64 / 56 | ✅ |

## bfs (n = 40 000 000)

Débit = arêtes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (2747 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 27.53 M/s | 69.738 s | ×1.00 | 68.96 s | 99 % | 2901 Mo | ×1.06 | 781418 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 26.09 M/s | 73.592 s | ×1.06 | 72.53 s | 99 % | 2901 Mo | ×1.06 | 781483 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 25.86 M/s | 74.241 s | ×1.06 | 73.16 s | 99 % | 2901 Mo | ×1.06 | 781417 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 24.44 M/s | 78.566 s | ×1.13 | 77.42 s | 99 % | 2901 Mo | ×1.06 | 781430 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 22.77 M/s | 84.305 s | ×1.21 | 83.12 s | 99 % | 2901 Mo | ×1.06 | 781429 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 16.78 M/s | 114.431 s | ×1.64 | 113.24 s | 99 % | 3080 Mo | ×1.12 | 783270 / 0 | 0 / 64 | ✅ |

## ntt (n = 67 108 864)

Débit = papillons/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (384 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust + Creusot | 74.56 M/s | 23.402 s | ×1.00 | 23.14 s | 99 % | 386 Mo | ×1.01 | 98481 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 72.94 M/s | 23.922 s | ×1.02 | 23.58 s | 99 % | 386 Mo | ×1.00 | 98537 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 66.47 M/s | 26.250 s | ×1.12 | 25.89 s | 99 % | 386 Mo | ×1.00 | 98473 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 63.59 M/s | 27.437 s | ×1.17 | 27.13 s | 99 % | 386 Mo | ×1.01 | 98480 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 54.44 M/s | 32.052 s | ×1.37 | 31.62 s | 99 % | 385 Mo | ×1.00 | 98471 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 34.74 M/s | 50.222 s | ×2.15 | 49.77 s | 99 % | 542 Mo | ×1.41 | 133607 / 0 | 16 / 88 | ✅ |

## ptrchase (n = 16 000 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (305 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust + Creusot | 1.47 M/s | 81.395 s | ×1.00 | 80.31 s | 99 % | 307 Mo | ×1.01 | 78305 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 1.40 M/s | 85.662 s | ×1.05 | 84.20 s | 98 % | 307 Mo | ×1.00 | 78296 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 1.37 M/s | 87.883 s | ×1.08 | 86.79 s | 99 % | 307 Mo | ×1.01 | 78358 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 1.34 M/s | 89.369 s | ×1.10 | 88.23 s | 99 % | 307 Mo | ×1.01 | 78292 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 1.21 M/s | 99.448 s | ×1.22 | 98.02 s | 99 % | 307 Mo | ×1.01 | 78300 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 1.09 M/s | 110.220 s | ×1.35 | 109.13 s | 99 % | 334 Mo | ×1.09 | 80154 / 1 | 64 / 80 | ✅ |

## sha256 (n = 16 000 000 000)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 211.71 M/s | 75.574 s | ×1.00 | 74.77 s | 99 % | 2 Mo | ×1724.00 | 167 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 202.01 M/s | 79.205 s | ×1.05 | 78.19 s | 99 % | 2 Mo | ×1764.00 | 166 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 197.42 M/s | 81.046 s | ×1.07 | 79.89 s | 99 % | 2 Mo | ×2048.00 | 231 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 156.48 M/s | 102.253 s | ×1.35 | 101.03 s | 99 % | 2 Mo | ×2020.00 | 172 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 117.29 M/s | 136.409 s | ×1.80 | 135.10 s | 99 % | 2 Mo | ×2212.00 | 176 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 108.09 M/s | 148.023 s | ×1.96 | 146.35 s | 99 % | 28 Mo | ×28804.00 | 1986 / 0 | 0 / 56 | ✅ |

## stream (n = 6 144)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (1 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust + Creusot | 177.29 M/s | 36.338 s | ×1.00 | 34.66 s | 95 % | 3 Mo | ×3.07 | 435 / 3 | 12588840 / 8 | ✅ |
| Rust (sans Creusot) | 136.15 M/s | 47.318 s | ×1.30 | 45.63 s | 96 % | 3 Mo | ×3.06 | 434 / 3 | 12589008 / 0 | ✅ |
| F* (Pulse) → C | 131.66 M/s | 48.933 s | ×1.35 | 47.29 s | 97 % | 3 Mo | ×2.77 | 421 / 2 | 12587632 / 0 | ✅ |
| C natif (à la main) | 126.89 M/s | 50.771 s | ×1.40 | 49.15 s | 97 % | 3 Mo | ×2.75 | 423 / 2 | 12587592 / 0 | ✅ |
| Zig 0.17 (à la main) | 124.33 M/s | 51.818 s | ×1.43 | 50.05 s | 97 % | 3 Mo | ×2.86 | 488 / 2 | 12588056 / 0 | ✅ |
| F* (Pulse) → F# | 26.74 M/s | 240.950 s | ×6.63 | 238.14 s | 99 % | 30 Mo | ×29.58 | 2325 / 316 | 12639912 / 128 | ✅ |

## strsort (n = 50 000 000)

Débit = chaînes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (1574 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 833.99 k/s | 59.953 s | ×1.00 | 59.04 s | 98 % | 1552 Mo | ×0.99 | 396975 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 669.02 k/s | 74.736 s | ×1.25 | 73.68 s | 99 % | 1551 Mo | ×0.99 | 396909 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 661.68 k/s | 75.565 s | ×1.26 | 74.66 s | 99 % | 1551 Mo | ×0.99 | 396910 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 614.32 k/s | 81.391 s | ×1.36 | 79.97 s | 98 % | 1552 Mo | ×0.99 | 396918 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 578.03 k/s | 86.501 s | ×1.44 | 85.64 s | 99 % | 1552 Mo | ×0.99 | 396920 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 300.10 k/s | 166.613 s | ×2.78 | 164.70 s | 99 % | 1708 Mo | ×1.09 | 432052 / 3 | 72 / 88 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| Zig 0.17 (à la main) | ×1.09 | ×1.09 | ×1.03 | 7 |
| F* (Pulse) → C | ×1.12 | ×1.12 | ×1.02 | 7 |
| C natif (à la main) | ×1.16 | ×1.16 | ×1.00 | 7 |
| Rust + Creusot | ×1.18 | ×1.18 | ×1.07 | 7 |
| Rust (sans Creusot) | ×1.23 | ×1.23 | ×1.05 | 7 |
| F* (Pulse) → F# | ×2.39 | ×2.41 | ×2.76 | 7 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 5 ms | 2.0 Mo |
| F* (Pulse) → C | 5 ms | 1.7 Mo |
| F* (Pulse) → F# | 73 ms | 28.0 Mo |
| Rust (sans Creusot) | 6 ms | 2.1 Mo |
| C natif (à la main) | 5 ms | 1.8 Mo |
| Zig 0.17 (à la main) | 7 ms | 2.1 Mo |

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
