/* Workloads parallèles (parmatmul, parsort, parsha) : threads POSIX. Spécifications : spec/workloads.d/par*.md.
 * Ce fichier définit aussi l'assistant `par_do`, utilisé par w_parsort.c et w_parsha.c (inclus après lui). */
typedef struct { void (*fn)(void *, int); void *arg; int t; } par_task;
static void *par_tramp(void *p) { par_task *k = p; k->fn(k->arg, k->t); return NULL; }
/* Exécute fn(arg, t) pour t = 0..T-1 : le thread appelant fait t = 0, les autres sont des pthreads. */
static void par_do(int T, void (*fn)(void *, int), void *arg) {
    pthread_t th[64]; par_task tk[64];
    if (T < 1) T = 1;
    if (T > 64) T = 64;
    for (int t = 1; t < T; t++) { tk[t] = (par_task){fn, arg, t}; if (pthread_create(&th[t], NULL, par_tramp, &tk[t])) { fprintf(stderr, "pthread_create\n"); exit(3); } }
    fn(arg, 0);
    for (int t = 1; t < T; t++) pthread_join(th[t], NULL);
}

typedef struct { const u32 *a, *b; u32 *c; size_t n; int T; } pm_ctx;
/* Noyau séquentiel : lignes i0..i1 de C = A*B (c est supposé initialisé à 0 sur ces lignes). */
static void pm_rows(const u32 *a, const u32 *b, u32 *c, size_t n, size_t i0, size_t i1) {
    for (size_t i = i0; i < i1; i++)
        for (size_t k = 0; k < n; k++) {
            u32 aik = a[i * n + k];
            for (size_t j = 0; j < n; j++) c[i * n + j] += aik * b[k * n + j];
        }
}
static void pm_worker(void *p, int t) {
    pm_ctx *x = p;
    pm_rows(x->a, x->b, x->c, x->n, (size_t)((u64)x->n * t / x->T), (size_t)((u64)x->n * (t + 1) / x->T));
}
static u64 run_parmatmul(size_t n, int threads) {
    size_t nn = n * n;
    u32 *a = xalloc(nn * 4, 0), *b = xalloc(nn * 4, 0), *c = xalloc(nn * 4, 1);
    for (size_t i = 0; i < nn; i++) a[i] = (u32)(mix64(0x0FEDCBA987654321ULL + i) >> 32);
    for (size_t i = 0; i < nn; i++) b[i] = (u32)(mix64(0x0FEDCBA987654321ULL + nn + i) >> 32);
    pm_ctx x = {a, b, c, n, threads < 1 ? 1 : threads};
    par_do(x.T, pm_worker, &x);
    u64 ck = CK_INIT;
    for (size_t i = 0; i < nn; i++) ck = ckfold(ck, c[i]);
    free(a); free(b); free(c);
    return ck;
}
