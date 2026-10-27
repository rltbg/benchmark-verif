# parmatmul — produit matriciel multi-thread

Même spécification, mêmes données et **même checksum** que `matmul` (spec/WORKLOADS.md) au même `n` : A, B de n×n u32
(`hi32(mix64(0x0FEDCBA987654321 + off + i))`, off = 0 pour A, n² pour B), C = A·B modulo 2³², boucles i-k-j, checksum = ckfold des
n² éléments de C en ordre ligne-majeur. Le calcul de C est réparti sur `T` threads : le thread t calcule les lignes
`[ t·n/T , (t+1)·n/T )` (division entière). L'allocation, l'initialisation et le checksum sont séquentiels (inclus dans le temps mesuré).
Noyau séquentiel (prouvé côté F*/Creusot) : `rows(a, b, c, n, i0, i1)`. Les threads vivent dans le code non prouvé
(pilotes C/F#/OCaml pour F*, wrapper `#[trusted]` pour Creusot). Le checksum ne dépend pas de T. Borne : 1 ≤ n ≤ 16384.
