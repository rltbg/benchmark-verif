# Résultats — preset `saturate`

_2026-10-27 08:18:05 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v165 · médiane de 1 exécutions_

## parmatmul (n = 12 288 — 4 threads)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (1728 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 9.35 G/s | 198.464 s | ×1.00 | 770.99 s | 388 % | 1731 Mo | ×1.00 | 442731 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 8.64 G/s | 214.630 s | ×1.08 | 839.16 s | 391 % | 1730 Mo | ×1.00 | 589941 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 8.36 G/s | 221.988 s | ×1.12 | 866.16 s | 390 % | 1730 Mo | ×1.00 | 589928 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 7.72 G/s | 240.244 s | ×1.21 | 933.87 s | 389 % | 1730 Mo | ×1.00 | 577658 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 7.22 G/s | 257.083 s | ×1.30 | 1002.97 s | 390 % | 1730 Mo | ×1.00 | 442471 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 3.18 G/s | 582.956 s | ×2.94 | 2282.05 s | 391 % | 1756 Mo | ×1.02 | 444371 / 1 | 64 / 72 | ✅ |

## parsha (n = 64 000 000 000 — 4 threads)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (2 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 849.54 M/s | 75.335 s | ×1.00 | 294.45 s | 391 % | 2 Mo | ×0.95 | 102 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 729.54 M/s | 87.726 s | ×1.16 | 342.24 s | 390 % | 2 Mo | ×0.96 | 105 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 592.78 M/s | 107.965 s | ×1.43 | 421.96 s | 391 % | 2 Mo | ×1.10 | 115 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 513.53 M/s | 124.627 s | ×1.65 | 481.14 s | 386 % | 3 Mo | ×1.53 | 359 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 450.01 M/s | 142.219 s | ×1.89 | 555.05 s | 390 % | 28 Mo | ×14.08 | 1968 / 0 | 0 / 72 | ✅ |
| Rust + Creusot | 409.62 M/s | 156.241 s | ×2.07 | 610.83 s | 391 % | 2 Mo | ×1.15 | 117 / 0 | 0 / 0 | ✅ |

## parsort (n = 1 000 000 000 — 4 threads)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (7629 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 15.63 M/s | 63.978 s | ×1.00 | 209.23 s | 327 % | 7632 Mo | ×1.00 | 1953484 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 14.17 M/s | 70.557 s | ×1.10 | 233.75 s | 331 % | 7631 Mo | ×1.00 | 1953230 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 14.00 M/s | 71.420 s | ×1.12 | 231.88 s | 325 % | 7632 Mo | ×1.00 | 1953243 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 13.28 M/s | 75.277 s | ×1.18 | 252.73 s | 336 % | 7632 Mo | ×1.00 | 1953240 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 10.47 M/s | 95.527 s | ×1.49 | 260.03 s | 272 % | 7631 Mo | ×1.00 | 1953228 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 9.09 M/s | 109.998 s | ×1.72 | 320.95 s | 291 % | 7658 Mo | ×1.00 | 1955180 / 0 | 0 / 64 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| C natif (à la main) | ×1.07 | ×1.08 | ×1.00 | 3 |
| Zig 0.17 (à la main) | ×1.18 | ×1.18 | ×1.17 | 3 |
| Rust (sans Creusot) | ×1.22 | ×1.24 | ×1.05 | 3 |
| F* (Pulse) → C | ×1.31 | ×1.23 | ×1.00 | 3 |
| Rust + Creusot | ×1.41 | ×1.41 | ×1.06 | 3 |
| F* (Pulse) → F# | ×2.12 | ×2.05 | ×2.47 | 3 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 5 ms | 2.0 Mo |
| F* (Pulse) → C | 5 ms | 1.7 Mo |
| F* (Pulse) → F# | 103 ms | 27.8 Mo |
| Rust (sans Creusot) | 4 ms | 2.0 Mo |
| C natif (à la main) | 4 ms | 1.7 Mo |
| Zig 0.17 (à la main) | 8 ms | 2.1 Mo |

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
