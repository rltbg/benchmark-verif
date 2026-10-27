/* ptrchase : voir spec/workloads.d/ptrchase.md (arène par indices, BST + recherches + chasse de pointeurs). */
#define PC_NIL 0xFFFFFFFFu

static u64 run_ptrchase(size_t n, int threads) {
    (void)threads;
    u64 *a = malloc(2 * n * sizeof(u64));
    uint32_t *next = malloc(n * sizeof(uint32_t));
    if (!a || !next) { fprintf(stderr, "oom\n"); exit(3); }
    /* phase 1 : insertions */
    u64 cnt = 1, plen = 0;
    a[0] = mix64(0xD00D000000000001ULL) | 1;
    a[1] = (u64)PC_NIL | ((u64)PC_NIL << 32);
    for (u64 i = 1; i < n; i++) {
        u64 nk = mix64(0xD00D000000000001ULL + i) | 1;
        u64 cur = 0, v = 0;
        for (;;) {
            v++;
            u64 key = a[2 * cur];
            if (nk == key) break;
            u64 lnk = a[2 * cur + 1];
            int left = nk < key;
            u64 c = left ? (lnk & 0xFFFFFFFFu) : (lnk >> 32);
            if (c < cnt) { cur = c; continue; }
            a[2 * cnt] = nk;
            a[2 * cnt + 1] = (u64)PC_NIL | ((u64)PC_NIL << 32);
            a[2 * cur + 1] = left ? ((lnk & ~0xFFFFFFFFULL) | cnt) : ((lnk & 0xFFFFFFFFULL) | (cnt << 32));
            cnt++;
            break;
        }
        plen += v;
    }
    u64 r1 = ckfold(ckfold(CK_INIT, cnt), plen);
    /* phase 2 : recherches */
    u64 r2 = CK_INIT;
    u64 total = n + n / 2;
    for (u64 k = 0; k < total; k++) {
        u64 key = k < n ? (mix64(0xD00D000000000001ULL + (3 * k) % n) | 1) : (mix64(0xD00D000000000003ULL + (k - n)) & ~1ULL);
        u64 cur = 0, v = 0, found = 0;
        while (v < cnt) {
            v++;
            u64 kc = a[2 * cur];
            if (key == kc) { found = 1; break; }
            u64 lnk = a[2 * cur + 1];
            u64 c = key < kc ? (lnk & 0xFFFFFFFFu) : (lnk >> 32);
            if (c < cnt) cur = c; else break;
        }
        r2 = ckfold(r2, (v << 1) | found);
    }
    /* phase 3 : permutation de Sattolo puis chasse */
    for (size_t i = 0; i < n; i++) next[i] = (uint32_t)i;
    for (u64 i = n - 1; i >= 1; i--) {
        u64 j = mix64(0xD00D000000000002ULL + i) % i;
        uint32_t t = next[i]; next[i] = next[j]; next[j] = t;
    }
    u64 cur = 0, sum = 0;
    for (u64 h = 0; h < 4 * (u64)n; h++) { cur = next[cur]; sum += cur; }
    u64 r3 = ckfold(ckfold(CK_INIT, cur), sum);
    free(a); free(next);
    return ckfold(ckfold(ckfold(CK_INIT, r1), r2), r3);
}
