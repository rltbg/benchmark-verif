# `ptrchase` — graphe de pointeurs (latence mémoire)

Trois phases, chronométrées ensemble, sur une **arène par indices** (pas de vrais pointeurs de tas : un noyau F* qui manipule des
pointeurs de tas exige une preuve de séparation hors de portée ; une arène donne le même schéma d'accès — chaque saut est un chargement
dépendant à une adresse imprévisible — et le même code dans les 8 colonnes). Aucune vectorisation possible : le débit est borné par la
latence de la mémoire. 1 ≤ n ≤ 2^31 − 2 (les indices tiennent dans un u32, `NIL = 0xFFFFFFFF`).

Constantes : `SEED1 = 0xD00D000000000001`, `SEED2 = 0xD00D000000000002`, `SEED3 = 0xD00D000000000003`, `M = 0xFFFFFFFF`.

## Arène

`a` : tableau de `2n` u64. Le nœud `i` occupe `a[2i]` (clé) et `a[2i+1]` (liens : fils gauche dans les 32 bits de poids faible, fils
droit dans les 32 bits de poids fort ; `NIL = M` = pas de fils). 16 octets par nœud, contigus. `cnt` = nombre de nœuds alloués.

## Phase 1 — arbre binaire de recherche non équilibré

Pour `i = 0 … n−1` : `nk = mix64(SEED1 + i) | 1`.
* `i = 0` : le nœud 0 (racine) reçoit `nk`, liens `NIL/NIL`, `cnt = 1`, longueur de chemin 0.
* sinon : descente depuis la racine (`cur = 0`, `v = 0`) : à chaque itération `v += 1` ; si `nk == a[2·cur]` → doublon, ignoré, fin ;
  sinon `c` = fils gauche si `nk < a[2·cur]`, droit sinon ; si `c < cnt` : `cur = c` ; sinon (`c = NIL`) : le nouveau nœud `cnt` reçoit
  `(nk, NIL/NIL)`, le lien de `cur` est remplacé par `cnt`, `cnt += 1`, fin. La descente est bornée à `cnt` visites (garde de sûreté, jamais
  atteinte sur un arbre valide).
* `plen` = somme (u64, modulo 2^64) des `v` de toutes les insertions (`v` = nombre de nœuds visités, 0 pour la racine).
* `r1 = fold(fold(CK_INIT, cnt), plen)`.

## Phase 2 — recherches

* `n` succès : pour `k = 0 … n−1`, clé `mix64(SEED1 + (3k mod n)) | 1` ;
* puis `n/2` (division entière) échecs : pour `m = 0 … n/2 − 1`, clé `mix64(SEED3 + m) & ~1` (clé paire : jamais présente, car toutes les clés insérées sont impaires).
* Recherche : `cur = 0`, `v = 0` ; tant que `v < cnt` : `v += 1` ; si `clé == a[2·cur]` : trouvée, fin ; sinon `c` = fils gauche/droit comme ci-dessus ;
  si `c < cnt` : `cur = c`, sinon fin (absente). Valeur : `val = (v << 1) | trouvée`.
* `r2 = fold` (depuis `CK_INIT`) des `val`, dans l'ordre (succès puis échecs).

## Phase 3 — chasse de pointeurs

`next` : tableau de `n` u32. `next[i] = i`, puis permutation de **Sattolo** (cycle unique de longueur `n`) : pour `i = n−1` descendant à `1` :
`j = mix64(SEED2 + i) mod i` (u64, `j < i`), échange `next[i] ↔ next[j]`. Puis `cur = 0`, `sum = 0` ; `4n` fois : `cur = next[cur]` ; `sum += cur` (u64, modulo 2^64).
`r3 = fold(fold(CK_INIT, cur), sum)`.

## Checksum

`ck = fold(fold(fold(CK_INIT, r1), r2), r3)`.

Mémoire idéale : `16·n + 4·n = 20·n` octets (arène + `next`). Travail : `work_items = 2·n + n/2 + 4·n + n = 7.5·n` accès dépendants (`n` insertions, `3n/2` recherches, `4n` sauts, `n` pour Sattolo).

## Preuve (F* Pulse, Creusot)

Mémoire-sûreté, bornes (`cur < cnt ≤ n`, `2·cur+1 < 2n`, indices Sattolo `j < i < n`), absence de débordement, terminaison (descentes bornées par `cnt`).
Les liens lus dans l'arène sont **gardés** par `c < cnt` (un lien invalide serait traité comme `NIL`) et les sauts de la chasse par `v < n` : sur une
exécution correcte ces gardes ne changent rien, elles évitent seulement d'avoir à prouver un invariant quantifié sur tout le tableau. Les colonnes natives
(C, Zig, Rust) n'ont pas ces gardes sur la chasse de pointeurs.
