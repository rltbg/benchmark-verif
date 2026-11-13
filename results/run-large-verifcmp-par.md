# Résultats — preset `large`

_2026-11-13 21:56:11 · 4 CPU · 16086 Mo RAM · Linux vm 6.18.44-fc-v445 · médiane de 3 exécutions_

## parmatmul (n = 6 144 — 4 threads)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (432 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust + Creusot | 11.89 G/s | 19.504 s | ×1.00 | 76.07 s | 390 % | 434 Mo | ×1.01 | 141432 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 10.70 G/s | 21.666 s | ×1.11 | 84.82 s | 391 % | 434 Mo | ×1.01 | 147574 / 0 | 0 / 0 | ✅ |
| Rust + Verus | 10.57 G/s | 21.952 s | ×1.13 | 85.93 s | 391 % | 434 Mo | ×1.01 | 141433 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 9.92 G/s | 23.391 s | ×1.20 | 91.28 s | 390 % | 434 Mo | ×1.00 | 147563 / 0 | 0 / 0 | ✅ |

## parsha (n = 5 120 000 000 — 4 threads)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (2 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 1.07 G/s | 4.787 s | ×1.00 | 18.78 s | 392 % | 2 Mo | ×0.95 | 102 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 760.01 M/s | 6.737 s | ×1.41 | 26.19 s | 389 % | 2 Mo | ×1.09 | 115 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 736.89 M/s | 6.948 s | ×1.45 | 27.01 s | 389 % | 2 Mo | ×1.10 | 116 / 0 | 0 / 0 | ✅ |
| Rust + Verus | 653.96 M/s | 7.829 s | ×1.64 | 29.84 s | 381 % | 2 Mo | ×1.10 | 117 / 0 | 0 / 0 | ✅ |

## parsort (n = 100 000 000 — 4 threads)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (763 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 20.24 M/s | 4.941 s | ×1.00 | 15.61 s | 316 % | 765 Mo | ×1.00 | 195417 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 19.63 M/s | 5.093 s | ×1.03 | 15.97 s | 313 % | 765 Mo | ×1.00 | 195429 / 0 | 0 / 0 | ✅ |
| Rust + Verus | 19.37 M/s | 5.162 s | ×1.04 | 16.26 s | 315 % | 765 Mo | ×1.00 | 195429 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 19.16 M/s | 5.219 s | ×1.06 | 16.36 s | 313 % | 765 Mo | ×1.00 | 195430 / 0 | 0 / 0 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| C natif (à la main) | ×1.06 | ×1.06 | ×1.00 | 3 |
| Rust + Creusot | ×1.13 | ×1.13 | ×1.05 | 3 |
| Rust (sans Creusot) | ×1.19 | ×1.19 | ×1.05 | 3 |
| Rust + Verus | ×1.24 | ×1.23 | ×1.05 | 3 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 4 ms | 2.0 Mo |
| Rust (sans Creusot) | 4 ms | 2.0 Mo |
| C natif (à la main) | 3 ms | 1.7 Mo |
| Rust + Verus | 4 ms | 2.0 Mo |

## Empreinte disque et coût de construction

| implémentation | exécutable | runtime/deps requis | compilation | vérification |
|---|--:|--:|--:|--:|
| Rust + Creusot | 0.46 Mo | 0.0 Mo | 0.2 s | 107.5 s |
| F* (Pulse) → C | 0.05 Mo | 0.0 Mo | 3.5 s | 428.5 s |
| F* (Pulse) → OCaml (runtime livré) | 5.83 Mo | 0.0 Mo | 5.3 s | 428.5 s |
| F* (Pulse) → F# | 0.09 Mo | 72.4 Mo | 9.6 s | 428.5 s |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 5.74 Mo | 0.0 Mo | 7.9 s | 428.5 s |
| Rust (sans Creusot) | 0.46 Mo | 0.0 Mo | 0.1 s | — |
| C natif (à la main) | 0.04 Mo | 0.0 Mo | 0.8 s | — |
| Zig 0.17 (à la main) | 3.71 Mo | 0.0 Mo | 18.1 s | — |
| Rust + Verus | 0.00 Mo | 0.0 Mo | 0.0 s | 13.8 s |
