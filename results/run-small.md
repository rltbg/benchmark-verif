# Résultats — preset `small`

_2026-10-27 02:42:37 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v165 · médiane de 3 exécutions_

## sort (n = 1 000 000)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (8 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 8.49 M/s | 0.118 s | ×1.00 | 0.12 s | 98 % | 9 Mo | ×1.18 | 2105 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 7.64 M/s | 0.131 s | ×1.11 | 0.13 s | 98 % | 9 Mo | ×1.20 | 2107 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 6.73 M/s | 0.149 s | ×1.26 | 0.15 s | 98 % | 10 Mo | ×1.25 | 2116 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 6.66 M/s | 0.150 s | ×1.27 | 0.15 s | 100 % | 10 Mo | ×1.26 | 2116 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 6.49 M/s | 0.154 s | ×1.31 | 0.15 s | 94 % | 9 Mo | ×1.18 | 2107 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 4.89 M/s | 0.205 s | ×1.74 | 0.27 s | 99 % | 37 Mo | ×4.79 | 3909 / 0 | 0 / 0 | ✅ |

## sieve (n = 20 000 000)

Débit = entiers/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (19 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 157.43 M/s | 0.127 s | ×1.00 | 0.13 s | 98 % | 21 Mo | ×1.08 | 5037 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 154.35 M/s | 0.130 s | ×1.02 | 0.14 s | 101 % | 21 Mo | ×1.08 | 5037 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 147.94 M/s | 0.135 s | ×1.06 | 0.13 s | 92 % | 21 Mo | ×1.10 | 5044 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 146.49 M/s | 0.137 s | ×1.07 | 0.20 s | 99 % | 48 Mo | ×2.50 | 6825 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 145.12 M/s | 0.138 s | ×1.08 | 0.14 s | 98 % | 21 Mo | ×1.10 | 5045 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 144.31 M/s | 0.139 s | ×1.09 | 0.14 s | 97 % | 21 Mo | ×1.08 | 5035 / 0 | 0 / 0 | ✅ |

## matmul (n = 512)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (3 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 7.79 G/s | 0.017 s | ×1.00 | 0.02 s | 89 % | 4 Mo | ×1.42 | 1181 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 6.46 G/s | 0.021 s | ×1.21 | 0.02 s | 92 % | 4 Mo | ×1.45 | 1179 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 6.14 G/s | 0.022 s | ×1.27 | 0.02 s | 81 % | 5 Mo | ×1.63 | 1190 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 4.28 G/s | 0.031 s | ×1.82 | 0.03 s | 82 % | 5 Mo | ×1.64 | 932 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 2.57 G/s | 0.052 s | ×3.04 | 0.06 s | 99 % | 5 Mo | ×1.50 | 923 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 1.11 G/s | 0.121 s | ×7.04 | 0.18 s | 99 % | 32 Mo | ×10.62 | 2716 / 0 | 0 / 0 | ✅ |

## lines (n = 1 000 000)

Débit = lignes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (30 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust (sans Creusot) | 10.36 M/s | 0.097 s | ×1.00 | 0.10 s | 95 % | 31 Mo | ×1.03 | 11730 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 9.72 M/s | 0.103 s | ×1.07 | 0.11 s | 102 % | 31 Mo | ×1.01 | 11723 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 9.27 M/s | 0.108 s | ×1.12 | 0.11 s | 98 % | 31 Mo | ×1.01 | 11724 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 9.13 M/s | 0.110 s | ×1.13 | 0.11 s | 96 % | 31 Mo | ×1.01 | 11723 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 8.23 M/s | 0.121 s | ×1.26 | 0.12 s | 98 % | 31 Mo | ×1.03 | 11732 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 7.79 M/s | 0.128 s | ×1.33 | 0.20 s | 101 % | 59 Mo | ×1.95 | 9713 / 0 | 0 / 0 | ✅ |

## hashmap (n = 500 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (16 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 36.70 M/s | 0.041 s | ×1.00 | 0.04 s | 97 % | 18 Mo | ×1.09 | 4249 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 32.56 M/s | 0.046 s | ×1.13 | 0.04 s | 86 % | 18 Mo | ×1.09 | 4250 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 31.45 M/s | 0.048 s | ×1.17 | 0.05 s | 92 % | 17 Mo | ×1.09 | 8346 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 31.13 M/s | 0.048 s | ×1.18 | 0.05 s | 97 % | 18 Mo | ×1.12 | 8354 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 30.88 M/s | 0.049 s | ×1.19 | 0.05 s | 100 % | 18 Mo | ×1.12 | 8353 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 18.92 M/s | 0.079 s | ×1.94 | 0.15 s | 101 % | 45 Mo | ×2.80 | 6043 / 0 | 0 / 0 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| C natif (à la main) | ×1.08 | ×1.09 | ×1.00 | 5 |
| F* (Pulse) → C | ×1.15 | ×1.11 | ×1.00 | 5 |
| Rust (sans Creusot) | ×1.15 | ×1.14 | ×1.06 | 5 |
| Zig 0.17 (à la main) | ×1.29 | ×1.30 | ×1.01 | 5 |
| Rust + Creusot | ×1.30 | ×1.26 | ×1.06 | 5 |
| F* (Pulse) → F# | ×2.02 | ×2.97 | ×3.23 | 5 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 5 ms | 2.1 Mo |
| F* (Pulse) → C | 5 ms | 1.7 Mo |
| F* (Pulse) → F# | 69 ms | 28.9 Mo |
| Rust (sans Creusot) | 5 ms | 2.0 Mo |
| C natif (à la main) | 6 ms | 1.7 Mo |
| Zig 0.17 (à la main) | 5 ms | 1.7 Mo |

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
