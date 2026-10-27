# Résultats — preset `large`

_2026-10-27 02:45:04 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v165 · médiane de 3 exécutions_

## sort (n = 32 000 000)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (244 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 6.22 M/s | 5.142 s | ×1.00 | 5.11 s | 99 % | 246 Mo | ×1.01 | 62654 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 5.89 M/s | 5.430 s | ×1.06 | 5.39 s | 99 % | 246 Mo | ×1.01 | 62655 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 5.44 M/s | 5.886 s | ×1.14 | 5.86 s | 100 % | 246 Mo | ×1.01 | 62656 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 5.23 M/s | 6.124 s | ×1.19 | 6.07 s | 99 % | 246 Mo | ×1.01 | 62664 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 4.90 M/s | 6.525 s | ×1.27 | 6.46 s | 99 % | 246 Mo | ×1.01 | 62664 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 3.56 M/s | 8.977 s | ×1.75 | 8.97 s | 99 % | 273 Mo | ×1.12 | 64472 / 0 | 0 / 0 | ✅ |

## sieve (n = 1 000 000 000)

Débit = entiers/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (954 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust (sans Creusot) | 57.12 M/s | 17.506 s | ×1.00 | 17.46 s | 100 % | 956 Mo | ×1.00 | 244302 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 54.28 M/s | 18.424 s | ×1.05 | 17.40 s | 94 % | 955 Mo | ×1.00 | 244294 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 54.26 M/s | 18.431 s | ×1.05 | 18.24 s | 99 % | 955 Mo | ×1.00 | 244292 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 53.32 M/s | 18.754 s | ×1.07 | 18.79 s | 100 % | 983 Mo | ×1.03 | 246103 / 0 | 0 / 40 | ✅ |
| Rust + Creusot | 53.19 M/s | 18.800 s | ×1.07 | 17.94 s | 95 % | 956 Mo | ×1.00 | 244301 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 51.80 M/s | 19.304 s | ×1.10 | 19.09 s | 99 % | 955 Mo | ×1.00 | 244295 / 0 | 0 / 0 | ✅ |

## matmul (n = 2 500)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (72 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust (sans Creusot) | 6.22 G/s | 2.511 s | ×1.00 | 2.50 s | 99 % | 73 Mo | ×1.03 | 24576 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 6.13 G/s | 2.548 s | ×1.01 | 2.53 s | 99 % | 73 Mo | ×1.02 | 24568 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 5.33 G/s | 2.930 s | ×1.17 | 2.88 s | 98 % | 73 Mo | ×1.02 | 24569 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 4.70 G/s | 3.322 s | ×1.32 | 3.30 s | 99 % | 73 Mo | ×1.03 | 24567 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 2.30 G/s | 6.801 s | ×2.71 | 6.68 s | 98 % | 73 Mo | ×1.02 | 18461 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 1.04 G/s | 15.087 s | ×6.01 | 15.05 s | 99 % | 101 Mo | ×1.41 | 20278 / 0 | 0 / 8 | ✅ |

## lines (n = 40 000 000)

Débit = lignes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (588 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust (sans Creusot) | 13.27 M/s | 3.014 s | ×1.00 | 2.89 s | 96 % | 546 Mo | ×0.93 | 143407 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 12.06 M/s | 3.315 s | ×1.10 | 3.29 s | 99 % | 545 Mo | ×0.93 | 143396 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 11.24 M/s | 3.559 s | ×1.18 | 3.51 s | 99 % | 545 Mo | ×0.93 | 143399 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 10.28 M/s | 3.892 s | ×1.29 | 3.86 s | 99 % | 545 Mo | ×0.93 | 143398 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 8.80 M/s | 4.545 s | ×1.51 | 4.60 s | 99 % | 617 Mo | ×1.05 | 152552 / 0 | 0 / 8 | ✅ |
| Rust + Creusot | 8.21 M/s | 4.871 s | ×1.62 | 4.80 s | 99 % | 546 Mo | ×0.93 | 143406 / 0 | 0 / 0 | ✅ |

## hashmap (n = 16 000 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (512 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 18.46 M/s | 2.601 s | ×1.00 | 2.60 s | 100 % | 514 Mo | ×1.00 | 131225 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 17.14 M/s | 2.801 s | ×1.08 | 2.80 s | 100 % | 513 Mo | ×1.00 | 262299 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 15.06 M/s | 3.187 s | ×1.23 | 3.17 s | 99 % | 514 Mo | ×1.00 | 131228 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 14.53 M/s | 3.303 s | ×1.27 | 3.29 s | 99 % | 514 Mo | ×1.00 | 262306 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 14.11 M/s | 3.402 s | ×1.31 | 3.35 s | 98 % | 514 Mo | ×1.00 | 262308 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 10.27 M/s | 4.675 s | ×1.80 | 4.68 s | 98 % | 541 Mo | ×1.06 | 133032 / 0 | 0 / 0 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| C natif (à la main) | ×1.07 | ×1.07 | ×1.00 | 5 |
| Rust (sans Creusot) | ×1.09 | ×1.09 | ×1.00 | 5 |
| F* (Pulse) → C | ×1.18 | ×1.19 | ×1.00 | 5 |
| Zig 0.17 (à la main) | ×1.26 | ×1.26 | ×1.00 | 5 |
| Rust + Creusot | ×1.30 | ×1.29 | ×1.00 | 5 |
| F* (Pulse) → F# | ×1.98 | ×2.01 | ×1.13 | 5 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 6 ms | 1.9 Mo |
| F* (Pulse) → C | 5 ms | 1.7 Mo |
| F* (Pulse) → F# | 86 ms | 28.8 Mo |
| Rust (sans Creusot) | 5 ms | 1.9 Mo |
| C natif (à la main) | 6 ms | 1.7 Mo |
| Zig 0.17 (à la main) | 6 ms | 1.7 Mo |

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
