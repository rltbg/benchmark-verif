# Résultats — preset `smoke`

_2026-10-26 17:15:35 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v153 · médiane de 1 exécutions_

## sort (n = 10 000)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 12.94 M/s | 0.001 s | ×1.00 | 0.00 s | 53 % | 2 Mo | ×22.48 | 96 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 12.11 M/s | 0.001 s | ×1.07 | 0.00 s | 56 % | 2 Mo | ×25.80 | 107 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 893.96 k/s | 0.011 s | ×14.48 | 0.07 s | 97 % | 29 Mo | ×384.46 | 2145 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 153.02 k/s | 0.065 s | ×84.59 | 0.07 s | 97 % | 8 Mo | ×99.53 | 1067 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 24.82 k/s | 0.403 s | ×521.51 | 0.40 s | 99 % | 8 Mo | ×101.02 | 1075 / 0 | 0 / 0 | ✅ |

## sieve (n = 1 000 000)

Débit = entiers/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (1 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 312.93 M/s | 0.003 s | ×1.00 | 0.00 s | 75 % | 2 Mo | ×2.39 | 324 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 302.04 M/s | 0.003 s | ×1.04 | 0.00 s | 66 % | 3 Mo | ×2.98 | 331 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 80.41 M/s | 0.012 s | ×3.89 | 0.09 s | 98 % | 30 Mo | ×31.02 | 2163 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 4.05 M/s | 0.247 s | ×77.36 | 0.25 s | 99 % | 15 Mo | ×15.72 | 2922 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 582.60 k/s | 1.716 s | ×537.13 | 1.68 s | 97 % | 15 Mo | ×15.84 | 2980 / 0 | 0 / 0 | ✅ |

## matmul (n = 64)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 2.98 G/s | 0.000 s | ×1.00 | 0.00 s | 41 % | 2 Mo | ×36.58 | 91 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 2.73 G/s | 0.000 s | ×1.09 | 0.00 s | 56 % | 2 Mo | ×42.00 | 100 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 62.67 M/s | 0.004 s | ×47.54 | 0.06 s | 96 % | 29 Mo | ×608.50 | 1924 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 3.69 M/s | 0.071 s | ×807.57 | 0.07 s | 95 % | 8 Mo | ×165.75 | 1092 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 638.36 k/s | 0.411 s | ×4667.23 | 0.40 s | 97 % | 8 Mo | ×172.92 | 1164 / 0 | 0 / 0 | ✅ |

## lines (n = 10 000)

Débit = lignes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (16 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust + Creusot | 584.72 k/s | 0.017 s | ×1.00 | 0.02 s | 92 % | 17 Mo | ×1.04 | 7976 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 496.51 k/s | 0.020 s | ×1.18 | 0.02 s | 89 % | 16 Mo | ×1.01 | 7969 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 426.40 k/s | 0.023 s | ×1.37 | 0.08 s | 98 % | 45 Mo | ×2.82 | 6258 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 20.91 k/s | 0.478 s | ×27.97 | 0.48 s | 98 % | 25 Mo | ×1.55 | 5481 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 1.93 k/s | 5.195 s | ×303.73 | 5.16 s | 99 % | 26 Mo | ×1.59 | 5642 / 0 | 0 / 0 | ✅ |

## hashmap (n = 10 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 37.89 M/s | 0.001 s | ×1.00 | 0.00 s | 50 % | 2 Mo | ×3.81 | 209 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 36.16 M/s | 0.001 s | ×1.05 | 0.00 s | 61 % | 2 Mo | ×4.91 | 344 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 3.14 M/s | 0.010 s | ×12.05 | 0.08 s | 88 % | 32 Mo | ×63.52 | 2669 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 522.61 k/s | 0.057 s | ×72.49 | 0.06 s | 97 % | 8 Mo | ×16.66 | 1210 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 67.25 k/s | 0.446 s | ×563.35 | 0.44 s | 97 % | 8 Mo | ×16.73 | 1231 / 0 | 0 / 0 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| F* (Pulse) → C | ×1.03 | ×1.04 | ×1.00 | 5 |
| Rust + Creusot | ×1.05 | ×1.13 | ×1.17 | 5 |
| F* (Pulse) → F# | ×8.50 | ×24.15 | ×11.15 | 5 |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | ×101.39 | ×41.62 | ×3.89 | 5 |
| F* (Pulse) → OCaml (runtime livré) | ×741.20 | ×292.71 | ×3.96 | 5 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 4 ms | 2.0 Mo |
| F* (Pulse) → C | 3 ms | 1.7 Mo |
| F* (Pulse) → F# | 77 ms | 29.4 Mo |
| F* (Pulse) → OCaml (runtime livré) | 416 ms | 7.7 Mo |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 74 ms | 7.6 Mo |

## Empreinte disque et coût de construction

| implémentation | exécutable | runtime/deps requis | compilation | vérification |
|---|--:|--:|--:|--:|
| Rust + Creusot | 0.38 Mo | 0.0 Mo | 10.2 s | 12.7 s |
| F* (Pulse) → C | 0.02 Mo | 0.0 Mo | 2.4 s | 153.7 s |
| F* (Pulse) → OCaml (runtime livré) | 4.96 Mo | 0.0 Mo | 2.8 s | 153.7 s |
| F* (Pulse) → F# | 0.05 Mo | 72.4 Mo | 10.0 s | 153.7 s |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 4.94 Mo | 0.0 Mo | 4.1 s | 153.7 s |
