# Résultats — preset `medium`

_2026-10-26 17:20:26 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v153 · médiane de 3 exécutions_

## sort (n = 8 000 000)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (61 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 7.46 M/s | 1.072 s | ×1.00 | 1.06 s | 98 % | 62 Mo | ×1.02 | 15706 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 7.28 M/s | 1.099 s | ×1.03 | 1.10 s | 100 % | 63 Mo | ×1.03 | 15713 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 4.56 M/s | 1.753 s | ×1.64 | 1.78 s | 98 % | 109 Mo | ×1.78 | 22321 / 0 | 0 / 0 | ✅ |

## sieve (n = 200 000 000)

Débit = entiers/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (191 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 61.79 M/s | 3.237 s | ×1.00 | 3.17 s | 98 % | 192 Mo | ×1.01 | 48908 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 59.65 M/s | 3.353 s | ×1.04 | 3.32 s | 99 % | 193 Mo | ×1.01 | 48916 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 48.65 M/s | 4.111 s | ×1.27 | 4.15 s | 99 % | 219 Mo | ×1.15 | 50731 / 0 | 0 / 0 | ✅ |

## matmul (n = 1 600)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (29 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust + Creusot | 3.24 G/s | 1.266 s | ×1.00 | 1.24 s | 98 % | 31 Mo | ×1.06 | 9989 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 3.02 G/s | 1.357 s | ×1.07 | 1.34 s | 98 % | 31 Mo | ×1.05 | 10083 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 807.05 M/s | 5.075 s | ×4.01 | 5.03 s | 98 % | 115 Mo | ×3.93 | 24069 / 0 | 0 / 0 | ✅ |

## lines (n = 10 000 000)

Débit = lignes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (159 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 15.58 M/s | 0.642 s | ×1.00 | 0.66 s | 102 % | 149 Mo | ×0.94 | 42035 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 12.44 M/s | 0.804 s | ×1.25 | 0.78 s | 97 % | 150 Mo | ×0.94 | 42043 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 7.37 M/s | 1.356 s | ×2.11 | 1.42 s | 99 % | 322 Mo | ×2.03 | 76962 / 0 | 0 / 0 | ✅ |

## hashmap (n = 4 000 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (128 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 18.41 M/s | 0.652 s | ×1.00 | 0.63 s | 96 % | 130 Mo | ×1.01 | 32849 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 16.31 M/s | 0.736 s | ×1.13 | 0.73 s | 99 % | 130 Mo | ×1.02 | 65623 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 6.38 M/s | 1.881 s | ×2.88 | 1.92 s | 98 % | 291 Mo | ×2.27 | 69021 / 0 | 0 / 0 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| F* (Pulse) → C | ×1.01 | ×1.01 | ×1.00 | 5 |
| Rust + Creusot | ×1.08 | ×1.09 | ×1.01 | 5 |
| F* (Pulse) → F# | ×2.19 | ×2.26 | ×2.05 | 5 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 4 ms | 2.0 Mo |
| F* (Pulse) → C | 4 ms | 1.7 Mo |
| F* (Pulse) → F# | 78 ms | 29.3 Mo |

## Empreinte disque et coût de construction

| implémentation | exécutable | runtime/deps requis | compilation | vérification |
|---|--:|--:|--:|--:|
| Rust + Creusot | 0.38 Mo | 0.0 Mo | 10.2 s | 12.7 s |
| F* (Pulse) → C | 0.02 Mo | 0.0 Mo | 2.4 s | 153.7 s |
| F* (Pulse) → OCaml (runtime livré) | 4.96 Mo | 0.0 Mo | 2.8 s | 153.7 s |
| F* (Pulse) → F# | 0.05 Mo | 72.4 Mo | 10.0 s | 153.7 s |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 4.94 Mo | 0.0 Mo | 4.1 s | 153.7 s |
