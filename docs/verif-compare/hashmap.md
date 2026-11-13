# `hashmap` : Creusot contre Verus

Fichiers : `impls/rust/src/hashmap.rs` (Creusot) et `impls/rust-verus/src/hashmap.rs` (Verus).
Journaux : `runs/hashmap-creusot.csv`, `runs/hashmap-verus.csv`.

**Biais d'ordre.** La version Creusot a été écrite en premier. La version Verus a repris le même
invariant et les mêmes lemmes : son temps de mise au point est celui d'une traduction.

## Cible et résultat (identiques pour les deux outils)

La cible du BRIEF (« modèle dictionnaire ») est **atteinte et dépassée** par les deux outils.

* **Modèle.** Un dictionnaire logique : `Mapping<u64, Option<u64>>` en Creusot, `Map<u64, u64>` en
  Verus. Il est passé en paramètre fantôme : `Snapshot<Model>` / `Ghost<Map>`, de taille nulle à
  l'exécution.
* **Invariant `table_inv(t, cap, lg, m)`.** Quatre clauses :
  * (A) chaque case occupée porte une clé du modèle, avec sa valeur ;
  * (B) le chemin de sondage de `home(clé)` jusqu'à sa case ne contient aucune case vide ;
  * (C) les clés sont uniques ;
  * (D) toute clé du modèle est non nulle et présente dans la table.
* **`insert`** donne `table_inv(t', …, add(m, key, v))`, où `add` ajoute la valeur modulo 2⁶⁴ si la
  clé existe déjà. **`lookup`** renvoie `get0(m, key)` : la valeur associée, 0 si la clé est absente.
* **`insert_all`** produit `model(n)`. C'est une fonction logique des seules clés `key_l(0, i)`,
  écrite avec un `mix64` logique.
* **`run`** renvoie `ck_lookups(model(n), n, n, ck_lookups(model(n), 0, n, CK_INIT))` : le checksum
  entier est spécifié.
* **Terminaison** prouvée partout, sauf `run` en Creusot (voir « Raccourcis restants »).

### Gardes supprimées

Elles figuraient dans l'ancien Creusot et, pour certaines, dans `rust-plain`. Le comportement ne
change pas, car aucune ne se déclenchait jamais.

* **`slot`** : `if s < cap { s } else { 0 }` est supprimé. La propriété
  `(key·PHI) >> (64−lg) < 2^lg = cap` est prouvée en mode bitvecteur. Le paramètre `cap` de `slot`
  disparaît aussi.
* **`insert` / `lookup`** : le compteur d'essais `tries < cap` est supprimé ; la boucle est un
  `loop`. La terminaison repose sur un compteur **fantôme** `tries` et sur un argument d'occupation :
  * `occ(t, cap)`, le nombre de cases occupées, reste `<= n <= cap/2`, donc une case vide `e` existe
    (lemme `occ_has_empty`) ;
  * toutes les cases à distance `<= tries` de `home` sont occupées, donc `tries < dist(home, e) < cap`.
* **Calcul de `cap`** : les gardes `lg < 27 && cap <= 33554432` sont retirées. La borne découle de
  `n <= 2^26`, avec `cap = 2^lg` comme invariant.

## Chiffres

| | Creusot | Verus |
|---|---|---|
| lignes exécutables / spécification / preuve (*) | 99 / 144 / 100 (ancien : 86 / 19 / 0) | 102 / 189 / 238 |
| vérification finale à froid | 48,4 s pour la copie `common` + `sieve` + `hashmap` (cache `.why3find` vidé, charge ≈ 11). Un premier essai à froid (charge ≈ 13) a laissé un but de `vc_insert` en échec par délai : il prend 0,42 s sur un budget de 1 s par prouveur | **2,2 s** (`cargo verus focus --verify-module hashmap` après `cargo clean`, dont 0,39 s de SMT) |
| appels jusqu'au premier « ok » | 13 : 2 erreurs de compilation, 10 échecs (dont 3 relances `-X` pour voir les buts et 3 séries de lemmes d'essai), 1 ok. Puis 1 échec et 1 ok après avoir retiré `cap` de `slot` | 6 : 1 erreur de compilation, 4 échecs, 1 ok |
| autres appels | 3 erreurs d'outil (why3find ne peut pas recalibrer z3 quand la machine est chargée), 1 échec à froid (but instable), 1 ok avec `-X`, 1 ok à froid | 1 build release ok, 3 mesures (dont 1 erreur de ligne de commande et 1 sans effet) |
| `trusted` / `assume` / `external_body` | aucun | aucun |
| checksum | vérifié | vérifié |

(*) Même méthode que pour `sieve.md`. Le style `verusfmt` gonfle les chiffres Verus. En Creusot,
`mix64` et `ckfold` sont recopiés dans le module avec leur contrat logique : la version de
`common.rs` n'en a pas, et `common.rs` ne devait pas être modifié.

### Raccourcis restants

* **Creusot** : `run` n'a pas `#[check(terminates)]`, parce que `std::vec::from_elem` n'est pas
  `terminates` dans creusot-std.
* **Verus** : aucun.
* **Les deux** : `main` n'est pas vérifié, donc `1 <= n <= 2^26` n'y est pas contrôlé.

## Ce qui a coûté le plus

### Creusot

