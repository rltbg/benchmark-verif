# Résultats — preset `tiny`

_2026-10-27 02:42:34 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v165 · médiane de 3 exécutions_

## sort (n = 200 000)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (2 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 9.66 M/s | 0.021 s | ×1.00 | 0.02 s | 93 % | 3 Mo | ×1.98 | 543 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 8.38 M/s | 0.024 s | ×1.15 | 0.03 s | 91 % | 3 Mo | ×1.98 | 546 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 8.09 M/s | 0.025 s | ×1.19 | 0.03 s | 92 % | 3 Mo | ×1.90 | 545 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 7.86 M/s | 0.025 s | ×1.23 | 0.03 s | 92 % | 3 Mo | ×2.21 | 554 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 7.28 M/s | 0.027 s | ×1.33 | 0.03 s | 95 % | 3 Mo | ×2.26 | 553 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 5.17 M/s | 0.039 s | ×1.87 | 0.10 s | 98 % | 30 Mo | ×19.90 | 2339 / 0 | 0 / 0 | ✅ |

## sieve (n = 5 000 000)

Débit = entiers/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (5 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 190.33 M/s | 0.026 s | ×1.00 | 0.03 s | 94 % | 6 Mo | ×1.31 | 1372 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 187.10 M/s | 0.027 s | ×1.02 | 0.03 s | 93 % | 6 Mo | ×1.30 | 1374 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 185.19 M/s | 0.027 s | ×1.03 | 0.03 s | 94 % | 7 Mo | ×1.41 | 1383 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 177.59 M/s | 0.028 s | ×1.07 | 0.03 s | 93 % | 6 Mo | ×1.30 | 1375 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 165.58 M/s | 0.030 s | ×1.15 | 0.03 s | 94 % | 7 Mo | ×1.41 | 1384 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 153.18 M/s | 0.033 s | ×1.24 | 0.10 s | 97 % | 33 Mo | ×7.02 | 3164 / 0 | 0 / 0 | ✅ |

## matmul (n = 128)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust (sans Creusot) | 5.72 G/s | 0.000 s | ×1.00 | 0.00 s | 61 % | 2 Mo | ×11.04 | 223 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 4.53 G/s | 0.000 s | ×1.26 | 0.00 s | 52 % | 2 Mo | ×9.81 | 215 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 4.24 G/s | 0.000 s | ×1.35 | 0.00 s | 62 % | 2 Mo | ×9.79 | 214 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 2.77 G/s | 0.001 s | ×2.06 | 0.00 s | 67 % | 2 Mo | ×11.10 | 209 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 2.26 G/s | 0.001 s | ×2.53 | 0.00 s | 67 % | 2 Mo | ×9.79 | 198 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 455.66 M/s | 0.005 s | ×12.55 | 0.07 s | 100 % | 29 Mo | ×154.71 | 1996 / 0 | 0 / 0 | ✅ |

## lines (n = 50 000)

Débit = lignes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (17 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → F# | 2.34 M/s | 0.021 s | ×1.00 | 0.08 s | 97 % | 46 Mo | ×2.73 | 6227 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 2.31 M/s | 0.022 s | ×1.01 | 0.03 s | 102 % | 19 Mo | ×1.11 | 8524 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 2.20 M/s | 0.023 s | ×1.06 | 0.03 s | 92 % | 18 Mo | ×1.08 | 8514 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 2.19 M/s | 0.023 s | ×1.06 | 0.02 s | 93 % | 18 Mo | ×1.08 | 8512 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 2.01 M/s | 0.025 s | ×1.16 | 0.03 s | 90 % | 19 Mo | ×1.11 | 8523 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 1.73 M/s | 0.029 s | ×1.35 | 0.03 s | 94 % | 18 Mo | ×1.08 | 8515 / 0 | 0 / 0 | ✅ |

## hashmap (n = 100 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (4 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 51.01 M/s | 0.006 s | ×1.00 | 0.01 s | 84 % | 5 Mo | ×1.35 | 1177 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 50.79 M/s | 0.006 s | ×1.00 | 0.01 s | 81 % | 5 Mo | ×1.35 | 1179 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 37.37 M/s | 0.008 s | ×1.36 | 0.01 s | 77 % | 6 Mo | ×1.48 | 2209 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 35.65 M/s | 0.008 s | ×1.43 | 0.01 s | 84 % | 6 Mo | ×1.48 | 2210 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 35.29 M/s | 0.009 s | ×1.45 | 0.01 s | 71 % | 5 Mo | ×1.32 | 2201 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 19.19 M/s | 0.016 s | ×2.66 | 0.08 s | 103 % | 33 Mo | ×8.21 | 2972 / 0 | 0 / 0 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| F* (Pulse) → C | ×1.10 | ×1.04 | ×1.01 | 5 |
| Rust (sans Creusot) | ×1.12 | ×1.11 | ×1.11 | 5 |
| Zig 0.17 (à la main) | ×1.22 | ×1.04 | ×1.02 | 5 |
| C natif (à la main) | ×1.27 | ×1.15 | ×1.01 | 5 |
| Rust + Creusot | ×1.39 | ×1.28 | ×1.11 | 5 |
| F* (Pulse) → F# | ×2.39 | ×6.68 | ×6.76 | 5 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 5 ms | 2.1 Mo |
| F* (Pulse) → C | 5 ms | 1.7 Mo |
| F* (Pulse) → F# | 69 ms | 28.9 Mo |
| Rust (sans Creusot) | 5 ms | 2.0 Mo |
| C natif (à la main) | 5 ms | 1.7 Mo |
| Zig 0.17 (à la main) | 4 ms | 1.7 Mo |

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
