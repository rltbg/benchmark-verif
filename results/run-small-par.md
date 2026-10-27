# Résultats — preset `small`

_2026-10-27 05:12:38 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v165 · médiane de 3 exécutions_

## parmatmul (n = 1 024 — 4 threads)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (12 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 22.16 G/s | 0.048 s | ×1.00 | 0.15 s | 299 % | 15 Mo | ×1.25 | 3434 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 17.89 G/s | 0.060 s | ×1.24 | 0.19 s | 305 % | 14 Mo | ×1.15 | 3177 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 16.19 G/s | 0.066 s | ×1.37 | 0.22 s | 310 % | 14 Mo | ×1.17 | 3194 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 14.09 G/s | 0.076 s | ×1.57 | 0.27 s | 340 % | 14 Mo | ×1.16 | 4213 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 12.42 G/s | 0.086 s | ×1.78 | 0.30 s | 334 % | 14 Mo | ×1.16 | 4201 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 3.36 G/s | 0.320 s | ×6.60 | 1.16 s | 299 % | 41 Mo | ×3.38 | 5055 / 0 | 0 / 0 | ✅ |

## parsha (n = 256 000 000 — 4 threads)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (2 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 879.60 M/s | 0.291 s | ×1.00 | 1.11 s | 379 % | 2 Mo | ×0.95 | 102 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 821.89 M/s | 0.311 s | ×1.07 | 1.18 s | 374 % | 2 Mo | ×0.96 | 104 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 618.17 M/s | 0.414 s | ×1.42 | 1.62 s | 388 % | 2 Mo | ×1.10 | 114 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 527.94 M/s | 0.485 s | ×1.67 | 1.88 s | 385 % | 3 Mo | ×1.53 | 359 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 447.72 M/s | 0.572 s | ×1.96 | 2.20 s | 382 % | 2 Mo | ×1.10 | 117 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 365.61 M/s | 0.700 s | ×2.41 | 2.70 s | 346 % | 28 Mo | ×14.07 | 1971 / 0 | 0 / 0 | ✅ |

## parsort (n = 4 000 000 — 4 threads)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (31 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 22.57 M/s | 0.177 s | ×1.00 | 0.53 s | 294 % | 33 Mo | ×1.10 | 8174 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 19.88 M/s | 0.201 s | ×1.14 | 0.63 s | 304 % | 33 Mo | ×1.07 | 7929 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 19.82 M/s | 0.202 s | ×1.14 | 0.62 s | 302 % | 32 Mo | ×1.06 | 7917 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 19.42 M/s | 0.206 s | ×1.16 | 0.65 s | 311 % | 33 Mo | ×1.07 | 7930 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 17.61 M/s | 0.227 s | ×1.28 | 0.65 s | 283 % | 32 Mo | ×1.06 | 7916 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 12.71 M/s | 0.315 s | ×1.78 | 0.99 s | 254 % | 59 Mo | ×1.92 | 9780 / 0 | 0 / 0 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| Zig 0.17 (à la main) | ×1.19 | ×1.19 | ×1.22 | 3 |
| F* (Pulse) → C | ×1.19 | ×1.18 | ×1.00 | 3 |
| C natif (à la main) | ×1.27 | ×1.31 | ×1.00 | 3 |
| Rust (sans Creusot) | ×1.36 | ×1.44 | ×1.06 | 3 |
| Rust + Creusot | ×1.46 | ×1.50 | ×1.06 | 3 |
| F* (Pulse) → F# | ×3.04 | ×3.25 | ×4.28 | 3 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 4 ms | 2.1 Mo |
| F* (Pulse) → C | 4 ms | 1.7 Mo |
| F* (Pulse) → F# | 79 ms | 27.8 Mo |
| Rust (sans Creusot) | 4 ms | 2.1 Mo |
| C natif (à la main) | 4 ms | 1.7 Mo |
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
