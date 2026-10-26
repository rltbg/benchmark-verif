# Résultats — preset `large`

_2026-10-26 13:34:05 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v153 · médiane de 2 exécutions_

## sort (n = 32 000 000)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (244 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C (main, non vérifié) | 8.06 M/s | 3.969 s | ×1.00 | 3.91 s | 98 % | 245 Mo | ×1.01 | 62568 / 0 | 0 / 0 | ✅ |
| Rust + contrats Creusot | 7.14 M/s | 4.484 s | ×1.13 | 4.40 s | 98 % | 246 Mo | ×1.01 | 62578 / 0 | 0 / 0 | ✅ |
| F# (main, non vérifié) | 5.79 M/s | 5.522 s | ×1.39 | 5.53 s | 99 % | 271 Mo | ×1.11 | 64510 / 0 | 0 / 36 | ✅ |
| OCaml (main, non vérifié) | 5.38 M/s | 5.945 s | ×1.50 | 5.78 s | 96 % | 501 Mo | ×2.05 | 129788 / 0 | 0 / 0 | ✅ |

## sieve (n = 1 000 000 000)

Débit = entiers/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (954 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust + contrats Creusot | 47.08 M/s | 21.241 s | ×1.00 | 20.93 s | 99 % | 955 Mo | ×1.00 | 244217 / 0 | 0 / 0 | ✅ |
| F# (main, non vérifié) | 47.05 M/s | 21.255 s | ×1.00 | 21.00 s | 98 % | 981 Mo | ×1.03 | 246141 / 0 | 0 / 36 | ✅ |
| C (main, non vérifié) | 45.79 M/s | 21.841 s | ×1.03 | 21.50 s | 98 % | 955 Mo | ×1.00 | 244208 / 0 | 0 / 0 | ✅ |
| OCaml (main, non vérifié) | 43.00 M/s | 23.258 s | ×1.09 | 23.03 s | 99 % | 973 Mo | ×1.02 | 252514 / 0 | 0 / 0 | ✅ |

## matmul (n = 2 500)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (72 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C (main, non vérifié) | 3.29 G/s | 4.752 s | ×1.00 | 4.65 s | 98 % | 73 Mo | ×1.02 | 24483 / 0 | 0 / 0 | ✅ |
| Rust + contrats Creusot | 2.92 G/s | 5.344 s | ×1.12 | 5.29 s | 99 % | 73 Mo | ×1.03 | 24482 / 0 | 0 / 0 | ✅ |
| F# (main, non vérifié) | 415.72 M/s | 37.586 s | ×7.91 | 36.99 s | 98 % | 99 Mo | ×1.39 | 26438 / 0 | 0 / 68 | ✅ |
| OCaml (main, non vérifié) | 228.30 M/s | 68.441 s | ×14.40 | 67.14 s | 98 % | 149 Mo | ×2.09 | 37826 / 0 | 0 / 0 | ✅ |

## lines (n = 40 000 000)

Débit = lignes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (588 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C (main, non vérifié) | 16.85 M/s | 2.373 s | ×1.00 | 2.33 s | 98 % | 545 Mo | ×0.93 | 143312 / 0 | 0 / 0 | ✅ |
| Rust + contrats Creusot | 12.87 M/s | 3.109 s | ×1.31 | 3.08 s | 99 % | 545 Mo | ×0.93 | 143322 / 0 | 0 / 0 | ✅ |
| F# (main, non vérifié) | 11.76 M/s | 3.400 s | ×1.43 | 3.49 s | 99 % | 571 Mo | ×0.97 | 145262 / 0 | 0 / 0 | ✅ |
| OCaml (main, non vérifié) | 6.62 M/s | 6.045 s | ×2.55 | 5.94 s | 98 % | 557 Mo | ×0.95 | 143938 / 0 | 0 / 0 | ✅ |

## hashmap (n = 16 000 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (512 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C (main, non vérifié) | 15.32 M/s | 3.132 s | ×1.00 | 3.08 s | 98 % | 513 Mo | ×1.00 | 262212 / 0 | 0 / 0 | ✅ |
| Rust + contrats Creusot | 14.71 M/s | 3.264 s | ×1.04 | 3.24 s | 99 % | 514 Mo | ×1.00 | 262221 / 0 | 0 / 0 | ✅ |
| F# (main, non vérifié) | 11.74 M/s | 4.089 s | ×1.31 | 4.11 s | 98 % | 539 Mo | ×1.05 | 263620 / 0 | 0 / 0 | ✅ |
| OCaml (main, non vérifié) | 8.12 M/s | 5.914 s | ×1.89 | 5.87 s | 99 % | 517 Mo | ×1.01 | 131764 / 0 | 0 / 0 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| C (main, non vérifié) | ×1.01 | ×1.01 | ×1.00 | 5 |
| Rust + contrats Creusot | ×1.12 | ×1.12 | ×1.00 | 5 |
| F# (main, non vérifié) | ×1.83 | ×1.86 | ×1.11 | 5 |
| OCaml (main, non vérifié) | ×2.58 | ×2.58 | ×1.34 | 5 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| C (main, non vérifié) | 2 ms | 0.2 Mo |
| Rust + contrats Creusot | 3 ms | 0.4 Mo |
| OCaml (main, non vérifié) | 5 ms | 3.4 Mo |
| F# (main, non vérifié) | 71 ms | 27.1 Mo |

## Empreinte disque et coût de construction

| implémentation | exécutable | runtime/deps requis | compilation | vérification |
|---|--:|--:|--:|--:|
| C (main, non vérifié) | 0.02 Mo | 0.0 Mo | 0.6 s | — |
| Rust + contrats Creusot | 0.38 Mo | 0.0 Mo | 6.3 s | — |
| OCaml (main, non vérifié) | 1.56 Mo | 0.0 Mo | 0.3 s | — |
| F# (main, non vérifié) | 0.01 Mo | 66.8 Mo | 2.9 s | — |
