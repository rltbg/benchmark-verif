# Résultats — preset `small`

_2026-10-27 17:12:37 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v165 · médiane de 1 exécutions_

## parmatmul (n = 1 024 — 4 threads)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (12 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 12.34 M/s | 86.992 s | ×1.00 | 321.59 s | 370 % | 306 Mo | ×25.49 | 77305 / 0 | 0 / 0 | ✅ |

## parsha (n = 256 000 000 — 4 threads)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (2 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 1.64 M/s | 156.139 s | ×1.00 | 598.28 s | 383 % | 17 Mo | ×8.70 | 3444 / 0 | 0 / 0 | ✅ |

## parsort (n = 4 000 000 — 4 threads)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (31 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 219.35 k/s | 18.235 s | ×1.00 | 46.36 s | 254 % | 172 Mo | ×5.64 | 43608 / 0 | 0 / 0 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | ×1.00 | ×1.00 | ×1.00 | 3 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 83 ms | 8.7 Mo |

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
