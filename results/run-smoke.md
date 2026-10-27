# Résultats — preset `smoke`

_2026-10-26 18:36:52 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v153 · médiane de 1 exécutions_

## sort (n = 10 000)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 12.37 M/s | 0.001 s | ×1.00 | 0.00 s | 52 % | 2 Mo | ×22.48 | 95 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 12.25 M/s | 0.001 s | ×1.01 | 0.00 s | 62 % | 2 Mo | ×25.65 | 107 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 11.81 M/s | 0.001 s | ×1.05 | 0.00 s | 56 % | 2 Mo | ×22.53 | 97 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 10.71 M/s | 0.001 s | ×1.16 | 0.00 s | 63 % | 2 Mo | ×25.40 | 107 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 4.37 M/s | 0.002 s | ×2.83 | 0.00 s | 44 % | 2 Mo | ×22.53 | 98 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 917.58 k/s | 0.011 s | ×13.49 | 0.07 s | 95 % | 29 Mo | ×385.69 | 2144 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 104.95 k/s | 0.095 s | ×117.91 | 0.10 s | 97 % | 8 Mo | ×100.76 | 1066 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 27.06 k/s | 0.370 s | ×457.34 | 0.36 s | 97 % | 8 Mo | ×101.02 | 1077 / 0 | 0 / 0 | ✅ |

## sieve (n = 1 000 000)

Débit = entiers/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (1 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust + Creusot | 362.93 M/s | 0.003 s | ×1.00 | 0.00 s | 70 % | 3 Mo | ×2.98 | 333 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 308.74 M/s | 0.003 s | ×1.18 | 0.00 s | 72 % | 3 Mo | ×2.95 | 331 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 306.58 M/s | 0.003 s | ×1.18 | 0.00 s | 75 % | 2 Mo | ×2.39 | 324 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 294.55 M/s | 0.003 s | ×1.23 | 0.00 s | 75 % | 2 Mo | ×2.53 | 322 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 277.16 M/s | 0.004 s | ×1.31 | 0.00 s | 69 % | 2 Mo | ×2.39 | 324 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 103.17 M/s | 0.010 s | ×3.52 | 0.07 s | 93 % | 30 Mo | ×31.17 | 2163 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 3.48 M/s | 0.287 s | ×104.29 | 0.29 s | 98 % | 15 Mo | ×15.63 | 2921 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 596.56 k/s | 1.676 s | ×608.37 | 1.65 s | 98 % | 15 Mo | ×16.06 | 2981 / 0 | 0 / 0 | ✅ |

## matmul (n = 64)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust + Creusot | 2.84 G/s | 0.000 s | ×1.00 | 0.00 s | 44 % | 2 Mo | ×43.92 | 100 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 2.82 G/s | 0.000 s | ×1.01 | 0.00 s | 52 % | 2 Mo | ×41.58 | 98 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 2.69 G/s | 0.000 s | ×1.06 | 0.00 s | 40 % | 2 Mo | ×36.67 | 89 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 2.62 G/s | 0.000 s | ×1.08 | 0.00 s | 45 % | 2 Mo | ×36.67 | 90 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 2.15 G/s | 0.000 s | ×1.32 | 0.00 s | 42 % | 2 Mo | ×36.67 | 88 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 61.88 M/s | 0.004 s | ×45.86 | 0.05 s | 94 % | 29 Mo | ×611.67 | 1916 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 3.81 M/s | 0.069 s | ×745.21 | 0.07 s | 96 % | 8 Mo | ×163.92 | 1093 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 678.39 k/s | 0.386 s | ×4183.00 | 0.39 s | 99 % | 8 Mo | ×173.50 | 1165 / 0 | 0 / 0 | ✅ |

## lines (n = 10 000)

Débit = lignes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (16 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust + Creusot | 579.96 k/s | 0.017 s | ×1.00 | 0.02 s | 92 % | 17 Mo | ×1.03 | 7978 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 555.05 k/s | 0.018 s | ×1.04 | 0.02 s | 90 % | 17 Mo | ×1.04 | 7976 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 553.46 k/s | 0.018 s | ×1.05 | 0.02 s | 92 % | 16 Mo | ×1.00 | 7967 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 531.15 k/s | 0.019 s | ×1.09 | 0.02 s | 92 % | 16 Mo | ×1.00 | 7969 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 424.74 k/s | 0.024 s | ×1.37 | 0.02 s | 84 % | 16 Mo | ×1.01 | 7968 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 380.88 k/s | 0.026 s | ×1.52 | 0.09 s | 94 % | 46 Mo | ×2.84 | 6256 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 20.42 k/s | 0.490 s | ×28.40 | 0.48 s | 97 % | 25 Mo | ×1.55 | 5479 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 1.97 k/s | 5.069 s | ×293.95 | 4.96 s | 98 % | 26 Mo | ×1.59 | 5642 / 0 | 0 / 0 | ✅ |

## hashmap (n = 10 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 68.08 M/s | 0.000 s | ×1.00 | 0.00 s | 49 % | 2 Mo | ×3.81 | 205 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 61.80 M/s | 0.000 s | ×1.10 | 0.00 s | 52 % | 2 Mo | ×3.81 | 208 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 46.12 M/s | 0.001 s | ×1.48 | 0.00 s | 58 % | 2 Mo | ×4.89 | 345 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 45.46 M/s | 0.001 s | ×1.50 | 0.00 s | 57 % | 2 Mo | ×4.93 | 344 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 43.66 M/s | 0.001 s | ×1.56 | 0.00 s | 52 % | 2 Mo | ×3.56 | 336 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 3.17 M/s | 0.009 s | ×21.51 | 0.07 s | 95 % | 32 Mo | ×63.31 | 2667 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 405.13 k/s | 0.074 s | ×168.05 | 0.08 s | 95 % | 8 Mo | ×16.50 | 1213 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 64.42 k/s | 0.466 s | ×1056.88 | 0.44 s | 94 % | 8 Mo | ×16.68 | 1228 / 0 | 0 / 0 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| F* (Pulse) → C | ×1.10 | ×1.09 | ×1.01 | 5 |
| Zig 0.17 (à la main) | ×1.11 | ×1.04 | ×1.02 | 5 |
| Rust + Creusot | ×1.11 | ×1.12 | ×1.19 | 5 |
| Rust (sans Creusot) | ×1.13 | ×1.19 | ×1.18 | 5 |
| C natif (à la main) | ×1.53 | ×1.17 | ×1.00 | 5 |
| F* (Pulse) → F# | ×9.34 | ×22.99 | ×11.35 | 5 |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | ×134.33 | ×50.09 | ×3.93 | 5 |
| F* (Pulse) → OCaml (runtime livré) | ×815.91 | ×295.24 | ×4.03 | 5 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 4 ms | 2.0 Mo |
| F* (Pulse) → C | 3 ms | 1.7 Mo |
| F* (Pulse) → F# | 77 ms | 29.5 Mo |
| F* (Pulse) → OCaml (runtime livré) | 411 ms | 7.7 Mo |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 66 ms | 7.7 Mo |
| Rust (sans Creusot) | 4 ms | 1.9 Mo |
| C natif (à la main) | 3 ms | 1.7 Mo |
| Zig 0.17 (à la main) | 3 ms | 1.7 Mo |

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
