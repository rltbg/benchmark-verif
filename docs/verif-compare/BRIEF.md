# Creusot contre Verus — consigne commune

But : pour chaque workload, (1) réécrire le module **Creusot** existant pour qu'il soit un exemple de *bon code Creusot selon sa doc*,
(2) écrire le **même** module en **Verus**, avec la même cible de spécification, puis (3) consigner honnêtement l'expérience
(ce qui a marché, ce qui a bloqué, combien d'essais) pour comparer les deux outils. Le binaire compilé doit garder **exactement** le même
comportement (même checksum que les 8 autres colonnes) et les mêmes performances que possible.

## Outils et documentation (à lire, pas à deviner)
| | Creusot | Verus |
|---|---|---|
| crate du dépôt | `impls/rust` (lib `bench_rust`, bin `src/bin/bench.rs`) | `impls/rust-verus` (bin `src/main.rs`, modules `src/<nom>.rs`) |
| guide | `/opt/tools/creusot/guide/src/` (tutoriel, `view.md`, `snapshots.md`, `ghost*`, `logic_functions*`, `termination.md`, `type_invariants.md`, `erasure.md`, `limitations.md`) | `/home/user/verus-lang/verus/source/docs/guide/src/` (`SUMMARY.md`, `modes.md`, `integers.md`, `nonlinear.md`, `bitvec.md`, `triggers*`, `induction.md`, `seqs`…, `llmforverusproof.md`) |
| bibliothèque | `/opt/tools/creusot/creusot-std/src/` (`Seq`, `permutation_of`, `Int`, lemmes) | `/home/user/verus-lang/verus/source/vstd/` (`seq_lib`, `arithmetic/*`, `bytes`, `thread`, …) |
| exemples | `/opt/tools/creusot/tests/should_succeed/` | `/home/user/verus-lang/verus/examples/`, `source/rust_verify_test/tests/` |
| vérifier | dans **votre copie** : `cargo creusot -- --lib < /dev/null` (jamais `cargo creusot clean`) ; un but isolé : `script -qc "cargo creusot --why3find-arg=-X verif/…/f.coma -- --lib" /dev/null` | dans **votre copie** : `export PATH=/opt/tools/verus/verus-x86-linux:$PATH; cargo verus verify` (ou `cargo verus focus`) ; options utiles `-- --time`, `--expand-errors`, `--rlimit` |
| construire | `cargo build --release` | `cargo verus build --release` (vérifie puis compile) |

Spécifications des workloads : `spec/WORKLOADS.md` (5 de base) et `spec/workloads.d/<nom>.md` ; checksums de référence `spec/checksums.json`, `spec/checksums.d/*.json` ;
implémentation de référence sans preuve : `impls/rust-plain/src/`. Les tailles `smoke`/`tiny`/`small` sont dans `harness/presets.json` et `harness/presets.d/`.

## Isolation (plusieurs agents en parallèle sur 4 cœurs)
* Travaillez dans **vos copies privées** : `cp -r impls/rust $SCRATCH/creusot-<groupe>` et `cp -r impls/rust-verus $SCRATCH/verus-<groupe>` (`$SCRATCH` = `/tmp/claude-0/verif-<groupe>`), en ne gardant dans
  `lib.rs`/`main.rs` que `common` et vos modules (+ dépendances). Les autres agents modifient d'autres modules en même temps : une copie partagée casserait leurs preuves.
* Quand vos modules passent, recopiez **uniquement vos fichiers** dans le dépôt : `impls/rust/src/<nom>.rs` (+ le dossier `impls/rust/verif/bench_rust_rlib/<nom>/` produit par Creusot) et
  `impls/rust-verus/src/<nom>.rs`, puis dans `impls/rust-verus/src/main.rs` ajoutez **une ligne** sous `// @@MODS@@` et **une ligne** sous `// @@DISPATCH@@` (Edit minimal, ancres conservées).
  Ne modifiez pas `common.rs` du dépôt : si vous avez besoin d'une fonction commune, mettez-la dans votre module.
* Ne lancez ni `harness/bench.py` sur de grandes tailles, ni `fstar/build.sh`, ni `scripts/build-all.sh`. Testez vos binaires à la main (tailles smoke/small) en comparant le checksum. `nice -n 5` pour les preuves lourdes.
* Ne commitez pas.

## Ce qu'est du « bon code » ici (les deux outils)
1. **La preuve remplace les gardes.** Les gardes à l'exécution ajoutées seulement pour faciliter la preuve (`if v < n`, `if s < cap`, compteurs de visites…) doivent disparaître : prouvez l'invariant à la place.
   Gardez une garde seulement si la spécification du workload l'exige, et dites-le.
2. **Une spécification fonctionnelle** : le contrat dit *ce que* la fonction calcule, avec des fonctions logiques nommées (`#[logic]` / `spec fn`) et des lemmes, pas seulement « pas de panique ».
   Cible par workload ci-dessous ; si elle résiste, atteignez le niveau le plus haut possible et **écrivez précisément ce qui manque et pourquoi**.
3. **Terminaison** prouvée partout où l'outil le permet (`#[check(terminates)]` + `#[variant]` ; `decreases` en Verus).
4. **Aucun raccourci** : pas d'`assume`, `admit`, `#[trusted]`, `#[verifier::external_body]`, `#[verifier::exec_allows_no_decreases_clause]` sauf pour ce qui ne peut vraiment pas être vérifié (lecture de fichier, horloge, création de threads *si* l'outil ne les gère pas) — chacun est listé dans l'en-tête du module avec sa raison.
   Verus a une bibliothèque de threads spécifiée (`vstd::thread`) : utilisez-la avant de conclure que c'est impossible. Creusot a des permissions fantômes (`Perm`) et `#[erasure]`.
