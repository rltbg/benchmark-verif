# Résultats — preset `medium`

_2026-10-27 02:42:48 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v165 · médiane de 3 exécutions_

## sort (n = 8 000 000)

Débit = éléments/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (61 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 7.03 M/s | 1.138 s | ×1.00 | 1.12 s | 98 % | 63 Mo | ×1.02 | 15777 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 6.44 M/s | 1.243 s | ×1.09 | 1.24 s | 100 % | 62 Mo | ×1.02 | 15780 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 6.33 M/s | 1.265 s | ×1.11 | 1.23 s | 97 % | 63 Mo | ×1.02 | 15778 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 5.76 M/s | 1.388 s | ×1.22 | 1.38 s | 99 % | 63 Mo | ×1.03 | 15787 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 5.56 M/s | 1.438 s | ×1.26 | 1.43 s | 99 % | 63 Mo | ×1.03 | 15787 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 4.18 M/s | 1.912 s | ×1.68 | 1.97 s | 99 % | 90 Mo | ×1.47 | 17583 / 0 | 0 / 0 | ✅ |

## sieve (n = 200 000 000)

Débit = entiers/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (191 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Rust (sans Creusot) | 66.15 M/s | 3.023 s | ×1.00 | 3.03 s | 100 % | 193 Mo | ×1.01 | 48992 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 65.58 M/s | 3.050 s | ×1.01 | 3.05 s | 100 % | 192 Mo | ×1.01 | 48983 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 65.47 M/s | 3.055 s | ×1.01 | 3.12 s | 100 % | 220 Mo | ×1.15 | 50780 / 0 | 0 / 16 | ✅ |
| F* (Pulse) → C | 64.39 M/s | 3.106 s | ×1.03 | 3.07 s | 99 % | 192 Mo | ×1.01 | 48982 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 63.80 M/s | 3.135 s | ×1.04 | 3.12 s | 99 % | 192 Mo | ×1.01 | 48981 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 60.67 M/s | 3.297 s | ×1.09 | 3.27 s | 99 % | 193 Mo | ×1.01 | 48990 / 0 | 0 / 0 | ✅ |

## matmul (n = 1 600)

Débit = mult-add/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (29 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| C natif (à la main) | 6.03 G/s | 0.679 s | ×1.00 | 0.66 s | 97 % | 31 Mo | ×1.05 | 10156 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 6.02 G/s | 0.681 s | ×1.00 | 0.67 s | 98 % | 31 Mo | ×1.07 | 10166 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 5.83 G/s | 0.703 s | ×1.03 | 0.70 s | 99 % | 31 Mo | ×1.05 | 10156 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 5.17 G/s | 0.793 s | ×1.17 | 0.78 s | 98 % | 31 Mo | ×1.07 | 10064 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 2.44 G/s | 1.679 s | ×2.47 | 1.67 s | 99 % | 31 Mo | ×1.05 | 7655 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 1.03 G/s | 3.969 s | ×5.85 | 4.01 s | 99 % | 58 Mo | ×1.99 | 9464 / 0 | 0 / 0 | ✅ |

## lines (n = 10 000 000)

Débit = lignes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (159 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → C | 16.57 M/s | 0.603 s | ×1.00 | 0.60 s | 99 % | 149 Mo | ×0.94 | 42109 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 15.70 M/s | 0.637 s | ×1.06 | 0.65 s | 100 % | 150 Mo | ×0.94 | 42118 / 0 | 0 / 0 | ✅ |
| Zig 0.17 (à la main) | 12.27 M/s | 0.815 s | ×1.35 | 0.81 s | 99 % | 149 Mo | ×0.94 | 42107 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 10.90 M/s | 0.917 s | ×1.52 | 0.90 s | 98 % | 149 Mo | ×0.94 | 42109 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 10.40 M/s | 0.961 s | ×1.59 | 0.95 s | 99 % | 150 Mo | ×0.94 | 42117 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 8.57 M/s | 1.167 s | ×1.93 | 1.24 s | 100 % | 188 Mo | ×1.18 | 42676 / 0 | 0 / 0 | ✅ |

## hashmap (n = 4 000 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (128 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| Zig 0.17 (à la main) | 21.67 M/s | 0.554 s | ×1.00 | 0.56 s | 100 % | 130 Mo | ×1.01 | 32920 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → C | 20.59 M/s | 0.583 s | ×1.05 | 0.57 s | 97 % | 130 Mo | ×1.01 | 32923 / 0 | 0 / 0 | ✅ |
| Rust (sans Creusot) | 18.83 M/s | 0.637 s | ×1.15 | 0.63 s | 98 % | 130 Mo | ×1.02 | 65698 / 0 | 0 / 0 | ✅ |
| C natif (à la main) | 18.83 M/s | 0.637 s | ×1.15 | 0.63 s | 98 % | 129 Mo | ×1.01 | 65691 / 0 | 0 / 0 | ✅ |
| Rust + Creusot | 17.26 M/s | 0.695 s | ×1.26 | 0.69 s | 98 % | 130 Mo | ×1.02 | 65698 / 0 | 0 / 0 | ✅ |
| F* (Pulse) → F# | 11.70 M/s | 1.025 s | ×1.85 | 1.08 s | 97 % | 157 Mo | ×1.23 | 34719 / 0 | 0 / 0 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| F* (Pulse) → C | ×1.04 | ×1.04 | ×1.00 | 5 |
| Rust (sans Creusot) | ×1.08 | ×1.09 | ×1.01 | 5 |
| C natif (à la main) | ×1.14 | ×1.13 | ×1.00 | 5 |
| Rust + Creusot | ×1.26 | ×1.26 | ×1.01 | 5 |
| Zig 0.17 (à la main) | ×1.28 | ×1.29 | ×1.00 | 5 |
| F* (Pulse) → F# | ×2.04 | ×2.13 | ×1.37 | 5 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| Rust + Creusot | 6 ms | 2.1 Mo |
| F* (Pulse) → C | 5 ms | 1.7 Mo |
| F* (Pulse) → F# | 68 ms | 28.9 Mo |
| Rust (sans Creusot) | 5 ms | 2.1 Mo |
| C natif (à la main) | 5 ms | 1.7 Mo |
| Zig 0.17 (à la main) | 6 ms | 1.7 Mo |

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
