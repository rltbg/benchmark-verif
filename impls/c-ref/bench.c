/* Implémentation C de référence (écrite à la main, non vérifiée).
 * Sert d'oracle pour les checksums et de plancher de performance.
 * Spécification : spec/WORKLOADS.md */
#define _POSIX_C_SOURCE 200809L
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

typedef uint8_t u8; typedef uint32_t u32; typedef uint64_t u64;

static inline u64 mix64(u64 x) {
    u64 z = x + 0x9E3779B97F4A7C15ULL;
    z = (z ^ (z >> 30)) * 0xBF58476D1CE4E5B9ULL;
    z = (z ^ (z >> 27)) * 0x94D049BB133111EBULL;
    return z ^ (z >> 31);
}
#define CK_INIT 0xCBF29CE484222325ULL
static inline u64 fold(u64 ck, u64 v) { return (ck ^ v) * 0x100000001B3ULL; }

#define S_SORT 0x0123456789ABCDEFULL
#define S_SIEVE 0 /* inutilisé */
#define S_MAT  0x0FEDCBA987654321ULL
#define S_LINE 0x1357924680ACEBDFULL
#define S_HASH 0x2468ACE013579BDFULL

static void *xmalloc(size_t n) { void *p = malloc(n ? n : 1); if (!p) { fprintf(stderr, "oom\n"); exit(3); } return p; }

/* ---------- sort : tri fusion ascendant, un seul buffer de 2n u32 ---------- */
static u64 run_sort(u64 n) {
    u32 *buf = xmalloc(2 * n * sizeof(u32));
    u32 *src = buf, *dst = buf + n;
    for (u64 i = 0; i < n; i++) src[i] = (u32)(mix64(S_SORT + i) >> 32);
    for (u64 w = 1; w < n; w *= 2) {
        for (u64 lo = 0; lo < n; lo += 2 * w) {
            u64 mid = lo + w < n ? lo + w : n;
            u64 hi = lo + 2 * w < n ? lo + 2 * w : n;
            u64 i = lo, j = mid, k = lo;
            while (i < mid && j < hi) dst[k++] = (src[i] <= src[j]) ? src[i++] : src[j++];
            while (i < mid) dst[k++] = src[i++];
            while (j < hi) dst[k++] = src[j++];
        }
        u32 *t = src; src = dst; dst = t;
    }
    u64 ck = CK_INIT;
    for (u64 i = 0; i < n; i++) {
        if (i > 0 && src[i - 1] > src[i]) { free(buf); return 0; }
        ck = fold(ck, src[i]);
    }
    free(buf);
    return ck;
}

/* ---------- sieve : crible d'Ératosthène, 1 octet par entier ---------- */
static u64 run_sieve(u64 n) {
    u8 *f = calloc(n + 1, 1);
    if (!f) { fprintf(stderr, "oom\n"); exit(3); }
    for (u64 i = 2; i * i <= n; i++)
        if (f[i] == 0)
            for (u64 j = i * i; j <= n; j += i) f[j] = 1;
    u64 ck = CK_INIT, cnt = 0;
    for (u64 i = 2; i <= n; i++)
        if (f[i] == 0) { ck = fold(ck, i); cnt++; }
    free(f);
    return fold(ck, cnt);
}

/* ---------- matmul : C = A*B, u32 avec retenue perdue, ordre i-k-j ---------- */
static u64 run_matmul(u64 n) {
    size_t nn = n * n;
    u32 *a = xmalloc(nn * 4), *b = xmalloc(nn * 4), *c = calloc(nn, 4);
    if (!c) exit(3);
    for (u64 i = 0; i < nn; i++) a[i] = (u32)(mix64(S_MAT + i) >> 32);
    for (u64 i = 0; i < nn; i++) b[i] = (u32)(mix64(S_MAT + nn + i) >> 32);
    for (u64 i = 0; i < n; i++)
        for (u64 k = 0; k < n; k++) {
            u32 aik = a[i * n + k];
            for (u64 j = 0; j < n; j++) c[i * n + j] += aik * b[k * n + j];
        }
    u64 ck = CK_INIT;
    for (u64 i = 0; i < nn; i++) ck = fold(ck, c[i]);
    free(a); free(b); free(c);
    return ck;
}

