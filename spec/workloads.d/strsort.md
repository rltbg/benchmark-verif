# strsort — tri fusion stable de chaînes de longueur variable (comparaisons indirectes)

`strsort <n>` : n chaînes dans un pool d'octets contigu ; on trie un tableau d'**indices** (u32) selon la comparaison lexicographique
des chaînes pointées. Stresse les accès indirects (indice → offset → octets), le cache et les branches imprévisibles
(alphabet de 4 lettres → longs préfixes communs). 1 <= n <= 2^28.

## Génération (identique dans les 8 colonnes)
* `mix64` et `ckfold` : voir `spec/WORKLOADS.md`.
* Longueur de la chaîne i (0 <= i < n) : `L_i = 1 + (mix64(0xC0FFEE0000000001 + i) >> 59)`, soit 1..32.
* Offsets (u64, n+1 entrées) : `off[0] = 0`, `off[i+1] = off[i] + L_i` ; `total = off[n]` (≈ 16,5 n octets).
* Pool (u8, `total` octets) : octet k de la chaîne i = `'a' + (mix64(0xC0FFEE0000000002 + 64·i + k) & 3)`, à la position `off[i] + k`
  (arithmétique modulo 2^64 sur les graines).
* Indices : `a[i] = i` (u32).

## Tri
Tri fusion **descendant** (récursif, scission au milieu `mid = lo + (hi-lo)/2`, plages de taille < 2 inchangées) du tableau `a` avec un tampon `tmp`
de même taille : fusion de `a[lo,mid)` et `a[mid,hi)` dans `tmp[lo,hi)`, puis recopie de `tmp[lo,hi)` dans `a[lo,hi)`.
Dans la fusion on prend l'élément de gauche si `i < mid` et (`j >= hi` ou `le(a[i], a[j])`) — donc **tri stable** (à chaînes égales l'ordre des indices est conservé).

`le(x, y)` : soit `lx`, `ly` les longueurs ; `m = min(lx, ly)` ; pour k de 0 à m−1 : si `octet(x,k) != octet(y,k)` renvoyer `octet(x,k) < octet(y,k)` ;
renvoyer `lx <= ly`. (Comparaison octet par octet, pas de `memcmp`.)

## Checksum
`ck = CK_INIT` ; pour chaque indice `a[i]` dans l'ordre trié : `ck = ckfold(ck, a[i] as u64)` ; puis `ck = ckfold(ck, total)`.

## Remarques par colonne
* **F\***: noyau Pulse `Bench.Strsort.run`. Prouvé : mémoire-sûreté (en particulier `offset + k < taille du pool` dans la comparaison), absence de débordement non voulu,
  terminaison ; **pas** de correction fonctionnelle du tri. Des gardes runtime (`ia < n`, `offset + longueur <= taille du pool`) remplacent des invariants de forme entre
  tableaux ; elles ne sont pas dans les colonnes écrites à la main.
* **Rust + Creusot** : mêmes gardes, mêmes preuves (absence de panique, terminaison), offsets en `usize` (64 bits : mêmes valeurs).
* Mémoire idéale : pool + offsets (8(n+1)) + indices (4n) + tampon (4n) ≈ 33 n octets.
* Tailles : smoke 1e4 · tiny 2e4 · small 3e5 · medium 2e6 · large 8e6 · xlarge 5e7 · saturate 1,2e8 (≈ 4 Go, quelques minutes en C natif).
  Le F* → OCaml est ~1000× plus lent que le C : seules les petites tailles terminent.
