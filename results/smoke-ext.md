# Résultats — preset `smoke`

_2026-10-27 05:08:55 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v165 · médiane de 1 exécutions_

## sort (n = 10 000)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 11.69 M/s | 0.001 s | ×1.00 | 0.00 s | 61 % | 2 Mo | ×28.01 | 177 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 10.64 M/s | 0.001 s | ×1.10 | 0.00 s | 60 % | 2 Mo | ×22.73 | 112 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 10.32 M/s | 0.001 s | ×1.13 | 0.00 s | 57 % | 2 Mo | ×22.89 | 112 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 9.46 M/s | 0.001 s | ×1.24 | 0.00 s | 60 % | 2 Mo | ×26.27 | 124 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 8.70 M/s | 0.001 s | ×1.34 | 0.00 s | 68 % | 2 Mo | ×27.24 | 121 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 2.01 M/s | 0.005 s | ×5.81 | 0.08 s | 94 % | 28 Mo | ×368.44 | 1915 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 128.12 k/s | 0.078 s | ×91.25 | 0.08 s | 92 % | 9 Mo | ×114.74 | 1230 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 22.35 k/s | 0.447 s | ×523.15 | 0.45 s | 99 % | 9 Mo | ×116.53 | 1242 / 0 | 0 / 0 | ✅ |

## sieve (n = 1 000 000)

