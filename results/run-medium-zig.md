# Résultats — preset `medium`

_2026-10-27 03:39:51 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v165 · médiane de 3 exécutions_

## sort (n = 8 000 000)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (61 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 7.36 M/s | 1.087 s | ×1.00 | 1.09 s | 100 % | 63 Mo | ×1.02 | 15778 / 0 | 0 / 0 | ✅ |

## sieve (n = 200 000 000)

Débit = entiers/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (191 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 62.35 M/s | 3.208 s | ×1.00 | 3.18 s | 99 % | 192 Mo | ×1.01 | 48980 / 0 | 0 / 0 | ✅ |

## matmul (n = 1 600)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (29 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 5.83 G/s | 0.703 s | ×1.00 | 0.69 s | 98 % | 31 Mo | ×1.05 | 7654 / 0 | 0 / 0 | ✅ |

## lines (n = 10 000 000)

Débit = lignes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (159 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 12.18 M/s | 0.821 s | ×1.00 | 0.82 s | 100 % | 149 Mo | ×0.94 | 42108 / 0 | 0 / 0 | ✅ |

## hashmap (n = 4 000 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (128 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 20.48 M/s | 0.586 s | ×1.00 | 0.59 s | 100 % | 130 Mo | ×1.01 | 32921 / 0 | 0 / 0 | ✅ |

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
