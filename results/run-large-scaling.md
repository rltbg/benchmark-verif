# Résultats — preset `large`

_2026-10-27 05:58:05 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v165 · médiane de 1 exécutions_

## parmatmul@1 (n = 6 144)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (432 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 2.12 G/s | 109.652 s | ×1.00 | 108.34 s | 99 % | 434 Mo | ×1.00 | 110753 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 2.09 G/s | 111.178 s | ×1.01 | 109.99 s | 99 % | 434 Mo | ×1.00 | 147560 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 2.08 G/s | 111.314 s | ×1.02 | 110.30 s | 99 % | 434 Mo | ×1.00 | 110690 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 2.03 G/s | 114.134 s | ×1.04 | 113.10 s | 99 % | 434 Mo | ×1.00 | 141420 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 1.98 G/s | 117.003 s | ×1.07 | 115.69 s | 99 % | 433 Mo | ×1.00 | 147552 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 889.22 M/s | 260.822 s | ×2.38 | 258.38 s | 99 % | 460 Mo | ×1.07 | 112525 / 0 | 0 / 48 | ✅ |

## parmatmul@2 (n = 6 144 — 2 threads)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (432 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust + Creusot | 5.35 G/s | 43.341 s | ×1.00 | 85.08 s | 196 % | 434 Mo | ×1.01 | 141428 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 4.84 G/s | 47.941 s | ×1.11 | 94.15 s | 196 % | 434 Mo | ×1.00 | 110694 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 4.70 G/s | 49.370 s | ×1.14 | 97.04 s | 197 % | 434 Mo | ×1.01 | 147568 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 4.20 G/s | 55.176 s | ×1.27 | 107.59 s | 195 % | 435 Mo | ×1.01 | 110821 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 4.06 G/s | 57.122 s | ×1.32 | 110.98 s | 194 % | 434 Mo | ×1.00 | 147556 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 1.78 G/s | 130.243 s | ×3.01 | 257.19 s | 197 % | 460 Mo | ×1.07 | 112541 / 0 | 0 / 72 | ✅ |

## parmatmul@4 (n = 6 144 — 4 threads)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (432 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 11.02 G/s | 21.051 s | ×1.00 | 81.48 s | 387 % | 434 Mo | ×1.00 | 110695 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 10.75 G/s | 21.581 s | ×1.03 | 84.18 s | 390 % | 434 Mo | ×1.01 | 141435 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 10.58 G/s | 21.916 s | ×1.04 | 85.40 s | 390 % | 434 Mo | ×1.01 | 147574 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 10.50 G/s | 22.083 s | ×1.05 | 85.42 s | 387 % | 435 Mo | ×1.01 | 110955 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 9.26 G/s | 25.049 s | ×1.19 | 97.49 s | 389 % | 434 Mo | ×1.00 | 147561 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 3.69 G/s | 62.866 s | ×2.99 | 244.23 s | 388 % | 460 Mo | ×1.07 | 112554 / 0 | 0 / 56 | ✅ |

## parsha@1 (n = 5 120 000 000)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (2 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 220.76 M/s | 23.193 s | ×1.00 | 22.96 s | 99 % | 2 Mo | ×0.86 | 94 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 216.66 M/s | 23.632 s | ×1.02 | 23.37 s | 99 % | 2 Mo | ×0.81 | 94 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 154.02 M/s | 33.243 s | ×1.43 | 32.92 s | 99 % | 2 Mo | ×0.99 | 101 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 136.53 M/s | 37.500 s | ×1.62 | 37.17 s | 99 % | 2 Mo | ×1.00 | 157 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 118.66 M/s | 43.148 s | ×1.86 | 42.82 s | 99 % | 28 Mo | ×14.00 | 1928 / 0 | 0 / 64 | ✅ |
| Rust + Creusot | 115.79 M/s | 44.217 s | ×1.91 | 43.93 s | 99 % | 2 Mo | ×0.99 | 104 / 0 | 0 / 0 | ✅ |

## parsha@2 (n = 5 120 000 000 — 2 threads)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (2 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 432.06 M/s | 11.850 s | ×1.00 | 23.33 s | 197 % | 2 Mo | ×0.88 | 98 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 431.17 M/s | 11.875 s | ×1.00 | 23.47 s | 198 % | 2 Mo | ×0.89 | 99 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 305.52 M/s | 16.758 s | ×1.41 | 33.19 s | 198 % | 2 Mo | ×1.05 | 111 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 260.19 M/s | 19.678 s | ×1.66 | 38.60 s | 196 % | 2 Mo | ×1.21 | 228 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 235.21 M/s | 21.768 s | ×1.84 | 43.01 s | 197 % | 28 Mo | ×14.02 | 1943 / 0 | 0 / 72 | ✅ |
| Rust + Creusot | 230.47 M/s | 22.216 s | ×1.87 | 44.00 s | 198 % | 2 Mo | ×1.10 | 111 / 0 | 0 / 0 | ✅ |

