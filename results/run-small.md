# Résultats — preset `small`

_2026-10-26 18:04:36 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v153 · médiane de 3 exécutions_

## sort (n = 1 000 000)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (8 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 8.66 M/s | 0.115 s | ×1.00 | 0.11 s | 95 % | 9 Mo | ×1.19 | 2032 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 7.75 M/s | 0.129 s | ×1.12 | 0.13 s | 94 % | 10 Mo | ×1.25 | 2040 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 7.65 M/s | 0.131 s | ×1.13 | 0.12 s | 87 % | 9 Mo | ×1.19 | 2033 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 7.53 M/s | 0.133 s | ×1.15 | 0.13 s | 98 % | 10 Mo | ×1.26 | 2043 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 7.21 M/s | 0.139 s | ×1.20 | 0.14 s | 98 % | 9 Mo | ×1.19 | 2034 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 3.16 M/s | 0.317 s | ×2.74 | 0.38 s | 96 % | 63 Mo | ×8.27 | 10667 / 0 | 0 / 16 | ✅ |

## sieve (n = 20 000 000)

Débit = entiers/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (19 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 149.44 M/s | 0.134 s | ×1.00 | 0.13 s | 96 % | 21 Mo | ×1.08 | 4961 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 148.78 M/s | 0.134 s | ×1.00 | 0.13 s | 95 % | 21 Mo | ×1.08 | 4962 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 147.97 M/s | 0.135 s | ×1.01 | 0.13 s | 96 % | 21 Mo | ×1.10 | 4971 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 135.93 M/s | 0.147 s | ×1.10 | 0.14 s | 96 % | 21 Mo | ×1.10 | 4970 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 135.27 M/s | 0.148 s | ×1.10 | 0.15 s | 98 % | 21 Mo | ×1.08 | 4962 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 92.31 M/s | 0.217 s | ×1.62 | 0.28 s | 99 % | 48 Mo | ×2.49 | 6773 / 0 | 0 / 8 | ✅ |

## matmul (n = 512)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (3 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 4.29 G/s | 0.031 s | ×1.00 | 0.03 s | 93 % | 4 Mo | ×1.43 | 1106 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 3.85 G/s | 0.035 s | ×1.11 | 0.04 s | 94 % | 4 Mo | ×1.43 | 1106 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 3.85 G/s | 0.035 s | ×1.11 | 0.04 s | 95 % | 5 Mo | ×1.65 | 857 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 2.75 G/s | 0.049 s | ×1.56 | 0.05 s | 97 % | 5 Mo | ×1.65 | 1114 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 2.70 G/s | 0.050 s | ×1.59 | 0.05 s | 91 % | 5 Mo | ×1.51 | 848 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 422.28 M/s | 0.318 s | ×10.16 | 0.39 s | 100 % | 37 Mo | ×12.26 | 4018 / 0 | 0 / 0 | ✅ |

## lines (n = 1 000 000)

Débit = lignes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (30 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 10.14 M/s | 0.099 s | ×1.00 | 0.10 s | 98 % | 31 Mo | ×1.01 | 11647 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 10.00 M/s | 0.100 s | ×1.01 | 0.11 s | 105 % | 31 Mo | ×1.03 | 11656 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 9.15 M/s | 0.109 s | ×1.11 | 0.11 s | 101 % | 31 Mo | ×1.01 | 11649 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 7.82 M/s | 0.128 s | ×1.30 | 0.13 s | 99 % | 31 Mo | ×1.03 | 11657 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 7.26 M/s | 0.138 s | ×1.40 | 0.14 s | 99 % | 31 Mo | ×1.01 | 11650 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 4.99 M/s | 0.200 s | ×2.03 | 0.29 s | 107 % | 151 Mo | ×4.98 | 33194 / 0 | 0 / 0 | ✅ |

## hashmap (n = 500 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (16 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 31.52 M/s | 0.048 s | ×1.00 | 0.05 s | 97 % | 18 Mo | ×1.09 | 4174 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 29.99 M/s | 0.050 s | ×1.05 | 0.05 s | 92 % | 17 Mo | ×1.09 | 8271 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 28.51 M/s | 0.053 s | ×1.11 | 0.05 s | 91 % | 18 Mo | ×1.09 | 4176 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 26.27 M/s | 0.057 s | ×1.20 | 0.06 s | 93 % | 18 Mo | ×1.12 | 8279 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 23.53 M/s | 0.064 s | ×1.34 | 0.07 s | 96 % | 18 Mo | ×1.12 | 8280 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 6.67 M/s | 0.225 s | ×4.73 | 0.30 s | 96 % | 171 Mo | ×10.68 | 38339 / 0 | 0 / 0 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| F* (Pulse) → C | ×1.09 | ×1.06 | ×1.00 | 5 |
| Zig 0.17 (à la main) | ×1.10 | ×1.08 | ×1.01 | 5 |
| C natif (à la main) | ×1.14 | ×1.14 | ×1.00 | 5 |
| Rust + Creusot | ×1.18 | ×1.19 | ×1.06 | 5 |
| Rust (sans Creusot) | ×1.18 | ×1.20 | ×1.06 | 5 |
| F* (Pulse) → F# | ×3.37 | ×4.34 | ×5.82 | 5 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 4 ms | 2.0 Mo |
| F* (Pulse) → C | 4 ms | 1.7 Mo |
| F* (Pulse) → F# | 74 ms | 29.4 Mo |
| Rust (sans Creusot) | 4 ms | 2.1 Mo |
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
