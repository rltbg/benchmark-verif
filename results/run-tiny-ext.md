# Résultats — preset `tiny`

_2026-10-27 05:11:05 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v165 · médiane de 3 exécutions_

## alloc (n = 100 000)

Débit = allocations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 6.38 M/s | 0.016 s | ×1.00 | 0.02 s | 91 % | 2 Mo | ×8.75 | 229 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 6.05 M/s | 0.017 s | ×1.06 | 0.02 s | 88 % | 2 Mo | ×10.57 | 244 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 5.88 M/s | 0.017 s | ×1.09 | 0.02 s | 93 % | 2 Mo | ×9.41 | 239 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 5.82 M/s | 0.017 s | ×1.10 | 0.02 s | 91 % | 2 Mo | ×10.99 | 249 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 5.49 M/s | 0.018 s | ×1.16 | 0.02 s | 92 % | 2 Mo | ×10.65 | 294 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 2.22 M/s | 0.045 s | ×2.87 | 0.11 s | 94 % | 44 Mo | ×205.83 | 6009 / 0 | 8 / 8 | ✅ |

## bfs (n = 200 000)

Débit = arêtes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (14 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 145.16 M/s | 0.066 s | ×1.00 | 0.07 s | 102 % | 16 Mo | ×1.17 | 4071 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 144.46 M/s | 0.066 s | ×1.00 | 0.07 s | 101 % | 16 Mo | ×1.19 | 4136 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 141.26 M/s | 0.068 s | ×1.03 | 0.07 s | 97 % | 16 Mo | ×1.20 | 4084 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 141.11 M/s | 0.068 s | ×1.03 | 0.07 s | 97 % | 16 Mo | ×1.17 | 4073 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 133.73 M/s | 0.072 s | ×1.09 | 0.07 s | 97 % | 16 Mo | ×1.20 | 4083 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 69.40 M/s | 0.138 s | ×2.09 | 0.21 s | 98 % | 43 Mo | ×3.16 | 5874 / 0 | 0 / 0 | ✅ |

## ntt (n = 65 536)

Débit = papillons/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 233.06 M/s | 0.004 s | ×1.00 | 0.01 s | 80 % | 2 Mo | ×5.97 | 326 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 222.98 M/s | 0.005 s | ×1.05 | 0.01 s | 82 % | 2 Mo | ×6.26 | 271 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 207.85 M/s | 0.005 s | ×1.12 | 0.01 s | 79 % | 2 Mo | ×6.03 | 274 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 179.95 M/s | 0.006 s | ×1.30 | 0.01 s | 80 % | 2 Mo | ×4.89 | 262 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 166.32 M/s | 0.006 s | ×1.40 | 0.01 s | 71 % | 2 Mo | ×5.18 | 262 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 37.05 M/s | 0.028 s | ×6.29 | 0.10 s | 95 % | 43 Mo | ×115.41 | 5923 / 0 | 0 / 0 | ✅ |

## ptrchase (n = 30 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (1 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust (sans Creusot) | 29.23 M/s | 0.008 s | ×1.00 | 0.01 s | 87 % | 2 Mo | ×4.29 | 322 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 28.77 M/s | 0.008 s | ×1.02 | 0.01 s | 87 % | 2 Mo | ×4.07 | 375 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 23.89 M/s | 0.009 s | ×1.22 | 0.01 s | 83 % | 2 Mo | ×3.36 | 313 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 23.86 M/s | 0.009 s | ×1.22 | 0.01 s | 99 % | 2 Mo | ×4.14 | 325 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 19.95 M/s | 0.011 s | ×1.46 | 0.01 s | 89 % | 2 Mo | ×3.61 | 312 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 10.99 M/s | 0.020 s | ×2.66 | 0.09 s | 98 % | 28 Mo | ×49.66 | 2113 / 0 | 0 / 0 | ✅ |

## sha256 (n = 1 000 000)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 214.57 M/s | 0.005 s | ×1.00 | 0.01 s | 82 % | 2 Mo | ×1764.00 | 166 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 206.74 M/s | 0.005 s | ×1.04 | 0.01 s | 83 % | 2 Mo | ×2048.00 | 230 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 165.17 M/s | 0.006 s | ×1.30 | 0.01 s | 82 % | 2 Mo | ×1744.00 | 165 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 161.92 M/s | 0.006 s | ×1.33 | 0.01 s | 84 % | 2 Mo | ×2012.00 | 174 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 120.50 M/s | 0.008 s | ×1.78 | 0.01 s | 88 % | 2 Mo | ×2084.00 | 176 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 26.83 M/s | 0.037 s | ×8.00 | 0.11 s | 89 % | 28 Mo | ×28640.00 | 1965 / 0 | 0 / 0 | ✅ |

## stream (n = 4)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (1 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust (sans Creusot) | 248.92 M/s | 0.017 s | ×1.00 | 0.02 s | 71 % | 3 Mo | ×3.06 | 433 / 3 | 14216 / 0 | ✅ |
| Rust + Creusot | 238.31 M/s | 0.018 s | ×1.04 | 0.02 s | 67 % | 3 Mo | ×3.07 | 436 / 3 | 14072 / 0 | ✅ |
| C natif (à la main) | 233.97 M/s | 0.018 s | ×1.06 | 0.02 s | 73 % | 3 Mo | ×2.75 | 423 / 2 | 12864 / 0 | ✅ |
| Zig 0.17 (à la main) | 220.17 M/s | 0.019 s | ×1.13 | 0.03 s | 93 % | 3 Mo | ×2.86 | 488 / 2 | 13320 / 0 | ✅ |
| F* (Pulse) → C | 162.20 M/s | 0.026 s | ×1.53 | 0.03 s | 79 % | 3 Mo | ×2.76 | 420 / 2 | 12904 / 0 | ✅ |
| F* (Pulse) → F# | 9.74 M/s | 0.431 s | ×25.55 | 0.49 s | 93 % | 29 Mo | ×29.09 | 2207 / 314 | 65152 / 64 | ✅ |

## strsort (n = 20 000)

Débit = chaînes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (1 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 3.54 M/s | 0.006 s | ×1.00 | 0.01 s | 83 % | 2 Mo | ×3.26 | 324 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 3.50 M/s | 0.006 s | ×1.01 | 0.01 s | 82 % | 2 Mo | ×3.70 | 387 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 3.04 M/s | 0.007 s | ×1.16 | 0.01 s | 84 % | 3 Mo | ×4.13 | 334 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 2.72 M/s | 0.007 s | ×1.30 | 0.01 s | 86 % | 3 Mo | ×4.00 | 337 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 2.30 M/s | 0.009 s | ×1.54 | 0.01 s | 86 % | 2 Mo | ×3.28 | 324 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 1.14 M/s | 0.018 s | ×3.10 | 0.09 s | 88 % | 29 Mo | ×46.12 | 2242 / 0 | 8 / 8 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| Zig 0.17 (à la main) | ×1.05 | ×1.08 | ×1.14 | 7 |
| C natif (à la main) | ×1.08 | ×1.05 | ×1.00 | 7 |
| Rust (sans Creusot) | ×1.08 | ×1.07 | ×1.19 | 7 |
| Rust + Creusot | ×1.22 | ×1.20 | ×1.18 | 7 |
| F* (Pulse) → C | ×1.32 | ×1.23 | ×1.03 | 7 |
| F* (Pulse) → F# | ×4.85 | ×9.98 | ×12.75 | 7 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 6 ms | 2.1 Mo |
| F* (Pulse) → C | 5 ms | 1.7 Mo |
| F* (Pulse) → F# | 89 ms | 27.9 Mo |
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
