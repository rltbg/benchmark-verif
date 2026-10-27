# `alloc` — allocation en masse (allocateur, GC, empreinte mémoire)

Mesure ce que coûte réellement l'allocation de petits blocs du tas : `malloc`/`free` (C, Zig, F*→C), `Vec`/`Box` (Rust),
tableaux gérés par un GC (F# `Array.zeroCreate`, OCaml `Bytes`). Les allocations **ne sont jamais regroupées** : chaque bloc est
un objet du tas distinct, et les compilateurs n'ont pas le droit d'éliminer les paires allocation/libération (barrières :
`asm volatile`/`black_box`/`doNotOptimizeAway` ; `-fno-builtin-malloc -fno-builtin-free` pour le C).

## Spécification

Constantes : `SEED = 0xA110C00000000001`, `WMAX = 2^25`. Contrainte : `1 ≤ n ≤ 4 000 000 000`.
`W = min(max(1, n / 64), WMAX)` (division entière) : capacité de la fenêtre vivante.

Pour `i = 0 .. n-1` :
1. `r = mix64(SEED + i)` ; `s = 8 + 8·(r mod 32)` octets (8, 16, …, 256).
2. allocation **zéro-initialisée** d'un bloc du tas de `s` octets ;
3. remplissage : `b[k] = (i + k) mod 251` pour `k < s` ;
4. somme : `sum = Σ b[k]` (relue dans le bloc, `u64`) ; `ck = fold(ck, sum)` ;
5. **régime (a), churn** : si `i mod 16 ≠ 0`, le bloc est libéré immédiatement ;
   **régime (b), fenêtre vivante** : si `i mod 16 = 0`, le bloc est conservé dans une file FIFO de capacité W
   (c'est le j-ième bloc conservé, `j = 0, 1, …`) : quand la file contient déjà W blocs, le plus ancien est libéré
   (après l'allocation du nouveau). Implémentation libre tant que les allocations/libérations restent les mêmes : anneau de
   W emplacements (C, Zig, Rust, Rust + Creusot) ou file à deux piles de nœuds chaînés (F*, dont la slprop récursive porte la
   propriété de chaque bloc — c'est la forme qui passe en Pulse).

Fin : tous les blocs encore vivants sont libérés (temps compté). Checksum : `fold(ck, nombre_de_blocs_conservés)`
(`= ⌈n/16⌉`), avec `ck` initial `CK_INIT`.

Mémoire idéale (pour le rapport RAM/idéal) : `min(max(n/64,1), 2^25) · (132 + 8)` octets (blocs de 132 octets en moyenne + un
pointeur d'anneau) ; l'en-tête de l'allocateur (8–16 o/bloc glibc, 16–24 o/objet .NET, nœud de file F* de 16 o + son en-tête) vient en plus : c'est
précisément ce que le workload veut faire apparaître.

## Prouvé

* F* (Pulse, `fstar/src/Bench.Alloc.fst`) : mémoire-sûreté (indices de remplissage, absence de fuite et de double libération :
  la slprop récursive `owned` possède chaque bloc de la file, `drain` rend `emp`), bornes de la taille des blocs, pas de débordement.
* Creusot (`impls/rust/src/alloc.rs`) : pas de panique ni de débordement ; `fill` préserve la taille ; l'index d'anneau est dans les bornes.
  `#[trusted]` : la barrière `black_box` seulement ; `run` n'est pas `check(terminates)` (il alloue).

## Tailles (voir `harness/presets.d/alloc.json`) et checksums de référence

| preset | n | W | `checksum` |
|---|--:|--:|---|
| smoke | 20 000 | 312 | `1575315341748033` |
| tiny | 100 000 | 1 562 | `3051fa8407722d11` |
| small | 2 000 000 | 31 250 | `1cb80a794f97dc3f` |
| medium | 20 000 000 | 312 500 | `2e7aff6a37f04a16` |
| large | 100 000 000 | 1 562 500 | `7f2586b8212b4ee9` |
| xlarge | 500 000 000 | 7 812 500 | (voir `spec/checksums.d/alloc.json`) |
| saturate | 1 600 000 000 | 25 000 000 | (idem) |