Débit = entiers/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (1 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 281.71 M/s | 0.004 s | ×1.00 | 0.00 s | 72 % | 2 Mo | ×2.48 | 340 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 264.94 M/s | 0.004 s | ×1.06 | 0.01 s | 78 % | 3 Mo | ×2.83 | 402 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 256.46 M/s | 0.004 s | ×1.10 | 0.00 s | 72 % | 2 Mo | ×2.56 | 338 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 205.51 M/s | 0.005 s | ×1.37 | 0.01 s | 78 % | 3 Mo | ×2.99 | 348 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 204.39 M/s | 0.005 s | ×1.38 | 0.01 s | 81 % | 3 Mo | ×2.89 | 350 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 141.01 M/s | 0.007 s | ×2.00 | 0.08 s | 96 % | 29 Mo | ×30.54 | 2166 / 1 | 64 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 3.39 M/s | 0.295 s | ×83.02 | 0.30 s | 99 % | 16 Mo | ×16.69 | 3087 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 530.00 k/s | 1.887 s | ×531.53 | 1.89 s | 100 % | 16 Mo | ×16.94 | 3148 / 0 | 0 / 0 | ✅ |

## matmul (n = 64)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust (sans Creusot) | 4.75 G/s | 0.000 s | ×1.00 | 0.00 s | 42 % | 2 Mo | ×42.17 | 115 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 4.05 G/s | 0.000 s | ×1.17 | 0.00 s | 40 % | 2 Mo | ×37.00 | 105 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 3.40 G/s | 0.000 s | ×1.40 | 0.00 s | 50 % | 2 Mo | ×37.25 | 105 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 2.38 G/s | 0.000 s | ×1.99 | 0.00 s | 43 % | 2 Mo | ×45.58 | 169 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 1.37 G/s | 0.000 s | ×3.46 | 0.00 s | 48 % | 2 Mo | ×41.42 | 115 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 79.19 M/s | 0.003 s | ×59.92 | 0.08 s | 93 % | 28 Mo | ×593.67 | 1900 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 3.46 M/s | 0.076 s | ×1370.99 | 0.08 s | 98 % | 9 Mo | ×185.08 | 1254 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 606.38 k/s | 0.432 s | ×7825.27 | 0.43 s | 99 % | 9 Mo | ×198.33 | 1330 / 0 | 0 / 0 | ✅ |

## lines (n = 10 000)

Débit = lignes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (16 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 570.02 k/s | 0.018 s | ×1.00 | 0.02 s | 89 % | 18 Mo | ×1.10 | 4228 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 538.09 k/s | 0.019 s | ×1.06 | 0.02 s | 91 % | 17 Mo | ×1.04 | 7991 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 530.32 k/s | 0.019 s | ×1.07 | 0.02 s | 91 % | 16 Mo | ×1.00 | 7981 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 526.79 k/s | 0.019 s | ×1.08 | 0.02 s | 93 % | 17 Mo | ×1.03 | 8046 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 511.92 k/s | 0.020 s | ×1.11 | 0.09 s | 95 % | 44 Mo | ×2.72 | 6021 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 462.49 k/s | 0.022 s | ×1.23 | 0.02 s | 89 % | 17 Mo | ×1.04 | 7994 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 20.08 k/s | 0.498 s | ×28.39 | 0.50 s | 98 % | 26 Mo | ×1.60 | 5646 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 1.76 k/s | 5.680 s | ×323.77 | 5.66 s | 99 % | 27 Mo | ×1.66 | 5810 / 0 | 0 / 0 | ✅ |

## hashmap (n = 10 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 51.28 M/s | 0.001 s | ×1.00 | 0.00 s | 57 % | 2 Mo | ×4.41 | 286 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 50.46 M/s | 0.001 s | ×1.02 | 0.00 s | 48 % | 2 Mo | ×3.88 | 222 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 37.50 M/s | 0.001 s | ×1.37 | 0.00 s | 54 % | 2 Mo | ×4.91 | 358 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 37.11 M/s | 0.001 s | ×1.38 | 0.00 s | 63 % | 2 Mo | ×4.93 | 360 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 35.36 M/s | 0.001 s | ×1.45 | 0.00 s | 52 % | 2 Mo | ×3.73 | 352 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 5.74 M/s | 0.005 s | ×8.94 | 0.07 s | 94 % | 28 Mo | ×56.72 | 2019 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 614.51 k/s | 0.049 s | ×83.45 | 0.05 s | 97 % | 9 Mo | ×18.46 | 1376 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 69.83 k/s | 0.430 s | ×734.37 | 0.43 s | 100 % | 10 Mo | ×19.18 | 1395 / 0 | 0 / 0 | ✅ |

## alloc (n = 20 000)

Débit = allocations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 6.47 M/s | 0.003 s | ×1.00 | 0.00 s | 69 % | 2 Mo | ×38.07 | 109 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 5.80 M/s | 0.003 s | ×1.11 | 0.00 s | 73 % | 2 Mo | ×43.21 | 121 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 5.78 M/s | 0.003 s | ×1.12 | 0.00 s | 60 % | 2 Mo | ×46.90 | 175 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 5.07 M/s | 0.004 s | ×1.27 | 0.01 s | 74 % | 2 Mo | ×38.32 | 111 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 3.96 M/s | 0.005 s | ×1.63 | 0.01 s | 82 % | 2 Mo | ×46.47 | 124 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 1.77 M/s | 0.011 s | ×3.65 | 0.08 s | 96 % | 31 Mo | ×679.01 | 2699 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 32.57 k/s | 0.614 s | ×198.56 | 0.61 s | 99 % | 10 Mo | ×220.68 | 1582 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 4.92 k/s | 4.064 s | ×1314.30 | 4.03 s | 99 % | 11 Mo | ×235.17 | 1694 / 0 | 0 / 0 | ✅ |

## bfs (n = 20 000)

Débit = arêtes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (1 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 186.22 M/s | 0.005 s | ×1.00 | 0.01 s | 76 % | 3 Mo | ×2.33 | 527 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 180.49 M/s | 0.005 s | ×1.03 | 0.01 s | 78 % | 3 Mo | ×2.38 | 475 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 174.49 M/s | 0.006 s | ×1.07 | 0.01 s | 79 % | 3 Mo | ×2.14 | 464 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 166.04 M/s | 0.006 s | ×1.12 | 0.01 s | 75 % | 3 Mo | ×2.43 | 477 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 145.46 M/s | 0.007 s | ×1.28 | 0.01 s | 74 % | 3 Mo | ×2.13 | 463 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 76.19 M/s | 0.013 s | ×2.44 | 0.08 s | 98 % | 29 Mo | ×21.31 | 2279 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 704.71 k/s | 1.362 s | ×264.25 | 1.35 s | 98 % | 24 Mo | ×17.29 | 5102 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 86.70 k/s | 11.073 s | ×2147.87 | 10.96 s | 99 % | 24 Mo | ×17.72 | 5206 / 0 | 0 / 0 | ✅ |

## ntt (n = 4 096)

Débit = papillons/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust (sans Creusot) | 228.83 M/s | 0.000 s | ×1.00 | 0.00 s | 49 % | 2 Mo | ×83.50 | 108 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 225.98 M/s | 0.000 s | ×1.01 | 0.00 s | 39 % | 2 Mo | ×89.17 | 108 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 181.55 M/s | 0.000 s | ×1.26 | 0.00 s | 53 % | 2 Mo | ×91.17 | 162 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 149.86 M/s | 0.000 s | ×1.53 | 0.00 s | 48 % | 2 Mo | ×74.67 | 97 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 123.28 M/s | 0.000 s | ×1.86 | 0.00 s | 51 % | 2 Mo | ×76.17 | 100 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 7.25 M/s | 0.007 s | ×31.57 | 0.08 s | 97 % | 29 Mo | ×1225.33 | 2142 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 661.47 k/s | 0.074 s | ×345.94 | 0.08 s | 96 % | 9 Mo | ×395.33 | 1375 / 2 | 16 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 69.37 k/s | 0.709 s | ×3298.69 | 0.71 s | 99 % | 9 Mo | ×400.83 | 1391 / 0 | 0 / 0 | ✅ |

## parmatmul (n = 64 — 4 threads)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 1.03 G/s | 0.000 s | ×1.00 | 0.00 s | 59 % | 2 Mo | ×39.42 | 112 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 877.18 M/s | 0.000 s | ×1.18 | 0.00 s | 52 % | 2 Mo | ×39.75 | 111 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 676.47 M/s | 0.000 s | ×1.52 | 0.00 s | 65 % | 2 Mo | ×45.58 | 130 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 511.43 M/s | 0.001 s | ×2.02 | 0.00 s | 61 % | 2 Mo | ×44.83 | 126 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 392.07 M/s | 0.001 s | ×2.63 | 0.00 s | 62 % | 3 Mo | ×61.33 | 371 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 35.16 M/s | 0.007 s | ×29.31 | 0.08 s | 97 % | 28 Mo | ×594.67 | 1935 / 3 | 528 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 7.13 M/s | 0.037 s | ×144.61 | 0.10 s | 231 % | 15 Mo | ×315.67 | 2883 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 1.29 M/s | 0.204 s | ×800.51 | 0.47 s | 223 % | 16 Mo | ×340.92 | 3055 / 0 | 24 / 0 | ✅ |

## parsha (n = 25 600 — 4 threads)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (2 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 89.23 M/s | 0.000 s | ×1.00 | 0.00 s | 71 % | 2 Mo | ×0.86 | 103 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 74.67 M/s | 0.000 s | ×1.20 | 0.00 s | 65 % | 2 Mo | ×1.05 | 114 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 72.98 M/s | 0.000 s | ×1.22 | 0.00 s | 59 % | 2 Mo | ×0.87 | 104 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 43.12 M/s | 0.001 s | ×2.07 | 0.00 s | 73 % | 2 Mo | ×1.09 | 118 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 32.13 M/s | 0.001 s | ×2.78 | 0.00 s | 73 % | 3 Mo | ×1.38 | 359 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 2.47 M/s | 0.010 s | ×36.07 | 0.08 s | 101 % | 28 Mo | ×14.00 | 1952 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 756.27 k/s | 0.034 s | ×117.99 | 0.09 s | 218 % | 16 Mo | ×7.78 | 3002 / 1 | 8 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 211.63 k/s | 0.121 s | ×421.63 | 0.39 s | 304 % | 16 Mo | ×8.11 | 3123 / 0 | 0 / 0 | ✅ |

## parsort (n = 10 000 — 4 threads)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust + Creusot | 15.69 M/s | 0.001 s | ×1.00 | 0.00 s | 87 % | 2 Mo | ×27.96 | 137 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 14.56 M/s | 0.001 s | ×1.08 | 0.00 s | 76 % | 2 Mo | ×24.42 | 120 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 12.54 M/s | 0.001 s | ×1.25 | 0.00 s | 80 % | 2 Mo | ×27.55 | 135 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 8.46 M/s | 0.001 s | ×1.85 | 0.00 s | 85 % | 3 Mo | ×37.68 | 377 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 2.30 M/s | 0.004 s | ×6.82 | 0.01 s | 93 % | 2 Mo | ×23.40 | 119 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 918.99 k/s | 0.011 s | ×17.07 | 0.09 s | 99 % | 29 Mo | ×373.76 | 1957 / 1 | 256 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 221.00 k/s | 0.045 s | ×70.98 | 0.09 s | 179 % | 15 Mo | ×192.92 | 3283 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 38.85 k/s | 0.257 s | ×403.79 | 0.49 s | 184 % | 16 Mo | ×205.16 | 3469 / 0 | 0 / 0 | ✅ |

## ptrchase (n = 10 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 38.30 M/s | 0.002 s | ×1.00 | 0.00 s | 71 % | 2 Mo | ×11.86 | 205 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 34.41 M/s | 0.002 s | ×1.11 | 0.00 s | 68 % | 2 Mo | ×10.92 | 151 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 33.24 M/s | 0.002 s | ×1.15 | 0.00 s | 73 % | 2 Mo | ×11.20 | 155 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 29.36 M/s | 0.003 s | ×1.30 | 0.00 s | 69 % | 2 Mo | ×9.75 | 142 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 24.15 M/s | 0.003 s | ×1.59 | 0.00 s | 70 % | 2 Mo | ×9.83 | 141 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 8.02 M/s | 0.009 s | ×4.77 | 0.08 s | 98 % | 28 Mo | ×146.80 | 1944 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 355.44 k/s | 0.211 s | ×107.76 | 0.21 s | 98 % | 9 Mo | ×48.82 | 1382 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 44.12 k/s | 1.700 s | ×868.15 | 1.69 s | 99 % | 9 Mo | ×49.64 | 1424 / 0 | 0 / 0 | ✅ |

## sha256 (n = 100 000)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 216.10 M/s | 0.000 s | ×1.00 | 0.00 s | 44 % | 2 Mo | ×1664.00 | 94 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 210.21 M/s | 0.000 s | ×1.03 | 0.00 s | 53 % | 2 Mo | ×1764.00 | 92 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 201.98 M/s | 0.000 s | ×1.07 | 0.00 s | 58 % | 2 Mo | ×2048.00 | 159 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 163.58 M/s | 0.001 s | ×1.32 | 0.00 s | 52 % | 2 Mo | ×2120.00 | 103 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 118.98 M/s | 0.001 s | ×1.82 | 0.00 s | 60 % | 2 Mo | ×2176.00 | 105 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 10.42 M/s | 0.010 s | ×20.74 | 0.08 s | 96 % | 28 Mo | ×28492.00 | 1900 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 429.35 k/s | 0.233 s | ×503.33 | 0.23 s | 98 % | 9 Mo | ×8792.00 | 1213 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 86.50 k/s | 1.156 s | ×2498.33 | 1.16 s | 100 % | 9 Mo | ×9312.00 | 1292 / 0 | 0 / 0 | ✅ |

## stream (n = 1)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (1 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust (sans Creusot) | 185.80 M/s | 0.006 s | ×1.00 | 0.01 s | 61 % | 3 Mo | ×2.96 | 361 / 2 | 4576 / 0 | ✅ |
| C natif (à la main) | 184.78 M/s | 0.006 s | ×1.01 | 0.01 s | 53 % | 3 Mo | ×2.59 | 351 / 1 | 3224 / 0 | ✅ |
| F* (Pulse) → C | 157.27 M/s | 0.007 s | ×1.18 | 0.01 s | 52 % | 3 Mo | ×2.72 | 349 / 1 | 3264 / 0 | ✅ |
| Zig 0.17 (à la main) | 155.50 M/s | 0.007 s | ×1.19 | 0.01 s | 51 % | 3 Mo | ×2.66 | 416 / 1 | 3688 / 0 | ✅ |
| Rust + Creusot | 138.99 M/s | 0.008 s | ×1.34 | 0.01 s | 54 % | 3 Mo | ×2.96 | 364 / 2 | 4440 / 0 | ✅ |
| F* (Pulse) → F# | 7.73 M/s | 0.136 s | ×24.03 | 0.19 s | 84 % | 29 Mo | ×29.19 | 2140 / 313 | 58392 / 56 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 494.38 k/s | 2.121 s | ×375.82 | 2.09 s | 98 % | 17 Mo | ×16.88 | 3327 / 51 | 10648 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 82.32 k/s | 12.738 s | ×2257.04 | 12.64 s | 99 % | 21 Mo | ×20.57 | 4273 / 59 | 11088 / 0 | ✅ |

## strsort (n = 10 000)

Débit = chaînes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 3.50 M/s | 0.003 s | ×1.00 | 0.00 s | 73 % | 2 Mo | ×7.58 | 236 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 3.21 M/s | 0.003 s | ×1.09 | 0.00 s | 70 % | 2 Mo | ×7.06 | 182 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 3.15 M/s | 0.003 s | ×1.11 | 0.00 s | 72 % | 2 Mo | ×5.91 | 172 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 2.95 M/s | 0.003 s | ×1.19 | 0.00 s | 71 % | 2 Mo | ×6.36 | 173 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 1.27 M/s | 0.008 s | ×2.77 | 0.01 s | 83 % | 2 Mo | ×7.58 | 183 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 754.26 k/s | 0.013 s | ×4.64 | 0.09 s | 95 % | 29 Mo | ×91.04 | 2036 / 0 | 8 / 8 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 24.08 k/s | 0.415 s | ×145.38 | 0.41 s | 98 % | 10 Mo | ×33.00 | 1657 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 2.60 k/s | 3.847 s | ×1347.08 | 3.80 s | 99 % | 11 Mo | ×35.03 | 1788 / 0 | 0 / 0 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| F* (Pulse) → C | ×1.18 | ×1.10 | ×1.03 | 15 |
| Rust (sans Creusot) | ×1.20 | ×1.18 | ×1.17 | 15 |
| C natif (à la main) | ×1.30 | ×1.09 | ×1.01 | 15 |
| Zig 0.17 (à la main) | ×1.31 | ×1.13 | ×1.25 | 15 |
| Rust + Creusot | ×1.50 | ×1.35 | ×1.19 | 15 |
| F* (Pulse) → F# | ×9.39 | ×28.11 | ×13.25 | 15 |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | ×163.55 | ×70.82 | ×5.65 | 15 |
| F* (Pulse) → OCaml (runtime livré) | ×1102.63 | ×471.45 | ×5.96 | 15 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 5 ms | 2.0 Mo |
| F* (Pulse) → C | 4 ms | 1.7 Mo |
| F* (Pulse) → F# | 85 ms | 27.8 Mo |
| F* (Pulse) → OCaml (runtime livré) | 472 ms | 8.8 Mo |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 73 ms | 8.7 Mo |
| Rust (sans Creusot) | 4 ms | 2.0 Mo |
| C natif (à la main) | 4 ms | 1.7 Mo |
| Zig 0.17 (à la main) | 4 ms | 2.1 Mo |

## Empreinte disque et coût de construction

| implémentation | exécutable | runtime/deps requis | compilation | vérification |
|---|--:|--:|--:|--:|
| Rust + Creusot | 0.46 Mo | 0.0 Mo | 0.2 s | 16.3 s |
| F* (Pulse) → C | 0.05 Mo | 0.0 Mo | 3.5 s | 428.5 s |
| F* (Pulse) → OCaml (runtime livré) | 5.83 Mo | 0.0 Mo | 5.3 s | 428.5 s |
| F* (Pulse) → F# | 0.09 Mo | 72.4 Mo | 9.6 s | 428.5 s |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 5.74 Mo | 0.0 Mo | 7.9 s | 428.5 s |
| Rust (sans Creusot) | 0.46 Mo | 0.0 Mo | 0.1 s | — |
| C natif (à la main) | 0.04 Mo | 0.0 Mo | 0.8 s | — |
| Zig 0.17 (à la main) | 3.71 Mo | 0.0 Mo | 18.1 s | — |
