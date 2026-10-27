/* Workload `ntt` : produit cyclique par NTT entière (voir spec/workloads.d/ntt.md). */
#define NTT_P 2013265921ULL
#define NTT_G 31ULL

static u64 ntt_pow(u64 b, u64 e) {
    u64 r = 1;
    while (e > 0) {
        if (e % 2 == 1) r = r * b % NTT_P;
        b = b * b % NTT_P;
        e /= 2;
    }
    return r;
}

/* Permutation bit-reverse sur k bits, puis étages de papillons Cooley-Tukey (twiddles w^(j*stride)). */
static void ntt_transform(uint32_t *a, const uint32_t *tw, size_t n, unsigned k) {
    for (size_t i = 0; i < n; i++) {
        size_t x = i, rev = 0;
        for (unsigned b = 0; b < k; b++) { rev = rev * 2 + x % 2; x /= 2; }
        if (i < rev) { uint32_t t = a[i]; a[i] = a[rev]; a[rev] = t; }
    }
    for (size_t len = 2; len <= n; len *= 2) {
        size_t half = len / 2, stride = n / len;
        for (size_t blk = 0; blk < stride; blk++) {
            size_t start = blk * len;
            for (size_t j = 0; j < half; j++) {
                u64 u = a[start + j];
                u64 v = (u64)a[start + j + half] * tw[j * stride] % NTT_P;
                a[start + j] = (uint32_t)((u + v) % NTT_P);
                a[start + j + half] = (uint32_t)((u + NTT_P - v) % NTT_P);
            }
        }
    }
}

static u64 run_ntt(size_t n, int threads) {
    (void)threads;
    if (n > ((size_t)1 << 27)) return 0;
    size_t pw = 1; unsigned k = 0;
    while (pw < n) { pw *= 2; k++; }
    if (pw != n) return 0;
    uint32_t *a = xalloc(n * sizeof *a, 0);
    uint32_t *tw = xalloc((n / 2 + 1) * sizeof *tw, 0);
    for (size_t i = 0; i < n; i++) a[i] = (uint32_t)(mix64(0xA5A5F00D12345678ULL + i) % NTT_P);
    u64 w = ntt_pow(NTT_G, (NTT_P - 1) / n);
    u64 c = 1;
    for (size_t j = 0; j < n / 2; j++) { tw[j] = (uint32_t)c; c = c * w % NTT_P; }
    ntt_transform(a, tw, n, k);
    for (size_t i = 0; i < n; i++) a[i] = (uint32_t)((u64)a[i] * a[i] % NTT_P);
    ntt_transform(a, tw, n, k);
    for (size_t i = 1; 2 * i < n; i++) { uint32_t t = a[i]; a[i] = a[n - i]; a[n - i] = t; }
    u64 ninv = ntt_pow(n, NTT_P - 2);
    u64 ck = CK_INIT;
    for (size_t i = 0; i < n; i++) ck = ckfold(ck, (u64)a[i] * ninv % NTT_P);
    free(a); free(tw);
    return ck;
}
