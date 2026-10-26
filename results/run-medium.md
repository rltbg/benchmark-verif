# Résultats — preset `medium`

_2026-10-26 13:42:37 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v153 · médiane de 3 exécutions_

## sort (n = 8 000 000)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (61 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C (main, non vérifié) | 8.15 M/s | 0.982 s | ×1.00 | 0.92 s | 93 % | 62 Mo | ×1.02 | 15693 / 0 | 0 / 0 | ✅ |
| Rust + contrats Creusot | 7.39 M/s | 1.082 s | ×1.10 | 1.07 s | 98 % | 63 Mo | ×1.03 | 15703 / 0 | 0 / 0 | ✅ |
| F# (main, non vérifié) | 6.26 M/s | 1.278 s | ×1.30 | 1.32 s | 98 % | 88 Mo | ×1.44 | 17630 / 0 | 0 / 0 | ✅ |
| OCaml (main, non vérifié) | 6.23 M/s | 1.285 s | ×1.31 | 1.27 s | 98 % | 129 Mo | ×2.12 | 32963 / 0 | 0 / 0 | ✅ |

## sieve (n = 200 000 000)

Débit = entiers/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (191 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust + contrats Creusot | 59.00 M/s | 3.390 s | ×1.00 | 3.30 s | 97 % | 192 Mo | ×1.01 | 48906 / 0 | 0 / 0 | ✅ |
| C (main, non vérifié) | 58.72 M/s | 3.406 s | ×1.00 | 3.36 s | 98 % | 192 Mo | ×1.01 | 48896 / 0 | 0 / 0 | ✅ |
| F# (main, non vérifié) | 58.19 M/s | 3.437 s | ×1.01 | 3.44 s | 98 % | 218 Mo | ×1.14 | 50822 / 0 | 0 / 0 | ✅ |
| OCaml (main, non vérifié) | 57.85 M/s | 3.457 s | ×1.02 | 3.46 s | 100 % | 196 Mo | ×1.03 | 50031 / 0 | 0 / 0 | ✅ |

## matmul (n = 1 600)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (29 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C (main, non vérifié) | 3.24 G/s | 1.262 s | ×1.00 | 1.26 s | 99 % | 31 Mo | ×1.05 | 10070 / 0 | 0 / 0 | ✅ |
| Rust + contrats Creusot | 2.86 G/s | 1.432 s | ×1.13 | 1.40 s | 97 % | 31 Mo | ×1.06 | 9980 / 0 | 0 / 0 | ✅ |
| F# (main, non vérifié) | 685.21 M/s | 5.978 s | ×4.74 | 5.92 s | 97 % | 57 Mo | ×1.94 | 12023 / 0 | 0 / 0 | ✅ |
| OCaml (main, non vérifié) | 326.97 M/s | 12.527 s | ×9.92 | 12.41 s | 99 % | 64 Mo | ×2.19 | 15947 / 0 | 0 / 0 | ✅ |

## lines (n = 10 000 000)

Débit = lignes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (159 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C (main, non vérifié) | 14.06 M/s | 0.711 s | ×1.00 | 0.69 s | 97 % | 149 Mo | ×0.94 | 42023 / 0 | 0 / 0 | ✅ |
| Rust + contrats Creusot | 12.60 M/s | 0.794 s | ×1.12 | 0.79 s | 99 % | 150 Mo | ×0.94 | 42033 / 0 | 0 / 0 | ✅ |
| F# (main, non vérifié) | 11.96 M/s | 0.836 s | ×1.18 | 0.90 s | 98 % | 175 Mo | ×1.10 | 43969 / 0 | 0 / 0 | ✅ |
| OCaml (main, non vérifié) | 7.59 M/s | 1.318 s | ×1.85 | 1.28 s | 97 % | 155 Mo | ×0.98 | 39574 / 0 | 0 / 0 | ✅ |

## hashmap (n = 4 000 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (128 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C (main, non vérifié) | 19.21 M/s | 0.625 s | ×1.00 | 0.62 s | 99 % | 129 Mo | ×1.01 | 65602 / 0 | 0 / 0 | ✅ |
| Rust + contrats Creusot | 15.41 M/s | 0.779 s | ×1.25 | 0.77 s | 99 % | 130 Mo | ×1.01 | 65613 / 0 | 0 / 0 | ✅ |
| F# (main, non vérifié) | 14.45 M/s | 0.830 s | ×1.33 | 0.90 s | 97 % | 155 Mo | ×1.21 | 67414 / 0 | 0 / 0 | ✅ |
| OCaml (main, non vérifié) | 10.31 M/s | 1.164 s | ×1.86 | 1.12 s | 95 % | 133 Mo | ×1.04 | 33460 / 0 | 0 / 0 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| C (main, non vérifié) | ×1.00 | ×1.00 | ×1.00 | 5 |
| Rust + contrats Creusot | ×1.12 | ×1.13 | ×1.01 | 5 |
| F# (main, non vérifié) | ×1.58 | ×1.68 | ×1.33 | 5 |
| OCaml (main, non vérifié) | ×2.15 | ×2.17 | ×1.37 | 5 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| C (main, non vérifié) | 2 ms | 0.3 Mo |
| Rust + contrats Creusot | 2 ms | 0.3 Mo |
| OCaml (main, non vérifié) | 3 ms | 0.6 Mo |
| F# (main, non vérifié) | 68 ms | 27.2 Mo |

## Empreinte disque et coût de construction

| implémentation | exécutable | runtime/deps requis | compilation | vérification |
|---|--:|--:|--:|--:|
| C (main, non vérifié) | 0.02 Mo | 0.0 Mo | 0.6 s | — |
| Rust + contrats Creusot | 0.38 Mo | 0.0 Mo | 6.3 s | — |
| OCaml (main, non vérifié) | 1.56 Mo | 0.0 Mo | 0.3 s | — |
| F# (main, non vérifié) | 0.01 Mo | 66.8 Mo | 2.9 s | — |
