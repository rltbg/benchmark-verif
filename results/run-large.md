# Résultats — preset `large`

_2026-10-26 18:07:25 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v153 · médiane de 3 exécutions_

## sort (n = 32 000 000)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (244 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 7.34 M/s | 4.361 s | ×1.00 | 4.32 s | 99 % | 246 Mo | ×1.01 | 62579 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 6.91 M/s | 4.633 s | ×1.06 | 4.57 s | 99 % | 246 Mo | ×1.01 | 62581 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 6.44 M/s | 4.970 s | ×1.14 | 4.91 s | 99 % | 246 Mo | ×1.01 | 62588 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 6.18 M/s | 5.178 s | ×1.19 | 5.13 s | 99 % | 246 Mo | ×1.01 | 62590 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 5.98 M/s | 5.349 s | ×1.23 | 5.25 s | 98 % | 246 Mo | ×1.01 | 62581 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 4.81 M/s | 6.658 s | ×1.53 | 6.69 s | 99 % | 294 Mo | ×1.20 | 69702 / 0 | 0 / 16 | ✅ |

## sieve (n = 1 000 000 000)

Débit = entiers/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (954 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 53.45 M/s | 18.710 s | ×1.00 | 18.44 s | 99 % | 955 Mo | ×1.00 | 244221 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 53.44 M/s | 18.712 s | ×1.00 | 18.43 s | 98 % | 955 Mo | ×1.00 | 244220 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 52.72 M/s | 18.967 s | ×1.01 | 18.65 s | 98 % | 956 Mo | ×1.00 | 244228 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 52.67 M/s | 18.985 s | ×1.01 | 18.78 s | 99 % | 955 Mo | ×1.00 | 244218 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 51.99 M/s | 19.234 s | ×1.03 | 19.06 s | 99 % | 956 Mo | ×1.00 | 244229 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 42.82 M/s | 23.353 s | ×1.25 | 23.10 s | 98 % | 982 Mo | ×1.03 | 246054 / 0 | 0 / 56 | ✅ |

## matmul (n = 2 500)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (72 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 3.39 G/s | 4.604 s | ×1.00 | 4.57 s | 99 % | 73 Mo | ×1.02 | 24494 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 3.19 G/s | 4.894 s | ×1.06 | 4.82 s | 98 % | 73 Mo | ×1.03 | 24493 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 3.00 G/s | 5.215 s | ×1.13 | 5.15 s | 99 % | 73 Mo | ×1.02 | 24495 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 2.41 G/s | 6.483 s | ×1.41 | 6.38 s | 98 % | 73 Mo | ×1.03 | 24493 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 2.36 G/s | 6.623 s | ×1.44 | 6.54 s | 99 % | 73 Mo | ×1.02 | 18389 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 713.59 M/s | 21.896 s | ×4.76 | 21.77 s | 99 % | 231 Mo | ×3.22 | 53537 / 0 | 0 / 56 | ✅ |

## lines (n = 40 000 000)

Débit = lignes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (588 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 16.57 M/s | 2.414 s | ×1.00 | 2.37 s | 98 % | 545 Mo | ×0.93 | 143325 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 16.00 M/s | 2.500 s | ×1.04 | 2.48 s | 99 % | 545 Mo | ×0.93 | 143332 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 15.05 M/s | 2.658 s | ×1.10 | 2.62 s | 98 % | 545 Mo | ×0.93 | 143324 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 13.36 M/s | 2.994 s | ×1.24 | 2.96 s | 99 % | 545 Mo | ×0.93 | 143322 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 12.36 M/s | 3.236 s | ×1.34 | 3.18 s | 98 % | 546 Mo | ×0.93 | 143332 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 7.98 M/s | 5.011 s | ×2.08 | 5.03 s | 99 % | 751 Mo | ×1.28 | 186832 / 0 | 0 / 0 | ✅ |

## hashmap (n = 16 000 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (512 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 16.64 M/s | 2.885 s | ×1.00 | 2.88 s | 100 % | 513 Mo | ×1.00 | 262223 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 15.78 M/s | 3.042 s | ×1.05 | 3.01 s | 99 % | 514 Mo | ×1.00 | 131150 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 15.51 M/s | 3.095 s | ×1.07 | 3.00 s | 97 % | 514 Mo | ×1.00 | 131152 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 14.19 M/s | 3.383 s | ×1.17 | 3.35 s | 99 % | 514 Mo | ×1.00 | 262233 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 13.88 M/s | 3.457 s | ×1.20 | 3.41 s | 99 % | 514 Mo | ×1.00 | 262233 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 5.69 M/s | 8.441 s | ×2.93 | 8.46 s | 99 % | 675 Mo | ×1.32 | 167331 / 0 | 0 / 0 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| F* (Pulse) → C | ×1.05 | ×1.04 | ×1.00 | 5 |
| C natif (à la main) | ×1.06 | ×1.06 | ×1.00 | 5 |
| Zig 0.17 (à la main) | ×1.14 | ×1.14 | ×1.00 | 5 |
| Rust (sans Creusot) | ×1.15 | ×1.15 | ×1.00 | 5 |
| Rust + Creusot | ×1.15 | ×1.15 | ×1.00 | 5 |
| F* (Pulse) → F# | ×2.23 | ×2.25 | ×1.48 | 5 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 5 ms | 2.0 Mo |
| F* (Pulse) → C | 3 ms | 1.7 Mo |
| F* (Pulse) → F# | 74 ms | 29.4 Mo |
| Rust (sans Creusot) | 4 ms | 2.1 Mo |
| C natif (à la main) | 4 ms | 1.7 Mo |
| Zig 0.17 (à la main) | 3 ms | 1.7 Mo |

## Empreinte disque et coût de construction

| implémentation | exécutable | runtime/deps requis | compilation | vérification |
|---|--:|--:|--:|--:|
| Rust + Creusot | 0.38 Mo | 0.0 Mo | 8.9 s | 16.3 s |
| F* (Pulse) → C | 0.02 Mo | 0.0 Mo | 2.4 s | 236.5 s |
| F* (Pulse) → OCaml (runtime livré) | 4.96 Mo | 0.0 Mo | 3.1 s | 236.5 s |
| F* (Pulse) → F# | 0.05 Mo | 72.4 Mo | 7.2 s | 236.5 s |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 4.94 Mo | 0.0 Mo | 4.5 s | 236.5 s |
| Rust (sans Creusot) | 0.38 Mo | 0.0 Mo | 4.1 s | — |
| C natif (à la main) | 0.02 Mo | 0.0 Mo | 0.2 s | — |
| Zig 0.17 (à la main) | 0.06 Mo | 0.0 Mo | 0.3 s | — |
