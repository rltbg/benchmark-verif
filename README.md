# Programmation vérifiée : F* (→ C, OCaml, F#) contre Rust + Creusot

Question : *« le code vérifié tient-il la route, en CPU, en RAM, en disque, et surtout en débit ? »*

Ce dépôt compare **huit colonnes**, sur **cinq workloads identiques** (spécification dans
[`spec/WORKLOADS.md`](spec/WORKLOADS.md)), avec un harnais de mesure externe (temps, CPU, pic RAM et courbe,
défauts de page, E/S disque, taille des binaires, coût de démarrage). Chaque colonne doit produire le **même
checksum** ; sinon le résultat est marqué ❌.

| colonne | source | vérifiée ? | extraction / compilation |
|---|---|---|---|
| `fstar-c` | `fstar/src/*.fst` (Pulse) | **oui (F\*)** | `fstar.exe --codegen C` → `gcc -O3` |
| `fstar-fsharp` | **les mêmes** sources F* | oui | `fstar.exe --codegen FSharp` → .NET 10 |
| `fstar-ocaml` / `fstar-ocaml-fast` | **les mêmes** sources F* | oui | `fstar.exe --codegen OCaml` → `ocamlopt` (runtime livré / conversions corrigées) |
| `rust-creusot` | `impls/rust` (Rust annoté Creusot) | **oui (Creusot)** | `cargo build --release` (les contrats sont effacés) |
| `rust-plain` | `impls/rust-plain` (Rust idiomatique, sans Creusot) | non | `cargo build --release` (opt 3, LTO, `panic=abort`) |
| `c-native` | `impls/c-native/bench.c` (C11 écrit à la main) | non | `gcc -O3 -std=c11` |
| `zig` | `impls/zig/bench.zig` (**Zig 0.17.0**, à la main) | non | `zig build-exe -O ReleaseFast -lc` |

Les trois colonnes **non vérifiées** (`rust-plain`, `c-native`, `zig`) sont des *références* demandées explicitement :
même algorithme, mêmes graines, même checksum, écrites sans preuve. Elles servent à répondre à « combien coûte le fait de
passer par F* ou par Creusot ? » en comparant à ce qu'on obtient sans. Les colonnes F* restent **100 % générées** (aucune
implémentation F* écrite à la main dans un autre langage).

## Réponses aux deux questions de fond

### « Pourquoi des implémentations à la main ? »

Dans une version précédente j'avais mis des ports « à la main » de F\* en OCaml/F#/C à la place des extractions ; vous aviez
(à raison) refusé : les colonnes F\* → C / F# / OCaml sont maintenant **toutes générées par F\*** à partir des mêmes
sources vérifiées. Les trois colonnes à la main d'aujourd'hui sont d'une autre nature, et demandées par vous : un **C natif**, un
**Zig 0.17** et un **Rust sans Creusot**, comme *références* de ce qu'un programmeur obtient sans vérification. Un seul
oracle de checksums existe aussi aux petites tailles ([`spec/checksums.json`](spec/checksums.json)).

### « Que veut dire Rust + contrats Creusot ? »

* **Creusot** est un outil de *vérification déductive* pour Rust. On écrit dans le code Rust des **contrats** :
  `#[requires(…)]` (précondition), `#[ensures(…)]` (postcondition), `#[invariant(…)]` (invariant de boucle),
  `#[variant(…)]` (terminaison), et des fonctions de spécification en logique (`#[logic]`). `cargo creusot`
  traduit le MIR en Why3/Coma, génère des obligations de preuve et les confie à des solveurs SMT
  (Alt-Ergo, Z3, CVC4/5).
* Un `cargo build` ordinaire **ignore** ces annotations : le binaire est du Rust normal. Le surcoût à l'exécution
  vient uniquement des choix de code que la preuve impose, jamais des preuves elles-mêmes.
* Ce qui est **réellement prouvé** ici (`make creusot`, voir « Niveau de preuve »): absence de panique et de
  débordement arithmétique pour tout le crate, terminaison de tout ce qui n'alloue pas, et — pour le tri —
  la **correction fonctionnelle complète** (le résultat est trié *et* est une permutation de l'entrée, rien
  d'autre n'est modifié). Les preuves sont exécutées (Alt-Ergo/Z3/CVC5) : un invariant affaibli fait échouer le
  but correspondant, je l'ai vérifié.
* Dans la version précédente du dépôt, les contrats étaient écrits mais **jamais prouvés** (Creusot n'était pas
  installé). Ce n'est plus le cas.

## Niveau de preuve (ce qui est démontré, et ce qui ne l'est pas)

| workload | F* (Pulse) | Rust (Creusot) |
|---|---|---|
| `sort` | **fonctionnel complet** : trié ∧ permutation ∧ cadre ; mémoire, débordements, terminaison | **fonctionnel complet** (idem) ; terminaison sauf l'allocation |
| `sieve` | mémoire sûre, indices, débordements, terminaison | idem |
| `matmul` | idem + indices non linéaires `i·n+j < n²` | idem |
| `lines` | idem ; l'analyseur est gardé par `p < len` | idem |
| `hashmap` | idem ; sondage borné par `cap` essais | idem |

