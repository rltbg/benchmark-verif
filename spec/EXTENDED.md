# Workloads « extrêmes » — contrat commun

Chaque workload étendu doit exister dans **les 8 colonnes** (c-native, zig, rust-plain, rust-creusot, fstar-c, fstar-fsharp,
fstar-ocaml, fstar-ocaml-fast) avec **le même checksum** à toutes les tailles. Les colonnes F* sont **générées** à partir d'un noyau
Pulse unique `fstar/src/Bench.<Nom>.fst` (mêmes extractions C / F# / OCaml que les 5 workloads de base : lire `Bench.Sieve.fst`,
`Bench.Hashmap.fst`, `Bench.Common.fst`, `spec/WORKLOADS.md` pour les conventions : `mix64`, `ckfold`, graines, format de sortie).

## Fichiers à créer (un jeu par workload `<nom>`, tout en minuscules sans tiret)
| fichier | rôle |
|---|---|
| `spec/workloads.d/<nom>.md` | spécification précise (algorithme, graines, formule du checksum, tailles) — suffisante pour réimplémenter |
| `spec/workloads.d/<nom>.json` | `{"name","unit","work_items","ideal_bytes","desc"}` ; les deux formules sont des expressions Python en `n` et `t` (threads) |
| `spec/checksums.d/<nom>.json` | `{"<nom>:<n>": "<checksum 16 hex>"}` aux tailles smoke (et plus si utile) |
| `harness/presets.d/<nom>.json` | `{"<nom>": {"smoke":..,"tiny":..,"small":..,"medium":..,"large":..,"xlarge":..,"saturate":..}}` ; valeur = entier n, ou `{"n":..,"threads":..}` |
| `impls/c-native/w_<nom>.c` | `static u64 run_<nom>(size_t n, int threads)` ; une ligne `#include "w_<nom>.c"` sous l'ancre de `w_all.h`, une ligne de dispatch sous `/* @@DISPATCH@@ */` de `bench.c` |
| `impls/zig/w_<nom>.zig` | `pub fn run(n: usize, threads: usize) u64` ; import sous `// @@IMPORTS@@`, dispatch sous `// @@DISPATCH@@` dans `extended()` de `bench.zig` |
| `impls/rust-plain/src/w_<nom>.rs` | `pub fn run(n: usize, threads: usize) -> u64` ; `mod` sous `// @@MODS@@`, arm sous `// @@DISPATCH@@` de `main.rs` |
| `impls/rust/src/<nom>.rs` | même signature, **avec contrats Creusot** (même niveau que les modules existants : pas de panique, pas de débordement) ; `pub mod` sous l'ancre de `lib.rs`, arm dans `src/bin/bench.rs` |
| `fstar/src/Bench.<Nom>.fst` | noyau Pulse `run : SZ.t -> stt`… (même forme que `Bench.Sieve.run`) ; ajouter `<Nom>.fst` sous l'ancre `# @@MODS@@` de `fstar/build.sh`, `--custard_entry Bench.<Nom>.run` sous `# @@ENTRIES@@`, `let <nom> = Bench.<Nom>.run` sous `(* @@ALL@@` de `Bench.All.fst` |
| `fstar/drivers/{driver.c,Driver.fs,driver.ml}` | une ligne de dispatch sous `@@DISPATCH@@` dans chacun |

CLI de tous les binaires : `<bin> <workload> <n> [threads]` ; sortie `RESULT workload=<nom> n=<n> checksum=<16 hex> time_ns=<ns>`
(temps mesuré **autour du calcul seul**, hors démarrage). Les ancres (`@@…@@`) doivent être **conservées** : on insère une ligne
*avant ou après* l'ancre avec un Edit minimal, jamais en réécrivant tout le fichier (d'autres agents éditent les mêmes fichiers en parallèle).

## Règles de travail (plusieurs agents en parallèle sur la même machine, 4 cœurs / 15 Go)
* **Ne lancez pas** `fstar/build.sh`, `scripts/build-all.sh`, `make`, ni `harness/bench.py run` sur de grandes tailles (mesures de temps
  faussées pour les autres, répertoires `_build` partagés). Testez vos binaires à la main sur tailles smoke/small seulement.
* F* : vérifiez dans **votre propre cache** : `cd fstar/src && fstar.exe -c --cache_dir ../_build/cache-<nom> Bench.Common.fst` puis
  `... Bench.<Nom>.fst` (chaîne des .fsti/.fst dont vous dépendez). Pour tester les trois extractions, extrayez **votre module seul**
  vers `fstar/_build/x-<nom>/{c,fsharp,ocaml}` avec `fstar.exe --cache_dir ../_build/cache-<nom> Bench.<Nom>.fst --codegen C|FSharp|OCaml --custard_entry Bench.<Nom>.run --odir …`,
  et un petit pilote jetable (voir `fstar/build.sh`, fonctions `target_c`, `target_fsharp`, `target_ocaml` pour la marche à suivre exacte). Environnement : `export PATH=/opt/tools/fstar2/fstar/bin:$PATH`.
  Utilisez `nice -n 5` pour les vérifications lourdes. L'intégration complète (un seul cache, Bench.All) sera faite par le coordinateur.
* Creusot : `cd impls/rust && cargo creusot -- --lib < /dev/null` (voir README pour les pièges : jamais `cargo creusot clean`). Spécification Creusot au même niveau
  que les modules existants ; `#[trusted]` seulement pour ce qui ne peut pas être prouvé (threads, fichiers, allocation de gros blocs) et le dire dans l'en-tête du module.
* Preuve F* / Creusot : mémoire-sûreté, bornes, absence de débordement ; **pas** de correction fonctionnelle complète exigée. Si une preuve résiste, simplifiez la *forme* du code
  (boucles plates, gardes explicites, `opaque_to_smt`, invariants plus simples — voir le résumé des idiomes Pulse dans la conversation) plutôt que d'abandonner. En dernier recours seulement,
  déclarez la colonne non implémentée : `"unsupported": {"<nom>": "<raison>"}` dans l'entrée de `harness/impls.json` (c'est un résultat honnête, pas un échec silencieux).
* Parallélisme (si votre workload en a) : le **noyau reste séquentiel et prouvé** (il prend des bornes de plage) ; les threads sont dans le code non prouvé qui l'appelle
  (pilotes C/F#/OCaml pour F*, wrapper non prouvé pour Creusot). Si une chaîne ne peut pas le faire, `unsupported` avec la raison.
* Ne commitez pas : le coordinateur commite. Ne touchez pas aux fichiers d'un autre workload.
* Rapport final (≤ 25 lignes) : fichiers créés, checksums de référence, ce qui est prouvé/pas prouvé, colonnes `unsupported` + raison, tailles recommandées par preset avec durée mesurée
  en C natif (cible : `large` ≈ 5–30 s, `xlarge` ≈ 30–120 s, `saturate` ≈ 60–300 s ou ~12 Go de RAM au maximum — jamais plus de 13 Go de pic), problèmes rencontrés.

* OCaml : ajoutez une ligne `Bench_<Nom>.ml` à `fstar/ocaml/extra-modules.txt` (modules compilés après Bench_Hashmap, utilisés par `build-ocaml.sh` ; les variables `ML_MODULES` et `ML_DRIVER` permettent de tester un module seul).