/* ---------- lines : génération puis parsing/agrégation de lignes "clé;valeur\n" ---------- */
#define LINE_K (1u << 20)
#define LINE_MAX 15
static size_t put_dec(u8 *p, u64 v) {
    u8 tmp[20]; size_t l = 0;
    do { tmp[l++] = (u8)('0' + v % 10); v /= 10; } while (v);
    for (size_t i = 0; i < l; i++) p[i] = tmp[l - 1 - i];
    return l;
}
static u64 run_lines(u64 n) {
    u8 *txt = xmalloc(n * LINE_MAX);
    size_t len = 0;
    for (u64 i = 0; i < n; i++) {
        u64 r = mix64(S_LINE + i);
        u64 key = r >> 44, val = ((r >> 8) & 0xFFFFFFFFULL) % 1000000ULL;
        len += put_dec(txt + len, key); txt[len++] = ';';
        len += put_dec(txt + len, val); txt[len++] = '\n';
    }
    u64 *tab = calloc(2 * (size_t)LINE_K, 8);
    if (!tab) exit(3);
    size_t p = 0;
    while (p < len) {
        u64 key = 0, val = 0;
        while (txt[p] != ';') key = key * 10 + (txt[p++] - '0');
        p++;
        while (txt[p] != '\n') val = val * 10 + (txt[p++] - '0');
        p++;
        tab[2 * key] += val; tab[2 * key + 1] += 1;
    }
    u64 ck = CK_INIT;
    for (size_t k = 0; k < LINE_K; k++) ck = fold(fold(ck, tab[2 * k]), tab[2 * k + 1]);
    free(txt); free(tab);
    return ck;
}

/* ---------- hashmap : adressage ouvert, sondage linéaire, clés/valeurs entrelacées ---------- */
static u64 run_hashmap(u64 n) {
    unsigned lg = 1; while (((u64)1 << lg) < 2 * n) lg++;
    u64 cap = (u64)1 << lg, mask = cap - 1;
    u64 *t = calloc(2 * cap, 8);
    if (!t) { fprintf(stderr, "oom\n"); exit(3); }
    for (u64 i = 0; i < n; i++) {
        u64 key = mix64(S_HASH + i) | 1;
        u64 s = (key * 0x9E3779B97F4A7C15ULL) >> (64 - lg);
        for (;;) {
            if (t[2 * s] == 0) { t[2 * s] = key; t[2 * s + 1] = i + 1; break; }
            if (t[2 * s] == key) { t[2 * s + 1] += i + 1; break; }
            s = (s + 1) & mask;
        }
    }
    u64 ck = CK_INIT;
    for (int phase = 0; phase < 2; phase++)
        for (u64 i = 0; i < n; i++) {
            u64 key = mix64(S_HASH + (phase ? n : 0) + i) | 1;
            u64 s = (key * 0x9E3779B97F4A7C15ULL) >> (64 - lg), v = 0;
            for (;;) {
                if (t[2 * s] == 0) break;
                if (t[2 * s] == key) { v = t[2 * s + 1]; break; }
                s = (s + 1) & mask;
            }
            ck = fold(ck, v);
        }
    free(t);
    return ck;
}

int main(int argc, char **argv) {
    if (argc != 3) { fprintf(stderr, "usage: %s <sort|sieve|matmul|lines|hashmap> <n>\n", argv[0]); return 2; }
    u64 n = strtoull(argv[2], NULL, 10), ck;
    struct timespec t0, t1;
    clock_gettime(CLOCK_MONOTONIC, &t0);
    if (!strcmp(argv[1], "sort")) ck = run_sort(n);
    else if (!strcmp(argv[1], "sieve")) ck = run_sieve(n);
    else if (!strcmp(argv[1], "matmul")) ck = run_matmul(n);
    else if (!strcmp(argv[1], "lines")) ck = run_lines(n);
    else if (!strcmp(argv[1], "hashmap")) ck = run_hashmap(n);
    else { fprintf(stderr, "workload inconnu\n"); return 2; }
    clock_gettime(CLOCK_MONOTONIC, &t1);
    u64 ns = (u64)(t1.tv_sec - t0.tv_sec) * 1000000000ULL + (u64)(t1.tv_nsec - t0.tv_nsec);
    printf("RESULT workload=%s n=%llu checksum=%016llx time_ns=%llu\n", argv[1],
           (unsigned long long)n, (unsigned long long)ck, (unsigned long long)ns);
    return 0;
}
