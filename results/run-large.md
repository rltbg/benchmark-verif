# Résultats — preset `large`

_2026-10-26 17:21:52 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v153 · médiane de 2 exécutions_

## sort (n = 32 000 000)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (244 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 7.13 M/s | 4.487 s | ×1.00 | 4.44 s | 99 % | 246 Mo | ×1.01 | 62581 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 6.62 M/s | 4.835 s | ×1.08 | 4.75 s | 98 % | 246 Mo | ×1.01 | 62588 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 4.56 M/s | 7.023 s | ×1.57 | 6.99 s | 98 % | 292 Mo | ×1.20 | 69332 / 0 | 0 / 0 | ✅ |

## sieve (n = 1 000 000 000)

Débit = entiers/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (954 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 53.44 M/s | 18.712 s | ×1.00 | 18.50 s | 99 % | 955 Mo | ×1.00 | 244220 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 48.34 M/s | 20.685 s | ×1.11 | 20.47 s | 99 % | 956 Mo | ×1.00 | 244228 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 42.32 M/s | 23.632 s | ×1.26 | 23.45 s | 99 % | 982 Mo | ×1.03 | 246058 / 0 | 0 / 32 | ✅ |

## matmul (n = 2 500)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (72 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust + Creusot | 3.29 G/s | 4.753 s | ×1.00 | 4.69 s | 99 % | 73 Mo | ×1.03 | 24492 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 3.16 G/s | 4.947 s | ×1.04 | 4.90 s | 99 % | 73 Mo | ×1.02 | 24494 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 662.09 M/s | 23.599 s | ×4.97 | 23.42 s | 99 % | 230 Mo | ×3.22 | 53538 / 0 | 0 / 56 | ✅ |

## lines (n = 40 000 000)

Débit = lignes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (588 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 16.99 M/s | 2.355 s | ×1.00 | 2.30 s | 98 % | 545 Mo | ×0.93 | 143324 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 14.12 M/s | 2.833 s | ×1.20 | 2.81 s | 99 % | 546 Mo | ×0.93 | 143332 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 7.81 M/s | 5.123 s | ×2.18 | 5.17 s | 99 % | 751 Mo | ×1.28 | 186833 / 0 | 0 / 4 | ✅ |

## hashmap (n = 16 000 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (512 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 16.74 M/s | 2.868 s | ×1.00 | 2.84 s | 99 % | 514 Mo | ×1.00 | 131152 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 13.81 M/s | 3.476 s | ×1.21 | 3.43 s | 98 % | 514 Mo | ×1.00 | 262232 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 5.75 M/s | 8.355 s | ×2.91 | 8.35 s | 99 % | 675 Mo | ×1.32 | 167326 / 0 | 0 / 28 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| F* (Pulse) → C | ×1.01 | ×1.01 | ×1.00 | 5 |
| Rust + Creusot | ×1.12 | ×1.12 | ×1.00 | 5 |
| F* (Pulse) → F# | ×2.28 | ×2.31 | ×1.48 | 5 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 4 ms | 1.9 Mo |
| F* (Pulse) → C | 4 ms | 1.7 Mo |
| F* (Pulse) → F# | 77 ms | 29.4 Mo |

## Empreinte disque et coût de construction

| implémentation | exécutable | runtime/deps requis | compilation | vérification |
|---|--:|--:|--:|--:|
| Rust + Creusot | 0.38 Mo | 0.0 Mo | 10.2 s | 12.7 s |
| F* (Pulse) → C | 0.02 Mo | 0.0 Mo | 2.4 s | 153.7 s |
| F* (Pulse) → OCaml (runtime livré) | 4.96 Mo | 0.0 Mo | 2.8 s | 153.7 s |
| F* (Pulse) → F# | 0.05 Mo | 72.4 Mo | 10.0 s | 153.7 s |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 4.94 Mo | 0.0 Mo | 4.1 s | 153.7 s |
