# Benchmark de programmation vérifiée : F* (→ C / OCaml / F#) et Rust + Creusot

Question posée : *« le code vérifié (F*, Creusot) tient-il la route face au code écrit à la main, en
CPU, en RAM, en disque et surtout en débit (5 lignes/s ou 150 000 ?) »*.

Ce dépôt contient **cinq workloads identiques** (spécification dans [`spec/WORKLOADS.md`](spec/WORKLOADS.md)),
implémentés dans plusieurs langages, un **harnais de mesure** externe (temps, CPU, pic RAM + courbe
RAM, défauts de page, E/S disque, taille des binaires, coût de démarrage) et un **rapport** (Markdown,
HTML, CSV). Chaque implémentation doit produire le **même checksum** ; sinon le résultat est marqué ❌.

## Ce qui est mesuré, et ce qui ne l'est pas encore — lisez ceci d'abord

| colonne | source | état dans cette session |
|---|---|---|
| `c-ref` — C écrit à la main | `impls/c-ref` | ✅ construit, exécuté, mesuré (oracle des checksums) |
| `rust-creusot` — Rust annoté Creusot | `impls/rust` | ✅ construit (`cargo build` efface les contrats), exécuté, mesuré. ⚠️ **les preuves Creusot n'ont pas été lancées** (Creusot n'est pas installé ici) |
| `ocaml-ref` — OCaml écrit à la main | `impls/ocaml-ref` | ✅ construit, exécuté, mesuré (contrôle) |
| `fsharp-ref` — F# écrit à la main | `impls/fsharp-ref` | ✅ construit, exécuté, mesuré (contrôle) |
| `fstar-c` — F* → Low* → KaRaMeL → C | `fstar/` | ⚠️ **sources et scripts écrits, jamais exécutés** |
| `fstar-ocaml` — F* → OCaml | `fstar/` | ⚠️ idem |
| `fstar-fsharp` — F* → F# | `fstar/` | ⚠️ idem, et **expérimental** (voir plus bas) |

**Pourquoi les trois colonnes F* n'ont pas de chiffres.** J'ai pu télécharger le binaire officiel de F*
(v2025.03.25) mais l'exécution d'un binaire téléchargé a été refusée par le contrôle de permissions de
la session. Je n'ai donc **pas pu vérifier ni extraire** les modules F*. Les six fichiers
`fstar/src/Bench.*.fst` ont été écrits en lisant les signatures de la ulib livrée (`LowStar.Buffer`,
`FStar.UInt32`, `FStar.Math.Lemmas`…) mais **n'ont jamais été typés par F\***. Attendez-vous à
quelques itérations de correction au premier `fstar/build.sh verify` (obligations de preuve SMT
manquantes, noms de lemmes, syntaxe). Les lignes F* du rapport apparaissent automatiquement dès que
les binaires `fstar/_build/{c,ocaml,fsharp}` existent.

Les checksums des quatre implémentations exécutables sont **identiques sur les cinq workloads**,
donc la spécification est non ambiguë et les futures sorties F* pourront être comparées à un oracle.

## Mise en route

```bash
make refs                     # C, Rust, OCaml, F# : build + empreinte disque (gcc, cargo, ocamlfind, dotnet 8)
make bench PRESET=medium      # exécute tout ce qui est construit ; écrit results/run-medium.{json,md,html,csv}
python3 harness/bench.py list # implémentations détectées + tailles des presets

# F* (optionnel, voir ci-dessous)
scripts/setup-fstar.sh && scripts/setup-karamel.sh
make fstar-verify             # ne fait que vérifier les modules
make fstar                    # vérifie puis construit fstar-c, fstar-ocaml, fstar-fsharp

# Creusot (optionnel)
scripts/setup-creusot.sh && make creusot   # `cargo creusot prove`, temps enregistré dans le rapport
```

