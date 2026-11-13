# Creusot contre Verus : rapport de comparaison

Deux choses ont été faites sur les 15 workloads du benchmark :

1. le crate Creusot (`impls/rust`) a été **réécrit selon la documentation de Creusot** : spécifications fonctionnelles nommées, preuves qui
   remplacent les gardes à l'exécution, terminaison, aucune hypothèse non prouvée hors de ce que l'outil ne sait pas faire ;
2. le **même** code a été écrit en **Verus** (`impls/rust-verus`), avec la même cible de spécification et le même comportement.

Consigne commune : [`BRIEF.md`](BRIEF.md). Détail par workload : `docs/verif-compare/<nom>.md`. Journal de chaque appel au vérificateur :
`docs/verif-compare/runs/<nom>-<outil>.csv`. Expérience « modèles d'IA plus faibles » : [`ai-experiment/RESULTATS.md`](ai-experiment/RESULTATS.md).
Versions : Creusot (dépôt au 2026-10-24, why3find, Alt-Ergo / Z3 / CVC4 / CVC5), Verus 0.2026.11.02 (Z3, toolchain Rust 1.99.1).

## 1. Ce qui est prouvé (identique dans les deux outils)

| workload | propriété prouvée | raccourcis restants |
|---|---|---|
| sort | trié ∧ permutation | aucun |
| strsort | indices triés pour l'ordre (chaîne, indice) ∧ permutation de 0..n (= tri stable) | aucun |
| parsort | trié ∧ permutation, **threads compris** | Creusot : contrats de threads de `creusot-std` (eux-mêmes admis) ; Verus : 2 `external_body` (threads « scoped » sur `&mut` disjoints, que `vstd::thread` ne sait pas exprimer) |
| sieve | `f[k] == 0 ⇔ k premier`, et le checksum complet | aucun |
| hashmap | modèle dictionnaire (insertion, recherche = valeur du modèle ou 0) | aucun |
| matmul | `c = A·B mod 2³²` — **la preuve qui avait échoué au début du projet** | aucun |
| parmatmul | idem, **threads compris** | contrats de threads des bibliothèques |
| ntt | valeurs < p, bit-reverse involution, `tw[j] = wʲ mod p` (pas l'égalité avec la DFT) | aucun |
| lines | agrégat par clé = somme des valeurs ; lemme « lecture décimale ∘ écriture décimale = identité » | aucun |
| bfs | caractérisation complète des distances BFS | aucun |
| ptrchase | arbre de recherche ; une recherche trouve ⇔ clé insérée ; la chasse suit `next` (pas : cycle unique de Sattolo) | aucun |
| alloc | contenu et somme de chaque bloc ; contenu de la fenêtre FIFO | Verus : 2 `external_body` (`black_box` n'a pas de spécification dans `vstd`) |
| sha256, parsha | condensé = SHA-256 logique écrit d'après FIPS 180-4 ; `parsha` threads compris | contrats de threads des bibliothèques |
| stream | CRC-32 = CRC logique bit à bit ; compteurs = automate logique | lecture du fichier admise dans les deux (`#[trusted]` / `external_body`) |

Terminaison : prouvée partout en Verus ; en Creusot, les fonctions qui allouent (`vec!`) ou lancent des threads ne peuvent pas être marquées
`#[check(terminates)]`, car la bibliothèque ne déclare pas ces primitives terminantes.

**Les deux outils ont atteint exactement les mêmes cibles.** Aucun workload n'a été possible dans l'un et pas dans l'autre. Les différences portent sur le coût.

## 2. Performance du code compilé (taille `large`, médiane de 3, épinglé sur un cœur)

| | C natif | Rust simple | Rust + Creusot | Rust + Verus |
|---|--:|--:|--:|--:|
| temps relatif à Rust simple (moyenne géométrique, 15 workloads) | ×0,97 | ×1,00 | **×0,98** | **×1,02** |
| pic de RAM, pire cas relatif à Rust simple | ×1,00 | ×1,00 | ×1,05 | ×1,06 |
| exécutable | 20 Ko | 395 Ko | 470 Ko | 461 Ko |

* **Avant la réécriture, Rust + Creusot était ×1,24 plus lent que Rust simple** (taille `saturate`). Ce surcoût venait des gardes à l'exécution que l'ancienne
  version gardait faute de preuve, et des formes de code imposées par des spécifications plus faibles. Prouver davantage a rendu le code **plus rapide**.
* Verus est au niveau de Rust simple sauf sur `sha256`/`parsha` (×1,12–1,13). Environ la moitié vient de la toolchain imposée par Verus : Rust simple compilé
  avec 1.99.1 au lieu de 1.97 passe de 9,77 s à 10,29 s. Le reste (10,29 s → 11,65 s) n'est pas expliqué.
* Les contrats et le code fantôme sont effacés dans les deux outils : la différence de performance vient uniquement du code exécutable écrit.

## 3. Coût de la preuve

| | Creusot | Verus |
|---|--:|--:|
| vérification du crate complet à froid | **~110 s** (252 fichiers, portefeuille de 4 solveurs) | **13,8 s** (378 obligations, Z3, 3 threads) ; 49 s avec la vérification de compilation |
| temps d'un essai pendant le développement | 20 à 70 s (recompilation + why3find) | 1 à 5 s |
| lignes non vides des 15 modules (code + spec + preuve) | **4 232** (avant réécriture : 2 007) | **6 064** (+43 %) |
| appels au vérificateur pendant le développement (journaux) | 238, dont 172 non réussis | 132, dont 80 non réussis |
| stabilité | fragile sous charge : des buts passant seuls échouent par délai dans le crate complet ; échecs de calibrage de why3find | stable ; un seul piège : `rlimit` dépassé par une boucle de déclencheurs |

**Biais à garder en tête** : pour chaque workload, la version Creusot a été écrite en premier, et la version Verus est une **traduction d'une preuve déjà
conçue**. Les appels Verus sont donc sous-estimés par rapport à une conception de zéro. L'expérience contrôlée de la section 5 n'a pas ce biais.

## 4. Expérience de développement, critère par critère

| critère | meilleur | pourquoi (observé dans les journaux et les rapports) |
|---|---|---|
| vitesse de la boucle essai → erreur | **Verus** | 1–5 s contre 20–70 s ; c'est l'écart le plus grand et le plus constant |
| localisation des erreurs | **Verus** | il désigne la clause ou l'expression exacte ; Creusot donne « fonction ✘ (31/32) » et il faut relancer `why3find -x/-X` puis lire la tâche Why3 |
| automatisation des preuves | **Creusot** | portefeuille de solveurs ; gros lemmes à quantificateurs prouvés sans corps ; `permutation_of` gratuit après `swap` ; les préconditions restent visibles dans les boucles |
| concision | **Creusot** | 30 % de lignes en moins pour la même cible (pas d'isolation des boucles à compenser, pas de déclencheurs à écrire) |
| arithmétique | **Verus** | `vstd::arithmetic`, `by (nonlinear_arith)`, `by (bit_vector)` : `powmod` immédiat contre 8 essais en Creusot ; en Pearlite pas de conversion `Int → u64`, `%` tronqué, opérations sur bits opaques hors `#[bitwise_proof]` (non documenté) |
| quantificateurs et déclencheurs | partagé | Verus exige des déclencheurs explicites, mais les explique (`--expand-errors`) ; Creusot les choisit seul, mais un échec de déclencheur est presque impossible à diagnostiquer (`cf[..x]` et `cf[0..x]` sont deux termes différents) |
| threads | **Creusot** (légèrement) | `thread::scope` vérifié avec des `&mut` disjoints ; Verus n'a que `spawn` avec fermetures `'static` (il faut `Arc`, ou admettre le code « scoped ») |
| pointeurs bruts et code `unsafe` | non testé | les deux ont des permissions fantômes (`Perm` en Creusot, `PPtr` + `PointsTo` en Verus) ; Verus a plus d'exemples (`examples/doubly_linked.rs`) |
| terminaison | **Verus** | prouvée partout ; en Creusot, `vec!` et `spawn` ne sont pas déclarés terminants |
| stabilité de la toolchain | **Creusot** | Verus impose sa propre version de Rust (1.99.1) et a changé la syntaxe des `&mut` en 2026 (`final(v)`) ; Creusot suit une nightly fixe et la compilation finale se fait avec le Rust stable du projet |
| intégration au projet | partagé | Creusot : `cargo build` normal, contrats en attributs ; Verus : code dans `verus! { }`, `cargo verus build` |

## 5. Quel outil une IA, même plus faible, prend-elle mieux en main ?

Expérience contrôlée (même énoncé, agents neufs, solutions re-vérifiées par moi, détail dans `ai-experiment/RESULTATS.md`) :

| | Haiku | Sonnet | Opus |
|---|---|---|---|
| Creusot | 4/4 en **5** appels (383 s) | 4/4 en **4** appels (296 s) | 4/4 en **4** appels (332 s) |
| Verus | 4/4 en **18** appels (293 s) | 4/4 en **5** appels (61 s) | 4/4 en **5** appels (74 s) |

* **Creusot pardonne davantage** : un modèle faible y réussit avec presque aucun essai raté. En Verus, Haiku a buté sur l'isolation des boucles, les déclencheurs,
  les conversions `usize → int` et la preuve de permutation à écrire soi-même, et il a quand même réussi grâce à des essais très rapides.
* **Verus va plus vite en temps réel** pour un bon modèle (Sonnet, Opus : 4 à 5 fois plus rapide), parce qu'un essai coûte 1,5 s au lieu d'une minute.
* Sur les gros workloads (section 3), le constat se renverse : là, c'est le diagnostic qui coûte, et Creusot y est nettement plus pénible
  (but en échec introuvable sans outils annexes, fragilité sous charge).
* Verus publie un chapitre officiel pour les agents d'IA (`llmforverusproof.md`, avec un « cheat checker ») ; Creusot n'a rien d'équivalent.
* Risque propre à Verus : la syntaxe évolue vite (références mutables réécrites en 2026), donc un modèle entraîné avant ces changements se trompe ; les messages d'erreur indiquent la correction.

## 6. Avantages et inconvénients

**Creusot**
* \+ Preuves plus automatiques et spécifications plus courtes (portefeuille Alt-Ergo / Z3 / CVC5, théories Why3 comme les permutations).
* \+ Les contrats sont des attributs sur du Rust ordinaire ; le binaire final est compilé par le Rust du projet.
* \+ Threads « scoped » vérifiables ; meilleur score pour une IA faible sur de petits exercices.
* − Boucle lente (20–70 s) et diagnostics pauvres ; fragilité sous charge (délais de 1 s des solveurs, calibrage de why3find).
* − Pearlite limité en arithmétique (`Int → u64`, `%`, bits) et fonctionnalités non documentées (`#[bitwise_proof]`).
* − Terminaison impossible à prouver pour ce qui alloue ou lance un thread.

**Verus**
* \+ Boucle très rapide (secondes) et messages précis ; vérification complète en 14 s.
* \+ Arithmétique non linéaire et vecteurs de bits bien outillés ; terminaison prouvée partout.
* \+ Documentation pour les agents d'IA ; communauté et exemples système (pointeurs, concurrence).
* − Plus verbeux (+43 %) : isolation des boucles (répéter les hypothèses), déclencheurs explicites, permutations à prouver soi-même.
* − Toolchain Rust imposée et syntaxe encore mouvante ; `vstd::thread` limité aux fermetures `'static`.
* − Quelques primitives sans spécification (`black_box`) qui obligent à des `external_body`.

## 7. Lequel privilégier

* **Pour ce projet (algorithmes sur tableaux, specs fonctionnelles, Rust sûr)** : les deux atteignent les mêmes cibles avec les mêmes performances.
  **Verus** est le choix le plus productif dès que les preuves dépassent quelques dizaines de lignes. Ce qui le départage, c'est la boucle de 2 s au lieu d'une minute
  et des erreurs qui disent où chercher. Ce sont justement les deux choses qui ont coûté le plus cher en Creusot sur les gros workloads.
* **Creusot** reste préférable pour du code court où l'automatisation fait tout le travail (tri, recherche, permutations), si l'on veut garder la toolchain Rust du projet,
  ou si les preuves sont confiées à un modèle d'IA faible sur de petits morceaux.
* Ces conclusions viennent d'un seul jeu de 15 workloads, avec un biais d'ordre en faveur de Verus et une expérience IA d'une exécution par case.
