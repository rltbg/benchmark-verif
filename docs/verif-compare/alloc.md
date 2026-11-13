# `alloc` : Creusot contre Verus

Fichiers : `impls/rust/src/alloc.rs` (Creusot, preuves dans `impls/rust/verif/bench_rust_rlib/alloc/`),
`impls/rust-verus/src/alloc.rs` (Verus). Journaux : `runs/alloc-creusot.csv`, `runs/alloc-verus.csv`.

**Biais d'ordre.** Creusot d'abord, Verus ensuite, avec les mêmes définitions et les mêmes lemmes : le Verus est une
traduction d'une preuve déjà conçue (et ses lemmes ont été écrits avec leurs étapes dès le départ).

## Cible et résultat

Cible du BRIEF **atteinte** par les deux outils (« la somme repliée de chaque bloc est celle de son contenu ;
la fenêtre FIFO garde les W derniers blocs »), avec en plus le checksum entier spécifié :

* `fill` : `b[k] == (i + k) mod 251` pour tout `k` (`is_block`), longueur conservée.
* `sum` : renvoie `seq_sum(b, len)`, la somme exacte du contenu **relu** ; plus de `wrapping_add` : l'absence de
  débordement est prouvée (≤ 256 octets ≤ 250).
* Lemme `sum_block` : la somme relue d'un bloc au contenu attendu est `block_sum(i, len)`, somme logique calculée
  sans construire le bloc.
* `run` : le résultat est le repli, bloc par bloc, de `block_sum(i, size(i))` (taille `8 + 8·(mix64(SEED+i) mod 32)`),
  suivi de `fold(·, ⌈n/16⌉)`.
  - Verus : `r == spec_ckfold(ck_model(n), ((n + 15) / 16) as u64)` avec `ck_model` récursive.
  - Creusot : `∃ sums: Seq<u64>, kept: u64. block_sums(sums, n) ∧ kept@ == (n+15)/16 ∧ result == fold(fold_seq(sums, n), kept)`
    où `block_sums` impose `sums[i]@ == block_sum(i, size_l(i))` pour tout `i < n` (suite donc unique). Pearlite
    n'a ni `%`/`/` sur `u64` ni conversion `Int as u64` (E0605, E0277) : impossible d'écrire `ck_model(n)` directement.
* **Fenêtre FIFO** (invariant de la boucle de `run`, prédicat `window`) : pour chacun des `W` derniers blocs conservés
  (n° `j`, bloc `16 j`), l'emplacement `j mod W` de l'anneau contient un bloc de taille `size(16 j)` et de contenu
  `is_block(·, 16 j)` ; les emplacements jamais utilisés sont vides. Comme les `W` derniers numéros occupent `W`
  emplacements distincts (lemme `mod_distinct`), l'anneau contient exactement la fenêtre. Le bloc évincé est libéré par
  `Drop` lors de l'affectation `ring[kept % w] = Some(b)` (sémantique Rust ; aucun des deux outils ne spécifie la
  désallocation elle-même).
* Terminaison : `fill`, `sum` (et les lemmes) ; en Verus aussi `run`. En Creusot `run` reste non `check(terminates)` :
  `vec![x; n]` (`from_elem`) n'est pas déclaré terminant dans `creusot-std`.

Ce qui manque : rien de la cible ; la libération des blocs n'est pas un fait logique (pas de modèle de tas dans
ces preuves — voir F* Pulse pour cela).

Gardes : aucune garde de preuve dans l'ancien code ; `s.wrapping_add` → `s +=` (prouvé sans débordement),
`if w < 1 … if w > WMAX …` → `(n >> 6).clamp(1, WMAX)` (comme `rust-plain`, spécifié dans les deux bibliothèques),
anneau initialisé par `vec![None; w]` au lieu d'une boucle de `push`.

## Raccourcis restants

