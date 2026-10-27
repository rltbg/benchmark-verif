# Résultats — preset `small`

_2026-10-27 05:11:16 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v165 · médiane de 3 exécutions_

## alloc (n = 2 000 000)

Débit = allocations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (4 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 6.32 M/s | 0.316 s | ×1.00 | 0.32 s | 99 % | 7 Mo | ×1.60 | 1448 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 6.07 M/s | 0.329 s | ×1.04 | 0.32 s | 96 % | 6 Mo | ×1.48 | 1363 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 6.05 M/s | 0.330 s | ×1.04 | 0.33 s | 99 % | 7 Mo | ×1.68 | 1539 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 5.83 M/s | 0.343 s | ×1.08 | 0.34 s | 98 % | 7 Mo | ×1.67 | 1449 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 5.44 M/s | 0.367 s | ×1.16 | 0.37 s | 98 % | 7 Mo | ×1.73 | 1513 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 2.34 M/s | 0.855 s | ×2.70 | 0.93 s | 98 % | 166 Mo | ×39.85 | 37598 / 0 | 8 / 8 | ✅ |

## bfs (n = 1 000 000)

Débit = arêtes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (69 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 102.68 M/s | 0.467 s | ×1.00 | 0.47 s | 100 % | 74 Mo | ×1.08 | 19763 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 90.75 M/s | 0.529 s | ×1.13 | 0.53 s | 100 % | 74 Mo | ×1.08 | 19698 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 88.30 M/s | 0.544 s | ×1.16 | 0.54 s | 99 % | 74 Mo | ×1.08 | 19697 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 86.04 M/s | 0.558 s | ×1.19 | 0.56 s | 100 % | 74 Mo | ×1.08 | 19711 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 85.93 M/s | 0.559 s | ×1.19 | 0.55 s | 98 % | 74 Mo | ×1.08 | 19709 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 37.20 M/s | 1.290 s | ×2.76 | 1.36 s | 100 % | 104 Mo | ×1.52 | 21499 / 0 | 0 / 0 | ✅ |

## ntt (n = 1 048 576)

Débit = papillons/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (6 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust (sans Creusot) | 183.63 M/s | 0.114 s | ×1.00 | 0.12 s | 98 % | 8 Mo | ×1.33 | 1712 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 183.07 M/s | 0.115 s | ×1.00 | 0.12 s | 98 % | 8 Mo | ×1.31 | 1768 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 170.29 M/s | 0.123 s | ×1.08 | 0.12 s | 97 % | 8 Mo | ×1.33 | 1712 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 163.00 M/s | 0.129 s | ×1.13 | 0.13 s | 99 % | 8 Mo | ×1.26 | 1703 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 134.38 M/s | 0.156 s | ×1.37 | 0.15 s | 95 % | 7 Mo | ×1.24 | 1704 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 50.56 M/s | 0.415 s | ×3.63 | 0.49 s | 98 % | 164 Mo | ×27.37 | 36848 / 0 | 0 / 0 | ✅ |

## ptrchase (n = 300 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (6 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 15.00 M/s | 0.150 s | ×1.00 | 0.15 s | 100 % | 8 Mo | ×1.32 | 1695 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 13.91 M/s | 0.162 s | ×1.08 | 0.16 s | 99 % | 8 Mo | ×1.34 | 1639 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 13.89 M/s | 0.162 s | ×1.08 | 0.16 s | 95 % | 8 Mo | ×1.35 | 1641 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 12.84 M/s | 0.175 s | ×1.17 | 0.18 s | 99 % | 7 Mo | ×1.23 | 1632 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 10.91 M/s | 0.206 s | ×1.38 | 0.21 s | 99 % | 7 Mo | ×1.27 | 1629 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 8.76 M/s | 0.257 s | ×1.71 | 0.32 s | 99 % | 34 Mo | ×5.91 | 3436 / 0 | 0 / 0 | ✅ |

## sha256 (n = 20 000 000)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 214.34 M/s | 0.093 s | ×1.00 | 0.10 s | 98 % | 2 Mo | ×1764.00 | 165 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 212.24 M/s | 0.094 s | ×1.01 | 0.09 s | 96 % | 2 Mo | ×1728.00 | 167 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 204.52 M/s | 0.098 s | ×1.05 | 0.10 s | 97 % | 2 Mo | ×2048.00 | 230 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 166.23 M/s | 0.120 s | ×1.29 | 0.12 s | 97 % | 2 Mo | ×2028.00 | 173 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 119.00 M/s | 0.168 s | ×1.80 | 0.16 s | 96 % | 2 Mo | ×2076.00 | 175 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 32.57 M/s | 0.614 s | ×6.58 | 0.68 s | 99 % | 28 Mo | ×28640.00 | 1970 / 0 | 0 / 0 | ✅ |

## stream (n = 256)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (1 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust + Creusot | 273.84 M/s | 0.980 s | ×1.00 | 0.97 s | 98 % | 3 Mo | ×3.07 | 436 / 3 | 530168 / 0 | ✅ |
| Rust (sans Creusot) | 270.83 M/s | 0.991 s | ×1.01 | 0.99 s | 98 % | 3 Mo | ×3.06 | 433 / 3 | 530312 / 0 | ✅ |
| C natif (à la main) | 256.63 M/s | 1.046 s | ×1.07 | 1.04 s | 98 % | 3 Mo | ×2.75 | 422 / 2 | 528960 / 0 | ✅ |
| Zig 0.17 (à la main) | 244.76 M/s | 1.097 s | ×1.12 | 1.09 s | 98 % | 3 Mo | ×2.86 | 489 / 2 | 529424 / 0 | ✅ |
| F* (Pulse) → C | 230.27 M/s | 1.166 s | ×1.19 | 1.16 s | 98 % | 3 Mo | ×2.76 | 421 / 2 | 529000 / 0 | ✅ |
| F* (Pulse) → F# | 28.97 M/s | 9.266 s | ×9.45 | 9.25 s | 99 % | 30 Mo | ×29.61 | 2325 / 316 | 581280 / 64 | ✅ |

## strsort (n = 300 000)

Débit = chaînes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (9 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 2.38 M/s | 0.126 s | ×1.00 | 0.13 s | 98 % | 11 Mo | ×1.15 | 2548 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 2.21 M/s | 0.136 s | ×1.08 | 0.14 s | 97 % | 11 Mo | ×1.19 | 2555 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 2.13 M/s | 0.141 s | ×1.12 | 0.15 s | 101 % | 11 Mo | ×1.18 | 2611 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 1.94 M/s | 0.155 s | ×1.23 | 0.16 s | 99 % | 11 Mo | ×1.20 | 2556 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 1.63 M/s | 0.184 s | ×1.46 | 0.19 s | 97 % | 11 Mo | ×1.15 | 2548 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 1.18 M/s | 0.255 s | ×2.03 | 0.32 s | 99 % | 44 Mo | ×4.71 | 6123 / 0 | 16 / 16 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| Zig 0.17 (à la main) | ×1.04 | ×1.05 | ×1.07 | 7 |
| Rust (sans Creusot) | ×1.10 | ×1.09 | ×1.09 | 7 |
| C natif (à la main) | ×1.11 | ×1.10 | ×1.00 | 7 |
| F* (Pulse) → C | ×1.18 | ×1.18 | ×1.02 | 7 |
| Rust + Creusot | ×1.20 | ×1.18 | ×1.10 | 7 |
| F* (Pulse) → F# | ×3.45 | ×3.86 | ×8.39 | 7 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 6 ms | 2.1 Mo |
| F* (Pulse) → C | 5 ms | 1.7 Mo |
| F* (Pulse) → F# | 78 ms | 27.9 Mo |
| Rust (sans Creusot) | 7 ms | 2.0 Mo |
| C natif (à la main) | 5 ms | 1.7 Mo |
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
