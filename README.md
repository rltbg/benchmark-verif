# Programmation vérifiée : F* (→ C, OCaml, F#) contre Rust + Creusot

Question : *« le code vérifié tient-il la route, en CPU, en RAM, en disque, et surtout en débit ? »*

Ce dépôt compare **quatre chaînes**, sur **cinq workloads identiques** (spécification dans
[`spec/WORKLOADS.md`](spec/WORKLOADS.md)), avec un harnais de mesure externe (temps, CPU, pic RAM et courbe,
défauts de page, E/S disque, taille des binaires, coût de démarrage). Chaque chaîne doit produire le **même
checksum** ; sinon le résultat est marqué ❌.

| colonne | source vérifiée | extraction / compilation |
|---|---|---|
| `rust-creusot` | `impls/rust` (Rust annoté Creusot) | `cargo build --release` (les contrats sont effacés) |
| `fstar-c` | `fstar/src/*.fst` (Pulse) | `fstar.exe --codegen C` → `gcc -O3` |
| `fstar-fsharp` | **les mêmes** sources F* | `fstar.exe --codegen FSharp` → .NET 10 |
| `fstar-ocaml` / `fstar-ocaml-fast` | **les mêmes** sources F* | `fstar.exe --codegen OCaml` → `ocamlopt` (runtime livré / conversions corrigées) |

Il n'y a **aucune implémentation écrite à la main** dans le benchmark. (Version précédente : j'avais ajouté des
ports C/OCaml/F# « à la main » comme contrôles ; ils ont été supprimés — voir plus bas.)

## Réponses aux deux questions de fond

### « Pourquoi des implémentations à la main ? »

Je les avais introduites pour deux raisons : (1) avoir un oracle de checksums avant de pouvoir exécuter F*, et
(2) disposer de « planchers » pour séparer le coût du langage du coût de l'extraction F*. Avec le binaire F* récent
(v2026.10.25), ces deux raisons disparaissent : F* s'exécute, et ses trois backends sont réels. Elles sont donc
supprimées. Il reste seulement une table de checksums de référence aux petites tailles
([`spec/checksums.json`](spec/checksums.json)), calculée une fois par un programme C jetable et retrouvée à
l'identique par les quatre chaînes. Si vous voulez un jour un plancher « C écrit à la main », il faudra le
réintroduire explicitement.

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
de preuve. Le checksum, comparé entre les quatre chaînes, tient lieu de test de non-régression pour ces quatre-là.
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

scripts/build-all.sh        # Rust + F* -> C, F#, OCaml ; écrit results/footprint.json
make creusot                # preuves Creusot (durée enregistrée)
fstar/build.sh verify       # preuves F* (durée enregistrée)

python3 harness/bench.py list
python3 harness/bench.py run --preset medium --repeat 3         # écrit results/run-medium.{json,md,html,csv}
python3 harness/bench.py run --preset tiny --impls fstar-ocaml  # l'OCaml livré ne termine que 'tiny'
```

Presets : `smoke`, `tiny`, `small`, `medium`, `large`, `xlarge` (voir `harness/presets.json`). Options :
`--repeat N`, `--timeout S`, `--impls a,b`, `--workloads w,…`, `--wrapper "taskset -c 2"` ou
`--wrapper "systemd-run --scope -p MemoryMax=2G --user"` pour plafonner la RAM.

## Organisation

```
spec/WORKLOADS.md        les 5 workloads (algorithmes, graines, checksums, tailles) ; spec/checksums.json
harness/bench.py         lanceur + mesures + rapport ; runner.c (fork/exec/wait4) ; impls.json ; presets.json
impls/rust               crate Rust + contrats Creusot (src/{common,sort,sieve,matmul,lines,hashmap}.rs)
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
* Équité des compilateurs : `gcc -O3` (sans `-march=native`) pour le C extrait ; `rustc -C opt-level=3 + LTO` ;
  `dotnet` Release (TieredPGO) ; `ocamlopt -inline 200 -unsafe` (sans flambda).

## Résultats

Machine : VM de 4 cœurs, 16 Go. Médiane de 2 à 3 exécutions. Débit = éléments (ou lignes, ou opérations) par seconde dans le processus ; temps = durée interne ; RAM = pic RSS. **Tous les checksums concordent** entre chaînes et avec la table de référence (100 comparaisons, aucune divergence, aucun timeout). Rapports complets : `results/run-*.{md,html,csv}`.

### `large` (jusqu'à ~1 Go de données)

