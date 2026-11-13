# `matmul` : Creusot contre Verus

Fichiers : `impls/rust/src/matmul.rs` (Creusot), `impls/rust-verus/src/matmul.rs` (Verus).
Journaux : `runs/matmul-creusot.csv`, `runs/matmul-verus.csv`.

**Biais d'ordre.** La version Creusot a été écrite en premier, puis traduite en Verus avec la même
décomposition (mêmes fonctions, mêmes invariants, mêmes lemmes). Le nombre d'essais Verus est celui
d'une traduction d'une preuve déjà conçue.

## Cible et résultat

Cible du BRIEF : `c` = produit matriciel modulo 2³² (fonction logique `dot`). C'est la preuve qui avait
échoué en Creusot. **Elle est atteinte dans les deux outils**, sans aucun `trusted`/`assume`/`external_body`.

* `dot(a, b, n, i, j, k) = Σ_{l<k} a[i·n+l]·b[l·n+j]`, dans les **entiers mathématiques** (sans réduction).
* `matmul(a, b, c, n)` (c nulle au départ) garantit `is_product(a, b, c', n, n)` :
  `∀ i, j < n. c'[i·n+j] = dot(a, b, n, i, j, n) mod 2³²`. `run` le réaffirme avant le checksum.
* Structure (identique dans les deux outils) : `row_step` (boucle j, pas k d'une ligne),
  `row` (boucle k), `matmul` (boucle i). Chaque niveau garantit aussi « le reste de `c` ne change pas »,
  exprimé sur les indices plats `x < i·n ∨ i·n+n ≤ x`, ce qui garde les boucles internes linéaires.
* Terminaison prouvée partout sauf `run` en Creusot (allocation, `vec!` n'est pas `terminates`).
  En Verus tout termine (`decreases`).
* Le code exécuté est le même que l'ancien (et que `rust-plain`) : boucles i-k-j, `wrapping_add`/`wrapping_mul`.
  Seul changement : les bases `ci = i·n` et `bk = k·n` sont calculées une fois avant la boucle j
  (l'ancien code appelait `idx(n, i, j)` à chaque tour ; le compilateur le faisait déjà).
  Aucune garde à l'exécution. En Verus, `1 <= n <= 4096` est une **précondition** de `run` (pas une garde).

### Ce qui a fait tomber la preuve (la congruence modulaire)

Le point délicat est la composition « le pas u32 `c ⊕ a ⊗ b` suit le pas entier modulo 2³² ». Il est isolé
dans un lemme, dans les deux outils :

* `wrap_step(c, x, y, d)` : `0 <= d ∧ c = d mod 2³² ⟹ (c ⊕ x⊗y) = (d + x·y) mod 2³²`,
* appuyé sur `mod_add(d, e)` : `(d mod M + e mod M) mod M = (d + e) mod M` pour `d, e >= 0`,
* et `dot_nonneg` (induction) : `%` de Pearlite est la division **tronquée** (`ComputerDivision`), `mod_add`
  est faux pour des négatifs ; il faut donc prouver `dot >= 0`.

**Creusot** (lemmes `#[logic]` avec corps) :
* `wrap_step` ne passe **qu'avec** deux assertions de réduction dans son corps :
  `(x*y)@ == (x@*y@) % 2^32` et `(c + x*y)@ == (c@ + (x*y)@) % 2^32`. Les axiomes Why3 de `u32`
  (`mul_wrapping : ∃k. t'int(mul x y) = x*y + k*2^32`) suffisent alors aux solveurs. Sans ces deux
  assertions, le lemme échoue (essai `exp` du journal, volontaire).
* `mod_add` avec corps : on écrit `d = (d/M)·M + d%M`, idem pour `e`, puis `d+e = (d/M + e/M)·M + (d%M + e%M)`.
  Sans corps : échec.
* **La vraie cause de l'échec historique n'était pas la congruence** mais une borne non linéaire oubliée :
  l'invariant `j2 < j` de la boucle interne a besoin de `i·n + n <= n·n` pour savoir que `c[i·n+j2]`
  est hors de la case modifiée *et dans le tableau*. Le message d'échec pointait l'`ensures` (ligne 113),
  pas l'invariant ; il a fallu `-X` (affichage de la tâche Why3) pour voir le but réel. Remède :
  `proof_assert! { in_matrix(n@, i@, n@ - 1); i@ * n@ + n@ <= n@ * n@ }` en tête de fonction.
* Piège Creusot : `proof_assert! { lemme(...); true }` ne laisse **aucun** fait dans le contexte (on a
  prouvé `true`). Il faut écrire la conclusion utile : `proof_assert! { lemme(..); conclusion }`. Le même
  motif sert d'« introduction de ∀ » : `proof_assert!(forall<i2, j> ... ==> { row_disjoint(..); conclusion })`.

**Verus** :
* `lemma_wrap_step` : `wrapping_mul` est spécifiée par `(x*y) % 2^32` et `wrapping_add` par cas ; une
  assertion `c.wrapping_add(p) == (c + p) % m32()` puis `vstd::arithmetic::div_mod::lemma_add_mod_noop`
  suffisent. Pas besoin de `by (nonlinear_arith)` ni de `by (bit_vector)` pour la congruence.
* Les bornes non linéaires (`i·n + n <= n·n`, `n·n <= 2^24`, disjonction de lignes) sont des petits lemmes
  `assert(...) by (nonlinear_arith) requires ...;` : contexte minimal, preuve immédiate.
* L'erreur évitée en Creusot ne s'est pas produite : en Verus il faut de toute façon écrire les bornes
  dans les invariants (isolement des boucles), et le seul échec a été un débordement `usize` signalé
  directement sur l'expression `ci + j` (il manquait `n·n <= 16777216` dans l'invariant).
* `by (bit_vector)` n'a pas servi : la spécification est entière, et les spécifications `vstd` de
  `wrapping_*` sont déjà entières.

## Chiffres

| | Creusot | Verus |
|---|---|---|
| lignes exécutables / spécification / preuve (*) | 54 / 73 / 43 (ancien module : 56 / 32 / 3) | 68 / 118 / 109 |
| vérification finale à froid | 9,5 s (module seul, `--no-cache`, compilation comprise) ; 32,6 s avec `-f` (recherche de stratégie refaite) | **0,8 s** de vérification pour le module (3,7 s mur pour les 3 modules après `cargo clean -p`, vstd compris) |
| appels jusqu'au premier « ok » | 7 : 6 échecs (dont 2 appels `-X` pour voir le but) puis ok | 3 : 1 erreur de compilation (`final(c)` exigé dans les `ensures` par les nouvelles références mutables), 1 échec (2 débordements), 1 ok |
| autres appels | 1 baseline, 1 nettoyage ok, 1 expérience volontaire en échec, 2 mesures à froid | 1 build release, 2 mesures |
| `trusted` / `assume` / `external_body` | aucun | aucun |
| checksum | `matmul:64` = `ca8d1bf8c2c39885` (référence), `n=512` = `c141871bd12b7322`, `n=1024` = `0d4dda6717b395ee`, identiques à `rust-plain` | idem |
| performance | identique à `rust-plain` aux bruits près (n=1024 : 230–300 ms pour les trois, machine chargée) | idem |

(*) Comptage approximatif par un script (lignes non vides hors commentaires) : spécification = attributs
`requires/ensures/invariant/variant`, fonctions `#[logic]`/`spec fn` ; preuve = lemmes, `proof_assert!`,
`snapshot!`, blocs `proof { }`, `assert`. Le code Verus est écrit au format `verusfmt` (une clause par
ligne), ce qui gonfle mécaniquement ses chiffres d'environ 30 %.

## Expérience

* **Ce qui a coûté le plus.** Creusot : trouver *quel* but échouait (le rapport désigne l'`ensures` alors
  que c'est l'invariant) et comprendre qu'un lemme invoqué dans `proof_assert!{ ..; true }` ne sert à rien.
  Verus : rien de notable ; la syntaxe `final(c)` des nouvelles références mutables (erreur claire avec lien
  vers le guide de migration).
* **Messages d'erreur.** Verus pointe l'expression exacte (débordement de `ci + j`, précondition non
  satisfaite avec la clause). Creusot/why3find donne un numéro de ligne parfois trompeur et un compteur
  (`16/17`) ; il faut relancer avec `-X` pour lire la tâche Why3 (illisible sans connaître la traduction).
* **Ce qu'une IA plus faible aurait raté.** (1) Que `%` de Pearlite est tronqué et que `mod_add` exige
  `d, e >= 0` (donc un lemme `dot_nonneg` par induction). (2) Que l'échec signalé sur l'`ensures` vient
  d'une borne non linéaire de l'invariant. (3) Le motif `proof_assert!{ lemme; conclusion }` au lieu de
  `true`. (4) En Verus, l'isolement des boucles : chaque fait sur les paramètres doit être répété dans
  l'invariant ; et le choix des déclencheurs (`#[trigger] dot(...)` plutôt que `c@[i*n+j]`, un terme
  arithmétique qui se déclenche mal ; non testé autrement, choix fait d'emblée).