**Limites assumées.** Hors du tri, je n'ai pas prouvé la correction « métier » (crible = nombres premiers,
produit matriciel = somme des produits, table = dictionnaire, analyse de lignes = somme par clé). Ce sont des
preuves de plusieurs jours chacune ; elles n'ajouteraient pas une instruction au code généré, seulement du temps
de preuve. Le checksum, comparé entre les huit colonnes, tient lieu de test de non-régression pour ces quatre-là.
Les corps de `mix64` et `ckfold` sont des opérations modulo 2^64 sans contrat.

## Ce que fait réellement F* 2026.10.25 (constats)

F* a changé : `LowStar.Buffer`/KaRaMeL ont disparu de la ulib ; la programmation de bas niveau passe par
**Pulse** (logique de séparation, `Pulse.Lib.Vec`, boucles `while` avec invariants) et l'extracteur
**Custard** (extraction « programme entier », monomorphisante).

* **C** : Custard produit directement du C11 autonome (boucles `while`, `malloc`/`free`, aucun runtime). Sur le
  tri et le crible, c'est exactement le C qu'on écrirait à la main.
* **F#** : les entiers machine deviennent les types .NET natifs (`uint32`, `uint64`, `byte[]`), sans bibliothèque de
  support. C'est le backend le plus proche du C en sémantique ; il refuse tout `Prims.int` à l'exécution.
* **OCaml** : le backend OCaml, lui, conserve le modèle de la bibliothèque F* : `FStar_SizeT.t` est un type boxé
  (`Sz of FStar_UInt64.t`, lui-même un `Stdint.Uint64.t`), les index passent par `Z.to_int (FStar_SizeT.v p)`,
  et **`FStar_UIntN.v` convertit l'entier en chaîne décimale puis la reparse avec Zarith**
  (`Prims.parse_int (M.to_string x)`), de même pour `uint_to_t`. C'est la cause principale de la lenteur : chaque
  accès tableau fait un aller-retour par une chaîne de caractères. D'où deux colonnes :
  * `fstar-ocaml` : runtime exactement tel que livré (compilé depuis `ulib/ml` ; les `.cmxa` précompilés exigent une
    variante exacte d'OCaml 5.3, rejetée par un OCaml standard) ;
  * `fstar-ocaml-fast` : **mêmes sources extraites**, seules `v` et `uint_to_t` remplacées par des conversions
    directes (`fstar/ocaml/build-ocaml.sh fast`).
  Un essai avec flambda (`-O3`) change les temps de ±20 % dans les deux sens : pas de colonne dédiée.
* **Bibliothèque** : la bibliothèque Pulse fournit un tri fusion vérifié (`Pulse.Lib.Sort.Merge.*`) et une table de
  hachage vérifiée (`Pulse.Lib.HashTable`). Je les ai examinées et **n'ai pas pu utiliser le tri** : sa
  spécification (`spec_merge`) renvoie `false` dès que deux éléments se comparent égaux, donc il ne trie que des
  clés toutes distinctes ; mon workload (8 M de u32 aléatoires) contient des doublons. J'ai donc écrit un tri
  fusion de même nature mais avec doublons. `Pulse.Lib.Vec`, `FStar.SizeT`, `FStar.Seq.Properties`
  (`permutation`, `append_permutations`, …) et `FStar.Math.Lemmas` sont, eux, utilisés.

## Mise en route

```bash
scripts/setup-fstar.sh      # F* 2026.10.25, .NET 10, switch opam OCaml 5.3 (stdint, zarith, ppx_deriving...)
scripts/setup-creusot.sh    # Creusot (clone, opam, solveurs, cargo-creusot)
scripts/setup-zig.sh        # Zig 0.17.0 (sha256 vérifié) dans /opt/tools/zig

scripts/build-all.sh        # Rust (x2), C natif, Zig + F* -> C, F#, OCaml ; écrit results/footprint.json
make creusot                # preuves Creusot (durée enregistrée)
fstar/build.sh verify       # preuves F* (durée enregistrée)

python3 harness/bench.py list
python3 harness/bench.py run --preset medium --repeat 3 --out results/run-medium.json  # + .md/.html/.csv
python3 harness/bench.py run --preset small --impls fstar-ocaml --timeout 600 --out results/run-small-fstar-ocaml.json
python3 harness/readme_tables.py     # régénère les tableaux de la section Résultats
```

Presets : `smoke`, `tiny`, `small`, `medium`, `large`, `xlarge` (voir `harness/presets.json`). Options :
`--repeat N`, `--timeout S`, `--impls a,b`, `--workloads w,…`, `--wrapper "taskset -c 2"` ou
`--wrapper "systemd-run --scope -p MemoryMax=2G --user"` pour plafonner la RAM.

## Organisation

```
spec/WORKLOADS.md        les 5 workloads (algorithmes, graines, checksums, tailles) ; spec/checksums.json
harness/bench.py         lanceur + mesures + rapport ; runner.c (fork/exec/wait4) ; impls.json ; presets.json
impls/rust               crate Rust + contrats Creusot (src/{common,sort,sieve,matmul,lines,hashmap}.rs)
impls/rust-plain         même algorithmes en Rust idiomatique, sans Creusot (src/main.rs)
impls/c-native           C11 écrit à la main (bench.c)
impls/zig                Zig 0.17 écrit à la main (bench.zig)
fstar/src                Bench.{Common,SortSpec,Sort,Sieve,Matmul,Lines,Hashmap,All}.fst(i) — Pulse
fstar/build.sh           verify -> extraction Custard -> C | F# | OCaml
fstar/ocaml/build-ocaml.sh   compile le code OCaml extrait avec le runtime F* (stock | fast)
fstar/drivers            pilotes C / F# / OCaml (parse des arguments, chronométrage, affichage)
scripts/                 installation et construction ; results/ : sorties
```

## Méthodologie de mesure

* **Débit** : mesuré *dans* le processus autour du calcul (hors démarrage du runtime) ; le démarrage est rapporté
  à part (run minuscule, médiane de 3).
* **CPU / RAM / disque** : mesurés par un petit lanceur C (`harness/runner.c`) qui fait `fork`/`exec`/`wait4`
  (user, sys, `ru_maxrss`, défauts de page, blocs E/S) plus un échantillonneur `/proc/<pid>` toutes les 20 ms
  (courbe de RSS). Le lanceur est nécessaire : `ru_maxrss` d'un enfant hérite du RSS du parent au `fork`, ce qui
  ajoutait ~15 Mo de plancher parasite quand le parent était l'interpréteur Python.
* **RAM/idéal** = pic RSS ÷ taille des structures de la spécification (voir `spec/WORKLOADS.md`).
* Médiane de N exécutions ; checksums vérifiés entre chaînes et contre `spec/checksums.json` (✅/❌).
* Limites : une VM de 4 cœurs, pas d'épinglage ni de gouverneur CPU figé ; les comparaisons relatives sont fiables à
  quelques pourcents, pas les valeurs absolues. Aucun workload ne lit de fichier : le « disque » est mesuré par les
  compteurs d'E/S du noyau (nuls) et par la taille des binaires/runtimes.
* **Réglages pour la performance maximale** (appliqués à *toutes* les colonnes natives, par `scripts/build-all.sh` et `fstar/build.sh`) :
  `gcc -O3 -march=native` (C natif et F*→C) ; `rustc -C opt-level=3 -C target-cpu=native` + LTO, `codegen-units=1`, `panic=abort`
  (`.cargo/config.toml` des deux crates) ; Zig `-O ReleaseFast -mcpu=native` ; .NET 10 Release avec `TieredPGO` et
  `TieredCompilationQuickJitForLoops=false` (les boucles sont compilées en pleine optimisation d'emblée : sans cela `matmul`
  était ~30 % plus lent, mesuré) ; OCaml `ocamlopt -inline 200 -unsafe` (sans flambda : ±20 %, pas de colonne dédiée). Le CPU de la
  machine a AVX2 et AVX-512 : les binaires natifs ne sont **pas portables** vers une machine plus ancienne. Toutes les colonnes natives
  sont épinglées sur un cœur (`taskset -c 1`) pour éviter les migrations. Le premier jeu de mesures utilisait une cible x86-64
  générique ; passer en `-march=native` a, par exemple, divisé par ~1,8 le temps de `matmul` en C et en Rust.
* **Zig, `matmul`** : la version idiomatique (`for (crow, brow) |*c, b| c.* += a*b`) n'était pas auto-vectorisée par LLVM (2,3 G/s à `large`,
  contre 6 en C) ; j'ai écrit la boucle interne avec `@Vector(16, u32)` et `noalias` (4,9 G/s). C'est le seul endroit où j'ai adapté le code à un
  compilateur ; le checksum est inchangé.