* Creusot : **aucun** `trusted` (l'ancienne barrière `#[trusted] fn barrier` est supprimée : `std::hint::black_box`
  a un contrat `result == dummy` dans `creusot-std` et s'appelle directement).
* Verus : `#[verifier::external_body]` sur `barrier_mut(&mut Vec<u8>)` (contrat supposé : ne modifie pas le bloc)
  et `barrier(&Vec<u8>)`, car `std::hint::black_box` n'a pas de spécification dans vstd.

## Chiffres

| | Creusot | Verus |
|---|---|---|
| lignes exécutables / spécification / preuve (≈, même script que `ptrchase.md`) | 63 / 80 / 33 | 68 / 64 / 54 |
| appels au vérificateur (journal) | 13 : 2 ok, 2 erreurs de compilation, 6 échecs de preuve, 3 échecs d'outil (calibration z3, machine chargée) | 5 : 3 ok, 1 erreur (option mal utilisée), 1 échec de preuve |
| temps de vérification final | à froid (cache vide) : 5,5 s ; somme des temps prouveurs (`proof.json`) : 2,8 s (67 appels) | module seul (`cargo verus focus`) : `total-time` 1,9 s dont SMT 0,33 s ; crate entière 3,3 s |
| checksum (n = 20 000, 100 000, 2 000 000) | identique aux références | identique aux références |
| temps d'exécution n = 2 000 000 (min de 7, machine chargée) | 274–289 ms (ancien Creusot : 276 ms ; plain : 236–248 ms) | 284 ms |

L'écart avec `rust-plain` existait déjà avant la réécriture (ancien binaire Creusot : 276 ms) : `Vec<u8>` (24 o, au lieu
de `Box<[u8]>`, 16 o) dans l'anneau et somme par indice au lieu de l'itérateur. Non corrigé ici pour ne pas changer
la forme du code entre colonnes vérifiées.

## Expérience

**Creusot.** Trois obstacles, tous liés à l'arithmétique de Pearlite : (1) pas de `%` ni `/` sur `u64` en logique,
pas de conversion `Int → u64` : le modèle a dû passer par une suite fantôme de sommes et une postcondition
existentielle, qui elle-même ne passe qu'avec des témoins explicites (`proof_assert!` avant le `return`) ;
(2) le repli sur `sums.snoc(x)` exige un lemme d'induction (`fold_snoc`) ; (3) l'arithmétique modulaire non linéaire
(`a ≠ b mod w` pour `0 < b − a < w`) ne passe pas directement : lemme `mul_mono` (« les multiples de `w` sont espacés
de `w` ») puis `mod_distinct` en forme universelle, utilisée par `window_push`. Bonne surprise : `window_push`
(mise à jour de l'anneau sous quantificateur avec `match`) passe sans corps une fois `mod_distinct` disponible.
Diagnostic : encore une fois, seul `-X` (sous `script`) dit quel but échoue. La recalibration de z3 sous charge
a fait échouer 3 appels, et un échec de calibration mis en cache bloquait les appels suivants.

**Verus.** Presque tout est passé au premier essai : `ck_model` s'écrit directement (`as u64` sur un `int` est
permis en spécification), l'anneau `Vec<Option<Vec<u8>>>`, `vec![None; w]` (spécification `cloned` d'`Option`) et
`clamp` sont spécifiés dans vstd. Seul échec : `a % w != b % w` par `nonlinear_arith` (rlimit), remplacé par
`vstd::arithmetic::div_mod::{lemma_mod_equivalence, lemma_small_mod}`. Message d'erreur précis (ligne de l'`assert`).

**Ce qu'un modèle plus faible aurait raté.** En Creusot, contourner l'absence de `%`/conversion en logique sans
affaiblir la cible (la tentation est de ne spécifier que `fill`/`sum`) ; trouver la forme « déclenchable » du lemme
non linéaire (`w * qb − w * qa`, dont les termes apparaissent via la division euclidienne). Dans les deux outils :
formuler la fenêtre par numéro de bloc conservé avec `j mod W` plutôt que par emplacement, ce qui rend la mise à jour
locale (un seul emplacement change, les autres numéros de la fenêtre ont un emplacement différent).

## Vrais pointeurs

Les blocs sont déjà de vrais objets du tas (`Vec<u8>`), possédés par l'anneau : Rust (et donc les deux outils) prouve
l'absence de double libération par le typage. Avec des pointeurs bruts, Creusot utiliserait `Perm::from_box` /
`Perm::new` (`creusot-std/src/std/ptr.rs`) et rangerait les `Ghost<Perm<*const [u8]>>` dans une séquence fantôme
parallèle à l'anneau (la libération demande un `drop` explicite avec la permission) ; Verus utiliserait
`vstd::raw_ptr` / `simple_pptr::PPtr` avec des `Tracked<PointsTo<…>>` rangés dans une `Seq`/`Map` fantôme, l'oubli
d'une permission étant détecté par le typage linéaire (fuite) — c'est ce que la version F* Pulse fait avec sa slprop
récursive `owned`.