1. **Les bits.**
   * Dans le mode par défaut, `UInt64` est un entier borné et `|`, `>>`, `<<` sont non interprétés :
     même `x | 1 != 0` est improuvable. Les lemmes d'essai échouaient tous.
   * La solution est `#[bitwise_proof]` sur les lemmes de bits. Cet attribut est documenté dans une
     seule ligne de `creusot-std/src/lib.rs` et dans des exemples de tests, pas dans le guide.
   * Ensuite, le `>>` du programme (`shr a (of_int (t'int b))`, avec un décalage `u32` calculé en
     programme) ne coïncidait pas avec celui de la logique (`shr_u64 x (UInt32.sub 64 lg)`). Il a
     fallu nommer `sh = 64 - lg` et affirmer `sh == 64u32 - lg`.
   * La géométrie `cap == 1 << lg` a dû être reformulée en `cap@ == lg@.pow2()`, en entiers.
2. **Appliquer un lemme.**
   * Les lemmes `inv_insert` et `inv_update`, qui portent toute la logique de l'invariant, ont été
     prouvés du premier coup : les quantificateurs passent bien avec quatre prouveurs et `split_vc`.
   * En revanche, leur postcondition n'était pas utilisée au point d'appel tant que leurs
     préconditions n'étaient pas réécrites en `proof_assert!` juste avant (deux tours).
3. **L'outil lui-même.**
   * À froid sur une machine chargée, why3find échoue avec `can not calibrate prover z3@` avant de
     prouver quoi que ce soit.
   * Un but de 0,4 s peut dépasser le budget de 1 s par prouveur.

### Verus

Cinq corrections, toutes guidées par des messages précis :

* `old(t)` / `final(t)` dans les `ensures` ;
* la borne `cap <= 2^27`, nécessaire pour `2*s`, ajoutée à `shape` et prouvée en `bit_vector` ;
* `occ_fill` trop faible : il fallait `c <= s0 ==> égalité` ;
* une `decreases n2 - cap` qui devenait négative, remplacée par `decreases 27 - lg`.

Le piège principal est **l'isolement des boucles**. La boucle de `insert` oubliait que
`t0 == old(t)@`. L'erreur se présentait comme une postcondition `occ(final(t)@) <= occ(old(t)@) + 1`
non prouvée, alors que l'assertion équivalente sur `t0` passait juste avant. `--expand-errors` a
montré que seule la clause sur `old(t)` échouait, d'où l'invariant manquant.

Le mode `bit_vector` a prouvé directement `x | 1 != 0`, `x >> (64−lg) < 1 << lg` et le doublement
`c + c == 1 << (lg+1)`, avec des `requires` explicites.

## Qualité des messages d'erreur

* **Creusot** : position au niveau de la fonction et compte de sous-buts, par exemple
  `vc_insert ✘ (31/32)`. Il faut `-X` pour voir la tâche. Le but y est lisible, mais noyé dans les
  axiomes, et les noms sont mutilés (`fin1`, `result5`).
* **Verus** : localisation exacte (ligne de l'`ensures` et `return` concerné), débordements
  désignés, note « recommendation not met » sur les conversions.

## Ce qu'un modèle plus faible aurait probablement raté

* **L'idée de preuve de la terminaison sans compteur.** Il faut une case vide garantie, par le
  comptage `occ`, et un argument de distance. Sans elle on garde le `for _ in 0..cap` de
  `rust-plain`, ou bien la spécification « 0 si absente » devient fausse (la boucle bornée ne le
  garantit pas sans invariant).
* **L'invariant « chemin sans trou » (B)**, indispensable pour qu'un échec de recherche signifie
  « absente ».
* **En Creusot** : `#[bitwise_proof]` ; la non-coïncidence du `>>` programme et logique ; l'option
  `-X` ; l'écriture explicite des préconditions des lemmes ; la distinction entre un échec de
  calibrage why3find (charge machine) et un échec de preuve.
* **En Verus** : l'isolement des boucles (réaffirmer `t0 == old(t)@`, `shape`, etc. dans
  l'invariant) ; les déclencheurs multi-variables de (B) et (C) ; une `decreases` qui doit rester
  positive ; `usize` de largeur inconnue.

## Checksums et performances

| n | Creusot | Verus | rust-plain / référence |
|---|---|---|---|
| 1 | 082f2207b4e88cc4 | 082f2207b4e88cc4 | 082f2207b4e88cc4 |
| 10 000 (smoke) | 4657001acc4f46d5 | 4657001acc4f46d5 | 4657001acc4f46d5 (`spec/checksums.json`) |
| 500 000 (small) | 18abe62441ec7845 | 18abe62441ec7845 | 18abe62441ec7845 |
| 4 000 000 | 5a8f52803b881025 | 5a8f52803b881025 | 5a8f52803b881025 |

Temps à n = 4·10⁶, trois essais chacun sur une machine chargée (charge 11 à 17 sur 4 cœurs) :

* rust-plain : 0,75 à 0,95 s ;
* Creusot : 0,86 à 1,05 s ;
* Verus : 0,93 à 1,04 s.

Ces écarts sont dans le bruit. Dans le code exécuté, deux différences seulement avec rust-plain :

* l'absence des gardes ;
* en Creusot, `mix64` et `ckfold` sont des copies locales identiques à ceux de `common`. Verus
  utilise ceux de `common`, qui ont déjà un contrat (`spec_mix64`, `spec_ckfold`).
