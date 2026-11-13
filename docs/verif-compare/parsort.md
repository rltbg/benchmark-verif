# parsort — Creusot contre Verus

Fichiers : `impls/rust/src/parsort.rs` (+ `impls/rust/verif/bench_rust_rlib/parsort/`), `impls/rust-verus/src/parsort.rs`.
Journaux : `runs/parsort-creusot.csv`, `runs/parsort-verus.csv`.

## Structure commune

Les noyaux sont ceux de `sort` (réutilisés, plus de copie du code). Le découpage des tableaux, au lieu d'une boucle
`split_at_mut` + `spawn` comme dans `rust-plain`, est **récursif** : `sort_runs(a, tmp, b, k, t)` détache le segment `k`,
le confie à un thread, traite récursivement la suite sur le thread courant (le dernier segment y est trié), puis joint ;
`merge_pairs` fait de même pour les fusions d'un niveau (`merge_back` = fusion + recopie). Même nombre de threads, mêmes
segments `n·k/t`, mêmes fusions, même ordre de niveaux — seules différences : jointures explicites (ordre inverse) au lieu
de la jointure implicite en fin de `scope`, et un `scope` par niveau (Creusot) ou par fourche (Verus). La récursion rend la
preuve **locale** : `a = ma ++ ra`, contrat du thread sur `ma`, contrat de l'appel récursif sur `ra`, au lieu d'invariants
de boucle sur une suite de prophéties d'emprunts. Les bornes sont construites par `push` (pas `map/collect`).

Spécifications : `wf_bounds`, `runs_sorted(s, b, k, m, base)`, `pairs_sorted`, lemmes `sorted_shift`, `runs_split`/
`pairs_split` (Creusot) ou `runs_concat`/`pairs_concat` (Verus). `run` : boucle des niveaux avec invariant « bornes
correctes ∧ chaque run trié ∧ permutation de l'entrée », reconstruction des bornes prouvée (cas nombre de runs pair/impair,
run impair final intact), croissance de `n·k/t` prouvée (Creusot : automatique ; Verus : `lemma_mul_inequality`,
`lemma_div_is_ordered`, `lemma_div_by_multiple`). Résultat final : `a` trié ∧ permutation de l'entrée.

## Cible : trié ∧ permutation, threads compris

| | Creusot | Verus |
|---|---|---|
| noyaux | prouvés (ceux de `sort`) | prouvés (ceux de `sort`) |
| orchestration (bornes, niveaux, découpage) | prouvée | prouvée |
| **threads** | **prouvés** via l'API spécifiée de creusot-std : `creusot_std::std::thread::{scope, Scope::spawn}` + `JoinHandleExt::join_unwrap` ; aucun `#[trusted]` dans le module | **non vérifiés** : 2 `#[verifier::external_body]` (`fork_sort`, `fork_merge`) |
| base de confiance pour les threads | les enveloppes `#[trusted]` de creusot-std (spec de `scope`/`spawn`/`join`, comme `vstd::thread` côté Verus) | la spec écrite à la main de `fork_sort`/`fork_merge` (voir ci-dessous) |
| terminaison | noyaux oui ; `sort_runs`/`merge_pairs`/`run` non (`spawn`/`join_unwrap` ne sont pas déclarés terminants dans creusot-std) | toutes les fonctions vérifiées ont `decreases` (lexicographique `t-k, 0/1` à travers les fourches) ; mais la récursion passe par les `external_body`, donc sa terminaison n'est réellement vérifiée que dans la variante séquentielle de contrôle |

### Creusot : ce qui est possible et ce qui ne l'est pas

* Possible : `Scope::spawn(f)` garantit, après `join_unwrap`, la postcondition de la clôture ; une clôture `move` qui
  capture des `&mut [u32]` disjoints (issus de `split_at_mut`) voit sa postcondition porter sur la valeur **finale**
  (prophétisée) de ces tranches, et les prophéties se recombinent avec la spec de `split_at_mut`
  (`(^self)@[..mid] == (^result.0)@`). Toute la chaîne se prouve sans rien admettre dans le module.
