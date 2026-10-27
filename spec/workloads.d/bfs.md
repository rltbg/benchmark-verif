# bfs — graphe creux aléatoire, CSR par tri par comptage, BFS (accès irréguliers)

`n` sommets, `m = 16 n` arêtes dirigées. 1 ≤ n ≤ 100 000 000 (m ≤ 1,6·10⁹ tient dans un u32).

**Arêtes** (S = `0xB5F0000000000001`, arithmétique u64 modulo 2⁶⁴, `mix64` = splitmix64 de `spec/WORKLOADS.md`) : pour 0 ≤ j < m,
`u_j = mix64(S + 2j) mod n`, `v_j = mix64(S + 2j + 1) mod n` (modulo arithmétique, pas de masque).

**CSR** (tout en u32) :
1. `off[0..n]` initialisé à 0 ; pour j croissant, `off[u_j + 1] += 1`.
2. somme préfixe : pour i = 1..n, `off[i] += off[i-1]`.
3. `cur[0..n) = off[0..n)` ; pour j croissant, `adj[cur[u_j]] = v_j ; cur[u_j] += 1`.
4. `cur` est libéré avant l'allocation de `dist` et `q` (pic mémoire identique dans toutes les colonnes : `72 n + 4` octets).

**BFS** depuis le sommet 0 : `dist[0..n)` = 0xFFFFFFFF, `dist[0] = 0`, file `q[0..n)` en tableau (`q[0] = 0`, `head = 0`, `tail = 1`) ;
tant que `head < tail` : `u = q[head++]` ; pour e de `off[u]` à `off[u+1]-1`, `v = adj[e]` ; si `dist[v] = 0xFFFFFFFF` alors `dist[v] = dist[u]+1 ; q[tail++] = v`.
(Le résultat ne dépend pas de l'ordre des voisins : `dist` est canonique.)

**Checksum** : `ck = CK_INIT` ; pour i = 0..n-1 `ck = ckfold(ck, dist[i])` (u32 étendu en u64) ; puis `ck = ckfold(ck, tail)` (sommets atteints) ;
puis `ck = ckfold(ck, maxd)` où `maxd` = plus grand `dist[i]` différent de 0xFFFFFFFF.

**Mesure** : la génération des arêtes, la construction CSR et le BFS sont dans le temps mesuré. Unité : arêtes traitées (3 passes de m arêtes = 48 n).
**Preuves** : F* et Creusot prouvent mémoire-sûreté et absence de débordement ; les invariants « offsets monotones ≤ m, sommets < n, file ≤ n » ne sont
pas prouvés mais **gardés à l'exécution** (`e < m`, `v < n`, `tail < n`) : sur les entrées de ce workload les gardes sont toujours vraies.
