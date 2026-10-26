# Résultats — preset `tiny`

_2026-10-26 18:02:24 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v153 · médiane de 3 exécutions_

## sort (n = 200 000)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (2 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 10.83 M/s | 0.018 s | ×1.00 | 0.02 s | 92 % | 3 Mo | ×1.99 | 469 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 10.01 M/s | 0.020 s | ×1.08 | 0.02 s | 88 % | 3 Mo | ×1.95 | 472 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 8.92 M/s | 0.022 s | ×1.21 | 0.02 s | 85 % | 3 Mo | ×1.99 | 471 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 8.62 M/s | 0.023 s | ×1.26 | 0.02 s | 82 % | 3 Mo | ×2.27 | 478 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 8.48 M/s | 0.024 s | ×1.28 | 0.02 s | 90 % | 3 Mo | ×2.23 | 480 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 1.41 M/s | 0.142 s | ×7.67 | 0.20 s | 98 % | 48 Mo | ×31.62 | 6986 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 123.11 k/s | 1.625 s | ×87.99 | 1.60 s | 98 % | 15 Mo | ×9.87 | 2970 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 16.34 k/s | 12.237 s | ×662.73 | 12.10 s | 99 % | 17 Mo | ×11.12 | 3451 / 0 | 0 / 0 | ✅ |

## sieve (n = 5 000 000)

