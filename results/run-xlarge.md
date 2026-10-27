# Résultats — preset `xlarge`

_2026-10-27 02:56:41 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v165 · médiane de 3 exécutions_

## sort (n = 100 000 000)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (763 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 6.11 M/s | 16.356 s | ×1.00 | 16.28 s | 100 % | 764 Mo | ×1.00 | 195466 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 5.40 M/s | 18.535 s | ×1.13 | 18.51 s | 100 % | 764 Mo | ×1.00 | 195467 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 5.36 M/s | 18.641 s | ×1.14 | 18.46 s | 99 % | 764 Mo | ×1.00 | 195467 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 4.47 M/s | 22.350 s | ×1.37 | 22.09 s | 99 % | 765 Mo | ×1.00 | 195475 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 4.42 M/s | 22.608 s | ×1.38 | 22.44 s | 99 % | 765 Mo | ×1.00 | 195476 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 3.58 M/s | 27.898 s | ×1.71 | 27.72 s | 99 % | 792 Mo | ×1.04 | 197292 / 0 | 0 / 56 | ✅ |

## sieve (n = 3 000 000 000)

Débit = entiers/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (2861 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 56.02 M/s | 53.552 s | ×1.00 | 52.89 s | 99 % | 2862 Mo | ×1.00 | 732575 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 54.21 M/s | 55.345 s | ×1.03 | 54.99 s | 99 % | 2863 Mo | ×1.00 | 732583 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 51.30 M/s | 58.481 s | ×1.09 | 58.23 s | 100 % | 2862 Mo | ×1.00 | 732576 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 49.67 M/s | 60.400 s | ×1.13 | 59.28 s | 98 % | 2863 Mo | ×1.00 | 732573 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 49.32 M/s | 60.829 s | ×1.14 | 60.34 s | 99 % | 2863 Mo | ×1.00 | 732583 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | **failed** | | | | | | | | | |

## matmul (n = 4 096)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (192 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 5.94 G/s | 11.565 s | ×1.00 | 11.50 s | 99 % | 193 Mo | ×1.01 | 65692 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 5.54 G/s | 12.414 s | ×1.07 | 12.33 s | 99 % | 194 Mo | ×1.01 | 61605 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 5.13 G/s | 13.398 s | ×1.16 | 13.24 s | 99 % | 193 Mo | ×1.01 | 65694 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 4.91 G/s | 13.993 s | ×1.21 | 13.85 s | 99 % | 194 Mo | ×1.01 | 65701 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 1.12 G/s | 61.416 s | ×5.31 | 60.87 s | 99 % | 194 Mo | ×1.01 | 49308 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 877.06 M/s | 78.352 s | ×6.77 | 77.78 s | 99 % | 221 Mo | ×1.15 | 51121 / 0 | 0 / 32 | ✅ |

## lines (n = 100 000 000)

Débit = lignes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (1447 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 11.58 M/s | 8.636 s | ×1.00 | 8.51 s | 99 % | 1336 Mo | ×0.92 | 345976 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 10.36 M/s | 9.650 s | ×1.12 | 8.75 s | 91 % | 1337 Mo | ×0.92 | 345983 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 10.32 M/s | 9.694 s | ×1.12 | 9.47 s | 98 % | 1336 Mo | ×0.92 | 345974 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 9.47 M/s | 10.562 s | ×1.22 | 10.02 s | 95 % | 1337 Mo | ×0.92 | 345982 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 9.13 M/s | 10.954 s | ×1.27 | 10.68 s | 97 % | 1336 Mo | ×0.92 | 345972 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 7.12 M/s | 14.045 s | ×1.63 | 12.96 s | 91 % | 1476 Mo | ×1.02 | 372290 / 0 | 0 / 16 | ✅ |

## hashmap (n = 48 000 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (2048 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust + Creusot | 6.63 M/s | 21.731 s | ×1.00 | 21.59 s | 99 % | 1026 Mo | ×0.50 | 524451 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 6.07 M/s | 23.716 s | ×1.09 | 23.56 s | 99 % | 1025 Mo | ×0.50 | 524442 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 5.97 M/s | 24.140 s | ×1.11 | 23.96 s | 99 % | 1026 Mo | ×0.50 | 524451 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 5.86 M/s | 24.556 s | ×1.13 | 24.40 s | 99 % | 1026 Mo | ×0.50 | 262298 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 5.72 M/s | 25.192 s | ×1.16 | 25.02 s | 99 % | 1026 Mo | ×0.50 | 262297 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 4.38 M/s | 32.865 s | ×1.51 | 32.69 s | 99 % | 1053 Mo | ×0.51 | 264111 / 0 | 0 / 56 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| F* (Pulse) → C | ×1.08 | ×1.08 | ×1.00 | 5 |
| C natif (à la main) | ×1.09 | ×1.09 | ×1.00 | 5 |
| Rust + Creusot | ×1.13 | ×1.12 | ×1.00 | 5 |
| Rust (sans Creusot) | ×1.19 | ×1.17 | ×1.00 | 5 |
| Zig 0.17 (à la main) | ×1.55 | ×1.54 | ×1.00 | 5 |
| F* (Pulse) → F# | ×2.31 | ×2.27 | ×1.08 | 4 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 5 ms | 2.1 Mo |
| F* (Pulse) → C | 5 ms | 1.7 Mo |
| F* (Pulse) → F# | 72 ms | 28.9 Mo |
| Rust (sans Creusot) | 6 ms | 2.1 Mo |
| C natif (à la main) | 6 ms | 1.7 Mo |
| Zig 0.17 (à la main) | 5 ms | 1.7 Mo |

## Empreinte disque et coût de construction

| implémentation | exécutable | runtime/deps requis | compilation | vérification |
|---|--:|--:|--:|--:|
| Rust + Creusot | 0.39 Mo | 0.0 Mo | 16.7 s | 16.3 s |
| F* (Pulse) → C | 0.02 Mo | 0.0 Mo | 4.0 s | 236.5 s |
| F* (Pulse) → OCaml (runtime livré) | 4.96 Mo | 0.0 Mo | 10.1 s | 236.5 s |
| F* (Pulse) → F# | 0.05 Mo | 72.4 Mo | 22.6 s | 236.5 s |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 4.94 Mo | 0.0 Mo | 11.5 s | 236.5 s |
| Rust (sans Creusot) | 0.39 Mo | 0.0 Mo | 4.0 s | — |
| C natif (à la main) | 0.02 Mo | 0.0 Mo | 1.1 s | — |
| Zig 0.17 (à la main) | 0.06 Mo | 0.0 Mo | 4.0 s | — |
