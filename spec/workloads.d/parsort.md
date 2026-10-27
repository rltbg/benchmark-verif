# parsort — tri fusion multi-thread

Mêmes données et **même checksum** que `sort` au même `n` (u32 = `hi32(mix64(0x0123456789ABCDEF + i))`, checksum = ckfold des éléments triés, 0 si l'ordre
n'est pas croissant). Algorithme : le tableau est découpé en `T` segments `[ t·n/T , (t+1)·n/T )` ; chaque thread trie son segment avec le noyau séquentiel
(tri fusion descendant avec tableau temporaire et recopie, celui de `sort`), puis les segments voisins sont fusionnés par paires, niveau par niveau
(fusion de `[lo,mid)` et `[mid,hi)` vers le tampon puis recopie ; les fusions d'un même niveau s'exécutent en parallèle, chacune par un thread ; la dernière
fusion est donc séquentielle : c'est la fraction non parallélisable de la loi d'Amdahl). Remplissage et checksum séquentiels. Borne : 1 ≤ n ≤ 2³⁰.
Noyaux prouvés : `msort(a,tmp,lo,hi)`, `merge(src,dst,lo,mid,hi)`, `copy_back` ; threads dans le code non prouvé.
