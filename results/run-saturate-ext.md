# Résultats — preset `saturate`

_2026-10-27 09:06:48 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v165 · médiane de 1 exécutions_

## sort (n = 400 000 000)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (3052 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 5.44 M/s | 73.549 s | ×1.00 | 72.35 s | 98 % | 3054 Mo | ×1.00 | 781484 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 4.79 M/s | 83.443 s | ×1.13 | 82.54 s | 99 % | 3053 Mo | ×1.00 | 781418 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 4.68 M/s | 85.466 s | ×1.16 | 84.60 s | 99 % | 3054 Mo | ×1.00 | 781426 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 4.66 M/s | 85.837 s | ×1.17 | 84.82 s | 99 % | 3054 Mo | ×1.00 | 781428 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 4.47 M/s | 89.535 s | ×1.22 | 88.75 s | 99 % | 3053 Mo | ×1.00 | 781419 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 3.43 M/s | 116.494 s | ×1.58 | 115.39 s | 99 % | 3080 Mo | ×1.01 | 783270 / 3 | 64 / 72 | ✅ |

## sieve (n = 8 000 000 000)

Débit = entiers/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (7629 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust (sans Creusot) | 50.64 M/s | 157.990 s | ×1.00 | 156.49 s | 99 % | 7631 Mo | ×1.00 | 1953301 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 48.25 M/s | 165.816 s | ×1.05 | 163.49 s | 99 % | 7631 Mo | ×1.00 | 1953358 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 45.52 M/s | 175.760 s | ×1.11 | 173.70 s | 99 % | 7631 Mo | ×1.00 | 1953291 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 44.07 M/s | 181.547 s | ×1.15 | 179.47 s | 99 % | 7631 Mo | ×1.00 | 1953302 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 44.06 M/s | 181.569 s | ×1.15 | 179.04 s | 99 % | 7631 Mo | ×1.00 | 1953290 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | **failed** | | | | | | | | | |

## matmul (n = 7 000)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (561 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 2.13 G/s | 160.810 s | ×1.00 | 158.69 s | 99 % | 563 Mo | ×1.00 | 143784 / 1 | 16 / 0 | ✅ |
| Rust (sans Creusot) | 2.08 G/s | 164.668 s | ×1.02 | 162.82 s | 99 % | 563 Mo | ×1.00 | 191578 / 1 | 16 / 0 | ✅ |
| C natif (à la main) | 1.96 G/s | 175.447 s | ×1.09 | 173.39 s | 99 % | 562 Mo | ×1.00 | 191571 / 1 | 16 / 0 | ✅ |
| F* (Pulse) → C | 1.66 G/s | 206.863 s | ×1.29 | 203.91 s | 99 % | 562 Mo | ×1.00 | 143720 / 1 | 16 / 0 | ✅ |
| Rust + Creusot | 1.60 G/s | 214.845 s | ×1.34 | 211.79 s | 99 % | 563 Mo | ×1.00 | 191418 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 847.62 M/s | 404.664 s | ×2.52 | 399.86 s | 99 % | 589 Mo | ×1.05 | 145543 / 0 | 0 / 48 | ✅ |

## lines (n = 300 000 000)

Débit = lignes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (4308 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust (sans Creusot) | 18.78 M/s | 15.975 s | ×1.00 | 15.82 s | 99 % | 3975 Mo | ×0.92 | 1021249 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 15.46 M/s | 19.402 s | ×1.21 | 19.18 s | 99 % | 4309 Mo | ×1.00 | 1102896 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 15.27 M/s | 19.645 s | ×1.23 | 19.35 s | 98 % | 3974 Mo | ×0.92 | 1021306 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 13.13 M/s | 22.853 s | ×1.43 | 22.56 s | 99 % | 3974 Mo | ×0.92 | 1021242 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 10.29 M/s | 29.144 s | ×1.82 | 28.71 s | 98 % | 3975 Mo | ×0.92 | 1021251 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | **failed** | | | | | | | | | |

## hashmap (n = 60 000 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (2048 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 4.40 M/s | 40.928 s | ×1.00 | 40.54 s | 99 % | 1026 Mo | ×0.50 | 262378 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 4.23 M/s | 42.545 s | ×1.04 | 42.04 s | 99 % | 1026 Mo | ×0.50 | 524463 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 3.38 M/s | 53.297 s | ×1.30 | 52.65 s | 99 % | 1026 Mo | ×0.50 | 262312 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 3.31 M/s | 54.382 s | ×1.33 | 53.54 s | 98 % | 1025 Mo | ×0.50 | 524454 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 3.28 M/s | 54.957 s | ×1.34 | 53.99 s | 98 % | 1026 Mo | ×0.50 | 524467 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 2.49 M/s | 72.151 s | ×1.76 | 71.25 s | 99 % | 1053 Mo | ×0.51 | 264166 / 1 | 40 / 80 | ✅ |

## alloc (n = 1 600 000 000)

Débit = allocations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (3338 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust (sans Creusot) | 5.72 M/s | 279.818 s | ×1.00 | 276.77 s | 99 % | 3945 Mo | ×1.18 | 1009486 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 5.65 M/s | 283.273 s | ×1.01 | 280.23 s | 99 % | 3684 Mo | ×1.10 | 942961 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 5.56 M/s | 287.738 s | ×1.03 | 283.92 s | 99 % | 3754 Mo | ×1.12 | 960741 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 5.21 M/s | 307.145 s | ×1.10 | 303.90 s | 99 % | 4234 Mo | ×1.27 | 1083649 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 4.92 M/s | 325.326 s | ×1.16 | 321.63 s | 99 % | 4135 Mo | ×1.24 | 1058302 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 1.85 M/s | 865.080 s | ×3.09 | 856.30 s | 99 % | 10196 Mo | ×3.05 | 5824483 / 0 | 0 / 80 | ✅ |

## bfs (n = 100 000 000)

Débit = arêtes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (6866 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust (sans Creusot) | 22.47 M/s | 213.614 s | ×1.00 | 211.16 s | 99 % | 7250 Mo | ×1.06 | 1953303 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 22.28 M/s | 215.446 s | ×1.01 | 212.70 s | 99 % | 7250 Mo | ×1.06 | 1953361 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 20.84 M/s | 230.380 s | ×1.08 | 227.64 s | 99 % | 7249 Mo | ×1.06 | 1953296 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 18.91 M/s | 253.770 s | ×1.19 | 250.74 s | 99 % | 7250 Mo | ×1.06 | 1953304 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 18.31 M/s | 262.133 s | ×1.23 | 259.10 s | 99 % | 7249 Mo | ×1.06 | 1953294 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 12.48 M/s | 384.570 s | ×1.80 | 380.73 s | 99 % | 7658 Mo | ×1.12 | 1955234 / 1 | 16 / 56 | ✅ |

## ntt (n = 134 217 728)

Débit = papillons/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (768 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 70.20 M/s | 51.624 s | ×1.00 | 50.92 s | 99 % | 770 Mo | ×1.00 | 196841 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 68.08 M/s | 53.233 s | ×1.03 | 52.56 s | 99 % | 770 Mo | ×1.00 | 196783 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 60.62 M/s | 59.781 s | ×1.16 | 58.95 s | 99 % | 770 Mo | ×1.00 | 196785 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 55.13 M/s | 65.735 s | ×1.27 | 64.86 s | 99 % | 770 Mo | ×1.00 | 196774 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 53.08 M/s | 68.271 s | ×1.32 | 67.50 s | 99 % | 769 Mo | ×1.00 | 196775 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 29.72 M/s | 121.930 s | ×2.36 | 120.61 s | 99 % | 927 Mo | ×1.21 | 231918 / 0 | 0 / 80 | ✅ |

## ptrchase (n = 40 000 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (763 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust (sans Creusot) | 1.23 M/s | 243.752 s | ×1.00 | 240.91 s | 99 % | 765 Mo | ×1.00 | 195489 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 1.22 M/s | 246.118 s | ×1.01 | 243.21 s | 99 % | 765 Mo | ×1.00 | 195544 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 1.14 M/s | 263.367 s | ×1.08 | 259.86 s | 99 % | 764 Mo | ×1.00 | 195481 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 1.13 M/s | 265.153 s | ×1.09 | 262.24 s | 99 % | 765 Mo | ×1.00 | 195491 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 1.08 M/s | 276.854 s | ×1.14 | 273.91 s | 99 % | 764 Mo | ×1.00 | 195481 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 930.12 k/s | 322.539 s | ×1.32 | 318.73 s | 99 % | 792 Mo | ×1.04 | 197308 / 0 | 0 / 80 | ✅ |

## sha256 (n = 40 000 000 000)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 208.24 M/s | 192.084 s | ×1.00 | 190.36 s | 99 % | 2 Mo | ×1724.00 | 165 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 202.12 M/s | 197.904 s | ×1.03 | 195.74 s | 99 % | 2 Mo | ×1764.00 | 166 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 201.43 M/s | 198.584 s | ×1.03 | 195.96 s | 99 % | 2 Mo | ×2048.00 | 229 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 161.29 M/s | 248.003 s | ×1.29 | 245.32 s | 99 % | 2 Mo | ×2024.00 | 173 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 116.59 M/s | 343.086 s | ×1.79 | 338.70 s | 99 % | 2 Mo | ×2200.00 | 176 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 103.61 M/s | 386.061 s | ×2.01 | 381.39 s | 99 % | 29 Mo | ×29304.00 | 2020 / 0 | 0 / 88 | ✅ |

## stream (n = 8 192)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (1 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 133.04 M/s | 64.568 s | ×1.00 | 62.55 s | 97 % | 3 Mo | ×2.75 | 423 / 2 | 16781896 / 0 | ✅ |
| Rust (sans Creusot) | 129.57 M/s | 66.295 s | ×1.03 | 64.42 s | 97 % | 3 Mo | ×3.06 | 433 / 3 | 16783312 / 0 | ✅ |
| Zig 0.17 (à la main) | 127.50 M/s | 67.370 s | ×1.04 | 65.61 s | 97 % | 3 Mo | ×2.86 | 488 / 2 | 16782360 / 0 | ✅ |
| F* (Pulse) → C | 113.36 M/s | 75.773 s | ×1.17 | 73.77 s | 97 % | 3 Mo | ×2.76 | 421 / 2 | 16781936 / 0 | ✅ |
| Rust + Creusot | 93.62 M/s | 91.757 s | ×1.42 | 88.58 s | 96 % | 3 Mo | ×3.07 | 436 / 3 | 16783096 / 8 | ✅ |
| F* (Pulse) → F# | 28.00 M/s | 306.741 s | ×4.75 | 303.09 s | 99 % | 30 Mo | ×29.64 | 2328 / 316 | 16834216 / 128 | ✅ |

## strsort (n = 120 000 000)

Débit = chaînes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (3777 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 778.34 k/s | 154.173 s | ×1.00 | 152.05 s | 99 % | 3721 Mo | ×0.99 | 952383 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 651.42 k/s | 184.213 s | ×1.19 | 182.02 s | 99 % | 3721 Mo | ×0.99 | 952318 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 589.33 k/s | 203.622 s | ×1.32 | 200.76 s | 99 % | 3721 Mo | ×0.99 | 952328 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 531.76 k/s | 225.665 s | ×1.46 | 223.57 s | 99 % | 3721 Mo | ×0.99 | 952320 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 413.58 k/s | 290.150 s | ×1.88 | 287.14 s | 99 % | 3721 Mo | ×0.99 | 952330 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 279.62 k/s | 429.149 s | ×2.78 | 424.32 s | 99 % | 3878 Mo | ×1.03 | 987489 / 3 | 72 / 72 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| Zig 0.17 (à la main) | ×1.03 | ×1.03 | ×1.02 | 12 |
| Rust (sans Creusot) | ×1.07 | ×1.07 | ×1.03 | 12 |
| C natif (à la main) | ×1.15 | ×1.15 | ×1.00 | 12 |
| F* (Pulse) → C | ×1.20 | ×1.20 | ×1.02 | 12 |
| Rust + Creusot | ×1.35 | ×1.35 | ×1.04 | 12 |
| F* (Pulse) → F# | ×2.24 | ×2.25 | ×1.94 | 10 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 5 ms | 2.1 Mo |
| F* (Pulse) → C | 7 ms | 1.7 Mo |
| F* (Pulse) → F# | 91 ms | 27.9 Mo |
| Rust (sans Creusot) | 6 ms | 2.0 Mo |
| C natif (à la main) | 5 ms | 1.8 Mo |
| Zig 0.17 (à la main) | 5 ms | 2.1 Mo |

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
