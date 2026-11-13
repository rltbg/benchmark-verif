# `ptrchase` : Creusot contre Verus

Fichiers : `impls/rust/src/ptrchase.rs` (Creusot, preuves dans `impls/rust/verif/bench_rust_rlib/ptrchase/`),
`impls/rust-verus/src/ptrchase.rs` (Verus). Journaux : `runs/ptrchase-creusot.csv`, `runs/ptrchase-verus.csv`.

**Biais d'ordre.** Creusot d'abord, Verus ensuite, avec la même formulation de l'invariant (conçue pour Creusot).
Le temps Verus est celui d'une traduction ; en revanche, les lemmes Verus ont été écrits avec leurs preuves détaillées
dès le premier jet, alors qu'en Creusot les mêmes lemmes sont passés sans corps.

## Cible et résultat (identiques pour les deux outils)

Cible du BRIEF **atteinte** par les deux outils :

* **Invariant d'arbre de recherche** `is_bst(a, cnt, paths)` : (1) chaque lien d'un nœud `i < cnt` est `NIL` ou
  un nœud d'indice dans `(i, cnt)` (les nœuds sont ajoutés en fin d'arène, donc tout chemin est strictement croissant :
  c'est ce qui fait terminer les descentes) ; (2) chaque nœud `t` possède un chemin fantôme `paths[t]` racine → `t`
  qui est **exactement le chemin de recherche de sa clé** (`search_path`) : aucun ancêtre ne porte la clé et chaque pas
  va à gauche ssi la clé est plus petite que celle de l'ancêtre. C'est la propriété « chaque clé est du bon côté de
  chacun de ses ancêtres », équivalente à la définition par sous-arbres, mais sans prédicat récursif sur l'arène.
  Le témoin `paths` est un `Snapshot<Seq<Seq<Int>>>` (Creusot) / `Ghost<Seq<Seq<int>>>` (Verus) renvoyé par `build`.
* **`build`** : l'ensemble des clés de l'arbre est exactement `{ ins_key(i) | i < n }` (deux postconditions :
  toute clé insérée est présente, toute clé présente a été insérée), `ins_key(i) = mix64(SEED1 + i) | 1`.
* **`search`** : `found == contains(a, cnt, k)` (∃ nœud portant `k`) — donc trouvée ⇔ insérée ; `1 <= visites <= cnt`.
  `search_all` prouve (assertions) que les `n` recherches positives trouvent et que les `n/2` négatives échouent
  (clé paire contre clés impaires : lemme sur bits).
* **Chasse** : `chase` renvoie `fold(fold(CK_INIT, chase_pos(next, 4n)), chase_sum(next, 4n))` où `chase_pos`/`chase_sum`
  sont les itérées logiques de `next`. `sattolo` produit des valeurs `< n` et une **permutation** de l'identité
  (Creusot : `permutation_of`, gratuit via `exchange` ; Verus : égalité des `to_multiset()`).
* Terminaison : tous les noyaux (`build`, `search`, `search_all`, `sattolo`, `chase`) ; `run` aussi en Verus. En Creusot,
  `run` n'est pas `check(terminates)` car `vec![x; n]` (`from_elem`) ne l'est pas dans `creusot-std`.

Ce qui manque (les deux outils) : le cycle unique de Sattolo (inutile à la sûreté) ; la valeur exacte de `plen` et
des compteurs de visites (seulement bornés) ; pas de modèle logique fermé du checksum `r1`/`r2`.

### Gardes supprimées

* `if c < cnt { cur = c } else { insérer }` → `if c == NIL { insérer } else { cur = c }` (descente et recherche) :
  l'invariant `link_ok` garantit `c == NIL || cur < c < cnt`.
* La borne de visites `v < cnt` de la descente d'insertion et de la recherche : supprimée (boucle `loop`, variant
  `cnt - cur`). Le comportement est identique : sur un arbre valide la borne n'est jamais atteinte (prouvé : `v <= cur < cnt`).
* `cur = if v < n { v } else { cur }` dans la chasse : supprimé, l'invariant `next[i] < n` vient de `sattolo`.
* `next[i] <-> next[j]` à la main → `next.swap(i, j)` (comme `rust-plain`).

## Chiffres

| | Creusot | Verus |
|---|---|---|
| lignes exécutables / spécification / preuve (≈) | 138 / 143 / 35 | 168 / 159 / 196 |
| appels au vérificateur (journal) | 16, dont 2 ok, 4 erreurs de compilation, 10 échecs (dont 4 « calibration z3 impossible », machine chargée, sans rapport avec la preuve) | 9, dont 3 ok, 3 erreurs de compilation, 3 échecs |
| temps de vérification final | module : 1,2 s avec cache ; temps prouveurs cumulés d'après `proof.json` : 5,9 s (150 appels de prouveur) ; à froid non mesurable (voir ci-dessous) | crate entière (common + ptrchase + alloc) à froid : 3,3 s (Verus `total-time` 2,95 s, SMT 1,05 s) |
| `trusted` / `assume` / `external_body` | aucun | aucun |
| checksum (n = 1, 10, 1000, 10000, 30000, 300000) | identique aux références | identique aux références |
| temps d'exécution n = 300 000 (min de 7, machine chargée) | 158 ms (plain : 152 ms) | 154 ms |

