# `parmatmul` : Creusot contre Verus

Fichiers : `impls/rust/src/parmatmul.rs` (Creusot), `impls/rust-verus/src/parmatmul.rs` (Verus).
Journaux : `runs/parmatmul-creusot.csv`, `runs/parmatmul-verus.csv`.

**Biais d'ordre.** Creusot d'abord, Verus ensuite. Les deux réutilisent `dot` et les lemmes de `matmul`
(écrits juste avant), donc le cœur arithmétique était déjà résolu ; ce document parle surtout des threads.

## Cible et résultat

Cible : `c` = produit matriciel modulo 2³², **threads compris si possible**. **Atteinte dans les deux
outils, threads compris**, sans `trusted`/`assume`/`external_body` dans le module.

### Creusot : `std::thread::scope` vérifié, avec la même structure mémoire que `rust-plain`

* Noyau `rows(a, b, c, n, m)` sur des tranches (bloc de m lignes) : `block_product(a, b, c', n, m)`,
  c'est-à-dire `c'[i·n+j] = dot(a, b, n, i, j, n) mod 2³²` (le `dot` de `matmul`).
* `blocks(s, a, b, c, n, t, k)` (récursive) : découpe `c` avec `split_at_mut`, lance le bloc k dans un thread
  avec `creusot_std::std::thread::Scope::spawn`, traite récursivement les blocs k+1.., puis
  `join_unwrap()`. Le dernier bloc est calculé par le thread courant, comme dans `rust-plain`.
  Garantie : `rows_from(A, B, ^c, n, n·k/t)` (lignes `n·k/t..n` correctes).
* `run` appelle `thread::scope(move |s| blocks(s, &a, &b, &mut c, n, t, 0))` puis prouve
  `is_product(A, B, C, n, n)` — même énoncé que `matmul`.
* Ce qui rend cela possible : `join_unwrap` donne `valid_result(r)`, et `spawn` garantit
  `valid_result(r) ⟹ f.postcondition_once(...)`. La postcondition de la fermeture **est inférée par
  Creusot** (aucune annotation : les attributs `#[requires]` sur une fermeture ne compilent pas en Rust
  stable, « attributes on expressions are experimental »). Elle parle de la valeur finale *prophétique*
  du bloc `^mine` ; `split_at_mut` relie `^mine`, `^tail` et `^c` (`(^mine)@ == (^c)@[..mid]`).
* Seules hypothèses : les contrats `#[trusted]` de `creusot_std::std::thread` (sémantique des threads de std).
* Différence avec l'ancien code : les threads sont joints explicitement (ordre inverse de création) au lieu
  de la jonction implicite de `scope` ; c'est ce qui donne accès à leur postcondition. Coût nul.
  `blocks` n'a pas de preuve de terminaison (`spawn` n'est pas `#[check(terminates)]`) ; le noyau, oui.