Débit = entiers/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (5 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 177.09 M/s | 0.028 s | ×1.00 | 0.03 s | 93 % | 6 Mo | ×1.30 | 1300 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 162.05 M/s | 0.031 s | ×1.09 | 0.03 s | 94 % | 6 Mo | ×1.32 | 1298 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 160.61 M/s | 0.031 s | ×1.10 | 0.03 s | 95 % | 7 Mo | ×1.41 | 1309 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 158.15 M/s | 0.032 s | ×1.12 | 0.03 s | 87 % | 7 Mo | ×1.41 | 1307 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 149.35 M/s | 0.033 s | ×1.19 | 0.03 s | 91 % | 6 Mo | ×1.30 | 1299 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 104.13 M/s | 0.048 s | ×1.70 | 0.11 s | 97 % | 33 Mo | ×6.96 | 3110 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 2.88 M/s | 1.736 s | ×61.48 | 1.73 s | 99 % | 45 Mo | ×9.52 | 10743 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 489.85 k/s | 10.207 s | ×361.52 | 10.08 s | 99 % | 47 Mo | ×9.86 | 11100 / 0 | 0 / 0 | ✅ |

## matmul (n = 128)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust (sans Creusot) | 3.16 G/s | 0.001 s | ×1.00 | 0.00 s | 57 % | 2 Mo | ×11.00 | 149 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 3.07 G/s | 0.001 s | ×1.03 | 0.00 s | 42 % | 2 Mo | ×9.83 | 140 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 2.64 G/s | 0.001 s | ×1.20 | 0.00 s | 57 % | 2 Mo | ×11.12 | 149 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 2.31 G/s | 0.001 s | ×1.37 | 0.00 s | 51 % | 2 Mo | ×9.83 | 140 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 1.64 G/s | 0.001 s | ×1.93 | 0.00 s | 62 % | 2 Mo | ×9.83 | 124 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 84.84 M/s | 0.025 s | ×37.19 | 0.09 s | 96 % | 29 Mo | ×154.58 | 2029 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 3.88 M/s | 0.541 s | ×813.42 | 0.54 s | 99 % | 10 Mo | ×53.33 | 1687 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 720.44 k/s | 2.911 s | ×4379.54 | 2.87 s | 99 % | 12 Mo | ×61.79 | 2072 / 0 | 0 / 0 | ✅ |

## lines (n = 50 000)

Débit = lignes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (17 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 2.16 M/s | 0.023 s | ×1.00 | 0.02 s | 83 % | 18 Mo | ×1.08 | 8440 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 1.98 M/s | 0.025 s | ×1.09 | 0.03 s | 94 % | 19 Mo | ×1.11 | 8447 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 1.95 M/s | 0.026 s | ×1.11 | 0.03 s | 94 % | 19 Mo | ×1.11 | 8448 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 1.73 M/s | 0.029 s | ×1.25 | 0.03 s | 93 % | 18 Mo | ×1.08 | 8440 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 1.68 M/s | 0.030 s | ×1.29 | 0.03 s | 95 % | 18 Mo | ×1.08 | 8439 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 1.46 M/s | 0.034 s | ×1.48 | 0.10 s | 97 % | 50 Mo | ×2.98 | 7352 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 65.64 k/s | 0.762 s | ×32.94 | 0.75 s | 97 % | 31 Mo | ×1.88 | 7122 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 6.60 k/s | 7.574 s | ×327.51 | 7.51 s | 99 % | 32 Mo | ×1.92 | 7310 / 0 | 0 / 0 | ✅ |

## hashmap (n = 100 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (4 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 44.16 M/s | 0.007 s | ×1.00 | 0.01 s | 94 % | 5 Mo | ×1.35 | 1105 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 39.80 M/s | 0.008 s | ×1.11 | 0.01 s | 78 % | 5 Mo | ×1.35 | 1102 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 37.79 M/s | 0.008 s | ×1.17 | 0.01 s | 82 % | 6 Mo | ×1.48 | 2137 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 35.51 M/s | 0.008 s | ×1.24 | 0.01 s | 91 % | 6 Mo | ×1.45 | 2134 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 30.80 M/s | 0.010 s | ×1.43 | 0.01 s | 73 % | 5 Mo | ×1.32 | 2128 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 4.73 M/s | 0.063 s | ×9.33 | 0.15 s | 103 % | 58 Mo | ×14.41 | 9384 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 538.82 k/s | 0.557 s | ×81.96 | 0.56 s | 99 % | 16 Mo | ×3.99 | 3173 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 69.80 k/s | 4.298 s | ×632.61 | 4.26 s | 99 % | 16 Mo | ×4.12 | 3319 / 0 | 0 / 0 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| F* (Pulse) → C | ×1.02 | ×1.04 | ×1.01 | 5 |
| Rust (sans Creusot) | ×1.14 | ×1.21 | ×1.10 | 5 |
| Rust + Creusot | ×1.17 | ×1.23 | ×1.10 | 5 |
| Zig 0.17 (à la main) | ×1.25 | ×1.23 | ×1.01 | 5 |
| C natif (à la main) | ×1.29 | ×1.24 | ×1.00 | 5 |
| F* (Pulse) → F# | ×5.83 | ×11.31 | ×8.36 | 5 |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | ×103.50 | ×83.68 | ×4.02 | 5 |
| F* (Pulse) → OCaml (runtime livré) | ×736.97 | ×593.94 | ×4.32 | 5 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 4 ms | 2.0 Mo |
| F* (Pulse) → C | 4 ms | 1.7 Mo |
| F* (Pulse) → F# | 74 ms | 29.4 Mo |
| F* (Pulse) → OCaml (runtime livré) | 442 ms | 7.7 Mo |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 65 ms | 7.6 Mo |
| Rust (sans Creusot) | 4 ms | 2.0 Mo |
| C natif (à la main) | 4 ms | 1.7 Mo |
| Zig 0.17 (à la main) | 4 ms | 1.7 Mo |

## Empreinte disque et coût de construction

| implémentation | exécutable | runtime/deps requis | compilation | vérification |
|---|--:|--:|--:|--:|
| Rust + Creusot | 0.38 Mo | 0.0 Mo | 8.9 s | 16.3 s |
| F* (Pulse) → C | 0.02 Mo | 0.0 Mo | 2.4 s | 236.5 s |
| F* (Pulse) → OCaml (runtime livré) | 4.96 Mo | 0.0 Mo | 3.1 s | 236.5 s |
| F* (Pulse) → F# | 0.05 Mo | 72.4 Mo | 7.2 s | 236.5 s |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 4.94 Mo | 0.0 Mo | 4.5 s | 236.5 s |
| Rust (sans Creusot) | 0.38 Mo | 0.0 Mo | 4.1 s | — |
| C natif (à la main) | 0.02 Mo | 0.0 Mo | 0.2 s | — |
| Zig 0.17 (à la main) | 0.06 Mo | 0.0 Mo | 0.3 s | — |
