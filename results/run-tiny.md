# Résultats — preset `tiny`

_2026-10-26 17:15:46 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v153 · médiane de 2 exécutions_

## sort (n = 200 000)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (2 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 10.22 M/s | 0.020 s | ×1.00 | 0.02 s | 92 % | 3 Mo | ×1.95 | 471 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 9.09 M/s | 0.022 s | ×1.12 | 0.02 s | 88 % | 3 Mo | ×2.27 | 478 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 1.32 M/s | 0.152 s | ×7.76 | 0.21 s | 96 % | 48 Mo | ×31.62 | 6986 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 121.11 k/s | 1.651 s | ×84.39 | 1.64 s | 99 % | 15 Mo | ×9.89 | 2970 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 16.64 k/s | 12.018 s | ×614.18 | 11.88 s | 99 % | 17 Mo | ×11.09 | 3450 / 0 | 0 / 0 | ✅ |

## sieve (n = 5 000 000)

Débit = entiers/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (5 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 180.45 M/s | 0.028 s | ×1.00 | 0.03 s | 91 % | 6 Mo | ×1.30 | 1300 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 123.99 M/s | 0.040 s | ×1.46 | 0.03 s | 72 % | 7 Mo | ×1.41 | 1308 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 109.70 M/s | 0.046 s | ×1.64 | 0.10 s | 97 % | 33 Mo | ×6.98 | 3108 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 2.75 M/s | 1.817 s | ×65.56 | 1.81 s | 99 % | 45 Mo | ×9.53 | 10744 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 498.05 k/s | 10.039 s | ×362.31 | 9.89 s | 98 % | 47 Mo | ×9.81 | 11097 / 0 | 0 / 0 | ✅ |

## matmul (n = 128)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 3.03 G/s | 0.001 s | ×1.00 | 0.00 s | 49 % | 2 Mo | ×9.81 | 140 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 2.54 G/s | 0.001 s | ×1.19 | 0.00 s | 58 % | 2 Mo | ×11.30 | 150 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 88.67 M/s | 0.024 s | ×34.13 | 0.08 s | 98 % | 29 Mo | ×154.64 | 2036 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 3.76 M/s | 0.557 s | ×804.10 | 0.55 s | 99 % | 10 Mo | ×53.97 | 1688 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 728.62 k/s | 2.878 s | ×4153.88 | 2.85 s | 99 % | 12 Mo | ×61.86 | 2072 / 0 | 0 / 0 | ✅ |

## lines (n = 50 000)

Débit = lignes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (17 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust + Creusot | 1.92 M/s | 0.026 s | ×1.00 | 0.03 s | 94 % | 19 Mo | ×1.11 | 8448 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 1.88 M/s | 0.027 s | ×1.02 | 0.03 s | 87 % | 18 Mo | ×1.08 | 8440 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 1.39 M/s | 0.036 s | ×1.38 | 0.10 s | 96 % | 50 Mo | ×2.98 | 7350 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 69.80 k/s | 0.716 s | ×27.53 | 0.71 s | 98 % | 31 Mo | ×1.88 | 7122 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 6.48 k/s | 7.720 s | ×296.75 | 7.64 s | 99 % | 32 Mo | ×1.92 | 7310 / 0 | 0 / 0 | ✅ |

## hashmap (n = 100 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (4 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 37.54 M/s | 0.008 s | ×1.00 | 0.01 s | 77 % | 5 Mo | ×1.35 | 1104 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 33.12 M/s | 0.009 s | ×1.13 | 0.01 s | 86 % | 6 Mo | ×1.49 | 2136 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 6.91 M/s | 0.043 s | ×5.43 | 0.11 s | 97 % | 58 Mo | ×14.48 | 9384 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 475.13 k/s | 0.631 s | ×79.00 | 0.61 s | 96 % | 16 Mo | ×3.99 | 3172 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (runtime livré) | 69.66 k/s | 4.306 s | ×538.83 | 4.26 s | 99 % | 16 Mo | ×4.12 | 3316 / 0 | 0 / 0 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| F* (Pulse) → C | ×1.00 | ×1.00 | ×1.00 | 5 |
| Rust + Creusot | ×1.17 | ×1.17 | ×1.11 | 5 |
| F* (Pulse) → F# | ×5.04 | ×9.75 | ×8.34 | 5 |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | ×99.35 | ×79.45 | ×4.02 | 5 |
| F* (Pulse) → OCaml (runtime livré) | ×682.24 | ×544.99 | ×4.30 | 5 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 4 ms | 2.0 Mo |
| F* (Pulse) → C | 4 ms | 1.7 Mo |
| F* (Pulse) → F# | 91 ms | 29.3 Mo |
| F* (Pulse) → OCaml (runtime livré) | 444 ms | 7.8 Mo |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 66 ms | 7.7 Mo |

## Empreinte disque et coût de construction

| implémentation | exécutable | runtime/deps requis | compilation | vérification |
|---|--:|--:|--:|--:|
| Rust + Creusot | 0.38 Mo | 0.0 Mo | 10.2 s | 12.7 s |
| F* (Pulse) → C | 0.02 Mo | 0.0 Mo | 2.4 s | 153.7 s |
| F* (Pulse) → OCaml (runtime livré) | 4.96 Mo | 0.0 Mo | 2.8 s | 153.7 s |
| F* (Pulse) → F# | 0.05 Mo | 72.4 Mo | 10.0 s | 153.7 s |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 4.94 Mo | 0.0 Mo | 4.1 s | 153.7 s |