Presets : `smoke` (validation + coût de démarrage), `small`, `medium`, `large` (≈ 1 Go de RAM par
workload), `xlarge` (jusqu'à plusieurs Go : sieve 3 G, matmul 4096, lines 100 M). Options utiles :
`--repeat N`, `--timeout S`, `--impls a,b`, `--workloads w1,w2`, et `--wrapper "…"` pour préfixer la
commande, par exemple `--wrapper "taskset -c 2"` pour épingler un cœur ou
`--wrapper "systemd-run --scope -p MemoryMax=2G --user"` pour plafonner la RAM et voir qui meurt.

## Organisation

```
spec/WORKLOADS.md           spécification des 5 workloads (algorithmes, graines, checksums, tailles)
harness/bench.py            lanceur + mesures + rapport ; impls.json / presets.json
impls/c-ref                 C à la main (plancher de perf, oracle de checksum)
impls/rust                  crate Rust annoté Creusot (#[requires]/#[ensures]/#[invariant])
impls/ocaml-ref, fsharp-ref ports idiomatiques à la main (contrôles pour les colonnes F* → OCaml / F#)
fstar/src                   Bench.{Common,Sort,Sieve,Matmul,Lines,Hashmap}.fst — Low* vérifié
fstar/drivers, build.sh     pilotes C/OCaml/F# + pipeline verify → krml → C | OCaml | F#
fstar/ocaml/fastbuf.ml      réalisation OCaml native des buffers (voir « constats »)
scripts/                    installation de F*, KaRaMeL, Creusot ; build des références
results/                    sorties des runs
```

## Les cinq workloads (ce qu'ils attaquent)

| workload | CPU | RAM | particularité |
|---|:-:|:-:|---|
| `sort` — tri fusion de n u32, buffer de 2n | ●●● | ●●○ | branches imprévisibles, deux flux d'écriture |
| `sieve` — crible d'Ératosthène jusqu'à n | ●○○ | ●●● | bande passante mémoire, 1 octet/entier, jusqu'à 3 Go |
| `matmul` — produit dense n×n en u32 | ●●● | ●○○ | arithmétique, vectorisation, indices non linéaires à prouver |
| `lines` — génère puis parse n lignes `clé;valeur`, agrège par clé | ●●○ | ●●○ | **lignes/seconde**, flux d'octets, table de 16 Mo en accès aléatoire |
| `hashmap` — adressage ouvert, n insertions + 2n recherches | ●○○ | ●●● | latence mémoire (un défaut de cache par sonde) |

Chaque workload dérive ses entrées d'une fonction pure de l'indice (splitmix64), pas de PRNG à état
ni de fichier d'entrée : aucune variance liée au générateur ni au disque. Le disque est mesuré par les
compteurs d'E/S du noyau (`ru_inblock/oublock`) et par la taille des binaires/runtimes ; aucun
workload ne lit de fichier (un benchmark « disque » mesurerait le disque, pas le langage).

## Méthodologie de mesure

* **Débit** : mesuré *dans* le processus autour du calcul seul (horloge monotone), donc sans le
  démarrage de .NET (~60 ms, ~27 Mo) ; le coût de démarrage est rapporté séparément.
* **CPU / RAM / disque** : mesurés *de l'extérieur* par le harnais (`wait4` : user, sys, `ru_maxrss`,
  défauts de page, blocs E/S) plus un échantillonneur `/proc/<pid>` toutes les 20 ms (courbe de RSS,
  `VmHWM`). `ru_maxrss` hérite du RSS du harnais Python lors du `fork` ; le harnais le corrige avec
  `VmHWM` quand le pic réel est inférieur — pour les runs de quelques millisecondes, la valeur de
  pic est donc une borne basse.
* **RAM/idéal** : pic RSS divisé par la taille des structures de la spécification (voir
  `spec/WORKLOADS.md`). 1.00 = aucune surconsommation ; c'est la mesure la plus parlante de
  l'empreinte d'un runtime et de sa représentation des données (par exemple un `int array` OCaml
  pèse 8 octets par u32).
* Médiane de N exécutions ; l'écart relatif min–max du temps interne est conservé dans le JSON.
* Les checksums de toutes les implémentations d'une même ligne doivent être égaux (✅/❌).
* Limites : une seule machine virtuelle, pas d'épinglage de cœur par défaut, pas de gouverneur CPU
  figé, pas d'isolation du bruit des autres VM. Les comparaisons relatives entre lignes d'un même
  run sont fiables à quelques pourcents ; les valeurs absolues ne le sont pas.
* Équité des compilateurs : `gcc -O3` (sans `-march=native`), `rustc -C opt-level=3 + LTO`,
  `ocamlopt -O3` (sans flambda, `-O3` est ignoré), `dotnet` Release avec TieredPGO. Les accès restent
  contrôlés (bornes) en Rust, OCaml et .NET, comme dans l'idiome de chaque langage.

## Ce que « vérifié » veut dire ici (et ce que ça coûte à l'exécution)

Le point à retenir : **les preuves sont effacées à la compilation**. Ce qui coûte à l'exécution, ce
sont les *choix de représentation et de style* que la vérification impose, pas les preuves elles-mêmes.