Lignes : comptage approximatif par un script (lignes non vides hors commentaires ; spécification = fonctions
`logic`/`spec`, `requires`/`ensures`/`invariant`/`variant`/`decreases` ; preuve = lemmes, `proof { }`, `assert`,
`proof_assert!`, `snapshot!`, variables fantômes). Le tri est grossier (±15 %). En Creusot les lemmes sans corps
sont comptés en spécification.

Temps à froid Creusot : vider le cache `.why3find` force why3find à recalibrer z3, ce qui a échoué 4 fois
(« can not calibrate prover z3@ (N=33, Timeout 5 s) », charge ≈ 13 sur 4 cœurs) ; le chiffre « prouveurs cumulés »
est la meilleure approximation disponible. Le premier passage complet sans preuves en cache a pris 202 s
(avec 5 fonctions en échec, donc des délais d'attente).

## Expérience

**Creusot.**
* Le plus coûteux : comprendre *quel* but échoue. Le message par défaut ne donne que la fonction
  (`Goal Coma.vc_build: ✘ (42/50)`) ; il faut relancer why3find avec `-X` (tâche complète) sous `script` (tty)
  et lire des centaines de lignes de contexte Why3 pour trouver le but (`goal vc_build : t'int cur2 < t'int c2`).
* Bits : avec `left(l) = (l & M)@` (en `Int`) les prouveurs échouaient sur `set_link` ; en laissant les liens en `u64`
  et en ajoutant `#[bitwise_proof]` (non documenté dans le guide, trouvé dans `tests/should_succeed/bitvectors`),
  tout passe.
* La descente interne modifie `a` (insertion avant `break`) : Creusot oublie alors tout de `a`, `cnt`, `paths`
  dans la boucle interne. Il a fallu un invariant de cadre `a@ == before.0 && cnt == before.1 && ...` avec
  `before = snapshot!((a@, cnt, *paths))`.
* Bonne surprise : le lemme `insert_preserves` (préservation de `is_bst` par l'insertion, quantificateurs imbriqués)
  est prouvé **sans corps** par Alt-Ergo en 0,3 s ; `permutation_of` après `swap` est gratuit.
* Pearlite refuse `Int as u64` (E0605) et n'a pas de `%` sur `u64` : il faut choisir le type des fonctions logiques
  en conséquence (`ins_key(i: u64)`).
* Une assertion évidente (`link_ok(c,..) ∧ c ≠ NIL ⇒ cur < c`) a échoué tant qu'elle n'était pas découpée en deux
  `proof_assert!` : sensibilité au délai de 1 s par but.

**Verus.**
* Les messages d'erreur désignent la clause exacte et la ligne de sortie de boucle concernée
  (« failed this invariant … at this loop exit ») : le diagnostic est immédiat.
* Isolation des boucles : les `requires` ne sont pas visibles dans les boucles ; il faut répéter `is_bst`, bornes, etc.
  dans chaque invariant (`search_all`, `sattolo`, `chase`, et la descente interne). `loop_isolation(false)` est
  refusé avec `invariant_except_break`/`ensures` de boucle.
* Déclencheurs : `forall t. exists i. key(t) == ins_key(i)` ne compile pas sans déclencheur ; le prédicat `inserted`
  règle la syntaxe, mais la présence simultanée de `contains` (∀ ∃) et `inserted` (∀ ∃) a créé une boucle de
  déclencheurs (rlimit dépassé) : il a fallu scinder le lemme en deux.
* Pièges : `usize` n'est pas supposé 64 bits (`2 * cur` peut déborder si `cnt < 2^32`) ; une variable modifiée dans
  la boucle (`dup`) est oubliée en tête d'itération (ajout de `!dup` en `invariant_except_break`) ; syntaxe
  `by (bit_vector)` placée avant `requires` pour une `proof fn`.
* Les lemmes `by (bit_vector)` sur les mots de liens passent immédiatement ; la vérification est ~50 fois plus
  rapide que Creusot.

**Ce qu'un modèle plus faible aurait raté.** Le choix de l'invariant (chemins de recherche fantômes plutôt qu'un
prédicat récursif de sous-arbre, ou des intervalles qui exigent un argument de plus petit ancêtre commun) ; le fait
qu'un nœud d'indice plus grand suffit à la terminaison ; l'invariant de cadre `before` en Creusot ; la boucle de
déclencheurs en Verus (le symptôme « rlimit » ne dit pas où elle est) ; l'extraction du but échoué en Creusot.

## Vrais pointeurs (non utilisés : le workload reste une arène par indices)

* **Creusot** : `creusot-std` fournit des pointeurs bruts avec permission fantôme `Perm<*const T>`
  (`ghost/perm.rs`, `std/ptr.rs` : `Perm::new(v) -> (*mut T, Ghost<Perm<*const T>>)`, `from_box`, `disjoint_lemma`,
  `drop` explicite pour libérer). Un BST à vrais pointeurs garderait une table fantôme pointeur → `Perm` (et les
  clés/fils logiques), l'invariant `is_bst` se transposant tel quel en remplaçant l'indice par l'adresse ; la
  terminaison viendrait d'une profondeur fantôme au lieu de l'ordre des indices.
* **Verus** : `vstd::simple_pptr::PPtr<V>` + permissions linéaires `Tracked<PointsTo<V>>` (exemple
  `examples/doubly_linked.rs`, qui garde une `Map` fantôme de `PointsTo`), ou `vstd::raw_ptr`. Même transposition ;
  les permissions sont des valeurs `tracked`, donc l'appartenance de chaque nœud est vérifiée par le typage linéaire.
