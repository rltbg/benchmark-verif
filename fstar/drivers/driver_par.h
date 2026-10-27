/* Pilote des workloads parallèles (parmatmul, parsort, parsha) pour le C extrait par F*. Les threads sont ici, dans le code non prouvé ;
 * les noyaux (Bench_Par_mat_rows, Bench_Sort_msort / merge / copy_back, Bench_Par_sha_seg) sont ceux vérifiés par F*. */
typedef struct { void (*fn)(void *, int); void *arg; int t; } drvpar_task;
static void *drvpar_tramp(void *p) { drvpar_task *k = p; k->fn(k->arg, k->t); return NULL; }
static void drvpar_do(int T, void (*fn)(void *, int), void *arg) {
    pthread_t th[64]; drvpar_task tk[64];
    if (T < 1) T = 1;
    if (T > 64) T = 64;
    for (int t = 1; t < T; t++) { tk[t] = (drvpar_task){fn, arg, t}; if (pthread_create(&th[t], NULL, drvpar_tramp, &tk[t])) { fprintf(stderr, "pthread_create\n"); exit(3); } }
    fn(arg, 0);
    for (int t = 1; t < T; t++) pthread_join(th[t], NULL);
}
static uint64_t drvpar_ckfold(uint64_t ck, uint64_t v) { return (ck ^ v) * 0x100000001B3ULL; }
#define DRVPAR_CK_INIT 0xCBF29CE484222325ULL

/* ---- parmatmul ---- */
typedef struct { uint32_t *a, *b, *c; size_t n; int T; } drvpm_ctx;
static void drvpm_worker(void *p, int t) {
    drvpm_ctx *x = p;
    Bench_Par_mat_rows(x->a, x->b, x->c, x->n, (size_t)((uint64_t)x->n * t / x->T), (size_t)((uint64_t)x->n * (t + 1) / x->T));
}
static uint64_t drv_parmatmul(size_t n, int T) {
    size_t nn = n * n;
    uint32_t *a = Bench_Par_u32_alloc(nn), *b = Bench_Par_u32_alloc(nn), *c = Bench_Par_u32_alloc(nn);
    Bench_Matmul_init_mat(a, nn, 0);
    Bench_Matmul_init_mat(b, nn, (uint64_t)nn);
    drvpm_ctx x = {a, b, c, n, T < 1 ? 1 : T};
    drvpar_do(x.T, drvpm_worker, &x);
    uint64_t ck = Bench_Matmul_checksum(c, nn);
    Bench_Par_u32_free(a); Bench_Par_u32_free(b); Bench_Par_u32_free(c);
    return ck;
}

/* ---- parsort ---- */
typedef struct { uint32_t *a, *tmp; size_t n; int T; size_t *b; int pairs; } drvps_ctx;
static void drvps_sort_worker(void *p, int t) {
    drvps_ctx *x = p;
    Bench_Sort_msort(x->a, x->tmp, (size_t)((uint64_t)x->n * t / x->T), (size_t)((uint64_t)x->n * (t + 1) / x->T));
}
static void drvps_merge_worker(void *p, int t) {
    drvps_ctx *x = p;
    if (t >= x->pairs) return;
    size_t lo = x->b[2 * t], mid = x->b[2 * t + 1], hi = x->b[2 * t + 2];
    Bench_Sort_merge(x->a, x->tmp, lo, mid, hi);
    Bench_Sort_copy_back(x->tmp, x->a, lo, hi);
}
static uint64_t drv_parsort(size_t n, int threads) {
    int T = threads < 1 ? 1 : (threads > 64 ? 64 : threads);
    uint32_t *a = Bench_Par_u32_alloc(n), *tmp = Bench_Par_u32_alloc(n);
    Bench_Sort_fill(a, n);
    size_t bounds[130];
    drvps_ctx x = {a, tmp, n, T, bounds, 0};
    drvpar_do(T, drvps_sort_worker, &x);
    int r = T;
    for (int t = 0; t <= T; t++) bounds[t] = (size_t)((uint64_t)n * t / T);
    while (r > 1) {
        x.pairs = r / 2;
        drvpar_do(x.pairs, drvps_merge_worker, &x);
        int nr = 0;
        for (int i = 0; i + 1 < r; i += 2) bounds[nr++] = bounds[i];
        if (r % 2) bounds[nr++] = bounds[r - 1];
        bounds[nr] = n;
        r = nr;
    }
    uint64_t ck = Bench_Sort_checksum(a, n);
    Bench_Par_u32_free(a); Bench_Par_u32_free(tmp);
    return ck;
}

/* ---- parsha ---- */
typedef struct { size_t m; uint32_t *h[64]; } drvsh_ctx;
static void drvsh_worker(void *p, int t) {
    drvsh_ctx *x = p;
    Bench_Par_sha_seg(0x5348413235362D42ULL + ((uint64_t)t << 40), x->m, x->h[t]);
}
static uint64_t drv_parsha(size_t n, int threads) {
    int T = threads < 1 ? 1 : (threads > 64 ? 64 : threads);
    drvsh_ctx x; x.m = n / (size_t)T;
    for (int t = 0; t < T; t++) x.h[t] = Bench_Par_u32_alloc(8);
    drvpar_do(T, drvsh_worker, &x);
    uint64_t ck = DRVPAR_CK_INIT;
    for (int s = 0; s < T; s++) for (int i = 0; i < 8; i++) ck = drvpar_ckfold(ck, x.h[s][i]);
    for (int t = 0; t < T; t++) Bench_Par_u32_free(x.h[t]);
    return ck;
}