* Contraintes rencontrées :
  - **pas de contrat explicite sur une clôture avec rustc stable** : `#[requires]/#[ensures]` sur une clôture passée en
    argument (ou liée par `let`) est un « attribute on expression », refusé par rustc 1.97 stable lors du `cargo build`
    normal (Creusot l'accepte). Contournement : clôtures **sans contrat** (Creusot infère leur spec du corps), dont le corps
    est un seul appel à une fonction contractée (`msort`, `merge_back`) ; et `scope` enveloppé dans des fonctions
    contractées `sort_level` / `merge_level` (sinon un but d'invariant de `run` devenait instable) ;
  - une clôture liée par `let` puis passée à `scope` casse l'inférence de type `for<'scope> FnOnce(&mut Scope<'scope,'env>)` :
    il faut l'écrire en ligne ;
  - dans le corps, la valeur finale `^a` du paramètre n'est reliée à celle du re-emprunt fait par `split_at_mut` qu'au
    moment de la postcondition (résolution de `a`) : une assertion sur `(^a)@` dans le corps échoue (même la longueur).
    Solution : énoncer les lemmes (`runs_split`, `pairs_split`) sur la suite finale elle-même et les rendre disponibles par
    `let _ = snapshot!(runs_split);` ; ils s'appliquent lors de la preuve de la postcondition ;
  - les jointures doivent être explicites (`join_unwrap`) : la jointure implicite de fin de `scope` ne transmet aucune
    postcondition.
* Impossible ici : prouver la terminaison des fonctions qui lancent des threads (pas de spec `terminates` pour `spawn`).

### Verus : pourquoi `external_body`, et ce qui est admis exactement

* `vstd::thread::spawn` exige `F: FnOnce() -> Ret + Send + 'static` : il ne peut pas recevoir des tranches **empruntées**
  au tableau. Essai consigné (`runs/parsort-verus.csv`, dernière ligne) : Verus refuse même avant rustc —
  « Verus does not currently support closures capturing a mutable reference (mutably captured variable `ma`) ».
  vstd n'a pas d'équivalent de `std::thread::scope`. Les seules alternatives vérifiables changeraient le programme :
  copier les segments dans des `Vec` possédés (allocations et copies supplémentaires, interdites par la consigne) ou passer
  à des pointeurs bruts avec permissions `PointsTo` par élément (réécriture de tous les noyaux).
* Donc deux fonctions `external_body` minimales : `fork_sort(ma, mt, ra, rt, b, k, t)` exécute `msort(ma, mt, 0, |ma|)`
  dans un thread scopé pendant que le thread courant exécute `sort_runs(ra, rt, b, k+1, t)` ; `fork_merge` idem avec
  `merge_back` ∥ `merge_pairs(…, p+1, …)`. Leur `requires`/`ensures` est **exactement la conjonction** des contrats vérifiés
  des deux appels. Contrôle (journalisé) : en remplaçant le corps par les deux appels **séquentiels** et en retirant
  `external_body`, Verus vérifie ces mêmes specs (avec terminaison). Ce qui est admis se réduit donc à : « exécuter en
  parallèle deux appels sur des `&mut` disjoints a le même effet que les exécuter l'un après l'autre » (garanti par
  l'absence de course de données en Rust sûr) et « `scope` joint ses threads ».

## Chiffres

| | Creusot | Verus |
|---|---|---|
| exec / spec / preuve (méthode de `sort.md`) | 69 / 61 / 47 | 96 / 167 / 80 |
| appels au vérificateur | 23 (11 échecs, 6 erreurs de compilation) | 8 (2 échecs, 3 erreurs de compilation, dont 2 essais volontaires) |
| temps final à froid | 12–22 s (`--no-cache 'parsort::*'`) ; recherches de preuve initiales jusqu'à 166 s | crate entière 3,3 s |
| `trusted`/`assume`/`external_body` | aucun dans le module (repose sur creusot-std `#[trusted]` pour `scope`/`spawn`/`join_unwrap`) | 2 `external_body` (`fork_sort`, `fork_merge`), raison ci-dessus |
| checksum | `parsort 10000` (T = 1, 2, 3, 4, 7) = 351c39298dce79ff (= `spec/checksums.d/par.json`) ; 4e6 × 4 = d1fe0d0fbb9f7715 (= rust-plain) | idem |
| performance (4e6, 4 threads, machine chargée) | ≈ rust-plain (0,17–0,47 s selon la charge, mêmes plages pour les trois) | idem |

### Robustesse des preuves Creusot sous charge

La machine (4 cœurs) était partagée par plusieurs agents (charge 7–10). Plusieurs buts de `parsort` (postconditions de
`sort_runs`/`merge_pairs`, un invariant de `run`) et un de `strsort::merge` passaient seuls mais **échouaient par délai
dépassé** dans la vérification de toute la lib (`cargo creusot -- --lib`, why3find `-j 4`, délai médian 1 s). Après ajout
d'indications et reformulation des lemmes (`runs_split`/`pairs_split`) : 3 des 4 dernières vérifications à froid de toute
la lib (`--no-cache`) passent, la 4e (la première) échouait sur un but ; le rejeu avec `proof.json` passe (2,5 s). Verus
n'a montré aucune instabilité (toujours ~3 s).

## Ce qui a coûté

* **Creusot** : les règles de capture des clôtures dans les contrats (`old(len)`, capture « consommée »), le refus des
  contrats de clôture par rustc stable, l'inférence HRTB de `scope`, puis la résolution tardive de `^a` ; enfin la fragilité
  sous charge. Une fois la structure récursive trouvée, la preuve elle-même (permutations, décalages d'indices) a été rapide.
* **Verus** : très peu sur la logique (2 échecs : déclencheurs `b[j+1]` à écrire sous la forme exacte `b[(2p+1)+1]`,
  longueurs finales de `tmp` à ajouter aux contrats). Le coût est ailleurs : constater que les threads empruntants sont
  hors de portée de vstd et isoler proprement le code admis.

## Messages d'erreur

* Creusot : les erreurs de traduction (« Use of a closure capture in an #[ensures(..)] clause, but it is consumed by the
  closure », « Use of borrowed or uninitialized variable ») sont claires et pointent l'expression ; les échecs de preuve
  demandent toujours `why3find -x/-X` pour savoir quel but échoue.
* Verus : clairs, y compris le refus des clôtures capturant `&mut` ; l'erreur de typage `int`/`nat` sur `b@[0 + 1]` est
  surprenante mais immédiate.

## Ce qu'une IA plus faible aurait raté

* Le passage de la boucle de `rust-plain` à une **récursion** pour rendre la preuve des threads locale (avec la boucle, il
  faut des invariants sur des suites d'emprunts prophétisés).
* En Creusot : que la postcondition d'un thread n'est obtenue qu'avec une jointure explicite ; que les contrats de clôture
  ne compilent pas en Rust stable (et l'astuce clôture-sans-contrat + fonction contractée) ; que `^a` n'est utilisable que
  dans la postcondition.
* En Verus : conclure trop vite « threads impossibles » sans isoler un noyau admis minimal, ou au contraire mettre tout
  `run` en `external_body` ; et la vérification séquentielle de contrôle qui montre que la spec admise n'est que la
  conjonction des contrats prouvés.
