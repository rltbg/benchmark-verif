# Résultats — preset `overflow`

_2026-10-27 13:05:40 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v165 · médiane de 1 exécutions_

## lines (n = 300 000 000)

Débit = lignes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (4308 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 6.90 M/s | 43.494 s | ×1.00 | 35.03 s | 81 % | 2380 Mo | ×0.55 | 1906889 / 127259 | 8130296 / 0 | ✅ |
| Rust (sans Creusot) | 6.81 M/s | 44.068 s | ×1.01 | 34.08 s | 77 % | 2380 Mo | ×0.55 | 1906713 / 127386 | 8131440 / 0 | ✅ |
| Rust + Creusot | 5.86 M/s | 51.192 s | ×1.18 | 43.01 s | 84 % | 2383 Mo | ×0.55 | 1907393 / 126702 | 8104072 / 0 | ✅ |
| F* (Pulse) → C | 5.72 M/s | 52.467 s | ×1.21 | 42.37 s | 81 % | 2382 Mo | ×0.55 | 2873658 / 254956 | 8175168 / 0 | ✅ |
| C natif (à la main) | 5.55 M/s | 54.006 s | ×1.24 | 41.93 s | 78 % | 2380 Mo | ×0.55 | 1906712 / 127367 | 8129968 / 0 | ✅ |

## bfs (n = 100 000 000)

Débit = arêtes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (6866 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust + Creusot | **timeout** | | | | | | | | | |
| F* (Pulse) → C | **timeout** | | | | | | | | | |
| Rust (sans Creusot) | **timeout** | | | | | | | | | |
| C natif (à la main) | **timeout** | | | | | | | | | |
| Zig 0.17 (à la main) | **timeout** | | | | | | | | | |

## parsort (n = 1 000 000 000)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (7629 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 1.65 M/s | 605.594 s | ×1.00 | 806.25 s | 133 % | 2380 Mo | ×0.31 | 5714668 / 12455438 | 150674392 / 0 | ✅ |
| C natif (à la main) | 1.63 M/s | 615.209 s | ×1.02 | 864.10 s | 140 % | 2380 Mo | ×0.31 | 5977146 / 12397776 | 156626088 / 0 | ✅ |
| Rust (sans Creusot) | 1.59 M/s | 629.067 s | ×1.04 | 878.23 s | 140 % | 2380 Mo | ×0.31 | 6141881 / 12418905 | 158470792 / 0 | ✅ |
| F* (Pulse) → C | 1.52 M/s | 659.941 s | ×1.09 | 876.83 s | 133 % | 2380 Mo | ×0.31 | 6510809 / 13218401 | 151839232 / 0 | ✅ |
| Rust + Creusot | **oom?** | | | | | | | | | |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| Zig 0.17 (à la main) | ×1.00 | ×1.01 | ×1.00 | 2 |
| Rust (sans Creusot) | ×1.03 | ×1.04 | ×1.00 | 2 |
| C natif (à la main) | ×1.12 | ×1.15 | ×1.00 | 2 |
| F* (Pulse) → C | ×1.15 | ×1.16 | ×1.00 | 2 |
| Rust + Creusot | ×1.18 | ×1.26 | ×1.00 | 1 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 5 ms | 2.1 Mo |
| F* (Pulse) → C | 5 ms | 1.8 Mo |
| Rust (sans Creusot) | 4 ms | 1.9 Mo |
| C natif (à la main) | 4 ms | 1.8 Mo |
| Zig 0.17 (à la main) | 5 ms | 2.2 Mo |

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