## parsha@4 (n = 5 120 000 000 — 4 threads)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (2 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 856.94 M/s | 5.975 s | ×1.00 | 23.38 s | 391 % | 2 Mo | ×0.96 | 103 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 833.07 M/s | 6.146 s | ×1.03 | 23.43 s | 381 % | 2 Mo | ×0.95 | 104 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 609.12 M/s | 8.406 s | ×1.41 | 32.76 s | 390 % | 2 Mo | ×1.10 | 115 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 490.58 M/s | 10.437 s | ×1.75 | 40.00 s | 383 % | 3 Mo | ×1.53 | 359 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 463.06 M/s | 11.057 s | ×1.85 | 43.11 s | 387 % | 28 Mo | ×14.20 | 1965 / 0 | 0 / 16 | ✅ |
| Rust + Creusot | 455.84 M/s | 11.232 s | ×1.88 | 44.06 s | 392 % | 2 Mo | ×1.10 | 116 / 0 | 0 / 0 | ✅ |

## parsort@1 (n = 100 000 000)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (763 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 5.92 M/s | 16.890 s | ×1.00 | 16.71 s | 99 % | 765 Mo | ×1.00 | 195471 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 5.32 M/s | 18.814 s | ×1.11 | 18.62 s | 99 % | 764 Mo | ×1.00 | 195408 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 5.17 M/s | 19.344 s | ×1.15 | 19.23 s | 99 % | 765 Mo | ×1.00 | 195419 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 5.17 M/s | 19.354 s | ×1.15 | 19.15 s | 99 % | 765 Mo | ×1.00 | 195416 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 5.14 M/s | 19.444 s | ×1.15 | 19.25 s | 99 % | 764 Mo | ×1.00 | 195407 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 3.82 M/s | 26.209 s | ×1.55 | 26.10 s | 99 % | 791 Mo | ×1.04 | 197262 / 0 | 0 / 80 | ✅ |

## parsort@2 (n = 100 000 000 — 2 threads)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (763 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 11.23 M/s | 8.908 s | ×1.00 | 16.41 s | 184 % | 765 Mo | ×1.00 | 195541 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 9.81 M/s | 10.189 s | ×1.14 | 18.98 s | 186 % | 765 Mo | ×1.00 | 195424 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 9.68 M/s | 10.327 s | ×1.16 | 19.06 s | 185 % | 765 Mo | ×1.00 | 195412 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 9.66 M/s | 10.351 s | ×1.16 | 19.25 s | 186 % | 765 Mo | ×1.00 | 195423 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 9.53 M/s | 10.495 s | ×1.18 | 19.32 s | 184 % | 765 Mo | ×1.00 | 195412 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 7.03 M/s | 14.225 s | ×1.60 | 26.49 s | 185 % | 791 Mo | ×1.04 | 197274 / 0 | 0 / 8 | ✅ |

## parsort@4 (n = 100 000 000 — 4 threads)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (763 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 18.78 M/s | 5.324 s | ×1.00 | 16.78 s | 315 % | 766 Mo | ×1.00 | 195672 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 17.12 M/s | 5.841 s | ×1.10 | 18.50 s | 317 % | 765 Mo | ×1.00 | 195418 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 16.66 M/s | 6.002 s | ×1.13 | 19.25 s | 320 % | 765 Mo | ×1.00 | 195427 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 16.46 M/s | 6.074 s | ×1.14 | 19.42 s | 320 % | 765 Mo | ×1.00 | 195432 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 16.30 M/s | 6.134 s | ×1.15 | 19.07 s | 311 % | 765 Mo | ×1.00 | 195417 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 11.82 M/s | 8.461 s | ×1.59 | 26.92 s | 314 % | 791 Mo | ×1.04 | 197291 / 0 | 0 / 8 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| F* (Pulse) → C | ×1.07 | ×1.07 | ×1.00 | 9 |
| C natif (à la main) | ×1.10 | ×1.10 | ×1.01 | 9 |
| Rust (sans Creusot) | ×1.20 | ×1.20 | ×1.06 | 9 |
| Zig 0.17 (à la main) | ×1.23 | ×1.22 | ×1.12 | 9 |
| Rust + Creusot | ×1.30 | ×1.31 | ×1.07 | 9 |
| F* (Pulse) → F# | ×2.01 | ×2.02 | ×2.60 | 9 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 4 ms | 2.0 Mo |
| F* (Pulse) → C | 4 ms | 1.7 Mo |
| F* (Pulse) → F# | 86 ms | 27.8 Mo |
| Rust (sans Creusot) | 4 ms | 2.1 Mo |
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
