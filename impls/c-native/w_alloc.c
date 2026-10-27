/* Workload `alloc` : allocation en masse (voir spec/workloads.d/alloc.md). Blocs réels du tas (malloc/free),
 * barrières d'optimisation : le compilateur n'a pas le droit d'éliminer les paires malloc/free. */
#define ALLOC_WMAX ((size_t)1 << 25)

static u64 run_alloc(size_t n, int threads) {
    (void)threads;
    size_t w = n >> 6;
    if (w < 1) w = 1;
    if (w > ALLOC_WMAX) w = ALLOC_WMAX;
    unsigned char **ring = (unsigned char **)calloc(w ? w : 1, sizeof *ring);
    u64 ck = CK_INIT;
    size_t kept = 0;
    for (size_t i = 0; i < n; i++) {
        u64 r = mix64(0xA110C00000000001ULL + i);
        size_t s = 8 + 8 * (size_t)(r & 31);
        unsigned char *b = (unsigned char *)malloc(s);
        memset(b, 0, s);
        __asm__ volatile("" : "+r"(b) : : "memory");
        unsigned v = (unsigned)(i % 251);
        for (size_t k = 0; k < s; k++) { b[k] = (unsigned char)v; v = v == 250 ? 0 : v + 1; }
        u64 sum = 0;
        for (size_t k = 0; k < s; k++) sum += b[k];
        ck = ckfold(ck, sum);
        if ((i & 15) == 0) {
            size_t slot = kept % w;
            if (kept >= w) free(ring[slot]);
            ring[slot] = b;
            kept++;
        } else {
            __asm__ volatile("" : : "r"(b) : "memory");
            free(b);
        }
    }
    size_t live = kept < w ? kept : w, first = kept >= w ? kept % w : 0;
    for (size_t j = 0; j < live; j++) free(ring[(first + j) % w]);
    free(ring);
    return ckfold(ck, (u64)kept);
}
