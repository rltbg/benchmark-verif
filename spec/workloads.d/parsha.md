# parsha — SHA-256 de T segments indépendants

`n` octets au total, `n` multiple de `64·T`. Segment s (0 ≤ s < T) = `m = n/T` octets : octet i du segment = octet (i mod 8) (petit-boutien) de
`mix64(0x5348413235362D42 + s·2⁴⁰ + ⌊i/8⌋)` ; le message est généré à la volée (jamais en entier en mémoire), haché par SHA-256 standard
(blocs de 64 o, padding sur m octets). Chaque thread calcule le condensé d'un segment (le thread s traite le segment s). Checksum = ckfold, dans l'ordre
des segments s = 0..T-1, des 8 mots H0..H7 (u32 → u64). **Le checksum dépend de T** ; pour T = 1 c'est celui du workload `sha256` au même `n`.
Noyau séquentiel : `sha_segment(seedbase, m, h[8])`. Threads dans le code non prouvé.
