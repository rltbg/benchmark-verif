# strsort — Creusot contre Verus

Fichiers : `impls/rust/src/strsort.rs` (+ `impls/rust/verif/bench_rust_rlib/strsort/`), `impls/rust-verus/src/strsort.rs`.
Journaux : `runs/strsort-creusot.csv`, `runs/strsort-verus.csv`.

## Cible : indices triés lexicographiquement (stable) ∧ permutation de 0..n — atteinte dans les deux outils

Avant : Creusot ne prouvait que l'absence de panique, **grâce à des gardes à l'exécution** (`ia < n ? ia : 0`,
`offset + longueur <= taille du pool`, `p < taille du pool` dans `gen_pool`, additions `wrapping_*`). Elles sont toutes
supprimées dans les deux versions et remplacées par des invariants prouvés :

* `wf_offsets(pool, off)` : offsets croissants, `off[n] = |pool|` (établi par `gen_offsets`, qui prouve aussi
  `off[n] <= 32 n`, donc pas de débordement : addition ordinaire comme dans `rust-plain`) ;
* `vals_in` : les indices manipulés sont `< n`.

Spécification fonctionnelle (identique dans les deux outils) :

* `str_of(pool, off, x) = pool[off[x] .. off[x+1])` ; `lex_le(s, t)` = ordre lexicographique large défini **récursivement**
  (tête par tête, la plus courte d'abord) ; lemmes `lex_trans`, `lex_total` par induction ;
* `le(pool, off, a, b)` (code, comparaison octet par octet sur des sous-tranches `pa`/`pb` comme `rust-plain`)
  **calcule exactement** `lex_le(str_of(a), str_of(b))` ;
* `key_le(x, y) = lex_le(sx, sy) ∧ (lex_le(sy, sx) ⇒ x <= y)` (chaîne puis indice) ; lemme `key_trans` ;
* `msort` part d'une plage **identité** (`a[p] = p`, vrai à chaque appel puisque chaque plage est triée une seule fois)
  et garantit : `sorted_key` (trié pour `key_le`), `vals_in [lo,hi)`, permutation, rien hors plage ne change ;
* `merge` exige que les indices de gauche soient `< mid <=` ceux de droite : c'est ce qui rend la prise « à gauche si
  `le` » conforme à `key_le` à chaînes égales → **stabilité** ;
* `run` : `sorted_key(a, 0, n)` ∧ `a` permutation de `0..n`. L'entrée étant l'identité, être trié pour (chaîne, indice)
  caractérise exactement la sortie d'un tri stable.

Raccourcis : aucun dans les deux outils. Terminaison : tous les noyaux (Creusot : `#[check(terminates)]` sauf `run` ;
Verus : `decreases` partout). Ce qui n'est pas spécifié : le **contenu** du pool (les octets générés par `chr_of`) — le tri
est prouvé correct pour n'importe quel contenu.

Particularité Verus : sans `global size_of usize == 8` (une seule déclaration autorisée **par crate**, donc conflictuelle
entre agents : essayé, « the size of `usize` can only be set once per crate »), `32 n` peut déborder un `usize` de 32 bits ;
`run` exige donc `32 * n < usize::MAX` (vrai pour n ≤ 2^28 sur 64 bits) au lieu de fixer la taille.

## Chiffres

| | Creusot | Verus |
|---|---|---|
| exec / spec / preuve (même méthode que `sort.md`) | 90 / 105 / 62 | 142 / 145 / 101 |
| appels au vérificateur | 5 (1 échec) | 8 (6 échecs, dont 2 diagnostics volontaires) |
| temps final à froid | 29,6 s (`--no-cache 'strsort::*'`) ; première recherche de preuve : 135 s | crate entière 3,3 s |
| checksum | 10000 → a35a5e208f53f5d1, 20000 → b885d6d0575fbcc0, 300000 → 521bf8ce8bb1075d, 1e6 → 60e2397cfb436f3e, 2e6 → 2bd863740310cc65 (= `spec/checksums.d/strsort.json`) | idem |
| performance (1e6, machine chargée) | ≈ rust-plain (0,50–0,65 s des deux côtés) | idem |

## Ce qui a coûté

* **Creusot** — étonnamment peu : `merge` et `msort` (stabilité comprise) sont passés **au premier appel**, en mettant à
  disposition les lemmes quantifiés par `let _ = snapshot!(key_trans);` (idiome de creusot-std). Échecs : bornes de
  `len_of`/`chr_of` (décalage `>> 59`, masque `& 3`) → `#[bitwise_proof]` ; initialisation de l'invariant de `le`
  (`sa[0..] == sa` à affirmer). Plus tard, sous forte charge machine, un but de `merge` (transitivité) a dépassé le délai :
  ajout d'instanciations explicites de `key_trans`/`lex_total` (`proof_assert!{ ...; key_le(...) }`) pour la robustesse.
* **Verus** — plus d'itérations, mais chacune en 4 s :
  1. les invariants quantifiés de `merge` ne se prouvent pas par le seul `broadcast use key_trans` : il a fallu des
     `assert forall ... by { key_trans(...) }` explicites, pour les deux branches et les trois invariants (≈ 40 lignes) ;
  2. **isolation des boucles** : les `let ghost sa = ...` définis avant la boucle de `le` ne sont pas visibles dans le corps ;
     la postcondition échouait au `return` anticipé. J'ai d'abord soupçonné le masquage des paramètres `a`/`b` (fausse piste,
     2 appels de diagnostic) avant de recopier `sa == str_of(...)` dans l'invariant ;
  3. bornes de bits par `assert(x >> 59u64 <= 31u64) by (bit_vector)`, sans difficulté.

## Messages d'erreur

* Creusot : comme pour `sort` (but non identifié sans `why3find -x/-X`). L'erreur de compilation sur l'import de fonctions
  logiques (`use crate::sort::{perm_range, …}` effacé hors Creusot) se corrige par `#[cfg(creusot)]` sur le `use`.
* Verus : précis sur la clause, mais rien n'indique que l'échec vient de l'isolation de boucle (le message est
  « postcondition not satisfied … at this exit ») ; c'est la règle la moins intuitive rencontrée.

## Ce qu'une IA plus faible aurait raté

* La formulation de la stabilité : un tri « trié par chaîne ∧ permutation » n'est pas stable ; il faut l'ordre (chaîne, indice)
  **et** l'hypothèse « indices de gauche < indices de droite » dans `merge`, qui découle de l'entrée identité.
* La définition récursive de `lex_le` (au lieu d'un `exists k` préfixe commun), qui rend transitivité/totalité prouvables
  par induction et l'invariant de `le` trivial.
* En Verus : l'isolation des boucles et l'unicité de `global size_of usize` par crate.
