# Résultats — preset `xlarge`

_2026-10-26 18:19:41 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v153 · médiane de 1 exécutions_

## sort (n = 100 000 000)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (763 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 7.12 M/s | 14.040 s | ×1.00 | 13.93 s | 99 % | 764 Mo | ×1.00 | 195392 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 6.64 M/s | 15.065 s | ×1.07 | 14.72 s | 98 % | 764 Mo | ×1.00 | 195393 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 6.20 M/s | 16.124 s | ×1.15 | 15.95 s | 99 % | 764 Mo | ×1.00 | 195393 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 6.00 M/s | 16.657 s | ×1.19 | 16.52 s | 99 % | 765 Mo | ×1.00 | 195401 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 5.74 M/s | 17.410 s | ×1.24 | 17.26 s | 99 % | 765 Mo | ×1.00 | 195402 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 4.32 M/s | 23.140 s | ×1.65 | 22.98 s | 99 % | 812 Mo | ×1.06 | 202356 / 0 | 0 / 56 | ✅ |

## sieve (n = 3 000 000 000)

Débit = entiers/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (2861 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 50.62 M/s | 59.260 s | ×1.00 | 58.63 s | 99 % | 2862 Mo | ×1.00 | 732501 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 49.86 M/s | 60.172 s | ×1.02 | 59.55 s | 99 % | 2863 Mo | ×1.00 | 732508 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 49.40 M/s | 60.727 s | ×1.02 | 59.97 s | 99 % | 2863 Mo | ×1.00 | 732499 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 48.79 M/s | 61.482 s | ×1.04 | 60.71 s | 99 % | 2862 Mo | ×1.00 | 732501 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 42.10 M/s | 71.262 s | ×1.20 | 70.24 s | 99 % | 2863 Mo | ×1.00 | 732509 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | **failed** | | | | | | | | | |

## matmul (n = 4 096)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (192 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust + Creusot | 2.87 G/s | 23.936 s | ×1.00 | 23.73 s | 99 % | 194 Mo | ×1.01 | 61530 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 2.50 G/s | 27.534 s | ×1.15 | 27.27 s | 99 % | 193 Mo | ×1.01 | 65618 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 1.59 G/s | 43.142 s | ×1.80 | 42.64 s | 99 % | 194 Mo | ×1.01 | 65626 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 1.57 G/s | 43.900 s | ×1.83 | 43.36 s | 99 % | 193 Mo | ×1.01 | 65618 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 1.14 G/s | 60.402 s | ×2.52 | 59.74 s | 99 % | 194 Mo | ×1.01 | 49232 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 513.56 M/s | 133.811 s | ×5.59 | 132.29 s | 99 % | 351 Mo | ×1.83 | 84386 / 0 | 0 / 72 | ✅ |

## lines (n = 100 000 000)

Débit = lignes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (1447 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 18.31 M/s | 5.462 s | ×1.00 | 5.44 s | 99 % | 1336 Mo | ×0.92 | 345901 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 16.07 M/s | 6.221 s | ×1.14 | 6.15 s | 99 % | 1337 Mo | ×0.92 | 345907 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 15.40 M/s | 6.495 s | ×1.19 | 6.37 s | 98 % | 1336 Mo | ×0.92 | 345899 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 14.16 M/s | 7.062 s | ×1.29 | 6.97 s | 99 % | 1337 Mo | ×0.92 | 345910 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 14.00 M/s | 7.142 s | ×1.31 | 7.02 s | 98 % | 1336 Mo | ×0.92 | 345900 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 7.96 M/s | 12.557 s | ×2.30 | 12.46 s | 98 % | 1609 Mo | ×1.11 | 406569 / 0 | 0 / 8 | ✅ |

## hashmap (n = 48 000 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (2048 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 6.74 M/s | 21.379 s | ×1.00 | 21.13 s | 99 % | 1025 Mo | ×0.50 | 524368 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 6.11 M/s | 23.585 s | ×1.10 | 23.38 s | 99 % | 1026 Mo | ×0.50 | 524375 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 6.06 M/s | 23.757 s | ×1.11 | 23.50 s | 99 % | 1026 Mo | ×0.50 | 524376 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 5.71 M/s | 25.221 s | ×1.18 | 24.94 s | 99 % | 1026 Mo | ×0.50 | 262222 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 5.23 M/s | 27.531 s | ×1.29 | 27.17 s | 99 % | 1026 Mo | ×0.50 | 262225 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 3.83 M/s | 37.559 s | ×1.76 | 37.22 s | 99 % | 1187 Mo | ×0.58 | 298405 / 0 | 0 / 48 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| C natif (à la main) | ×1.12 | ×1.12 | ×1.00 | 5 |
| Rust + Creusot | ×1.16 | ×1.16 | ×1.00 | 5 |
| F* (Pulse) → C | ×1.20 | ×1.20 | ×1.00 | 5 |
| Rust (sans Creusot) | ×1.22 | ×1.22 | ×1.00 | 5 |
| Zig 0.17 (à la main) | ×1.29 | ×1.29 | ×1.00 | 5 |
| F* (Pulse) → F# | ×2.47 | ×2.47 | ×1.28 | 4 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 4 ms | 2.1 Mo |
| F* (Pulse) → C | 3 ms | 1.7 Mo |
| F* (Pulse) → F# | 67 ms | 29.4 Mo |
| Rust (sans Creusot) | 4 ms | 2.0 Mo |
| C natif (à la main) | 4 ms | 1.7 Mo |
| Zig 0.17 (à la main) | 3 ms | 1.7 Mo |

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