## Résultats

Machine : VM de 4 cœurs, 16 Go. Chaque cellule : débit · durée interne · pic RAM. Colonnes natives : médiane de 3 exécutions à toutes les tailles, épinglées sur un cœur (`--wrapper "taskset -c 1"`), binaires compilés pour le CPU de la machine (voir « Réglages pour la performance maximale »). OCaml : 1 exécution, timeout 600 s, épinglé sur un autre cœur ; les deux OCaml tournaient en parallèle ; `xlarge` : timeout 1800 s. **490 exécutions, 479 réussies, 11 sans résultat (timeouts OCaml et un échec F#), aucun
checksum divergent** entre colonnes ni avec la table de référence. Rapports complets : `results/run-*.{md,html,csv}`.

#### `xlarge`

| workload (n) | C natif | Zig 0.17 | Rust | Rust + Creusot | F*→C | F*→F# |
|---|--:|--:|--:|--:|--:|--:|
| sort (100 000 000) | 5.40 M/s · 18.54 s · 764 Mo | 5.64 M/s · 17.72 s · 764 Mo | 4.42 M/s · 22.61 s · 765 Mo | 4.47 M/s · 22.35 s · 765 Mo | 5.36 M/s · 18.64 s · 764 Mo | 3.58 M/s · 27.90 s · 792 Mo |
| sieve (3 000 000 000) | 51.30 M/s · 58.48 s · 2862 Mo | 53.38 M/s · 56.20 s · 2863 Mo | 49.32 M/s · 60.83 s · 2863 Mo | 54.21 M/s · 55.34 s · 2863 Mo | 56.02 M/s · 53.55 s · 2862 Mo | **échec** |
| matmul (4 096) | 5.94 G/s · 11.57 s · 193 Mo | 5.09 G/s · 13.50 s · 194 Mo | 4.91 G/s · 13.99 s · 194 Mo | 5.54 G/s · 12.41 s · 194 Mo | 5.13 G/s · 13.40 s · 193 Mo | 877.06 M/s · 78.35 s · 221 Mo |
| lines (100 000 000) | 10.32 M/s · 9.69 s · 1336 Mo | 9.01 M/s · 11.10 s · 1336 Mo | 10.36 M/s · 9.65 s · 1337 Mo | 9.47 M/s · 10.56 s · 1337 Mo | 11.58 M/s · 8.64 s · 1336 Mo | 7.12 M/s · 14.05 s · 1476 Mo |
| hashmap (48 000 000) | 6.07 M/s · 23.72 s · 1025 Mo | 5.86 M/s · 24.58 s · 1026 Mo | 5.97 M/s · 24.14 s · 1026 Mo | 6.63 M/s · 21.73 s · 1026 Mo | 5.86 M/s · 24.56 s · 1026 Mo | 4.38 M/s · 32.87 s · 1053 Mo |

Moyenne géométrique du temps relatif au meilleur : F*→C ×1.07 (5/5), C natif ×1.07 (5/5), Rust + Creusot ×1.11 (5/5), Zig 0.17 ×1.12 (5/5), Rust ×1.17 (5/5), F*→F# ×2.26 (4/5).

#### `large`

| workload (n) | C natif | Zig 0.17 | Rust | Rust + Creusot | F*→C | F*→F# | F*→OCaml (corrigé) | F*→OCaml (livré) |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| sort (32 000 000) | 5.89 M/s · 5.43 s · 246 Mo | 6.58 M/s · 4.86 s · 246 Mo | 5.23 M/s · 6.12 s · 246 Mo | 4.90 M/s · 6.53 s · 246 Mo | 5.44 M/s · 5.89 s · 246 Mo | 3.56 M/s · 8.98 s · 273 Mo | 68.88 k/s · 464.57 s · 1267 Mo | délai dépassé |
| sieve (1 000 000 000) | 54.28 M/s · 18.42 s · 955 Mo | 54.53 M/s · 18.34 s · 955 Mo | 57.12 M/s · 17.51 s · 956 Mo | 53.19 M/s · 18.80 s · 956 Mo | 51.80 M/s · 19.30 s · 955 Mo | 53.32 M/s · 18.75 s · 983 Mo | 2.05 M/s · 487.39 s · 7648 Mo | délai dépassé |
| matmul (2 500) | 6.13 G/s · 2.55 s · 73 Mo | 4.91 G/s · 3.18 s · 73 Mo | 6.22 G/s · 2.51 s · 73 Mo | 4.70 G/s · 3.32 s · 73 Mo | 5.33 G/s · 2.93 s · 73 Mo | 1.04 G/s · 15.09 s · 101 Mo | délai dépassé | délai dépassé |
| lines (40 000 000) | 11.24 M/s · 3.56 s · 545 Mo | 10.42 M/s · 3.84 s · 545 Mo | 13.27 M/s · 3.01 s · 546 Mo | 8.21 M/s · 4.87 s · 546 Mo | 10.28 M/s · 3.89 s · 545 Mo | 8.80 M/s · 4.55 s · 617 Mo | 121.47 k/s · 329.29 s · 6469 Mo | délai dépassé |
| hashmap (16 000 000) | 17.14 M/s · 2.80 s · 513 Mo | 17.03 M/s · 2.82 s · 514 Mo | 14.11 M/s · 3.40 s · 514 Mo | 14.53 M/s · 3.30 s · 514 Mo | 15.06 M/s · 3.19 s · 514 Mo | 10.27 M/s · 4.68 s · 541 Mo | 392.43 k/s · 122.31 s · 1259 Mo | délai dépassé |

Moyenne géométrique du temps relatif au meilleur : C natif ×1.07 (5/5), Rust ×1.09 (5/5), Zig 0.17 ×1.11 (5/5), F*→C ×1.18 (5/5), Rust + Creusot ×1.29 (5/5), F*→F# ×1.97 (5/5), F*→OCaml (corrigé) ×59.68 (4/5).

#### `medium`

| workload (n) | C natif | Zig 0.17 | Rust | Rust + Creusot | F*→C | F*→F# | F*→OCaml (corrigé) | F*→OCaml (livré) |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| sort (8 000 000) | 6.33 M/s · 1.26 s · 63 Mo | 7.36 M/s · 1.09 s · 63 Mo | 5.76 M/s · 1.39 s · 63 Mo | 5.56 M/s · 1.44 s · 63 Mo | 6.44 M/s · 1.24 s · 62 Mo | 4.18 M/s · 1.91 s · 90 Mo | 85.29 k/s · 93.79 s · 322 Mo | délai dépassé |
| sieve (200 000 000) | 65.58 M/s · 3.05 s · 192 Mo | 62.35 M/s · 3.21 s · 192 Mo | 66.15 M/s · 3.02 s · 193 Mo | 60.67 M/s · 3.30 s · 193 Mo | 64.39 M/s · 3.11 s · 192 Mo | 65.47 M/s · 3.05 s · 220 Mo | 2.40 M/s · 83.31 s · 1536 Mo | 430.51 k/s · 464.56 s · 1592 Mo |
| matmul (1 600) | 6.03 G/s · 0.68 s · 31 Mo | 5.83 G/s · 0.70 s · 31 Mo | 6.02 G/s · 0.68 s · 31 Mo | 5.17 G/s · 0.79 s · 31 Mo | 5.83 G/s · 0.70 s · 31 Mo | 1.03 G/s · 3.97 s · 58 Mo | délai dépassé | délai dépassé |
| lines (10 000 000) | 10.90 M/s · 0.92 s · 149 Mo | 12.18 M/s · 0.82 s · 149 Mo | 15.70 M/s · 0.64 s · 150 Mo | 10.40 M/s · 0.96 s · 150 Mo | 16.57 M/s · 0.60 s · 149 Mo | 8.57 M/s · 1.17 s · 188 Mo | 132.24 k/s · 75.62 s · 1635 Mo | délai dépassé |
| hashmap (4 000 000) | 18.83 M/s · 0.64 s · 129 Mo | 20.48 M/s · 0.59 s · 130 Mo | 18.83 M/s · 0.64 s · 130 Mo | 17.26 M/s · 0.70 s · 130 Mo | 20.59 M/s · 0.58 s · 130 Mo | 11.70 M/s · 1.03 s · 157 Mo | 448.65 k/s · 26.75 s · 320 Mo | 61.42 k/s · 195.39 s · 347 Mo |

Moyenne géométrique du temps relatif au meilleur : F*→C ×1.04 (5/5), Rust ×1.08 (5/5), Zig 0.17 ×1.08 (5/5), C natif ×1.14 (5/5), Rust + Creusot ×1.26 (5/5), F*→F# ×2.04 (5/5), F*→OCaml (corrigé) ×60.81 (4/5), F*→OCaml (livré) ×226.95 (2/5).

#### `small`

| workload (n) | C natif | Zig 0.17 | Rust | Rust + Creusot | F*→C | F*→F# | F*→OCaml (corrigé) | F*→OCaml (livré) |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| sort (1 000 000) | 7.64 M/s · 0.13 s · 9 Mo | 7.57 M/s · 0.13 s · 9 Mo | 6.66 M/s · 0.15 s · 10 Mo | 6.73 M/s · 0.15 s · 10 Mo | 6.49 M/s · 0.15 s · 9 Mo | 4.89 M/s · 0.20 s · 37 Mo | 106.57 k/s · 9.38 s · 46 Mo | 14.85 k/s · 67.36 s · 58 Mo |
| sieve (20 000 000) | 154.35 M/s · 0.13 s · 21 Mo | 162.35 M/s · 0.12 s · 21 Mo | 147.94 M/s · 0.14 s · 21 Mo | 145.12 M/s · 0.14 s · 21 Mo | 157.43 M/s · 0.13 s · 21 Mo | 146.49 M/s · 0.14 s · 48 Mo | 2.68 M/s · 7.45 s · 160 Mo | 479.79 k/s · 41.68 s · 165 Mo |
| matmul (512) | 7.79 G/s · 0.02 s · 4 Mo | 7.97 G/s · 0.02 s · 4 Mo | 6.14 G/s · 0.02 s · 5 Mo | 4.28 G/s · 0.03 s · 5 Mo | 6.46 G/s · 0.02 s · 4 Mo | 1.11 G/s · 0.12 s · 32 Mo | 4.18 M/s · 32.14 s · 77 Mo | 668.22 k/s · 200.86 s · 81 Mo |
| lines (1 000 000) | 9.27 M/s · 0.11 s · 31 Mo | 10.42 M/s · 0.10 s · 31 Mo | 10.36 M/s · 0.10 s · 31 Mo | 8.23 M/s · 0.12 s · 31 Mo | 9.13 M/s · 0.11 s · 31 Mo | 7.79 M/s · 0.13 s · 59 Mo | 128.38 k/s · 7.79 s · 184 Mo | 13.82 k/s · 72.37 s · 197 Mo |
| hashmap (500 000) | 31.45 M/s · 0.05 s · 17 Mo | 39.80 M/s · 0.04 s · 18 Mo | 30.88 M/s · 0.05 s · 18 Mo | 31.13 M/s · 0.05 s · 18 Mo | 32.56 M/s · 0.05 s · 18 Mo | 18.92 M/s · 0.08 s · 45 Mo | 518.71 k/s · 2.89 s · 47 Mo | 66.34 k/s · 22.61 s · 50 Mo |

Moyenne géométrique du temps relatif au meilleur : Zig 0.17 ×1.00 (5/5), C natif ×1.09 (5/5), F*→C ×1.16 (5/5), Rust ×1.16 (5/5), Rust + Creusot ×1.31 (5/5), F*→F# ×2.04 (5/5), F*→OCaml (corrigé) ×138.85 (5/5), F*→OCaml (livré) ×987.79 (5/5).

#### `tiny`

| workload (n) | C natif | Zig 0.17 | Rust | Rust + Creusot | F*→C | F*→F# |
|---|--:|--:|--:|--:|--:|--:|
| sort (200 000) | 8.38 M/s · 0.02 s · 3 Mo | 9.37 M/s · 0.02 s · 3 Mo | 7.86 M/s · 0.03 s · 3 Mo | 7.28 M/s · 0.03 s · 3 Mo | 8.09 M/s · 0.02 s · 3 Mo | 5.17 M/s · 0.04 s · 30 Mo |
| sieve (5 000 000) | 177.59 M/s · 0.03 s · 6 Mo | 186.86 M/s · 0.03 s · 6 Mo | 185.19 M/s · 0.03 s · 7 Mo | 165.58 M/s · 0.03 s · 7 Mo | 187.10 M/s · 0.03 s · 6 Mo | 153.18 M/s · 0.03 s · 33 Mo |
| matmul (128) | 4.24 G/s · 0.00 s · 2 Mo | 4.99 G/s · 0.00 s · 2 Mo | 5.72 G/s · 0.00 s · 2 Mo | 2.77 G/s · 0.00 s · 2 Mo | 4.53 G/s · 0.00 s · 2 Mo | 455.66 M/s · 0.00 s · 29 Mo |
| lines (50 000) | 1.73 M/s · 0.03 s · 18 Mo | 2.04 M/s · 0.02 s · 18 Mo | 2.31 M/s · 0.02 s · 19 Mo | 2.01 M/s · 0.02 s · 19 Mo | 2.20 M/s · 0.02 s · 18 Mo | 2.34 M/s · 0.02 s · 46 Mo |
| hashmap (100 000) | 35.29 M/s · 0.01 s · 5 Mo | 48.29 M/s · 0.01 s · 5 Mo | 37.37 M/s · 0.01 s · 6 Mo | 35.65 M/s · 0.01 s · 6 Mo | 50.79 M/s · 0.01 s · 5 Mo | 19.19 M/s · 0.02 s · 33 Mo |

Moyenne géométrique du temps relatif au meilleur : Zig 0.17 ×1.07 (5/5), F*→C ×1.09 (5/5), Rust ×1.11 (5/5), C natif ×1.25 (5/5), Rust + Creusot ×1.38 (5/5), F*→F# ×2.36 (5/5).


Les moyennes géométriques avec « (4/5) » ou « (2/5) » ne portent que sur les workloads terminés : elles ne sont pas comparables aux autres.

## Synthèse des performances et graphes

Temps relatif au meilleur de la ligne, taille `large` (1,00 = meilleur ; « > 600 s » = délai dépassé, pas de résultat) :

| workload | C natif | Zig 0.17 | Rust | Rust + Creusot | F*→C | F*→F# | F*→OCaml (corrigé) | F*→OCaml (livré) |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| sort | ×1.12 | ×1.00 | ×1.26 | ×1.34 | ×1.21 | ×1.85 | ×95.56 | > 600 s |
| sieve | ×1.05 | ×1.05 | ×1.00 | ×1.07 | ×1.10 | ×1.07 | ×27.84 | > 600 s |
| matmul | ×1.01 | ×1.27 | ×1.00 | ×1.32 | ×1.17 | ×6.01 | > 600 s | > 600 s |
| lines | ×1.18 | ×1.27 | ×1.00 | ×1.62 | ×1.29 | ×1.51 | ×109.24 | > 600 s |
| hashmap | ×1.00 | ×1.01 | ×1.21 | ×1.18 | ×1.14 | ×1.67 | ×43.66 | > 600 s |

> **F\* → OCaml (livré et corrigé) est volontairement absent de tous les graphes** : il est de ×26 à ×136 plus lent que le meilleur à `large`
> (jusqu'à ×1000 à `small`), dépasse le délai de 600 s sur `matmul` et sur tout `large` pour le runtime livré, et consomme jusqu'à ×8 la RAM idéale.
> Sur la même échelle il écrase les autres courbes. Ses chiffres sont dans le tableau ci-dessus et dans les tableaux de la section Résultats.

1. **Temps relatif au meilleur, `large`** (échelle log) : les cinq colonnes natives ou vérifiées sont dans un mouchoir, seul F# s'en détache.
   ![relatif](results/charts/relatif-large.png)
2. **Débit en fonction de la taille** : un panneau par workload ; le débit baisse quand les données sortent des caches (`sieve`, `hashmap`) de la même façon pour toutes les colonnes natives, et les courbes F# restent en dessous sans converger.
   ![débit](results/charts/debit-vs-taille.png)
3. **Mémoire à `large`** (pic RAM ÷ taille des structures, échelle log) : C/Zig/Rust à ~1, .NET entre 1,03 et 3,2.
   ![mémoire](results/charts/memoire-large.png)
4. **Coût de la vérification** : le prix de la preuve, payé hors exécution.
   ![preuve](results/charts/cout-preuve.png)

## Empreinte : CPU, RAM, disque

Ces mesures existent depuis le début (colonnes `CPU`, `pic RAM`, `RAM/idéal`, fautes de page, E/S disque de chaque rapport `results/run-*.md`),
mais elles n'étaient pas dans ce README ; les voici pour `large`. Méthode : un lanceur C fait `fork`/`exec`/`wait4` et lit user+sys, `ru_maxrss`,
fautes de page et blocs d'E/S ; un échantillonneur lit `/proc/<pid>` toutes les 20 ms (courbe de RSS dans les rapports HTML).

### CPU et RAM, `large`

Cellule : temps CPU (user+sys) · utilisation d'un cœur · pic RAM (RSS) · RAM ÷ taille idéale des structures · fautes de page mineures.

| workload | C natif | Zig 0.17 | Rust | Rust + Creusot | F*→C | F*→F# | F*→OCaml (corrigé) | F*→OCaml (livré) |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| sort | 5.4 s · 99 % · 246 Mo · ×1.01 · 63 k | 4.8 s · 99 % · 246 Mo · ×1.01 · 63 k | 6.1 s · 99 % · 246 Mo · ×1.01 · 63 k | 6.5 s · 99 % · 246 Mo · ×1.01 · 63 k | 5.9 s · 100 % · 246 Mo · ×1.01 · 63 k | 9.0 s · 99 % · 273 Mo · ×1.12 · 64 k | 458.6 s · 99 % · 1267 Mo · ×5.19 · 324 k | — |
| sieve | 17.4 s · 94 % · 955 Mo · ×1.00 · 244 k | 18.0 s · 98 % · 955 Mo · ×1.00 · 244 k | 17.5 s · 100 % · 956 Mo · ×1.00 · 244 k | 17.9 s · 95 % · 956 Mo · ×1.00 · 244 k | 19.1 s · 99 % · 955 Mo · ×1.00 · 244 k | 18.8 s · 100 % · 983 Mo · ×1.03 · 246 k | 480.3 s · 99 % · 7648 Mo · ×8.02 · 1957 k | — |
| matmul | 2.5 s · 99 % · 73 Mo · ×1.02 · 25 k | 3.2 s · 99 % · 73 Mo · ×1.02 · 18 k | 2.5 s · 99 % · 73 Mo · ×1.03 · 25 k | 3.3 s · 99 % · 73 Mo · ×1.03 · 25 k | 2.9 s · 98 % · 73 Mo · ×1.02 · 25 k | 15.1 s · 99 % · 101 Mo · ×1.41 · 20 k | — | — |
| lines | 3.5 s · 99 % · 545 Mo · ×0.93 · 143 k | 3.8 s · 99 % · 545 Mo · ×0.93 · 143 k | 2.9 s · 96 % · 546 Mo · ×0.93 · 143 k | 4.8 s · 99 % · 546 Mo · ×0.93 · 143 k | 3.9 s · 99 % · 545 Mo · ×0.93 · 143 k | 4.6 s · 99 % · 617 Mo · ×1.05 · 153 k | 325.9 s · 99 % · 6469 Mo · ×11.00 · 1655 k | — |
| hashmap | 2.8 s · 100 % · 513 Mo · ×1.00 · 262 k | 2.8 s · 100 % · 514 Mo · ×1.00 · 131 k | 3.4 s · 98 % · 514 Mo · ×1.00 · 262 k | 3.3 s · 99 % · 514 Mo · ×1.00 · 262 k | 3.2 s · 99 % · 514 Mo · ×1.00 · 131 k | 4.7 s · 98 % · 541 Mo · ×1.06 · 133 k | 120.9 s · 99 % · 1259 Mo · ×2.46 · 321 k | — |

### Empreinte disque, démarrage, construction

| colonne | exécutable | runtime à installer | démarrage (run minuscule) | RAM au démarrage | compilation | vérification à froid |
|---|--:|--:|--:|--:|--:|--:|
| C natif | 20 Ko | 0 Mo | 6 ms | 1.7 Mo | 0.3 s | — |
| Zig 0.17 | 67 Ko | 0 Mo | 6 ms | 1.7 Mo | 0.3 s | — |
| Rust | 395 Ko | 0 Mo | 5 ms | 1.9 Mo | 0.1 s | — |
| Rust + Creusot | 396 Ko | 0 Mo | 6 ms | 1.9 Mo | 0.1 s | 16 s |
| F*→C | 24 Ko | 0 Mo | 5 ms | 1.7 Mo | 4.0 s | 236 s |
| F*→F# | 50 Ko | 72 Mo | 86 ms | 28.8 Mo | 22.6 s | 236 s |
| F*→OCaml (corrigé) | 5059 Ko | 0 Mo | — | — | 11.5 s | 236 s |
| F*→OCaml (livré) | 5077 Ko | 0 Mo | — | — | 10.1 s | 236 s |

Lecture :
* **CPU** : tous les programmes sont mono-thread (aucun workload n'est parallèle), donc ~98–100 % d'un cœur et temps CPU ≈ durée ; le temps CPU
  n'apporte une information distincte que pour .NET (compilation JIT, quelques centaines de ms au démarrage). Le classement par temps CPU est donc celui du temps.
* **RAM** : C, Zig, Rust et Rust + Creusot sont à ×1,00–1,03 de la taille des structures (×0,93 sur `lines` : le texte est consommé au fil de l'eau) ;
  F*→C est identique au C à la page près. .NET ajoute ~30 Mo fixes et ×1,03 à ×3,2 selon le workload ; l'OCaml extrait va de ×2,5 (`hashmap`) à ×11 (`lines`, 6,5 Go).
* **Disque** : aucun workload ne lit ni n'écrit de fichier (blocs E/S nuls, hors quelques blocs côté .NET) ; l'empreinte disque est celle de l'exécutable et du runtime
  (tableau ci-dessus) : 20–66 Ko en C/Zig/F*→C, ~400 Ko en Rust, 50 Ko + 72 Mo de runtime .NET, ~5 Mo pour OCaml.

### Ce que disent ces chiffres (et ce qu'ils ne disent pas)

* **Le code vérifié ne coûte rien de mesurable face au code à la main.** À `large`, moyenne géométrique du temps relatif au meilleur : C natif ×1,07,
  Rust ×1,09, Zig ×1,11, F*→C ×1,18, Rust + Creusot ×1,29 ; à `xlarge` : F*→C ×1,07, C natif ×1,07, Rust + Creusot ×1,11, Zig ×1,12, Rust ×1,17.
  Le classement change d'une taille à l'autre et d'un workload à l'autre : ce sont des écarts de génération de code, pas un coût de la preuve.
* **Rust + Creusot contre Rust simple** : les contrats sont effacés (binaires de même taille, ~396 Ko). À `large` le Rust + Creusot est plus lent
  (×1,29 contre ×1,09), surtout sur `lines` (4,9 s contre 3,0 s) et `matmul` ; à `xlarge` c'est l'inverse (×1,11 contre ×1,17). La différence vient du code (boucles
  indicées, tests de bornes que la preuve garantit mais que `rustc` ne supprime pas), pas des contrats.
* **F* → C est au niveau du C écrit à la main** (×1,04 à ×1,18 selon la taille, premier ou deuxième sur plusieurs workloads).
* **Zig 0.17** : au niveau du C sauf si on laisse LLVM seul vectoriser `matmul` (voir Méthodologie) ; premier sur `sort` à `medium`/`large`.
* **F* → F#** : ×2 environ en moyenne (×1,97 à `large`), de ×1,0 sur `sieve` (53,3 contre 54,3 M/s) à ×6 sur `matmul` (pas de SIMD explicite), ~×1,7 à ×1,9 sur
  `hashmap`/`sort`. **Échec à `xlarge`/`sieve`** (3 milliards d'octets) : le backend F# convertit la taille en `int` 32 bits (`Array.create` reçoit `-1294967295`) ; le code
  extrait ne peut pas allouer plus de 2³¹−1 éléments, le C extrait le fait sans souci (2,86 Go).
* **F* → OCaml n'est pas utilisable pour de la charge** avec ce backend : entiers boxés, `FStar_SizeT` boxé, et pour le runtime livré un aller-retour par une chaîne
  décimale à chaque accès tableau. Runtime livré : seul `tiny` passe en totalité (12 s pour trier 200 000 éléments, C : 0,02 s) ; à `medium`, seuls `sieve` (465 s) et
  `hashmap` (195 s) terminent avant 600 s ; à `large` rien ne termine. Corriger seulement `v`/`uint_to_t` apporte un facteur 5,6 à 9,3 (`small`) : `large` termine `sort`
  (465 s), `sieve` (487 s), `lines` (329 s) et `hashmap` (122 s), pas `matmul`, soit ×26 à ×140 le temps du C. Ces deux colonnes ont été mesurées avec une seule
  exécution et sont donc des ordres de grandeur.
* **Coût de la preuve** : vérification à froid de tous les modules F* : **236 s** (154 s lors d'une mesure antérieure : dépend de la charge) ; preuves Creusot sans cache : **16 s**.
  Ces durées ne changent pas une instruction de code généré.
* Ce que cette étude **ne montre pas** : l'effet de preuves fonctionnelles plus fortes (seul le tri en a), d'autres compilateurs (clang, rustc nightly), d'autres machines. Une VM
  partagée : seuls les écarts ≳ 15 % sont à prendre au sérieux.

## Ce qui a été exécuté

Tout ce qui est décrit ci-dessus a été exécuté sur cette machine : `make build` (construction des 8 colonnes), `make fstar-verify`
(tous les modules F*, à froid : 236 s), `make creusot` (36 fichiers prouvés), `make list`, `make bench`, les six presets
(`smoke`, `tiny`, `small`, `medium`, `large`, `xlarge`) pour toutes les colonnes qui en sont capables, relancés après le passage aux réglages « performance maximale », l'option `--wrapper` (`taskset -c N`
pour les runs OCaml), et les trois scripts d'installation (`setup-zig.sh` : téléchargement + sha256 vérifiés ; `setup-fstar.sh` dans un préfixe vierge ;
`setup-creusot.sh` sur un clone vierge). Ce qui n'a **pas** de résultat : OCaml aux tailles où il dépasse 600 s (« délai dépassé »,
colonnes concernées dans les tableaux), l'OCaml livré n'ayant été lancé ni à `xlarge` ni avec plus d'une exécution, et `fstar-fsharp`/`sieve` à `xlarge` (échec, ci-dessus).
`xlarge` pour les deux OCaml n'a pas été tenté : `large` ne termine déjà pas.

## Suite logique

1. **Preuves « métier » au-delà du tri.** Seul le tri a une correction fonctionnelle complète. J'ai essayé le produit
   matriciel en Creusot : le contrat de la boucle interne (`loop_j` : la ligne `i` est mise à jour, tout le reste est
   inchangé, `c'[i·n+j] ≡ c[i·n+j] + aik·b[k·n+j] mod 2³²`) se prouve, mais la composition dans `loop_k` bute sur un
   raisonnement de congruence modulaire que les solveurs ne retrouvent pas sans lemme d'instanciation ; je n'ai rien
   livré de partiellement prouvé. Les autres cibles naturelles : le crible (`f[i]==0 ⇔ i premier`), l'agrégation de
   `lines` (aller-retour décimal), la table (modèle dictionnaire). Elles n'ajouteraient aucune instruction au code
   généré — seulement du temps de preuve, ce qui est justement la mesure qui manque ici.
2. **Mesurer la valeur des gardes.** Plusieurs endroits du code F* utilisent une garde runtime (`p < len` dans l'analyseur,
   `if s < cap` dans la table) là où une preuve supplémentaire permettrait de s'en passer ; l'écart avec une version
   entièrement prouvée serait un chiffre intéressant.
3. **Runtime OCaml.** Réaliser `FStar_UInt64`/`FStar_SizeT` par des `int` natifs serait un meilleur point de comparaison que
   la colonne « conversions corrigées » ; ce n'est pas ce que livre F*, donc je ne l'ai pas fait.
4. **Autres machines / compilateurs** (clang, `-march=native`, Rust nightly, .NET en AOT, `zig cc`) pour valider les écarts de quelques pourcents, et comprendre l'écart de `matmul` (assembleur).
