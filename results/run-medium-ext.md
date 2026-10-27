# Résultats — preset `medium`

_2026-10-27 05:12:54 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v165 · médiane de 3 exécutions_

## alloc (n = 20 000 000)

Débit = allocations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (42 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 5.79 M/s | 3.453 s | ×1.00 | 3.43 s | 99 % | 48 Mo | ×1.14 | 11945 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 5.66 M/s | 3.536 s | ×1.02 | 3.52 s | 99 % | 49 Mo | ×1.17 | 12245 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 5.62 M/s | 3.559 s | ×1.03 | 3.56 s | 100 % | 54 Mo | ×1.31 | 13719 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 5.58 M/s | 3.586 s | ×1.04 | 3.52 s | 98 % | 51 Mo | ×1.23 | 12801 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 5.43 M/s | 3.682 s | ×1.07 | 3.65 s | 99 % | 54 Mo | ×1.29 | 13414 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 2.43 M/s | 8.223 s | ×2.38 | 8.26 s | 99 % | 389 Mo | ×9.33 | 95546 / 0 | 0 / 0 | ✅ |

## bfs (n = 4 000 000)

Débit = arêtes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (275 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust + Creusot | 59.02 M/s | 3.253 s | ×1.00 | 3.24 s | 99 % | 292 Mo | ×1.06 | 78304 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 59.00 M/s | 3.254 s | ×1.00 | 3.21 s | 98 % | 291 Mo | ×1.06 | 78293 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 55.35 M/s | 3.469 s | ×1.07 | 3.44 s | 99 % | 292 Mo | ×1.06 | 78301 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 54.00 M/s | 3.556 s | ×1.09 | 3.51 s | 99 % | 291 Mo | ×1.06 | 78292 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 53.58 M/s | 3.584 s | ×1.10 | 3.55 s | 99 % | 292 Mo | ×1.06 | 78357 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 32.88 M/s | 5.839 s | ×1.80 | 5.83 s | 98 % | 333 Mo | ×1.21 | 80106 / 0 | 0 / 16 | ✅ |

## ntt (n = 8 388 608)

Débit = papillons/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (48 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust + Creusot | 103.98 M/s | 1.855 s | ×1.00 | 1.84 s | 99 % | 50 Mo | ×1.04 | 12466 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 99.20 M/s | 1.945 s | ×1.05 | 1.92 s | 99 % | 50 Mo | ×1.03 | 12455 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 98.64 M/s | 1.956 s | ×1.05 | 1.97 s | 100 % | 50 Mo | ×1.04 | 12521 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 87.87 M/s | 2.196 s | ×1.18 | 2.15 s | 98 % | 50 Mo | ×1.04 | 12464 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 75.88 M/s | 2.543 s | ×1.37 | 2.49 s | 97 % | 49 Mo | ×1.03 | 12456 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 45.70 M/s | 4.222 s | ×2.28 | 4.26 s | 99 % | 206 Mo | ×4.30 | 47585 / 0 | 0 / 8 | ✅ |

## ptrchase (n = 1 500 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (29 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust + Creusot | 4.54 M/s | 2.480 s | ×1.00 | 2.43 s | 98 % | 31 Mo | ×1.07 | 7501 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 4.22 M/s | 2.667 s | ×1.08 | 2.63 s | 98 % | 30 Mo | ×1.05 | 7491 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 4.18 M/s | 2.694 s | ×1.09 | 2.67 s | 99 % | 30 Mo | ×1.07 | 7555 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 3.95 M/s | 2.848 s | ×1.15 | 2.83 s | 99 % | 31 Mo | ×1.07 | 7499 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 3.81 M/s | 2.950 s | ×1.19 | 3.01 s | 99 % | 57 Mo | ×1.98 | 9301 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 3.61 M/s | 3.118 s | ×1.26 | 3.10 s | 99 % | 30 Mo | ×1.05 | 7491 / 0 | 0 / 0 | ✅ |

## sha256 (n = 200 000 000)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 224.32 M/s | 0.892 s | ×1.00 | 0.89 s | 99 % | 2 Mo | ×1724.00 | 166 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 217.51 M/s | 0.920 s | ×1.03 | 0.91 s | 99 % | 2 Mo | ×1764.00 | 168 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 196.18 M/s | 1.019 s | ×1.14 | 1.00 s | 98 % | 2 Mo | ×2048.00 | 232 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 161.41 M/s | 1.239 s | ×1.39 | 1.22 s | 98 % | 2 Mo | ×2024.00 | 175 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 121.20 M/s | 1.650 s | ×1.85 | 1.65 s | 99 % | 2 Mo | ×2004.00 | 176 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 62.95 M/s | 3.177 s | ×3.56 | 3.21 s | 99 % | 28 Mo | ×28636.00 | 1983 / 0 | 0 / 8 | ✅ |

## stream (n = 1 024)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (1 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust + Creusot | 240.82 M/s | 4.459 s | ×1.00 | 4.36 s | 97 % | 3 Mo | ×3.07 | 436 / 3 | 2103080 / 0 | ✅ |
| Rust (sans Creusot) | 224.88 M/s | 4.775 s | ×1.07 | 4.64 s | 97 % | 3 Mo | ×3.03 | 433 / 3 | 2103184 / 0 | ✅ |
| C natif (à la main) | 214.47 M/s | 5.006 s | ×1.12 | 4.94 s | 98 % | 3 Mo | ×2.75 | 422 / 2 | 2101832 / 0 | ✅ |
| Zig 0.17 (à la main) | 210.66 M/s | 5.097 s | ×1.14 | 4.93 s | 97 % | 3 Mo | ×2.86 | 488 / 2 | 2102296 / 0 | ✅ |
| F* (Pulse) → C | 199.42 M/s | 5.384 s | ×1.21 | 5.23 s | 97 % | 3 Mo | ×2.76 | 422 / 2 | 2101872 / 0 | ✅ |
| F* (Pulse) → F# | 29.71 M/s | 36.143 s | ×8.11 | 35.87 s | 99 % | 30 Mo | ×29.61 | 2323 / 316 | 2154152 / 144 | ✅ |

## strsort (n = 2 000 000)

Débit = chaînes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (63 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 1.73 M/s | 1.154 s | ×1.00 | 1.14 s | 98 % | 64 Mo | ×1.01 | 16100 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 1.68 M/s | 1.188 s | ×1.03 | 1.18 s | 99 % | 64 Mo | ×1.01 | 16035 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 1.43 M/s | 1.395 s | ×1.21 | 1.39 s | 99 % | 64 Mo | ×1.01 | 16035 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 1.28 M/s | 1.559 s | ×1.35 | 1.56 s | 100 % | 64 Mo | ×1.02 | 16043 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 1.27 M/s | 1.574 s | ×1.36 | 1.55 s | 98 % | 64 Mo | ×1.02 | 16045 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 756.75 k/s | 2.643 s | ×2.29 | 2.71 s | 100 % | 136 Mo | ×2.16 | 29609 / 0 | 8 / 8 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| Zig 0.17 (à la main) | ×1.08 | ×1.08 | ×1.04 | 7 |
| F* (Pulse) → C | ×1.08 | ×1.08 | ×1.02 | 7 |
| C natif (à la main) | ×1.12 | ×1.12 | ×1.00 | 7 |
| Rust + Creusot | ×1.15 | ×1.15 | ×1.06 | 7 |
| Rust (sans Creusot) | ×1.17 | ×1.17 | ×1.05 | 7 |
| F* (Pulse) → F# | ×2.58 | ×2.63 | ×4.32 | 7 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 6 ms | 2.0 Mo |
| F* (Pulse) → C | 5 ms | 1.7 Mo |
| F* (Pulse) → F# | 72 ms | 27.9 Mo |
| Rust (sans Creusot) | 5 ms | 2.1 Mo |
| C natif (à la main) | 6 ms | 1.7 Mo |
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
