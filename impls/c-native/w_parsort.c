/* parsort : segments triés en parallèle (msort de bench.c), puis fusions par paires (spec/workloads.d/parsort.md). */
typedef struct { u32 *a, *tmp; size_t n; int T; size_t *b; int pairs; } ps_ctx;
static void ps_sort_worker(void *p, int t) {
    ps_ctx *x = p;
    size_t lo = (size_t)((u64)x->n * t / x->T), hi = (size_t)((u64)x->n * (t + 1) / x->T);
    msort(x->a, x->tmp, lo, hi);
}
/* x->b : bornes courantes des runs (r+1 valeurs) ; fusionne les runs 2t et 2t+1 (si le 2t+1 existe). */
static void ps_merge_worker(void *p, int t) {
    ps_ctx *x = p;
    if (t >= x->pairs) return;
    size_t lo = x->b[2 * t], mid = x->b[2 * t + 1], hi = x->b[2 * t + 2];
    merge(x->a, x->tmp, lo, mid, hi);
    memcpy(x->a + lo, x->tmp + lo, (hi - lo) * sizeof(u32));
}
static u64 run_parsort(size_t n, int threads) {
    int T = threads < 1 ? 1 : threads;
    u32 *a = xalloc(n * 4, 0), *tmp = xalloc(n * 4, 0);
    for (size_t i = 0; i < n; i++) a[i] = (u32)(mix64(0x0123456789ABCDEFULL + i) >> 32);
    size_t bounds[130];
    ps_ctx x = {a, tmp, n, T, bounds, 0};
    par_do(T, ps_sort_worker, &x);
    int r = T;
    for (int t = 0; t <= T; t++) bounds[t] = (size_t)((u64)n * t / T);
    while (r > 1) {
        x.pairs = r / 2;
        par_do(x.pairs, ps_merge_worker, &x);
        int nr = 0;
        for (int i = 0; i + 1 < r; i += 2) bounds[nr++] = bounds[i];
        if (r % 2) { bounds[nr++] = bounds[r - 1]; }
        bounds[nr] = n; /* fin du dernier run */
        r = nr;
    }
    u64 ck = CK_INIT; u32 prev = 0; int ok = 1;
    for (size_t i = 0; i < n; i++) { ok = ok && prev <= a[i]; prev = a[i]; ck = ckfold(ck, a[i]); }
    free(a); free(tmp);
    return ok ? ck : 0;
}
