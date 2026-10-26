# Résultats — preset `small`

_2026-10-26 17:17:15 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v153 · médiane de 3 exécutions_

## sort (n = 1 000 000)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (8 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 8.37 M/s | 0.119 s | ×1.00 | 0.12 s | 98 % | 9 Mo | ×1.19 | 2033 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 8.37 M/s | 0.120 s | ×1.00 | 0.12 s | 97 % | 10 Mo | ×1.26 | 2040 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 3.03 M/s | 0.330 s | ×2.76 | 0.39 s | 98 % | 60 Mo | ×7.81 | 9843 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 103.90 k/s | 9.625 s | ×80.60 | 9.47 s | 98 % | 47 Mo | ×6.10 | 11016 / 0 | 0 / 0 | ✅ |

## sieve (n = 20 000 000)

Débit = entiers/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (19 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 153.57 M/s | 0.130 s | ×1.00 | 0.13 s | 94 % | 21 Mo | ×1.08 | 4963 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 146.67 M/s | 0.136 s | ×1.05 | 0.14 s | 98 % | 21 Mo | ×1.10 | 4970 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 88.36 M/s | 0.226 s | ×1.74 | 0.30 s | 97 % | 47 Mo | ×2.49 | 6776 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 2.57 M/s | 7.772 s | ×59.68 | 7.69 s | 99 % | 160 Mo | ×8.39 | 40086 / 0 | 0 / 0 | ✅ |

## matmul (n = 512)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (3 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust + Creusot | 4.16 G/s | 0.032 s | ×1.00 | 0.03 s | 93 % | 5 Mo | ×1.65 | 858 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 2.80 G/s | 0.048 s | ×1.49 | 0.05 s | 96 % | 4 Mo | ×1.43 | 1106 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 397.29 M/s | 0.338 s | ×10.46 | 0.42 s | 100 % | 37 Mo | ×12.19 | 4017 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 4.15 M/s | 32.363 s | ×1002.06 | 31.99 s | 99 % | 77 Mo | ×25.74 | 18919 / 0 | 0 / 0 | ✅ |

## lines (n = 1 000 000)

Débit = lignes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (30 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 10.99 M/s | 0.091 s | ×1.00 | 0.09 s | 94 % | 31 Mo | ×1.01 | 11648 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 8.62 M/s | 0.116 s | ×1.27 | 0.11 s | 96 % | 31 Mo | ×1.03 | 11657 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 4.01 M/s | 0.249 s | ×2.74 | 0.33 s | 99 % | 151 Mo | ×4.98 | 33191 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 130.70 k/s | 7.651 s | ×84.06 | 7.59 s | 99 % | 184 Mo | ×6.08 | 46307 / 0 | 0 / 0 | ✅ |

## hashmap (n = 500 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (16 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 28.23 M/s | 0.053 s | ×1.00 | 0.06 s | 102 % | 18 Mo | ×1.09 | 4176 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 25.58 M/s | 0.059 s | ×1.10 | 0.06 s | 96 % | 18 Mo | ×1.12 | 8278 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 7.06 M/s | 0.212 s | ×4.00 | 0.28 s | 97 % | 171 Mo | ×10.68 | 38334 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 457.40 k/s | 3.279 s | ×61.72 | 3.25 s | 99 % | 46 Mo | ×2.90 | 10980 / 0 | 0 / 0 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| Rust + Creusot | ×1.08 | ×1.07 | ×1.06 | 5 |
| F* (Pulse) → C | ×1.08 | ×1.08 | ×1.00 | 5 |
| F* (Pulse) → F# | ×3.53 | ×4.41 | ×5.74 | 5 |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | ×120.12 | ×116.62 | ×6.49 | 5 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 4 ms | 2.0 Mo |
| F* (Pulse) → C | 5 ms | 1.7 Mo |
| F* (Pulse) → F# | 86 ms | 29.4 Mo |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 64 ms | 7.6 Mo |

## Empreinte disque et coût de construction

| implémentation | exécutable | runtime/deps requis | compilation | vérification |
|---|--:|--:|--:|--:|
| Rust + Creusot | 0.38 Mo | 0.0 Mo | 10.2 s | 12.7 s |
| F* (Pulse) → C | 0.02 Mo | 0.0 Mo | 2.4 s | 153.7 s |
| F* (Pulse) → OCaml (runtime livré) | 4.96 Mo | 0.0 Mo | 2.8 s | 153.7 s |
| F* (Pulse) → F# | 0.05 Mo | 72.4 Mo | 10.0 s | 153.7 s |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 4.94 Mo | 0.0 Mo | 4.1 s | 153.7 s |
