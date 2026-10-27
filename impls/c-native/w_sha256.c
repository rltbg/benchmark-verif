/* Workload sha256 : SHA-256 d'un message de n octets généré à la volée (spec/workloads.d/sha256.md).
 * Octet i = octet (i mod 8) (petit-boutien) de mix64(0x5348413235362D42 + i/8). */
static const uint32_t sha_k[64] = {
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2};

#define SHA_ROTR(x, k) (((x) >> (k)) | ((x) << (32 - (k))))

static inline void sha_compress(uint32_t h[8], const uint32_t m[16]) {
    uint32_t w[64];
    for (int t = 0; t < 16; t++) w[t] = m[t];
    for (int t = 16; t < 64; t++) {
        uint32_t s0 = SHA_ROTR(w[t - 15], 7) ^ SHA_ROTR(w[t - 15], 18) ^ (w[t - 15] >> 3);
        uint32_t s1 = SHA_ROTR(w[t - 2], 17) ^ SHA_ROTR(w[t - 2], 19) ^ (w[t - 2] >> 10);
        w[t] = w[t - 16] + s0 + w[t - 7] + s1;
    }
    uint32_t a = h[0], b = h[1], c = h[2], d = h[3], e = h[4], f = h[5], g = h[6], hh = h[7];
    for (int t = 0; t < 64; t++) {
        uint32_t t1 = hh + (SHA_ROTR(e, 6) ^ SHA_ROTR(e, 11) ^ SHA_ROTR(e, 25)) + ((e & f) ^ (~e & g)) + sha_k[t] + w[t];
        uint32_t t2 = (SHA_ROTR(a, 2) ^ SHA_ROTR(a, 13) ^ SHA_ROTR(a, 22)) + ((a & b) ^ (a & c) ^ (b & c));
        hh = g; g = f; f = e; e = d + t1; d = c; c = b; b = a; a = t1 + t2;
    }
    h[0] += a; h[1] += b; h[2] += c; h[3] += d; h[4] += e; h[5] += f; h[6] += g; h[7] += hh;
}

static inline uint32_t sha_bswap(uint32_t x) { return __builtin_bswap32(x); }
static inline uint8_t sha_byte(uint64_t i) { return (uint8_t)(mix64(0x5348413235362D42ULL + i / 8) >> (8 * (i % 8))); }

static void sha256_digest(size_t n, uint32_t h[8]) {
    static const uint32_t iv[8] = {0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19};
    memcpy(h, iv, sizeof iv);
    size_t blocks = n / 64;
    uint32_t m[16];
    for (size_t b = 0; b < blocks; b++) {
        for (int k = 0; k < 8; k++) {
            u64 x = mix64(0x5348413235362D42ULL + 8 * (u64)b + (u64)k);
            m[2 * k] = sha_bswap((uint32_t)x);
            m[2 * k + 1] = sha_bswap((uint32_t)(x >> 32));
        }
        sha_compress(h, m);
    }
    uint8_t tail[128];
    memset(tail, 0, sizeof tail);
    size_t t = n % 64;
    for (size_t j = 0; j < t; j++) tail[j] = sha_byte((u64)blocks * 64 + j);
    tail[t] = 0x80;
    size_t total = t < 56 ? 64 : 128;
    u64 bits = (u64)n * 8;
    for (int j = 0; j < 8; j++) tail[total - 8 + j] = (uint8_t)(bits >> (56 - 8 * j));
    for (size_t off = 0; off < total; off += 64) {
        for (int i = 0; i < 16; i++) {
            const uint8_t *p = tail + off + 4 * i;
            m[i] = ((uint32_t)p[0] << 24) | ((uint32_t)p[1] << 16) | ((uint32_t)p[2] << 8) | p[3];
        }
        sha_compress(h, m);
    }
}

static u64 run_sha256(size_t n, int threads) {
    (void)threads;
    uint32_t h[8];
    sha256_digest(n, h);
    u64 ck = CK_INIT;
    for (int i = 0; i < 8; i++) ck = ckfold(ck, h[i]);
    return ck;
}
