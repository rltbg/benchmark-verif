/* strsort : tri fusion stable d'indices de chaînes (1..32 octets, alphabet a..d) stockées dans un pool contigu.
 * Spécification : spec/workloads.d/strsort.md. Même algorithme que les autres colonnes (comparaison octet par octet). */
static inline u64 ss_len_of(u64 i) { return 1 + (mix64(0xC0FFEE0000000001ULL + i) >> 59); }

/* a <= b pour l'ordre (octets lexicographiques, puis longueur) */
static inline int ss_le(const u8 *pool, const u64 *off, u32 a, u32 b) {
    const u8 *pa = pool + off[a], *pb = pool + off[b];
    u64 la = off[a + 1] - off[a], lb = off[b + 1] - off[b];
    u64 m = la < lb ? la : lb;
    for (u64 k = 0; k < m; k++)
        if (pa[k] != pb[k]) return pa[k] < pb[k];
    return la <= lb;
}
static void ss_msort(const u8 *pool, const u64 *off, u32 *a, u32 *tmp, size_t lo, size_t hi) {
    if (hi - lo < 2) return;
    size_t mid = lo + (hi - lo) / 2;
    ss_msort(pool, off, a, tmp, lo, mid);
    ss_msort(pool, off, a, tmp, mid, hi);
    size_t i = lo, j = mid, k = lo;
    while (k < hi) {
        if (i < mid && (j >= hi || ss_le(pool, off, a[i], a[j]))) tmp[k++] = a[i++];
        else tmp[k++] = a[j++];
    }
    memcpy(a + lo, tmp + lo, (hi - lo) * sizeof(u32));
}
static u64 run_strsort(size_t n, int threads) {
    (void)threads;
    u64 *off = xalloc((n + 1) * sizeof(u64), 1);
    for (size_t i = 0; i < n; i++) off[i + 1] = off[i] + ss_len_of(i);
    u64 total = off[n];
    u8 *pool = xalloc(total, 0);
    for (size_t i = 0; i < n; i++) {
        u64 l = off[i + 1] - off[i];
        for (u64 k = 0; k < l; k++)
            pool[off[i] + k] = (u8)('a' + (mix64(0xC0FFEE0000000002ULL + 64 * (u64)i + k) & 3));
    }
    u32 *a = xalloc(n * sizeof(u32), 0), *tmp = xalloc(n * sizeof(u32), 0);
    for (size_t i = 0; i < n; i++) a[i] = (u32)i;
    ss_msort(pool, off, a, tmp, 0, n);
    u64 ck = CK_INIT;
    for (size_t i = 0; i < n; i++) ck = ckfold(ck, a[i]);
    ck = ckfold(ck, total);
    free(a); free(tmp); free(pool); free(off);
    return ck;
}