* Précondition `threads <= 2^32` (évite le débordement de `n·threads` ; l'usize Creusot est 64 bits).

### Verus : `vstd::thread::spawn`, A et B partagées par `Arc`, un bloc de C par thread

* `vstd::thread` n'offre que `spawn` avec des fermetures `'static` (pas de threads « scoped ») : on ne
  peut pas prêter une tranche de `c` à un thread. Choix fait : `Arc<Vec<u32>>` pour A et B (déplacées, pas
  copiées) ; chaque thread **alloue et rend** son bloc de C (`rows(&a, &b, n, r0, r1) -> Vec<u32>`) ; la
  mémoire totale est la même et il n'y a aucune copie. Le checksum replie les blocs dans l'ordre.
* La fermeture porte `requires`/`ensures` (syntaxe Verus, compile normalement) ; `JoinHandle::predicate`
  transporte l'`ensures` jusqu'à `join()`. Les poignées sont dans un `Vec<JoinHandle<_>>` avec l'invariant
  `∀ k, blk. handles[k].predicate(blk) ⟹ block_ok(A, B, blk, n, lo(k), lo(k+1)-lo(k))`, et sont jointes dans
  l'ordre avec `remove(0)`.
* **Au-delà de la cible** (nécessaire ici, puisque C n'existe plus comme tableau) : la boucle de jonction
  maintient une séquence fantôme `cg` = concaténation des blocs déjà joints, prouve
  `cg == product(A, B, n).subrange(0, lo(kk)·n)` (`lemma_append_block`, avec division/modulo par `n`), et à la
  fin `ck == ck_seq(CK_INIT, product(A, B, n))` : **le checksum rendu est celui des n² éléments de A·B mod 2³²
  en ordre ligne-majeur**.
* Cas `Err` de `join` (thread paniqué) : inatteignable avec `panic = "abort"`, mais Verus ne le sait pas ;
  il rend 0. C'est pour cela que la conclusion est un `assert` final et non un `ensures` de `run`.
* Précondition `threads <= 2^16` : Verus ne suppose pas un `usize` de 64 bits, `n·threads` doit tenir en 32 bits.
* `Arc::clone` n'a pas de spécification visible dans `std_specs`, mais Verus traite `Arc` comme un pointeur
  transparent : `ta@ == a@` après `clone` a été prouvé sans rien ajouter.

## Chiffres

| | Creusot | Verus |
|---|---|---|
| lignes exécutables / spécification / preuve (*) | 83 / 96 / 86 (ancien module : 68 / 27 / 2, `run` `#[trusted]`) | 113 / 183 / 243 (dont checksum = `ck_seq(product)`, absent en Creusot) |
| vérification finale à froid | 12,1 s (module seul, `--no-cache`) ; 76,5 s avec `-f` | **2,2 s** pour le module (3,7 s mur pour les 3 modules) |
| appels jusqu'au premier « ok » | 18 : 2 erreurs de compilation, 15 échecs (dont 7 appels `-X` et 1 relance à `-t 10`), 1 ok | 4 : 3 échecs (6, 5, 3 erreurs), 1 ok |
| autres appels | 2 mesures à froid | 1 build release, 1 mesure |
| `trusted` / `assume` / `external_body` | aucun dans le module ; contrats `#[trusted]` de `creusot_std::std::thread` | aucun ; spécifications `vstd` de `thread`, `Arc`, `Vec` |
| checksum | `parmatmul:64` = `ca8d1bf8c2c39885` pour T = 1, 2, 3, 4, 7 ; n=100/T=200 et n=1024/T=4 identiques à `rust-plain` | idem |
| performance | identique à `rust-plain` (n=1024, T=4 : ≈ 250 ms les deux, machine chargée) | identique (≈ 100–145 ms contre 100–115 ms pour `rust-plain` au même moment) |

(*) Même méthode que `matmul.md` ; format `verusfmt` côté Verus.

## Expérience

* **Ce qui a coûté le plus (Creusot) : le recollement après `split_at_mut`.** Le noyau et les threads sont
  passés vite ; l'`ensures` de `blocks` (« bloc correct ++ suite correcte = tout correct ») a demandé
  10 versions. Causes, dans l'ordre :
  1. après `split_at_mut`, on ne peut plus nommer `^c` dans un `proof_assert!` (« use of borrowed variable ») ;
     des `snapshot!(mine)` ne marchent pas mieux (le fait de résolution `^tail == fin` n'est pas encore là) ;
  2. un lemme quantifié sur la valeur finale `∀ cf. ... ⟹ rows_from(cf, r0)` est la bonne idée, mais
  3. ses préconditions écrites `cf[..m*n]` utilisent `index_Seq` (opaque, `RangeTo`) alors que la
     spécification des tranches produit `cf[0..m*n]` (`subsequence`) : **deux termes différents pour la même
     chose**, le déclencheur ne s'instancie jamais. Trouvé seulement en lisant l'axiome dans la sortie `-X`.
     Avec `.subsequence(0, m*n)` partout, tout passe ;
  4. il a aussi fallu sortir l'arithmétique (`split_index`, `dot_shift`) dans des lemmes point par point
     (`concat_point`) : dans le gros contexte de `blocks`, les solveurs échouent même à 10 s.
* **Ce qui a coûté le plus (Verus) : rien de spécifique aux threads.** Les échecs étaient des bornes
  (`n·k` en usize 32 bits, `m·n` dans un invariant) et un cas de base de `ck_seq` (fuel 2).
* **Messages d'erreur.** Verus : clause et expression exactes. Creusot : « lines 200-208: proof failed »
  sur la fonction entière, compteur `37/43` ; il faut `-X` pour voir les buts, et leurs numéros de ligne
  (`[assertion]`, `blocks requires #2`) changent d'un essai à l'autre parce que la preuve est fragile.
* **Ce qu'une IA plus faible aurait raté.** (1) Qu'il faut joindre explicitement les threads pour que leur
  postcondition existe (la jonction implicite de `scope` n'apporte rien à la preuve). (2) Que Creusot infère
  la spécification d'une fermeture non annotée (et que l'annoter casse la compilation stable). (3) Le piège
  `cf[..x]` / `subsequence` ; (4) en Verus, l'absence de threads « scoped » et la restructuration
  (Arc + blocs rendus) qui garde le même comportement sans copie, puis la preuve du checksum par blocs.
