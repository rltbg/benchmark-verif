# Résultats — preset `large`

_2026-10-27 05:49:36 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v165 · médiane de 2 exécutions_

## parmatmul (n = 6 144 — 4 threads)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (432 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 13.91 G/s | 16.670 s | ×1.00 | 64.47 s | 387 % | 435 Mo | ×1.01 | 110954 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 12.41 G/s | 18.687 s | ×1.12 | 72.89 s | 390 % | 434 Mo | ×1.01 | 147574 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 11.68 G/s | 19.855 s | ×1.19 | 77.14 s | 388 % | 434 Mo | ×1.00 | 147561 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 10.54 G/s | 21.996 s | ×1.32 | 85.41 s | 388 % | 434 Mo | ×1.00 | 110696 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 10.03 G/s | 23.131 s | ×1.39 | 90.38 s | 391 % | 434 Mo | ×1.01 | 141434 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 3.67 G/s | 63.139 s | ×3.79 | 244.95 s | 387 % | 460 Mo | ×1.07 | 112552 / 3 | 304 / 72 | ✅ |

## parsha (n = 5 120 000 000 — 4 threads)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (2 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 868.19 M/s | 5.897 s | ×1.00 | 23.04 s | 390 % | 2 Mo | ×0.95 | 102 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 831.46 M/s | 6.158 s | ×1.04 | 23.81 s | 386 % | 2 Mo | ×0.96 | 105 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 613.48 M/s | 8.346 s | ×1.42 | 32.65 s | 391 % | 2 Mo | ×1.10 | 116 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 514.27 M/s | 9.956 s | ×1.69 | 38.46 s | 386 % | 3 Mo | ×1.53 | 359 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 459.85 M/s | 11.134 s | ×1.89 | 43.82 s | 393 % | 2 Mo | ×1.11 | 117 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 457.70 M/s | 11.186 s | ×1.90 | 43.50 s | 386 % | 28 Mo | ×14.15 | 1971 / 0 | 0 / 32 | ✅ |

## parsort (n = 100 000 000 — 4 threads)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (763 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 19.13 M/s | 5.228 s | ×1.00 | 16.37 s | 313 % | 766 Mo | ×1.00 | 195674 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 17.33 M/s | 5.769 s | ×1.10 | 18.53 s | 321 % | 765 Mo | ×1.00 | 195417 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 16.80 M/s | 5.952 s | ×1.14 | 19.18 s | 322 % | 765 Mo | ×1.00 | 195428 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 16.37 M/s | 6.107 s | ×1.17 | 19.53 s | 320 % | 765 Mo | ×1.00 | 195431 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 15.93 M/s | 6.279 s | ×1.20 | 19.39 s | 309 % | 765 Mo | ×1.00 | 195416 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 11.74 M/s | 8.516 s | ×1.63 | 26.82 s | 311 % | 791 Mo | ×1.04 | 197292 / 0 | 128 / 12 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| C natif (à la main) | ×1.10 | ×1.11 | ×1.00 | 3 |
| F* (Pulse) → C | ×1.18 | ×1.17 | ×1.00 | 3 |
| Zig 0.17 (à la main) | ×1.19 | ×1.19 | ×1.17 | 3 |
| Rust (sans Creusot) | ×1.22 | ×1.23 | ×1.05 | 3 |
| Rust + Creusot | ×1.45 | ×1.47 | ×1.05 | 3 |
| F* (Pulse) → F# | ×2.27 | ×2.27 | ×2.54 | 3 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 5 ms | 2.1 Mo |
| F* (Pulse) → C | 4 ms | 1.7 Mo |
| F* (Pulse) → F# | 78 ms | 27.8 Mo |
| Rust (sans Creusot) | 4 ms | 2.0 Mo |
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
