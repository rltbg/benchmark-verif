# Résultats — preset `small`

_2026-10-27 16:05:40 · 4 CPU · 16091 Mo RAM · Linux vm 6.18.44-fc-v165 · médiane de 1 exécutions_

## alloc (n = 2 000 000)

Débit = allocations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (4 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 30.39 k/s | 65.816 s | ×1.00 | 65.02 s | 99 % | 116 Mo | ×27.70 | 28709 / 0 | 0 / 0 | ✅ |

## bfs (n = 1 000 000)

Débit = arêtes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (69 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 586.11 k/s | 81.896 s | ×1.00 | 80.98 s | 99 % | 797 Mo | ×11.61 | 203198 / 1 | 8 / 0 | ✅ |

## ntt (n = 1 048 576)

Débit = papillons/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (6 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 661.79 k/s | 31.689 s | ×1.00 | 31.24 s | 99 % | 132 Mo | ×21.97 | 32824 / 4 | 1880 / 0 | ✅ |

## ptrchase (n = 300 000)

Débit = opérations/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (6 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 203.06 k/s | 11.080 s | ×1.00 | 11.06 s | 100 % | 36 Mo | ×6.24 | 8244 / 0 | 0 / 0 | ✅ |

## sha256 (n = 20 000 000)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (0 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 433.57 k/s | 46.129 s | ×1.00 | 45.62 s | 99 % | 9 Mo | ×9260.00 | 1372 / 1 | 16 / 0 | ✅ |

## stream (n = 256)

Débit = octets/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (1 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | **timeout** | | | | | | | | | |

## strsort (n = 300 000)

Débit = chaînes/s mesuré à l'intérieur du processus (hors démarrage du runtime). RAM/idéal = pic RSS ÷ taille des structures de la spécification (9 Mo).

| implémentation | débit | temps | vs meilleur | CPU (user+sys) | CPU % | pic RAM | RAM/idéal | fautes de page | E/S disque (blocs r/w) | checksum |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|:-:|
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 19.09 k/s | 15.717 s | ×1.00 | 15.56 s | 99 % | 69 Mo | ×7.34 | 16810 / 1 | 8 / 0 | ✅ |

## Synthèse (moyenne géométrique sur les workloads, 1.00 = meilleur de la ligne)

| implémentation | temps | CPU | pic RAM | workloads |
|---|--:|--:|--:|--:|
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | ×1.00 | ×1.00 | ×1.00 | 6 |

## Coût de démarrage (run minuscule, médiane de 3)

| implémentation | durée | RAM |
|---|--:|--:|
| F* (Pulse) → OCaml (conversions d'entiers corrigées) | 73 ms | 8.7 Mo |

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