| workload (n) | Rust + Creusot | F* (Pulse) → C | F* (Pulse) → F# |
|---|--:|--:|--:|
| sort (32 000 000) | 6.62 M/s · 4.84 s · 246 Mo | 7.13 M/s · 4.49 s · 246 Mo | 4.56 M/s · 7.02 s · 292 Mo |
| sieve (1 000 000 000) | 48.34 M/s · 20.69 s · 956 Mo | 53.44 M/s · 18.71 s · 955 Mo | 42.32 M/s · 23.63 s · 982 Mo |
| matmul (2 500) | 3.29 G/s · 4.75 s · 73 Mo | 3.16 G/s · 4.95 s · 73 Mo | 662.09 M/s · 23.60 s · 230 Mo |
| lines (40 000 000) | 14.12 M/s · 2.83 s · 546 Mo | 16.99 M/s · 2.35 s · 545 Mo | 7.81 M/s · 5.12 s · 751 Mo |
| hashmap (16 000 000) | 13.81 M/s · 3.48 s · 514 Mo | 16.74 M/s · 2.87 s · 514 Mo | 5.75 M/s · 8.36 s · 675 Mo |

### `medium`

| workload (n) | Rust + Creusot | F* (Pulse) → C | F* (Pulse) → F# |
|---|--:|--:|--:|
| sort (8 000 000) | 7.28 M/s · 1.10 s · 63 Mo | 7.46 M/s · 1.07 s · 62 Mo | 4.56 M/s · 1.75 s · 109 Mo |
| sieve (200 000 000) | 59.65 M/s · 3.35 s · 193 Mo | 61.79 M/s · 3.24 s · 192 Mo | 48.65 M/s · 4.11 s · 219 Mo |
| matmul (1 600) | 3.24 G/s · 1.27 s · 31 Mo | 3.02 G/s · 1.36 s · 31 Mo | 807.05 M/s · 5.08 s · 115 Mo |
| lines (10 000 000) | 12.44 M/s · 0.80 s · 150 Mo | 15.58 M/s · 0.64 s · 149 Mo | 7.37 M/s · 1.36 s · 322 Mo |
| hashmap (4 000 000) | 16.31 M/s · 0.74 s · 130 Mo | 18.41 M/s · 0.65 s · 130 Mo | 6.38 M/s · 1.88 s · 291 Mo |

### `small` (première taille que l'OCaml corrigé termine)

| workload (n) | Rust + Creusot | F* (Pulse) → C | F* (Pulse) → F# | F* (Pulse) → OCaml (conversions d'entiers corrigées) |
|---|--:|--:|--:|--:|
| sort (1 000 000) | 8.37 M/s · 0.12 s · 10 Mo | 8.37 M/s · 0.12 s · 9 Mo | 3.03 M/s · 0.33 s · 60 Mo | 103.90 k/s · 9.62 s · 47 Mo |
| sieve (20 000 000) | 146.67 M/s · 0.14 s · 21 Mo | 153.57 M/s · 0.13 s · 21 Mo | 88.36 M/s · 0.23 s · 47 Mo | 2.57 M/s · 7.77 s · 160 Mo |
| matmul (512) | 4.16 G/s · 0.03 s · 5 Mo | 2.80 G/s · 0.05 s · 4 Mo | 397.29 M/s · 0.34 s · 37 Mo | 4.15 M/s · 32.36 s · 77 Mo |
| lines (1 000 000) | 8.62 M/s · 0.12 s · 31 Mo | 10.99 M/s · 0.09 s · 31 Mo | 4.01 M/s · 0.25 s · 151 Mo | 130.70 k/s · 7.65 s · 184 Mo |
| hashmap (500 000) | 25.58 M/s · 0.06 s · 18 Mo | 28.23 M/s · 0.05 s · 18 Mo | 7.06 M/s · 0.21 s · 171 Mo | 457.40 k/s · 3.28 s · 46 Mo |

### `tiny` (seule taille où l'OCaml livré termine)