5. **Idiomatique selon la doc** de l'outil (vues `@`/`view()`, `snapshot!`/`ghost!` en Creusot ; modes `spec`/`proof`/`exec`, `Seq`, `broadcast`, `by (nonlinear_arith)`, `by (bit_vector)` en Verus), lisible, commenté brièvement.
6. **Même comportement et mêmes performances** que `impls/rust-plain` : même checksum aux tailles de `spec/checksums*.json` ; pas de copie ni d'allocation supplémentaire dans le code exécuté.

## Cibles de spécification (identiques pour les deux outils)
| workload | cible |
|---|---|
| sort, parsort | trié ∧ permutation (déjà prouvé en Creusot pour `sort`) ; `parsort` : idem, threads compris si possible |
| strsort | indices triés lexicographiquement (stable) ∧ permutation de 0..n |
| sieve | `f[i] == 0 ⇔ i premier` pour 2 ≤ i ≤ n ; le compteur = nombre de premiers ≤ n |
| matmul, parmatmul | `c` = produit matriciel modulo 2³² (fonction logique `dot`) — c'est la preuve qui avait échoué en Creusot |
| ntt | bornes et valeurs < p ; la permutation bit-reverse est une involution ; au-delà si possible (transformée = DFT modulaire) |
| hashmap | modèle dictionnaire : après les insertions, une recherche renvoie la valeur associée à la clé dans le modèle logique, 0 si absente |
| lines | l'agrégat par clé est la somme des valeurs des lignes de cette clé (lemme : analyse décimale ∘ écriture décimale = identité) |
| sha256 | le condensé exécutable égale une fonction logique SHA-256 (compression, bourrage) écrite dans la logique |
| parsha | idem par segment ; threads compris si possible |
| bfs | `dist[0] = 0` ; tout sommet atteint ≠ 0 a un prédécesseur à `dist − 1` ; pour toute arête u→v avec u atteint : v atteint et `dist[v] ≤ dist[u] + 1` |
| ptrchase | invariant d'arbre de recherche ; une recherche trouve une clé ssi elle a été insérée ; la chasse suit `next` |
| alloc | la somme repliée de chaque bloc est celle de son contenu ; la fenêtre FIFO garde les W derniers blocs |
| stream | le CRC calculé égale le CRC-32 logique du flux ; les compteurs du tokeniseur correspondent à un automate logique |

## Journal (obligatoire, c'est la matière de la comparaison)
* **Chaque** appel au vérificateur : une ligne CSV dans `docs/verif-compare/runs/<nom>-<creusot|verus>.csv` :
  `horodatage,duree_s,resultat(ok|echec|erreur_compil),buts_echoues,note courte`.
* À la fin, `docs/verif-compare/<nom>.md` avec, pour chaque outil : cible atteinte (et ce qui manque), lignes de code exécutable / de spécification / de preuve (compte approximatif, dites comment),
  temps de vérification final (à froid), nombre d'appels au vérificateur et d'échecs, `trusted`/`assume`/`external_body` restants avec raison, ce qui a coûté le plus,
  qualité des messages d'erreur, ce qu'un modèle d'IA plus faible aurait raté selon vous, et checksum vérifié.
* Soyez factuels : un échec bien documenté vaut mieux qu'une cible affaiblie en silence.
