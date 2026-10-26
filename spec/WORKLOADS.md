# Spécification commune des workloads

Toutes les implémentations (C, Rust, OCaml, F#, F*→C/OCaml/F#) calculent **exactement la même
chose** et doivent afficher le **même checksum**. Le harnais refuse (❌) un résultat dont le
checksum diverge de celui des autres implémentations.

Interface : `bench <workload> <n>` affiche une ligne

```
RESULT workload=<w> n=<n> checksum=<16 hex> time_ns=<durée interne en ns>
```

La durée interne est mesurée autour du calcul seul (hors démarrage du runtime). Le harnais mesure en
plus, de l'extérieur, durée totale, CPU user/sys, pic RSS, fautes de page, blocs disque.

## Primitives (arithmétique non signée, modulo 2^k)

```
mix64(x)   z = x + 0x9E3779B97F4A7C15
           z = (z ^ (z >> 30)) * 0xBF58476D1CE4E5B9
           z = (z ^ (z >> 27)) * 0x94D049BB133111EB
           return z ^ (z >> 31)                                  (u64)
fold(ck,v) (ck ^ v) * 0x100000001B3                              (u64)
CK_INIT    0xCBF29CE484222325
```

Les entrées sont des fonctions pures de l'indice (`mix64(SEED + i)`) : pas de PRNG à état, donc
aucun coût de génération dépendant de l'implémentation du générateur.

## 1. `sort` — tri fusion (CPU + RAM)

* `a[i] = (u32)(mix64(0x0123456789ABCDEF + i) >> 32)` pour `i ∈ [0,n)`.
* Tri fusion **ascendant par le bas** (largeur w = 1, 2, 4, … ; fusion stable `gauche <= droite`)
  dans un unique buffer de **2n u32** : `[0,n)` et `[n,2n)` alternent source/destination.
* Checksum : `fold` de chaque élément trié dans l'ordre ; **0** si l'ordre n'est pas croissant.
* Mémoire idéale : 8·n octets. Contraintes : 1 ≤ n ≤ 2^30.

## 2. `sieve` — crible d'Ératosthène (bande passante mémoire)

* `flags[0..=n]` d'octets, initialisés à 0 ; pour `i = 2; i*i <= n; i++` : si `flags[i]==0`,
  marquer `flags[j]=1` pour `j = i*i; j <= n; j += i`.
* Checksum : pour chaque premier `p ∈ [2,n]` (dans l'ordre) `ck = fold(ck, p)`, puis
  `fold(ck, nombre_de_premiers)`.
* Mémoire idéale : n+1 octets. Contraintes : 2 ≤ n ≤ 4 000 000 000.

## 3. `matmul` — produit matriciel dense (CPU, caches)

* `A[i] = (u32)(mix64(S + i) >> 32)`, `B[i] = (u32)(mix64(S + n² + i) >> 32)`, `S = 0x0FEDCBA987654321`,
  `i ∈ [0,n²)`, stockage ligne par ligne.
* `C = A·B` en **u32 modulo 2^32**, boucles `i, k, j` : `C[i·n+j] += A[i·n+k] · B[k·n+j]`.
* Checksum : `fold` des `n²` éléments de C dans l'ordre.
* Mémoire idéale : 12·n² octets. Contraintes : 1 ≤ n ≤ 4096.

## 4. `lines` — débit de traitement de lignes (CPU + RAM, « lignes par seconde »)

* Génération de n lignes ASCII `clé;valeur\n` dans un buffer d'au plus 15·n octets,
  avec `r = mix64(0x1357924680ACEBDF + i)`, `clé = r >> 44` (20 bits), `valeur = ((r >> 8) & 0xFFFFFFFF) mod 1 000 000`,
  nombres en décimal sans zéros de tête.
* Puis relecture octet par octet de tout le buffer : pour chaque ligne, `tab[2·clé] += valeur`,
  `tab[2·clé+1] += 1` (`tab` : 2^21 u64, 16 Mo, accès aléatoires).
* Checksum : pour `k ∈ [0, 2^20)` : `ck = fold(fold(ck, tab[2k]), tab[2k+1])`.
* Débit rapporté : n / durée totale (génération + parsing). Mémoire idéale : 15·n + 16 Mo.
  Contraintes : 1 ≤ n ≤ 268 435 455.

## 5. `hashmap` — table de hachage à adressage ouvert (latence mémoire)

* `cap` = plus petite puissance de 2 ≥ 2n (donc taux de remplissage ≤ 50 %), `lg = log2(cap)`.
  Table : `cap` couples `(clé, valeur)` u64 **entrelacés** (`t[2s]`, `t[2s+1]`), clé 0 = case vide.
* `key(off,i) = mix64(0x2468ACE013579BDF + off + i) | 1`.
  Position initiale : `s = (key · 0x9E3779B97F4A7C15 mod 2^64) >> (64 − lg)`, sondage linéaire `s = (s+1) & (cap−1)`.
* Phase 1 : pour `i ∈ [0,n)` : insérer `key(0,i)` ; si présent `valeur += i+1`, sinon `valeur = i+1`.
* Phase 2 : pour `i ∈ [0,n)` : chercher `key(0,i)` (succès) ; `ck = fold(ck, valeur)`.
* Phase 3 : pour `i ∈ [0,n)` : chercher `key(n,i)` (quasi toujours un échec) ; `ck = fold(ck, valeur ou 0)`.
* Débit : 3n opérations / durée. Mémoire idéale : 16·cap octets. Contraintes : 1 ≤ n ≤ 2^26.

## Pourquoi ces cinq-là

| workload | ce qu'il met en difficulté |
|---|---|
| sort | calcul + écriture séquentielle de deux flux, branches imprévisibles, ~2n d'empreinte |
| sieve | bande passante mémoire pure, gros tableau d'octets (n jusqu'à plusieurs Go) |
| matmul | débit arithmétique, boucles imbriquées, vectorisation, preuves d'indices non linéaires |
| lines | traitement d'un flux d'octets, analyse de texte, accès aléatoires dans une table de 16 Mo |
| hashmap | latence mémoire (défauts de cache à chaque sonde), table de centaines de Mo |

## Tailles (voir `harness/presets.json`)

| preset | sort | sieve | matmul | lines | hashmap |
|---|--:|--:|--:|--:|--:|
| smoke | 10 k | 1 M | 64 | 10 k | 10 k |
| small | 1 M | 20 M | 512 | 1 M | 500 k |
| medium | 8 M | 200 M | 1600 | 10 M | 4 M |
| large | 32 M | 1 G | 2500 | 40 M | 16 M |
| xlarge | 100 M | 3 G | 4096 | 100 M | 48 M |
