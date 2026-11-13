# Résultats — preset `large`

_2026-11-13 21:32:50 · 4 CPU · 16086 Mo RAM · Linux vm 6.18.44-fc-v445 · médiane de 3 exécutions_

## sort (n = 32 000 000)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (244 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust + Creusot | 6.59 M/s | 4.854 s | ×1.00 | 4.83 s | 99 % | 246 Mo | ×1.01 | 62676 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 6.58 M/s | 4.866 s | ×1.00 | 4.79 s | 98 % | 246 Mo | ×1.01 | 62668 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 6.52 M/s | 4.911 s | ×1.01 | 4.83 s | 98 % | 246 Mo | ×1.01 | 62678 / 0 | 0 / 0 | ✅ |
| Rust + Verus | 6.13 M/s | 5.223 s | ×1.08 | 5.19 s | 99 % | 246 Mo | ×1.01 | 62677 / 0 | 0 / 0 | ✅ |

## sieve (n = 1 000 000 000)

Débit = entiers/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (954 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 58.09 M/s | 17.214 s | ×1.00 | 17.09 s | 99 % | 955 Mo | ×1.00 | 244306 / 0 | 0 / 0 | ✅ |
| Rust + Verus | 56.27 M/s | 17.770 s | ×1.03 | 17.56 s | 99 % | 956 Mo | ×1.00 | 244316 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 55.84 M/s | 17.909 s | ×1.04 | 17.78 s | 99 % | 956 Mo | ×1.00 | 244314 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 54.82 M/s | 18.242 s | ×1.06 | 18.17 s | 100 % | 956 Mo | ×1.00 | 244316 / 0 | 0 / 0 | ✅ |

## matmul (n = 2 500)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (72 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust (sans Creusot) | 5.96 G/s | 2.624 s | ×1.00 | 2.63 s | 100 % | 73 Mo | ×1.03 | 24588 / 0 | 0 / 0 | ✅ |
| Rust + Verus | 5.95 G/s | 2.627 s | ×1.00 | 2.62 s | 100 % | 73 Mo | ×1.03 | 24582 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 5.77 G/s | 2.710 s | ×1.03 | 2.72 s | 100 % | 73 Mo | ×1.02 | 24580 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 5.41 G/s | 2.890 s | ×1.10 | 2.86 s | 99 % | 73 Mo | ×1.03 | 24581 / 0 | 0 / 0 | ✅ |

## lines (n = 40 000 000)

Débit = lignes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (588 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust (sans Creusot) | 19.14 M/s | 2.090 s | ×1.00 | 2.06 s | 98 % | 546 Mo | ×0.93 | 143420 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 19.05 M/s | 2.100 s | ×1.00 | 2.04 s | 97 % | 546 Mo | ×0.93 | 143419 / 0 | 0 / 0 | ✅ |
| Rust + Verus | 17.35 M/s | 2.305 s | ×1.10 | 2.28 s | 99 % | 546 Mo | ×0.93 | 143420 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 14.89 M/s | 2.686 s | ×1.29 | 2.66 s | 99 % | 545 Mo | ×0.93 | 143411 / 0 | 0 / 0 | ✅ |

## hashmap (n = 16 000 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (512 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 19.50 M/s | 2.461 s | ×1.00 | 2.44 s | 99 % | 513 Mo | ×1.00 | 262312 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 18.86 M/s | 2.545 s | ×1.03 | 2.51 s | 98 % | 514 Mo | ×1.00 | 262320 / 0 | 0 / 0 | ✅ |
| Rust + Verus | 18.59 M/s | 2.582 s | ×1.05 | 2.58 s | 100 % | 514 Mo | ×1.00 | 262320 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 17.37 M/s | 2.763 s | ×1.12 | 2.76 s | 100 % | 514 Mo | ×1.00 | 262320 / 0 | 0 / 0 | ✅ |

## alloc (n = 100 000 000)

Débit = allocations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (209 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 7.85 M/s | 12.741 s | ×1.00 | 12.69 s | 99 % | 232 Mo | ×1.11 | 59117 / 0 | 0 / 0 | ✅ |
| Rust + Verus | 7.73 M/s | 12.935 s | ×1.02 | 12.90 s | 100 % | 260 Mo | ×1.25 | 66355 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 7.51 M/s | 13.316 s | ×1.05 | 13.21 s | 99 % | 260 Mo | ×1.25 | 66353 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 7.23 M/s | 13.825 s | ×1.09 | 13.70 s | 99 % | 249 Mo | ×1.19 | 63303 / 0 | 0 / 0 | ✅ |

## bfs (n = 15 000 000)

Débit = arêtes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (1030 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 41.72 M/s | 17.260 s | ×1.00 | 17.15 s | 99 % | 1089 Mo | ×1.06 | 293138 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 39.66 M/s | 18.153 s | ×1.05 | 18.04 s | 99 % | 1089 Mo | ×1.06 | 293147 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 38.48 M/s | 18.713 s | ×1.08 | 18.50 s | 99 % | 1089 Mo | ×1.06 | 293146 / 0 | 0 / 0 | ✅ |
| Rust + Verus | 35.87 M/s | 20.075 s | ×1.16 | 19.93 s | 99 % | 1089 Mo | ×1.06 | 293147 / 0 | 0 / 0 | ✅ |

## ntt (n = 33 554 432)

Débit = papillons/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (192 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust (sans Creusot) | 82.35 M/s | 10.186 s | ×1.00 | 10.10 s | 99 % | 194 Mo | ×1.01 | 49328 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 80.10 M/s | 10.473 s | ×1.03 | 10.40 s | 99 % | 194 Mo | ×1.01 | 49329 / 0 | 0 / 0 | ✅ |
| Rust + Verus | 78.53 M/s | 10.682 s | ×1.05 | 10.60 s | 99 % | 194 Mo | ×1.01 | 49327 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 62.57 M/s | 13.407 s | ×1.32 | 13.32 s | 99 % | 193 Mo | ×1.01 | 49321 / 0 | 0 / 0 | ✅ |

## ptrchase (n = 4 000 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (76 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust (sans Creusot) | 2.71 M/s | 11.077 s | ×1.00 | 11.02 s | 99 % | 78 Mo | ×1.03 | 19709 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 2.70 M/s | 11.118 s | ×1.00 | 11.00 s | 99 % | 78 Mo | ×1.03 | 19707 / 0 | 0 / 0 | ✅ |
| Rust + Verus | 2.63 M/s | 11.418 s | ×1.03 | 11.32 s | 99 % | 78 Mo | ×1.03 | 19708 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 2.45 M/s | 12.236 s | ×1.10 | 12.15 s | 99 % | 78 Mo | ×1.02 | 19699 / 0 | 0 / 0 | ✅ |

## sha256 (n = 2 000 000 000)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 251.99 M/s | 7.937 s | ×1.00 | 7.89 s | 99 % | 2 Mo | ×1760.00 | 165 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 194.80 M/s | 10.267 s | ×1.29 | 10.22 s | 100 % | 2 Mo | ×2020.00 | 173 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 190.67 M/s | 10.489 s | ×1.32 | 10.34 s | 99 % | 2 Mo | ×2024.00 | 175 / 0 | 0 / 0 | ✅ |
| Rust + Verus | 170.59 M/s | 11.724 s | ×1.48 | 11.56 s | 99 % | 2 Mo | ×2144.00 | 175 / 0 | 0 / 0 | ✅ |

## stream (n = 4 096)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (1 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust + Verus | 307.83 M/s | 13.952 s | ×1.00 | 13.71 s | 98 % | 3 Mo | ×3.02 | 433 / 3 | 8394440 / 0 | ✅ |
| Rust + Creusot | 306.75 M/s | 14.001 s | ×1.00 | 13.79 s | 98 % | 3 Mo | ×3.06 | 431 / 3 | 8394656 / 0 | ✅ |
| Rust (sans Creusot) | 289.59 M/s | 14.831 s | ×1.06 | 14.49 s | 98 % | 3 Mo | ×3.05 | 432 / 3 | 8394704 / 0 | ✅ |
| C natif (à la main) | 283.84 M/s | 15.131 s | ×1.08 | 14.81 s | 98 % | 3 Mo | ×2.75 | 421 / 2 | 8393288 / 0 | ✅ |

## strsort (n = 8 000 000)

Débit = chaînes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (252 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 1.32 M/s | 6.071 s | ×1.00 | 6.01 s | 99 % | 249 Mo | ×0.99 | 63633 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 1.24 M/s | 6.439 s | ×1.06 | 6.40 s | 99 % | 250 Mo | ×0.99 | 63642 / 0 | 0 / 0 | ✅ |
| Rust + Verus | 1.24 M/s | 6.442 s | ×1.06 | 6.39 s | 99 % | 250 Mo | ×0.99 | 63643 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 1.16 M/s | 6.920 s | ×1.14 | 6.83 s | 99 % | 250 Mo | ×0.99 | 63642 / 0 | 0 / 0 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| Rust + Creusot | ×1.05 | ×1.05 | ×1.03 | 12 |
| C natif (à la main) | ×1.06 | ×1.06 | ×1.00 | 12 |
| Rust (sans Creusot) | ×1.07 | ×1.07 | ×1.03 | 12 |
| Rust + Verus | ×1.08 | ×1.08 | ×1.04 | 12 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 11 ms | 2.0 Mo |
| Rust (sans Creusot) | 4 ms | 2.0 Mo |
| C natif (à la main) | 5 ms | 1.7 Mo |
| Rust + Verus | 5 ms | 2.0 Mo |

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
