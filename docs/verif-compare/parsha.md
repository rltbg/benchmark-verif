# parsha — Creusot contre Verus

Fichiers : `impls/rust/src/parsha.rs` (+ `verif/bench_rust_rlib/parsha/`), `impls/rust-verus/src/parsha.rs`.
Journaux : `runs/parsha-creusot.csv`, `runs/parsha-verus.csv`.

## Cible et résultat

Cible : « idem `sha256` par segment ; threads compris si possible ». **Atteinte dans les deux outils, threads compris.**

* `seg_digest(s, m) = sha256(gen_msg(SEED + s·2^40, m))` (fonctions de `sha256.rs`) ;
  `ck_segs(t, m)` = repliement dans l'ordre s = 0..t-1 des 8 mots de chaque condensé.
* `run(n, threads)` garantit `résultat == ck_segs(T, n / T)` avec `T = clamp(threads, 1, 64)`.
* Le noyau séquentiel n'est plus dupliqué : `segment` appelle `sha256::digest` (prouvé égal à la fonction logique),
  ce qui fait aussi le bourrage pour m quelconque (identique au bloc de bourrage de `rust-plain` quand 64 | m).
* Threads :
  * **Creusot** : `creusot_std::std::thread::spawn` + `JoinHandleExt::join_unwrap` (enveloppes `#[trusted]` de
    creusot-std) ; le contrat de la fermeture — **inféré** par Creusot, car Rust stable refuse les attributs
    `#[requires]/#[ensures]` sur une expression fermeture — est transmis à `valid_result` puis au résultat du join.
    La boucle de join parcourt `vec::IntoIter` avec un invariant sur sa vue (`it@ == handles[k-1..]`).
  * **Verus** : `vstd::thread::spawn` + `JoinHandle::join` (`external_body` de vstd) ; la fermeture porte un
    `ensures` explicite, qui devient `handle.predicate(r)`.

Écarts avec `rust-plain` (même travail par thread, même ordre de repliement, pas d'allocation en plus) :
`std::thread::spawn` (fermeture `'static`, ne capture que deux entiers) au lieu de `std::thread::scope` dans les deux
outils ; en Verus, `join` peut renvoyer `Err` (thread paniqué) et `panic!` exige `requires false`, d'où une branche
`Err(_) => segment(k, m)` (recalcul local) **inatteignable** puisque `segment` est prouvé sans panique — c'est la seule
garde ajoutée pour la preuve, imposée par la spécification de vstd ; en Verus les threads sont lancés du dernier
segment au premier pour que `Vec::pop` les rende dans l'ordre.

**Checksums vérifiés** (deux binaires) : `parsha@1:25600 = 88efd3aaddddca34`, `@2 = 2dc77ca2faac3765`,
`@4 = cc6118776b4becd3` (= `spec/checksums.d/par.json`). Temps n = 2,56·10^8, 4 threads (machine chargée) :
plain 0,35–1,15 s, Creusot 0,34–1,12 s, Verus 1,09–1,26 s — dispersion dominée par la charge.

## Chiffres

| | Creusot | Verus |
|---|---|---|
| lignes exec / spec / preuve (≈) | 47 / 21 / 17 | 56 / 30 / 19 |
| vérification finale à froid | 17,7 s mur (4 fichiers, why3find `-f --no-cache`) | 0,5 s (`verify-crate-time`, module seul) |
| appels (dev) | 8 (+ final) : 3 erreurs de compilation, 3 échecs | 4 (+ final) : 1 erreur de compilation, 2 échecs |
| `trusted` / `assume` / `external_body` dans le module | aucun (confiance : `spawn`/`join_unwrap` `#[trusted]` de creusot-std) | aucun (confiance : `spawn`/`join` `external_body` de vstd) |

(Les lignes `nul:` / `erreur d'usage CLI` du journal Verus sont des appels qui n'ont rien vérifié, gardés pour honnêteté.)

## Ce qui a coûté le plus

* **Creusot** : (1) trouver l'API (rien dans le guide ; lecture de `creusot-std/src/std/thread.rs`) ; type de
  l'argument `Ghost<Tokens>` à écrire ; (2) le contrat de fermeture : `#[ensures]` sur fermeture se vérifie mais ne
  compile pas en Rust stable (`attributes on expressions are experimental`) — découvert au `cargo build`, après une
  preuve réussie ; solution : laisser Creusot inférer le contrat ; (3) la boucle de join : l'invariant sur la vue de
  `IntoIter` et deux égalités de sous-séquences à fournir à la main ; un `snapshot!` du handle déplacé.
* **Verus** : `rlimit` dépassé sur `run` tant que la définition de `seg_digest` (donc toute la spec SHA-256) était
  visible ; corrigé par `#[verifier::opaque]` + `reveal` dans `segment`. Puis un dépliage de `ck_segs(0)` à donner.

## Messages d'erreur

Creusot : « proof failed » sur la fonction, sous-but identifié seulement via la session (« loop invariant #1 », sans
dire quelle boucle). Verus : « invariant not satisfied before loop » avec la ligne exacte ; « rlimit exceeded » avec
suggestion `--profile`.

## Ce qu'un modèle plus faible aurait probablement raté

* Conclure trop vite « threads non vérifiables » (c'était l'état d'origine : `run` `#[trusted]`), alors que les deux
  bibliothèques ont des enveloppes spécifiées.
* Creusot : vérifier aussi que le code *compile en Rust stable* après la preuve (le vérificateur accepte une syntaxe que
  `cargo build` refuse) ; savoir que les fermetures sans contrat ont un contrat inféré.
* Verus : comprendre qu'un `rlimit` vient d'un contexte trop riche et le réduire par opacité plutôt qu'en ajoutant des
  assertions ; gérer `join() -> Result` sans `unwrap` (interdit sans prouver `Ok`).
