# Résultats — preset `xlarge`

_2026-10-27 03:41:51 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v165 · médiane de 3 exécutions_

## sort (n = 100 000 000)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (763 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 5.64 M/s | 17.721 s | ×1.00 | 17.57 s | 99 % | 764 Mo | ×1.00 | 195465 / 0 | 0 / 0 | ✅ |

## sieve (n = 3 000 000 000)

Débit = entiers/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (2861 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 53.38 M/s | 56.201 s | ×1.00 | 55.82 s | 99 % | 2863 Mo | ×1.00 | 732573 / 0 | 0 / 0 | ✅ |

## matmul (n = 4 096)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (192 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 5.09 G/s | 13.503 s | ×1.00 | 13.42 s | 99 % | 194 Mo | ×1.01 | 49307 / 0 | 0 / 0 | ✅ |

## lines (n = 100 000 000)

Débit = lignes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (1447 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 9.01 M/s | 11.099 s | ×1.00 | 10.46 s | 94 % | 1336 Mo | ×0.92 | 345972 / 0 | 0 / 0 | ✅ |

## hashmap (n = 48 000 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (2048 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 5.86 M/s | 24.576 s | ×1.00 | 24.38 s | 99 % | 1026 Mo | ×0.50 | 262296 / 0 | 0 / 0 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| Zig 0.17 (à la main) | ×1.00 | ×1.00 | ×1.00 | 5 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Zig 0.17 (à la main) | 5 ms | 1.7 Mo |

## Empreinte disque et coût de construction

| implémentation | exécutable | runtime/deps requis | compilation | vérification |
|---|--:|--:|--:|--:|
| Rust + Creusot | 0.39 Mo | 0.0 Mo | 0.1 s | 16.3 s |
| F* (Pulse) → C | 0.02 Mo | 0.0 Mo | 4.0 s | 236.5 s |
| F* (Pulse) → OCaml (runtime livré) | 4.96 Mo | 0.0 Mo | 10.1 s | 236.5 s |
| F* (Pulse) → F# | 0.05 Mo | 72.4 Mo | 22.6 s | 236.5 s |
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 4.94 Mo | 0.0 Mo | 11.5 s | 236.5 s |
| Rust (sans Creusot) | 0.39 Mo | 0.0 Mo | 0.1 s | — |
| C natif (à la main) | 0.02 Mo | 0.0 Mo | 0.3 s | — |
| Zig 0.17 (à la main) | 0.07 Mo | 0.0 Mo | 0.3 s | — |
