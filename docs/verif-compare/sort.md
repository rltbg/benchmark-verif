# sort — Creusot contre Verus

Fichiers : `impls/rust/src/sort.rs` (+ `impls/rust/verif/bench_rust_rlib/sort/`), `impls/rust-verus/src/sort.rs`.
Journaux : `runs/sort-creusot.csv`, `runs/sort-verus.csv`.

Les noyaux (`fill`, `merge`, `msort`, `checksum`) travaillent maintenant sur des **tranches** (`&mut [u32]`) et non plus sur
`&Vec` : `parsort` les réutilise tels quels (une seule preuve du tri séquentiel pour les deux workloads). La recopie
`tmp[lo..hi) → a[lo..hi)` est `a[lo..hi].copy_from_slice(&tmp[lo..hi])`, comme dans `rust-plain` (l'ancienne boucle
`copy_back` et sa preuve disparaissent : les deux outils ont une spec de `copy_from_slice` et de l'indexation par intervalle).

## Cible : trié ∧ permutation — atteinte dans les deux outils

| | Creusot | Verus |
|---|---|---|
| `msort(a,tmp,lo,hi)` | `(^a)@.sorted_range(lo,hi)` ∧ `perm_range((^a)@, a@, lo, hi)` (`permutation_of` de creusot-std = `Permut` de Why3) ∧ `same_outside` | `sorted_range(final(a)@,lo,hi)` ∧ égalité des `to_multiset()` des sous-suites ∧ `same_outside` |
| `merge` | trié ∧ permutation ∧ hors plage intact (stable : à égalité la gauche d'abord) | idem |
| `checksum` | `!v@.sorted() ==> result == 0` (invariant `ok == sorted_range(0,i)`) | idem |
| `run` | `proof_assert!(a@.sorted() && a@.permutation_of(*input))` | `assert(sorted_range(a@,0,n) && perm_range(a@,input,0,n))` |
| terminaison | `#[check(terminates)]` + `#[variant]` partout sauf `run` (alloue) | `decreases` sur toutes les boucles/récursions (Verus l'exige) |
| raccourcis | aucun | aucun |

Ce qui manque (les deux) : `checksum` n'est pas relié à une fonction logique `fold` (il faudrait `mix64`/`ckfold` en logique ;
non demandé par la cible) ; `fill` n'a que la conservation de la longueur.

## Chiffres

Comptage approximatif (script heuristique : lignes non vides hors commentaires/`use` ; *spec* = contrats, invariants,
variants, prédicats logiques ; *preuve* = lemmes, `proof_assert!`/`snapshot!`, blocs `proof`/`assert`/`let ghost` ;
*exec* = le reste, accolades comprises — le formatage Verus, une clause par ligne, gonfle ses chiffres).

| | Creusot | Verus |
|---|---|---|
| exec / spec / preuve | 56 / 46 / 27 | 67 / 56 / 33 |
| appels au vérificateur (module seul) | 6 (4 échecs) + 1 final | 4 (2 échecs) + 1 final |
| appels « toute la lib » (consignés ici) | 7 (3 échecs : délais dépassés sous charge, voir `parsort.md`) | — |
| temps final à froid | 14,1 s (`--no-cache 'sort::*'`, compilation comprise) | crate entière (sort+strsort+parsort) 3,3 s, dont 2,3 s SMT |
| checksum | `sort 1000` = f2b5b60894a46cfb, `sort 8000000` = e83b33c2a58ec5c2 (identiques à `spec/checksums.json`) | idem |
| performance (8e6, machine chargée) | ≈ rust-plain (1,1–1,3 s des deux côtés) | idem |

Le premier appel Verus (185 s) inclut la vérification unique de `vstd` dans le cache cargo ; les suivants : 2–3 s.

## Ce qui a coûté

* **Creusot** : la preuve existait ; le passage aux tranches et à `copy_from_slice` a cassé un but de `msort`
  (`a[p] == tmp[p]` dérivé de l'égalité des sous-suites : il faut d'abord affirmer `a@[lo..hi] == tmp@[lo..hi]`, *puis*
  la version ponctuelle, sinon le prouveur ne trouve pas l'instanciation). Pour `checksum`, l'invariant oubliait `prev == 0`
  quand `i == 0` : `-X` (tâche Why3 affichée par `why3find`) montre l'hypothèse manquante, c'est lisible mais verbeux.
  Piège de compilation : une fonction qui n'a que `#[check(terminates)]` ne fait pas effacer ses `#[invariant]` par
  `cargo build` (« attributes on expressions are experimental ») — il faut un `#[requires]`/`#[ensures]`.
* **Verus** : la permutation passe par les multiensembles ; l'invariant naturel « Σ multiensembles » ne se prouve pas
  automatiquement. Il a fallu (1) écrire l'invariant sur la suite concaténée `dst[lo..k) + src[i..mid) + src[j..hi)`
  (prise à gauche : la suite ne change pas, `=~=` suffit) et (2) pour la prise à droite, appeler explicitement six fois
  `lemma_multiset_commutative` (distributivité de `to_multiset` sur `+`). `msort` est passé du premier coup.

## Messages d'erreur

* Creusot : « proof failed, Goal vc_msort: ✘ (25/26) » ne dit pas *quel* sous-but ; il faut relancer `why3find prove -x`
  (contexte source) ou `-X` (tâche complète) sur le `.coma`. Une fois fait, la ligne et le type de but (invariant #1,
  assertion…) sont précis.
* Verus : messages au niveau de la clause (« invariant not satisfied at end of loop body » + extrait), immédiatement
  exploitables ; vérification en 2–3 s, ce qui change le rythme de travail.

## Ce qu'une IA plus faible aurait raté

* Creusot : l'ordre des assertions (égalité de sous-suites puis ponctuelle), le cas `i == 0` de `prev`, et l'astuce
  `proof_assert! { lemme(...); true }` pour instancier un lemme.
* Verus : l'invariant formulé sur la suite concaténée plutôt que sur une somme de multiensembles, et les appels explicites
  de `lemma_multiset_commutative` (le `broadcast use` seul ne suffit pas).
