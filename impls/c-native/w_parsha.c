/* parsha : T segments SHA-256 indépendants, un par thread (spec/workloads.d/parsha.md). m = n/T multiple de 64. */
static const uint32_t psh_k[64] = {
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2};
#define PSH_ROTR(x, k) (((x) >> (k)) | ((x) << (32 - (k))))
static void psh_compress(uint32_t h[8], const uint32_t m[16]) {
    uint32_t w[64];
    for (int t = 0; t < 16; t++) w[t] = m[t];
    for (int t = 16; t < 64; t++) {
        uint32_t s0 = PSH_ROTR(w[t - 15], 7) ^ PSH_ROTR(w[t - 15], 18) ^ (w[t - 15] >> 3);
        uint32_t s1 = PSH_ROTR(w[t - 2], 17) ^ PSH_ROTR(w[t - 2], 19) ^ (w[t - 2] >> 10);
        w[t] = w[t - 16] + s0 + w[t - 7] + s1;
    }
    uint32_t a = h[0], b = h[1], c = h[2], d = h[3], e = h[4], f = h[5], g = h[6], hh = h[7];
    for (int t = 0; t < 64; t++) {
        uint32_t t1 = hh + (PSH_ROTR(e, 6) ^ PSH_ROTR(e, 11) ^ PSH_ROTR(e, 25)) + ((e & f) ^ (~e & g)) + psh_k[t] + w[t];
        uint32_t t2 = (PSH_ROTR(a, 2) ^ PSH_ROTR(a, 13) ^ PSH_ROTR(a, 22)) + ((a & b) ^ (a & c) ^ (b & c));
        hh = g; g = f; f = e; e = d + t1; d = c; c = b; b = a; a = t1 + t2;
    }
    h[0] += a; h[1] += b; h[2] += c; h[3] += d; h[4] += e; h[5] += f; h[6] += g; h[7] += hh;
}
/* Noyau séquentiel : condensé SHA-256 des m octets (m multiple de 64) du segment de base `seedbase`. */
static void psh_segment(u64 seedbase, size_t m, uint32_t h[8]) {
    static const uint32_t iv[8] = {0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19};
    memcpy(h, iv, sizeof iv);
    uint32_t blk[16];
    for (size_t b = 0; b < m / 64; b++) {
        for (int k = 0; k < 8; k++) {
            u64 x = mix64(seedbase + 8 * (u64)b + (u64)k);
            blk[2 * k] = __builtin_bswap32((uint32_t)x);
            blk[2 * k + 1] = __builtin_bswap32((uint32_t)(x >> 32));
        }
        psh_compress(h, blk);
    }
    memset(blk, 0, sizeof blk);
    blk[0] = 0x80000000u;
    u64 bits = (u64)m * 8;
    blk[14] = (uint32_t)(bits >> 32);
    blk[15] = (uint32_t)bits;
    psh_compress(h, blk);
}
typedef struct { size_t m; uint32_t (*h)[8]; } psh_ctx;
static void psh_worker(void *p, int t) {
    psh_ctx *x = p;
    psh_segment(0x5348413235362D42ULL + ((u64)t << 40), x->m, x->h[t]);
}
static u64 run_parsha(size_t n, int threads) {
    int T = threads < 1 ? 1 : threads;
    if (T > 64) T = 64;
    uint32_t h[64][8];
    psh_ctx x = {n / (size_t)T, h};
    par_do(T, psh_worker, &x);
    u64 ck = CK_INIT;
    for (int s = 0; s < T; s++)
        for (int i = 0; i < 8; i++) ck = ckfold(ck, h[s][i]);
    return ck;
}
