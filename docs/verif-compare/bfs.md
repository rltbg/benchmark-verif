# `bfs` : Creusot contre Verus

Fichiers : `impls/rust/src/bfs.rs` (Creusot) et `impls/rust-verus/src/bfs.rs` (Verus).
Journaux : `runs/bfs-creusot.csv`, `runs/bfs-verus.csv`.

**Biais d'ordre.** La partie principale (CSR et BFS sur le CSR) a été écrite d'abord en Creusot.
L'extension « CSR = arêtes générées » a été écrite d'abord en Verus, puis portée en Creusot.

## Cible et résultat (identiques pour les deux outils)

La cible du BRIEF est **atteinte**, et étendue aux arêtes générées.

### Modèle

* `src(j) = mix64(SEED + 2j) mod n` et `dst(j) = mix64(SEED + 2j + 1) mod n`.
  * En Creusot, ce sont des suites fantômes `us`/`vs`, caractérisées par
    `forall<j: u64> … == endp_l(j, k, n)`.
  * En Verus, ce sont des fonctions `spec` opaques (`endp_s`).
* `cnt(u, j)` : nombre d'arêtes de source `u` parmi les `j` premières.
* `psum(i)` : somme des `cnt(w, m)` pour `w < i`.
* `arc(off, adj, u, v)` : il existe `e` dans `[off[u], off[u+1])` avec `adj[e] = v`.
* `reached(d, k)` : nombre de sommets atteints parmi les `k` premiers.

### Propriétés prouvées

* **CSR.**
  * `count_deg` : `off[u+1] = cnt(u, m)`.
  * `prefix` : `off[i] = psum(i) <= m`, grâce au lemme `psum(n) = m`.
  * `fill` : `cur[u] = psum(u) + cnt(u, j)`, donc l'écriture tombe dans `[off[u], off[u+1])`.
    Toutes les cibles sont `< n`.