| implémentation | ce qui est démontré (visé) | gardes / écarts par rapport au C à la main |
|---|---|---|
| F* (Low*) | mémoire sûre (aucun accès hors bornes), seuls les buffers annoncés sont modifiés, pas de débordement des indices ; arithmétique des PRNG en `*_mod` | `lines` : `p < len` testé à chaque octet (machine à états) ; `hashmap` : indices masqués ; `sieve`/`hashmap` : quelques `*_mod` là où la preuve de non-débordement n'a pas été faite (documentés dans l'en-tête de chaque module) |
| Rust + Creusot | absence de panique : indices dans les bornes et pas de débordement hors `wrapping_*`, via `#[requires]`, `#[ensures]`, `#[invariant]` | les corps de `mix64` et `fold` sont `#[trusted]` ; l'invariant de forme du texte de `lines` n'est pas écrit (les accès restent contrôlés par Rust) ; `matmul` demande un lemme d'arithmétique non linéaire non fourni |

Aucune des deux chaînes ne prouve ici la **correction fonctionnelle** (le tri est une permutation
triée, le crible renvoie les premiers…). C'est volontaire : elle n'ajoute pas une instruction au code
généré, seulement du temps de preuve. Le checksum, comparé à l'oracle C, tient lieu de test de
non-régression. Quand les preuves tourneront, le temps de vérification s'affichera dans la table
« Empreinte disque et coût de construction ».

## Constats déjà établis en lisant la bibliothèque de F* (sans l'exécuter)

Ces points viennent de la lecture du tarball officiel `fstar-v2025.03.25-Linux-x86_64`, pas de
mesures ; ils expliquent pourquoi les colonnes OCaml/F# de F* ne sont pas un simple « F* mais plus
lent ».

1. **Le piège de `LowStar.Buffer` vers OCaml.** La réalisation OCaml fournie
   (`ulib.ml/LowStar_Monotonic_Buffer.ml`) stocke le contenu d'un buffer dans une `FStar.Seq`, qui est
   en OCaml une **liste chaînée** (`type 'a seq = MkSeq of 'a list`, `index`/`upd` en O(n)). Un tri de
   8 millions d'éléments y est quadratique : irréalisable. `FStar.Array` en OCaml est encore pire
   (séquence immuable dans une `ref`, copiée à chaque `upd`). `fstar/ocaml/fastbuf.ml` fournit une
   réalisation native (tableau OCaml + décalage) que `FASTBUF=1 fstar/build.sh ocaml` substitue par
   réécriture du code extrait. Sans elle, attendez-vous à des `timeout` dès `small`.
2. **Entiers machine = `Stdint` boxés.** `FStar.UInt32/UInt64/UInt8` sont réalisés par `Stdint`
   (blocs custom alloués dans le tas) et tout tableau extrait est un `'a array` générique : un `u8`
   coûte donc un pointeur plus un bloc boxé. Le crible de 1 G ne peut pas tenir dans 16 Go avec ce
   modèle ; `sieve` doit être mesuré à `small`/`medium` seulement pour `fstar-ocaml`.
3. **`int` et `nat` F\* = Zarith** (`Prims.int` est un `Z.t`). Le code de ce dépôt n'utilise des
   `int` que dans les spécifications (effacées) ; tout le calcul est en entiers machine.
4. **Pas de runtime F# dans la distribution.** Le tarball ne contient que `ulib/ml` (pas de `ulib/fs`)
   et un accès direct à `ulib/fs/Prims.fs` sur la branche principale du dépôt amont renvoie 404. Le backend
   `--codegen FSharp` existe encore dans le compilateur mais n'a plus de bibliothèque d'exécution ;
   `fstar/fsharp/Runtime.fs` n'en est qu'un squelette hypothétique (entiers, conversions, buffers) et
   `fstar-fsharp` est donc **expérimental**. Le contrôle `fsharp-ref` donne le plancher que .NET sait
   atteindre.
5. **Le chemin sérieux vers la vitesse est Low\* → C.** C'est celui que HACL\*, EverCrypt, etc.
   utilisent : KaRaMeL produit du C idiomatique que gcc optimise comme le C écrit à la main. Le surcoût
   attendu est celui des gardes et de la forme des boucles (récursions terminales converties en
   `while`), pas celui des preuves.

## Résultats

Voir [`results/`](results/) : `run-medium.{md,html,csv}` et `run-large.{md,html,csv}`. Synthèse
au moment de la rédaction, pour les implémentations exécutables (moyenne géométrique des cinq
workloads, 1.00 = meilleure du workload) :

### Preset `large` — débit (plus haut = mieux) / pic RAM

