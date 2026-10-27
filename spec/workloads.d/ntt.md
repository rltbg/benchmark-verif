# Workload `ntt` — transformée en nombres entiers (analogue de la FFT)

Produit cyclique de longueur `n` par NTT : accès « papillon » à stride variable (de 1 à n/2), table de racines (*twiddles*) lue avec stride,
permutation bit-reverse (accès aléatoires). **Entiers seulement** : F* n'extrait pas proprement les flottants (backends C/F#/OCaml de Custard),
donc l'analogue flottant de la FFT n'est pas faisable dans les huit colonnes ; la NTT a exactement le même motif d'accès mémoire.

* `n = 2^k`, `0 <= k <= 27` ; si `n` n'est pas une puissance de 2 ou `n > 2^27`, le résultat est `0` (rejet). Premier choisi : `p = 2013265921 = 15·2^27 + 1`,
  racine primitive `g = 31` (le premier classique 998244353 ne supporte que `n <= 2^23`, trop petit pour la charge visée). Tous les produits sont des `u64` (`a,b < 2^32`), sans débordement.
* `a[i] = mix64(0xA5A5F00D12345678 + i) mod p` (u32), `i < n`.
* `w = g^((p-1)/n) mod p` ; `tw[j] = w^j mod p`, `j < n/2` (u32), calculés par multiplication itérative `tw[j+1] = tw[j]·w mod p`.
* `NTT(a)` : (1) pour chaque `i`, `rev` = `i` inversé sur `k` bits (par la boucle `rev = 2·rev + x mod 2 ; x = x / 2`, `k` fois) ; si `i < rev`, échanger `a[i]` et `a[rev]` ;
  (2) pour `len = 2, 4, …, n` : `half = len/2`, `stride = n/len` ; pour `blk = 0 .. stride-1`, `start = blk·len` ; pour `j = 0 .. half-1` :
  `u = a[start+j]`, `v = a[start+j+half]·tw[j·stride] mod p`, `a[start+j] = (u+v) mod p`, `a[start+j+half] = (u+p-v) mod p`.
* Calcul : `NTT(a)` ; `a[i] = a[i]² mod p` ; `NTT(a)` ; renverser `a[1..n-1]` (échanger `a[i]` et `a[n-i]` pour `1 <= i`, `2i < n`) ;
  `ninv = n^(p-2) mod p` ; résultat `r[i] = a[i]·ninv mod p`. Le résultat est le produit cyclique `r[i] = Σ_j a0[j]·a0[(i-j) mod n] mod p` (vérifié contre la convolution naïve indépendante, `n <= 4096`).
* Checksum : `ck = ck_init` ; pour `i = 0..n-1` : `ck = ckfold(ck, r[i])`.
* Mémoire de la spécification : `a` (4n octets) + `tw` (2n octets) = **6n**. Unité de débit : papillons (`n·log2 n` : deux transformées de `(n/2)·log2 n`).

Tailles : smoke 2^12, tiny 2^16, small 2^20, medium 2^23, large 2^25, xlarge 2^26, saturate 2^27 (512 Mo + 256 Mo ; ~67 s en C natif, dominé par les accès à stride de grande amplitude et par le bit-reverse).
Checksums de référence : `spec/checksums.d/ntt.json`.