* **CSR = arêtes générées.**
  * Chaque arête `j` est rangée dans la plage de sa source. Chaque case de la plage de `u` contient
    la cible d'une arête de source `u`.
  * En Verus, ce sont les prédicats existentiels `placed` et `slot_ok`.
  * En Creusot, ce sont des témoins fantômes rendus par `fill` : `pos[j]` (case de l'arête `j`) et
    `who[e]` (arête de la case `e`).
* **BFS.**
  * `dist[0] = 0`.
  * Tout sommet atteint `v ≠ 0` a un prédécesseur atteint `u`, avec un arc `u→v` et
    `dist[u] + 1 = dist[v]`.
  * Pour tout arc `u→v` avec `u` atteint, `v` est atteint et `dist[v] <= dist[u] + 1`.
  * Le résultat (`tail`) est `reached(dist, n)`.
  * Ces trois propriétés caractérisent les distances BFS : récurrence immédiate sur la distance.
* **Report sur les arêtes générées.** Les mêmes propriétés sont prouvées en remplaçant « arc du
  CSR » par « arête générée `src(j) → dst(j)` ».
  * En Verus : lemme `lemma_bfs_on_edges`, appelé dans `run`.
  * En Creusot : fonctions fantômes `bfs_edges_forward` et `bfs_edges_backward`
    (`#[check(ghost)]`), appelées dans des blocs `ghost!` de `run`.
* **Absence de panique et de débordement.** `dist[u] + 1`, `cur[u] + 1`, `off[i] + off[i-1]` et
  `SEED + 2j + k` sont prouvés sans débordement.
* **Terminaison** prouvée partout, sauf `run` en Creusot (`vec!` n'est pas `terminates`).

### Gardes supprimées

Elles figuraient dans l'ancien Creusot :

* dans `relax` : `e < m` (avec `e0 <= m`), `w < n` et `tail < n` ;
* dans `bfs` : `usafe = if us < n { us } else { 0 }` ;
* dans `fill` : `if p < m`.

Toutes sont remplacées par des invariants prouvés :

* **Bornes du CSR** : `off[i] <= m` et `adj[e] < n`.
* **File** : `q[h] < n`.
* **`tail < n` au moment d'enfiler** : `tail = reached(dist, n)`, et `reached(dist, n) < n` dès
  qu'un sommet reste à `INF` (lemme `reached_bound`). C'est le principe des tiroirs, sans ensembles.

L'ancien `relax` (une fonction séparée) est réintégré dans `bfs` comme dans `rust-plain`.

## Chiffres

| | Creusot | Verus |
|---|---|---|
| lignes exécutables / spécification / preuve (*) | 122 / 100 / 261 (ancien : 114 / 16 / 14) | 112 / 127 / 457 |
| vérification finale à froid | **57 s** sur une copie sans `proof.json` ni cache (charge ≈ 4). 15 à 30 s avec `--no-cache` en rejouant les `proof.json` | **2,1 s** pour le crate entier `common` + `lines` + `bfs` (65 fonctions, dont 0,8 s de vérification) |
| appels, première version (CSR et BFS sur le CSR) | 6 : 5 échecs (dont 2 relances `-X`), 1 ok | 3 : 2 échecs, 1 ok |
| appels, extension CSR = arêtes générées | 23 : 2 erreurs de compilation, 18 échecs (dont 3 relances `-X`), 3 ok | 5 : 1 erreur de compilation, 2 échecs (`rlimit` instable), 2 ok |
| appels, stabilité et mesures | 6 : 2 échecs à froid (buts instables), 4 ok | 2 builds release ok, 3 appels sans effet (option `focus`), 1 mesure ok |
| `trusted` / `assume` / `external_body` | aucun | aucun |
| checksum | `bfs:20000`, `200000` et `1000000` identiques à `spec/checksums.d/bfs.json` | idem |
| temps d'exécution (n = 10⁶, meilleur de 5) | 542 ms (plain 576 ms, ancien Creusot 557 ms) | 497 ms (machine partagée, mesures bruitées) |

(*) Même méthode que `lines.md` (comptage par script, sections « Modèle » / « Lemmes » / code). Le
style `verusfmt` gonfle les chiffres Verus.

## Ce qui a coûté le plus

### Creusot

1. **Un appel de lemme n'instancie rien.**
   * `proof_assert!{ lemme(x); P }` est traduit en `let _ = lemme x in P`. La postcondition du lemme
     est un axiome global `forall args. pre -> post`, sans déclencheur.
   * Le lemme n'aide donc que si les prouveurs trouvent eux-mêmes l'instance. Avec
     `proof_assert!{ lemme(x); true }`, l'information disparaît complètement (vu avec `-X` : aucune
     hypothèse issue du lemme).
   * Méthode qui marche : écrire la conclusion attendue après l'appel
     (`{ reached_set(...); reached(d', n) == tail + 1 }`), ou énoncer le lemme avec un `forall` dans
     son `ensures` et l'appeler hors quantificateur.
2. **Les existentiels sous quantificateur** (« chaque case contient une arête j telle que… »).
   * Leur transfert d'un état à l'autre a échoué 9 fois de suite, avec des lemmes « keep » unitaires
     puis quantifiés.
   * La solution est l'idiome des **témoins fantômes** : `pos: Seq<Int>` et `who: Seq<Int>`, mis à
     jour à chaque écriture. Invariants et postconditions deviennent alors sans existentiel.
   * Même le passage `slot_ok(x, e) ⇒ ∃ j. …`, qui n'est que le dépliage de la définition, a échoué
     dans un grand contexte.
3. **Pas de `Int → u64` dans la logique.** Comme pour `lines`, il faut des suites fantômes
   `us`/`vs` et quantifier sur `j: u64`, puis affirmer `forall<x: u64> x@ == j@ ==> x == j as u64`
   pour raccorder les indices.
4. **Instabilité à froid.**
   * Avec le budget de 1 s par prouveur, sur une machine chargée (≈ 8 sur 4 cœurs), une vérification
     sans cache a laissé 3 buts en échec, puis 1 au passage suivant. La version avec cache passait.
   * Mettre les postconditions de `fill` sous forme de témoins a rendu la preuve stable : trois
     passages à froid sont ok.
5. **Un bogue de Creusot.** Une variable locale nommée `so`, un mot-clé de Why3, produit un `.coma`
   invalide (« syntax error »). Il suffit de la renommer.

### Verus

1. **Le `rlimit` de la BFS.**
   * La monotonie de la file par paires (`forall h1 <= h2. dist[q[h1]] <= dist[q[h2]]`) est un
     multi-déclencheur qui a fait exploser la boucle.
   * Remplacement par une **file par niveaux**, avec deux variables fantômes `lvl` et `s` :
     `q[..head] <= lvl`, `q[head..s] = lvl`, `q[s..tail] = lvl + 1`. Les quantificateurs n'ont plus
     qu'une variable.
2. **L'instabilité.**
   * L'ajout de spécifications sans rapport (`dst`, `placed`) dans le module a fait repasser une
     boucle au-dessus du `rlimit`.
   * Avec `#[verifier::rlimit(30)]`, c'est l'autre boucle qui échouait : l'attribut a été retiré.
   * Solution robuste : trois prédicats opaques (`queued`, `preds`, `processed`) et trois lemmes de
     pas (`lemma_discover`, `lemma_processed_next`, `lemma_bfs_done`). Résultat : 1,2 s, stable.
3. **Les existentiels se manipulent bien.** `choose|h| …` nomme un témoin, et
   `assert forall … implies … by { … }` structure la preuve. `fill`, avec `placed` et `slot_ok`, a
   été vérifié **du premier coup** ; `lemma_bfs_on_edges` aussi.

## Qualité des messages d'erreur

* **Verus.**
  * La clause en échec est indiquée, avec « at the end of the loop body », « before loop » ou
    « at this loop exit ».
  * Le `rlimit` est attribué à une boucle précise.
  * `--expand-errors` affiche la conjonction ✔/✘.
  * Limite : un `rlimit` dépassé ne dit pas quel quantificateur s'emballe ; il faudrait `--profile`.
* **Creusot.**
  * Il faut `-x` pour avoir la ligne et `-X` pour voir la tâche.
  * La tâche montre bien ce qui manque : par exemple l'absence de l'hypothèse du lemme, ou la
    conclusion `in_bounds(...)` d'un débordement.
  * Un échec par délai (instabilité sous charge) ne se distingue pas d'un échec logique.

## Ce qu'un modèle plus faible aurait probablement raté

* **Les deux.**
  * Le principe des tiroirs pour `tail < n` (compteur `reached` et lemme `reached_bound`), au lieu
    de garder la garde.
  * L'invariant de file par niveaux.
  * La décomposition `cur[u] = psum(u) + cnt(u, j)` et le lemme `psum(n, j) = j`, avec son cas de
    base `psum(i, 0) = 0` qu'il ne faut pas oublier.
* **Creusot.**
  * Le fait qu'un appel de lemme n'instancie rien.
  * Le passage aux témoins fantômes pour se débarrasser des existentiels.
  * Le bogue `so`.
* **Verus.**
  * L'usage de `#[verifier::opaque]` et de lemmes de pas, au lieu de monter le `rlimit`, qui se
    révèle instable.