| workload (n) | C (main, non vérifié) | Rust + contrats Creusot | F# (main, non vérifié) | OCaml (main, non vérifié) |
|---|--:|--:|--:|--:|
| sort (32 000 000) | 8.06 M éléments/s · 245 Mo | 7.14 M éléments/s · 246 Mo | 5.79 M éléments/s · 271 Mo | 5.38 M éléments/s · 501 Mo |
| sieve (1 000 000 000) | 45.79 M entiers/s · 955 Mo | 47.08 M entiers/s · 955 Mo | 47.05 M entiers/s · 981 Mo | 43.00 M entiers/s · 973 Mo |
| matmul (2 500) | 3.29 G mult-add/s · 73 Mo | 2.92 G mult-add/s · 73 Mo | 415.72 M mult-add/s · 99 Mo | 228.30 M mult-add/s · 149 Mo |
| lines (40 000 000) | 16.85 M lignes/s · 545 Mo | 12.87 M lignes/s · 545 Mo | 11.76 M lignes/s · 571 Mo | 6.62 M lignes/s · 557 Mo |
| hashmap (16 000 000) | 15.32 M opérations/s · 513 Mo | 14.71 M opérations/s · 514 Mo | 11.74 M opérations/s · 539 Mo | 8.12 M opérations/s · 517 Mo |

### Preset `medium` — débit (plus haut = mieux) / pic RAM

| workload (n) | C (main, non vérifié) | Rust + contrats Creusot | F# (main, non vérifié) | OCaml (main, non vérifié) |
|---|--:|--:|--:|--:|
| sort (8 000 000) | 8.15 M éléments/s · 62 Mo | 7.39 M éléments/s · 63 Mo | 6.26 M éléments/s · 88 Mo | 6.23 M éléments/s · 129 Mo |
| sieve (200 000 000) | 58.72 M entiers/s · 192 Mo | 59.00 M entiers/s · 192 Mo | 58.19 M entiers/s · 218 Mo | 57.85 M entiers/s · 196 Mo |
| matmul (1 600) | 3.24 G mult-add/s · 31 Mo | 2.86 G mult-add/s · 31 Mo | 685.21 M mult-add/s · 57 Mo | 326.97 M mult-add/s · 64 Mo |
| lines (10 000 000) | 14.06 M lignes/s · 149 Mo | 12.60 M lignes/s · 150 Mo | 11.96 M lignes/s · 175 Mo | 7.59 M lignes/s · 155 Mo |
| hashmap (4 000 000) | 19.21 M opérations/s · 129 Mo | 15.41 M opérations/s · 130 Mo | 14.45 M opérations/s · 155 Mo | 10.31 M opérations/s · 133 Mo |

Synthèse (moyenne géométrique du temps relatif au meilleur, `large`) : C ×1.01 · Rust+Creusot ×1.12 · F# ×1.83 · OCaml ×2.58 ; pic RAM relatif : C ×1.00 · Rust ×1.00 · F# ×1.11 · OCaml ×1.34. Coût de démarrage : C/Rust/OCaml 2–3 ms et < 1 Mo ; F# (.NET) ≈ 60 ms et 27 Mo, plus 67 Mo de runtime à installer. Les 40 checksums (2 presets × 5 workloads × 4 implémentations) concordent.

Lecture rapide : sur ces workloads le **Rust annoté Creusot coûte ~10 % de plus que le C** (hypothèse non mesurée : contrôles de bornes conservés, les preuves Creusot ne les éliminent pas) et n'a pas de surcoût mémoire ; les écarts viennent surtout de `lines` (+30 %), `hashmap` (+15 %) et `sort` (+10 %). Les ports OCaml/F# à la main sont 1.5–2.6× plus lents que le C, avec un écart maximal sur `matmul` (cause probable, non vérifiée : gcc vectorise la boucle interne, ni OCaml ni .NET ici). Ce sont les *planchers* auxquels comparer les colonnes F* → OCaml / F# : on s'attend à ce que l'extraction F* soit plus lente encore (Stdint boxés, buffers génériques, voir « Constats »), mais ce n'est pas mesuré.

## Suite logique

1. Autoriser l'exécution de `fstar.exe` (ou lancer `scripts/setup-fstar.sh` puis `make fstar-verify`
   sur votre machine) pour faire typer les modules ; corriger les obligations de preuve restantes.
2. Installer KaRaMeL et mesurer `fstar-c` — c'est la colonne qui répond vraiment à la question de départ.
3. Installer Creusot, lancer `make creusot`, compléter les invariants manquants (forme du texte de `lines`,
   lemme de `matmul`) et noter le temps de preuve.
4. Mesurer `fstar-ocaml` avec et sans `FASTBUF=1` aux tailles `smoke`/`small` pour chiffrer le piège ci-dessus.
5. Décider si l'on ajoute de la correction fonctionnelle (tri = permutation triée) pour mesurer
   *le coût de preuve*, qui est le vrai prix de la programmation vérifiée.
