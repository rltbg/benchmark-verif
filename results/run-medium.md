# Résultats — preset `medium`

_2026-10-26 18:04:49 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v153 · médiane de 3 exécutions_

## sort (n = 8 000 000)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (61 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 8.13 M/s | 0.984 s | ×1.00 | 0.95 s | 97 % | 63 Mo | ×1.02 | 15704 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 7.40 M/s | 1.081 s | ×1.10 | 1.07 s | 99 % | 62 Mo | ×1.02 | 15705 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 6.60 M/s | 1.212 s | ×1.23 | 1.18 s | 97 % | 63 Mo | ×1.03 | 15713 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 6.37 M/s | 1.257 s | ×1.28 | 1.24 s | 98 % | 63 Mo | ×1.03 | 15713 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 6.21 M/s | 1.289 s | ×1.31 | 1.25 s | 97 % | 63 Mo | ×1.02 | 15705 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 4.65 M/s | 1.719 s | ×1.75 | 1.78 s | 99 % | 109 Mo | ×1.79 | 22507 / 0 | 0 / 0 | ✅ |

## sieve (n = 200 000 000)

Débit = entiers/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (191 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 62.80 M/s | 3.185 s | ×1.00 | 3.15 s | 99 % | 192 Mo | ×1.01 | 48908 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 60.49 M/s | 3.307 s | ×1.04 | 3.25 s | 98 % | 193 Mo | ×1.01 | 48917 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 60.12 M/s | 3.327 s | ×1.04 | 3.27 s | 98 % | 192 Mo | ×1.01 | 48908 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 60.00 M/s | 3.333 s | ×1.05 | 3.30 s | 99 % | 192 Mo | ×1.01 | 48906 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 59.55 M/s | 3.359 s | ×1.05 | 3.30 s | 98 % | 193 Mo | ×1.01 | 48917 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 46.41 M/s | 4.309 s | ×1.35 | 4.31 s | 98 % | 219 Mo | ×1.15 | 50731 / 0 | 0 / 8 | ✅ |

## matmul (n = 1 600)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (29 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 3.20 G/s | 1.279 s | ×1.00 | 1.25 s | 98 % | 31 Mo | ×1.05 | 10082 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 3.19 G/s | 1.284 s | ×1.00 | 1.24 s | 96 % | 31 Mo | ×1.05 | 10083 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 3.03 G/s | 1.350 s | ×1.06 | 1.33 s | 98 % | 31 Mo | ×1.07 | 9990 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 2.59 G/s | 1.582 s | ×1.24 | 1.58 s | 100 % | 31 Mo | ×1.07 | 10090 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 2.25 G/s | 1.824 s | ×1.43 | 1.79 s | 98 % | 31 Mo | ×1.05 | 7580 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 816.45 M/s | 5.017 s | ×3.92 | 4.99 s | 98 % | 115 Mo | ×3.93 | 24079 / 0 | 0 / 0 | ✅ |

## lines (n = 10 000 000)

Débit = lignes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (159 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust (sans Creusot) | 16.06 M/s | 0.622 s | ×1.00 | 0.62 s | 99 % | 150 Mo | ×0.94 | 42043 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 13.53 M/s | 0.739 s | ×1.19 | 0.72 s | 97 % | 149 Mo | ×0.94 | 42033 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 12.16 M/s | 0.823 s | ×1.32 | 0.81 s | 99 % | 149 Mo | ×0.94 | 42035 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 11.54 M/s | 0.866 s | ×1.39 | 0.86 s | 98 % | 149 Mo | ×0.94 | 42036 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 9.47 M/s | 1.056 s | ×1.70 | 1.04 s | 98 % | 150 Mo | ×0.94 | 42043 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 7.59 M/s | 1.318 s | ×2.12 | 1.35 s | 97 % | 322 Mo | ×2.02 | 76967 / 0 | 0 / 0 | ✅ |

## hashmap (n = 4 000 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (128 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 20.06 M/s | 0.598 s | ×1.00 | 0.61 s | 101 % | 129 Mo | ×1.01 | 65615 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 18.60 M/s | 0.645 s | ×1.08 | 0.64 s | 99 % | 130 Mo | ×1.01 | 32848 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 18.02 M/s | 0.666 s | ×1.11 | 0.67 s | 100 % | 130 Mo | ×1.01 | 32846 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 15.61 M/s | 0.769 s | ×1.28 | 0.76 s | 98 % | 130 Mo | ×1.02 | 65624 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 15.48 M/s | 0.775 s | ×1.30 | 0.78 s | 100 % | 130 Mo | ×1.02 | 65622 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 5.99 M/s | 2.004 s | ×3.35 | 2.06 s | 99 % | 291 Mo | ×2.27 | 69022 / 0 | 0 / 0 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| F* (Pulse) → C | ×1.09 | ×1.09 | ×1.00 | 5 |
| C natif (à la main) | ×1.14 | ×1.14 | ×1.00 | 5 |
| Zig 0.17 (à la main) | ×1.15 | ×1.14 | ×1.00 | 5 |
| Rust (sans Creusot) | ×1.15 | ×1.16 | ×1.01 | 5 |
| Rust + Creusot | ×1.25 | ×1.25 | ×1.01 | 5 |
| F* (Pulse) → F# | ×2.31 | ×2.38 | ×2.05 | 5 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 4 ms | 2.1 Mo |
| F* (Pulse) → C | 3 ms | 1.7 Mo |
| F* (Pulse) → F# | 91 ms | 29.4 Mo |
| Rust (sans Creusot) | 4 ms | 2.0 Mo |
| C natif (à la main) | 4 ms | 1.7 Mo |
| Zig 0.17 (à la main) | 4 ms | 1.7 Mo |

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
