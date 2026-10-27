# Résultats — preset `large`

_2026-10-27 05:22:14 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v165 · médiane de 2 exécutions_

## alloc (n = 100 000 000)

Débit = allocations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (209 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 6.14 M/s | 16.278 s | ×1.00 | 16.17 s | 99 % | 232 Mo | ×1.11 | 59119 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 5.96 M/s | 16.780 s | ×1.03 | 16.71 s | 99 % | 249 Mo | ×1.19 | 63304 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 5.94 M/s | 16.840 s | ×1.03 | 16.73 s | 99 % | 237 Mo | ×1.13 | 60306 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 5.60 M/s | 17.852 s | ×1.10 | 17.73 s | 99 % | 266 Mo | ×1.28 | 67920 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 5.44 M/s | 18.373 s | ×1.13 | 18.22 s | 99 % | 261 Mo | ×1.25 | 66358 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 2.39 M/s | 41.763 s | ×2.57 | 41.62 s | 99 % | 1015 Mo | ×4.87 | 255453 / 2 | 40 / 56 | ✅ |

## bfs (n = 15 000 000)

Débit = arêtes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (1030 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 37.83 M/s | 19.033 s | ×1.00 | 18.86 s | 99 % | 1089 Mo | ×1.06 | 293202 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 36.74 M/s | 19.595 s | ×1.03 | 19.41 s | 99 % | 1089 Mo | ×1.06 | 293138 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 35.30 M/s | 20.395 s | ×1.07 | 20.25 s | 99 % | 1089 Mo | ×1.06 | 293146 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 34.49 M/s | 20.874 s | ×1.10 | 20.67 s | 99 % | 1089 Mo | ×1.06 | 293138 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 33.14 M/s | 21.724 s | ×1.14 | 21.60 s | 99 % | 1089 Mo | ×1.06 | 293148 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 22.57 M/s | 31.902 s | ×1.68 | 31.75 s | 99 % | 1173 Mo | ×1.14 | 294966 / 0 | 0 / 72 | ✅ |

## ntt (n = 33 554 432)

Débit = papillons/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (192 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 96.14 M/s | 8.725 s | ×1.00 | 8.66 s | 99 % | 194 Mo | ×1.01 | 49384 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 91.36 M/s | 9.182 s | ×1.05 | 9.09 s | 99 % | 194 Mo | ×1.01 | 49328 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 87.44 M/s | 9.593 s | ×1.10 | 9.53 s | 99 % | 194 Mo | ×1.01 | 49331 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 82.13 M/s | 10.214 s | ×1.17 | 10.15 s | 99 % | 194 Mo | ×1.01 | 49320 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 72.97 M/s | 11.496 s | ×1.32 | 11.38 s | 99 % | 193 Mo | ×1.01 | 49321 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 45.21 M/s | 18.554 s | ×2.13 | 18.48 s | 99 % | 350 Mo | ×1.82 | 84450 / 0 | 0 / 44 | ✅ |

## ptrchase (n = 4 000 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (76 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 2.67 M/s | 11.224 s | ×1.00 | 11.14 s | 99 % | 78 Mo | ×1.02 | 19700 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 2.67 M/s | 11.237 s | ×1.00 | 11.12 s | 99 % | 78 Mo | ×1.03 | 19708 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 2.64 M/s | 11.357 s | ×1.01 | 11.25 s | 99 % | 78 Mo | ×1.02 | 19764 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 2.47 M/s | 12.154 s | ×1.08 | 12.06 s | 99 % | 78 Mo | ×1.03 | 19708 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 2.42 M/s | 12.391 s | ×1.10 | 12.39 s | 99 % | 105 Mo | ×1.37 | 21522 / 0 | 0 / 16 | ✅ |
| F* (Pulse) → C | 2.26 M/s | 13.267 s | ×1.18 | 13.16 s | 99 % | 78 Mo | ×1.02 | 19699 / 0 | 0 / 0 | ✅ |

## sha256 (n = 2 000 000 000)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 214.69 M/s | 9.316 s | ×1.00 | 9.21 s | 99 % | 2 Mo | ×1788.00 | 165 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 214.54 M/s | 9.322 s | ×1.00 | 9.26 s | 99 % | 2 Mo | ×1872.00 | 166 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 207.30 M/s | 9.648 s | ×1.04 | 9.57 s | 99 % | 2 Mo | ×2048.00 | 232 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 163.88 M/s | 12.204 s | ×1.31 | 12.13 s | 99 % | 2 Mo | ×2090.00 | 174 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 120.46 M/s | 16.604 s | ×1.78 | 16.44 s | 99 % | 2 Mo | ×2138.00 | 176 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 108.81 M/s | 18.380 s | ×1.97 | 18.32 s | 99 % | 28 Mo | ×28772.00 | 1986 / 0 | 0 / 44 | ✅ |

## stream (n = 4 096)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (1 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust + Creusot | 180.57 M/s | 23.786 s | ×1.00 | 23.11 s | 97 % | 3 Mo | ×3.06 | 435 / 3 | 8394548 / 4 | ✅ |
| Rust (sans Creusot) | 170.01 M/s | 25.263 s | ×1.06 | 24.59 s | 97 % | 3 Mo | ×3.04 | 433 / 3 | 8394672 / 0 | ✅ |
| C natif (à la main) | 167.97 M/s | 25.570 s | ×1.08 | 24.86 s | 97 % | 3 Mo | ×2.75 | 422 / 2 | 8393288 / 0 | ✅ |
| F* (Pulse) → C | 156.99 M/s | 27.358 s | ×1.15 | 26.71 s | 98 % | 3 Mo | ×2.76 | 421 / 2 | 8393328 / 0 | ✅ |
| Zig 0.17 (à la main) | 156.94 M/s | 27.366 s | ×1.15 | 26.64 s | 97 % | 3 Mo | ×2.86 | 488 / 2 | 8393752 / 0 | ✅ |
| F* (Pulse) → F# | 29.14 M/s | 147.388 s | ×6.20 | 146.18 s | 99 % | 30 Mo | ×29.57 | 2320 / 316 | 8445608 / 140 | ✅ |

## strsort (n = 8 000 000)

Débit = chaînes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (252 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 1.27 M/s | 6.277 s | ×1.00 | 6.22 s | 99 % | 250 Mo | ×0.99 | 63698 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 1.27 M/s | 6.296 s | ×1.00 | 6.26 s | 99 % | 249 Mo | ×0.99 | 63632 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 1.08 M/s | 7.441 s | ×1.19 | 7.40 s | 99 % | 250 Mo | ×0.99 | 63642 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 975.06 k/s | 8.205 s | ×1.31 | 8.15 s | 99 % | 249 Mo | ×0.99 | 63634 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 883.36 k/s | 9.056 s | ×1.44 | 9.00 s | 99 % | 250 Mo | ×0.99 | 63645 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 530.38 k/s | 15.084 s | ×2.40 | 15.06 s | 99 % | 406 Mo | ×1.61 | 98768 / 2 | 40 / 44 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| Zig 0.17 (à la main) | ×1.03 | ×1.03 | ×1.03 | 7 |
| C natif (à la main) | ×1.06 | ×1.06 | ×1.01 | 7 |
| Rust (sans Creusot) | ×1.10 | ×1.10 | ×1.05 | 7 |
| F* (Pulse) → C | ×1.14 | ×1.14 | ×1.02 | 7 |
| Rust + Creusot | ×1.22 | ×1.22 | ×1.06 | 7 |
| F* (Pulse) → F# | ×2.26 | ×2.27 | ×3.17 | 7 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 5 ms | 2.1 Mo |
| F* (Pulse) → C | 5 ms | 1.7 Mo |
| F* (Pulse) → F# | 85 ms | 28.0 Mo |
| Rust (sans Creusot) | 5 ms | 2.0 Mo |
| C natif (à la main) | 5 ms | 1.8 Mo |
| Zig 0.17 (à la main) | 6 ms | 2.1 Mo |

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
