# Résultats — preset `medium`

_2026-10-27 05:21:03 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v165 · médiane de 3 exécutions_

## parmatmul (n = 2 048 — 4 threads)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (48 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 18.42 G/s | 0.466 s | ×1.00 | 1.61 s | 342 % | 51 Mo | ×1.06 | 12650 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 16.06 G/s | 0.535 s | ×1.15 | 1.96 s | 364 % | 50 Mo | ×1.04 | 14458 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 15.95 G/s | 0.539 s | ×1.15 | 1.96 s | 360 % | 50 Mo | ×1.05 | 16502 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 15.24 G/s | 0.564 s | ×1.21 | 2.03 s | 358 % | 50 Mo | ×1.04 | 16490 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 14.60 G/s | 0.588 s | ×1.26 | 1.99 s | 337 % | 50 Mo | ×1.04 | 12393 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 3.83 G/s | 2.241 s | ×4.80 | 8.41 s | 361 % | 76 Mo | ×1.59 | 14243 / 0 | 0 / 8 | ✅ |

## parsha (n = 1 024 000 000 — 4 threads)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (2 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 840.18 M/s | 1.219 s | ×1.00 | 4.67 s | 383 % | 2 Mo | ×0.96 | 105 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 799.44 M/s | 1.281 s | ×1.05 | 4.87 s | 380 % | 2 Mo | ×0.95 | 102 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 590.69 M/s | 1.734 s | ×1.42 | 6.68 s | 384 % | 2 Mo | ×1.10 | 114 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 501.27 M/s | 2.043 s | ×1.68 | 7.76 s | 379 % | 3 Mo | ×1.53 | 359 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 447.94 M/s | 2.286 s | ×1.88 | 8.84 s | 386 % | 2 Mo | ×1.11 | 117 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 426.75 M/s | 2.400 s | ×1.97 | 9.24 s | 373 % | 28 Mo | ×14.20 | 1996 / 0 | 0 / 0 | ✅ |

## parsort (n = 20 000 000 — 4 threads)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (153 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 19.84 M/s | 1.008 s | ×1.00 | 3.15 s | 311 % | 156 Mo | ×1.02 | 39423 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 17.54 M/s | 1.141 s | ×1.13 | 3.54 s | 310 % | 154 Mo | ×1.01 | 39166 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 17.43 M/s | 1.148 s | ×1.14 | 3.59 s | 312 % | 155 Mo | ×1.01 | 39181 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 17.09 M/s | 1.170 s | ×1.16 | 3.59 s | 306 % | 155 Mo | ×1.01 | 39179 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 16.80 M/s | 1.191 s | ×1.18 | 3.57 s | 299 % | 155 Mo | ×1.01 | 39165 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 12.07 M/s | 1.657 s | ×1.64 | 5.02 s | 289 % | 181 Mo | ×1.19 | 41034 / 0 | 0 / 8 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| C natif (à la main) | ×1.13 | ×1.14 | ×1.00 | 3 |
| F* (Pulse) → C | ×1.14 | ×1.12 | ×1.00 | 3 |
| Zig 0.17 (à la main) | ×1.19 | ×1.18 | ×1.18 | 3 |
| Rust (sans Creusot) | ×1.24 | ×1.26 | ×1.05 | 3 |
| Rust + Creusot | ×1.35 | ×1.38 | ×1.06 | 3 |
| F* (Pulse) → F# | ×2.50 | ×2.55 | ×2.99 | 3 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 5 ms | 2.1 Mo |
| F* (Pulse) → C | 5 ms | 1.7 Mo |
| F* (Pulse) → F# | 89 ms | 27.8 Mo |
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
