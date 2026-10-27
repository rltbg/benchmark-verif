# Résultats — preset `tiny`

_2026-10-27 05:11:13 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v165 · médiane de 3 exécutions_

## parmatmul (n = 256 — 4 threads)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (1 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 11.43 G/s | 0.001 s | ×1.00 | 0.01 s | 123 % | 2 Mo | ×3.31 | 297 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 8.40 G/s | 0.002 s | ×1.36 | 0.01 s | 127 % | 2 Mo | ×3.30 | 315 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 8.10 G/s | 0.002 s | ×1.41 | 0.01 s | 93 % | 4 Mo | ×4.83 | 555 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 7.01 G/s | 0.002 s | ×1.63 | 0.01 s | 160 % | 2 Mo | ×3.30 | 374 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 6.14 G/s | 0.003 s | ×1.86 | 0.01 s | 165 % | 2 Mo | ×2.96 | 361 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 1.16 G/s | 0.014 s | ×9.83 | 0.09 s | 111 % | 29 Mo | ×38.33 | 2119 / 0 | 16 / 8 | ✅ |

## parsha (n = 25 600 000 — 4 threads)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (2 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 843.59 M/s | 0.030 s | ×1.00 | 0.11 s | 331 % | 2 Mo | ×0.89 | 101 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 826.78 M/s | 0.031 s | ×1.02 | 0.12 s | 350 % | 2 Mo | ×0.91 | 106 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 575.29 M/s | 0.044 s | ×1.47 | 0.17 s | 345 % | 2 Mo | ×1.05 | 115 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 482.81 M/s | 0.053 s | ×1.75 | 0.20 s | 353 % | 3 Mo | ×1.47 | 360 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 441.13 M/s | 0.058 s | ×1.91 | 0.22 s | 362 % | 2 Mo | ×1.09 | 117 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 125.19 M/s | 0.204 s | ×6.74 | 0.84 s | 299 % | 28 Mo | ×14.03 | 1963 / 0 | 0 / 0 | ✅ |

## parsort (n = 500 000 — 4 threads)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (4 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 22.45 M/s | 0.022 s | ×1.00 | 0.07 s | 278 % | 6 Mo | ×1.50 | 1080 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 21.72 M/s | 0.023 s | ×1.03 | 0.06 s | 238 % | 7 Mo | ×1.74 | 1338 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 21.47 M/s | 0.023 s | ×1.05 | 0.07 s | 269 % | 6 Mo | ×1.58 | 1095 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 20.17 M/s | 0.025 s | ×1.11 | 0.07 s | 263 % | 6 Mo | ×1.57 | 1092 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 17.71 M/s | 0.028 s | ×1.27 | 0.08 s | 248 % | 6 Mo | ×1.50 | 1080 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 10.70 M/s | 0.047 s | ×2.10 | 0.18 s | 158 % | 32 Mo | ×8.36 | 2919 / 0 | 0 / 0 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| F* (Pulse) → C | ×1.09 | ×1.09 | ×1.04 | 3 |
| C natif (à la main) | ×1.23 | ×1.25 | ×1.00 | 3 |
| Zig 0.17 (à la main) | ×1.37 | ×1.21 | ×1.46 | 3 |
| Rust (sans Creusot) | ×1.39 | ×1.43 | ×1.11 | 3 |
| Rust + Creusot | ×1.40 | ×1.40 | ×1.13 | 3 |
| F* (Pulse) → F# | ×5.18 | ×7.31 | ×10.42 | 3 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 4 ms | 2.1 Mo |
| F* (Pulse) → C | 4 ms | 1.7 Mo |
| F* (Pulse) → F# | 90 ms | 27.8 Mo |
| Rust (sans Creusot) | 4 ms | 2.0 Mo |
| C natif (à la main) | 4 ms | 1.8 Mo |
| Zig 0.17 (à la main) | 4 ms | 2.1 Mo |

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
