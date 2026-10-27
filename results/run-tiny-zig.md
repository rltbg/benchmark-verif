# Résultats — preset `tiny`

_2026-10-27 03:39:49 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v165 · médiane de 3 exécutions_

## sort (n = 200 000)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (2 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 9.37 M/s | 0.021 s | ×1.00 | 0.02 s | 94 % | 3 Mo | ×1.98 | 544 / 0 | 0 / 0 | ✅ |

## sieve (n = 5 000 000)

Débit = entiers/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (5 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 186.86 M/s | 0.027 s | ×1.00 | 0.03 s | 98 % | 6 Mo | ×1.31 | 1371 / 0 | 0 / 0 | ✅ |

## matmul (n = 128)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 4.99 G/s | 0.000 s | ×1.00 | 0.00 s | 54 % | 2 Mo | ×9.79 | 197 / 0 | 0 / 0 | ✅ |

## lines (n = 50 000)

Débit = lignes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (17 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 2.04 M/s | 0.025 s | ×1.00 | 0.03 s | 94 % | 18 Mo | ×1.08 | 8513 / 0 | 0 / 0 | ✅ |

## hashmap (n = 100 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (4 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 48.29 M/s | 0.006 s | ×1.00 | 0.01 s | 81 % | 5 Mo | ×1.35 | 1176 / 0 | 0 / 0 | ✅ |

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