| workload (n) | Rust + Creusot | F* (Pulse) → C | F* (Pulse) → F# | F* (Pulse) → OCaml (conversions d'entiers corrigées) | F* (Pulse) → OCaml (runtime livré) |
|---|--:|--:|--:|--:|--:|
| sort (200 000) | 9.09 M/s · 0.02 s · 3 Mo | 10.22 M/s · 0.02 s · 3 Mo | 1.32 M/s · 0.15 s · 48 Mo | 121.11 k/s · 1.65 s · 15 Mo | 16.64 k/s · 12.02 s · 17 Mo |
| sieve (5 000 000) | 123.99 M/s · 0.04 s · 7 Mo | 180.45 M/s · 0.03 s · 6 Mo | 109.70 M/s · 0.05 s · 33 Mo | 2.75 M/s · 1.82 s · 45 Mo | 498.05 k/s · 10.04 s · 47 Mo |
| matmul (128) | 2.54 G/s · 0.00 s · 2 Mo | 3.03 G/s · 0.00 s · 2 Mo | 88.67 M/s · 0.02 s · 29 Mo | 3.76 M/s · 0.56 s · 10 Mo | 728.62 k/s · 2.88 s · 12 Mo |
| lines (50 000) | 1.92 M/s · 0.03 s · 19 Mo | 1.88 M/s · 0.03 s · 18 Mo | 1.39 M/s · 0.04 s · 50 Mo | 69.80 k/s · 0.72 s · 31 Mo | 6.48 k/s · 7.72 s · 32 Mo |
| hashmap (100 000) | 33.12 M/s · 0.01 s · 6 Mo | 37.54 M/s · 0.01 s · 5 Mo | 6.91 M/s · 0.04 s · 58 Mo | 475.13 k/s · 0.63 s · 16 Mo | 69.66 k/s · 4.31 s · 16 Mo |

### Synthèse (moyenne géométrique du temps relatif au meilleur, `large`)

| chaîne | temps | CPU | pic RAM |
|---|--:|--:|--:|
| F* (Pulse) → C | ×1.01 | ×1.01 | ×1.00 |
| Rust + Creusot | ×1.12 | ×1.12 | ×1.00 |
| F* (Pulse) → F# | ×2.28 | ×2.31 | ×1.48 |

À taille `small`, l'OCaml extrait avec conversions corrigées est ~60× (sieve) à ~650× (matmul) plus lent que le C (sort 1 M : 9,6 s contre 0,12 s ;
matmul 512 : 32 s contre 0,05 s) ; avec le runtime livré tel quel, `tiny` seulement : 12 s pour trier 200 000 éléments (C : 0,02 s).

### Ce que disent ces chiffres (et ce qu'ils ne disent pas)

* **F* → C est au niveau de Rust, parfois devant** (+8 % sur `sort`, +20 % sur `lines` et `hashmap` à `large`). Hypothèse
  non mesurée : le code F* est prouvé sûr, donc extrait sans aucun contrôle de bornes, alors que le Rust vérifié garde les siens
  (Creusot prouve qu'ils réussissent, mais `rustc` ne s'en sert pas pour les supprimer). Sur `matmul` (vectorisable) les deux
  sont à ±5 % à `large`.
* **F* → F# est ~2,2× plus lent que C** en moyenne (de 1,3× sur `sieve` à ~5× sur `matmul`, ~3× sur `hashmap`, ~2× sur `lines`), avec 1,2 à 3× plus
  de RAM (tableaux .NET et GC) et ~30 Mo / ~90 ms de démarrage du runtime. Cause du `matmul` : hypothèse non mesurée (pas de vectorisation SIMD).
* **F* → OCaml n'est pas utilisable pour de la charge** avec ce backend : entiers boxés, `FStar_SizeT` boxé, et pour le runtime livré un
  aller-retour par une chaîne décimale à chaque accès tableau. Corriger seulement `v`/`uint_to_t` apporte un facteur 5 à 11,
  mais on reste 60 à 650× derrière le C (`small`).
* **Mémoire** : C et Rust sont à ~1,0× la taille des structures ; .NET ajoute ~30 Mo de runtime et 1,2 à 3× sur les tableaux ; l'OCaml extrait
  consomme jusqu'à ~8× (crible `small` : 160 Mo pour 20 Mo de données, un mot par octet dans un tableau générique), ~5× sur le tri.
* **Disque** : les binaires pèsent 20 Ko (F*→C) et ~390 Ko (Rust) ; F# 50 Ko + 74 Mo de runtime .NET à installer ; OCaml ~5 Mo.
  Aucun workload ne lit de fichier ; les compteurs d'E/S du noyau sont nuls partout.
* **Coût de la preuve** (le vrai prix de la programmation vérifiée) : vérification **à froid** de tous les modules F* : **154 s** ;
  preuves Creusot du crate Rust : **13 s** (avec 4 solveurs en parallèle). Ces durées ne changent pas une instruction de code généré.
* Ce que cette étude **ne montre pas** : l'effet de preuves fonctionnelles plus fortes (seul le tri en a), l'effet d'un autre compilateur C
  ou d'`-march=native`, et la variabilité entre machines.

