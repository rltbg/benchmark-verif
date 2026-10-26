/* Implémentation C native, écrite à la main, NON vérifiée. Spécification : spec/WORKLOADS.md.
 * Même algorithmique que les autres colonnes (tri fusion descendant, analyseur à états, table à adressage ouvert). */
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
static inline u64 ckfold(u64 ck, u64 v) { return (ck ^ v) * 0x100000001B3ULL; }

static void *xalloc(size_t n, int zero) {
    void *p = zero ? calloc(n ? n : 1, 1) : malloc(n ? n : 1);
    if (!p) { fprintf(stderr, "oom\n"); exit(3); }
    return p;
}

/* ---------- sort ---------- */
static void merge(const u32 *src, u32 *dst, size_t lo, size_t mid, size_t hi) {
    size_t i = lo, j = mid, k = lo;
    while (k < hi) {
        if (i < mid && (j >= hi || src[i] <= src[j])) dst[k++] = src[i++];
        else dst[k++] = src[j++];
    }
}
static void msort(u32 *a, u32 *tmp, size_t lo, size_t hi) {
    if (hi - lo < 2) return;
    size_t mid = lo + (hi - lo) / 2;
    msort(a, tmp, lo, mid);
    msort(a, tmp, mid, hi);
    merge(a, tmp, lo, mid, hi);
    memcpy(a + lo, tmp + lo, (hi - lo) * sizeof(u32));
}
static u64 run_sort(size_t n) {
    u32 *a = xalloc(n * 4, 0), *tmp = xalloc(n * 4, 0);
    for (size_t i = 0; i < n; i++) a[i] = (u32)(mix64(0x0123456789ABCDEFULL + i) >> 32);
    msort(a, tmp, 0, n);
    u64 ck = CK_INIT; u32 prev = 0; int ok = 1;
    for (size_t i = 0; i < n; i++) { ok = ok && prev <= a[i]; prev = a[i]; ck = ckfold(ck, a[i]); }
    free(a); free(tmp);
    return ok ? ck : 0;
}

/* ---------- sieve ---------- */
static u64 run_sieve(size_t n) {
    u8 *f = xalloc(n + 1, 1);
    for (size_t i = 2; i <= n / i; i++)
        if (f[i] == 0)
            for (size_t j = i * i; j <= n; j += i) f[j] = 1;
    u64 ck = CK_INIT, cnt = 0;
    for (size_t i = 2; i <= n; i++)
        if (f[i] == 0) { ck = ckfold(ck, i); cnt++; }
    free(f);
    return ckfold(ck, cnt);
}

/* ---------- matmul ---------- */
static u64 run_matmul(size_t n) {
    size_t nn = n * n;
    u32 *a = xalloc(nn * 4, 0), *b = xalloc(nn * 4, 0), *c = xalloc(nn * 4, 1);
    for (size_t i = 0; i < nn; i++) a[i] = (u32)(mix64(0x0FEDCBA987654321ULL + i) >> 32);
    for (size_t i = 0; i < nn; i++) b[i] = (u32)(mix64(0x0FEDCBA987654321ULL + nn + i) >> 32);
    for (size_t i = 0; i < n; i++)
        for (size_t k = 0; k < n; k++) {
            u32 aik = a[i * n + k];
            for (size_t j = 0; j < n; j++) c[i * n + j] += aik * b[k * n + j];
        }
    u64 ck = CK_INIT;
    for (size_t i = 0; i < nn; i++) ck = ckfold(ck, c[i]);
    free(a); free(b); free(c);
    return ck;
}

/* ---------- lines ---------- */
static size_t put_dec(u8 *p, u32 v) {
    u8 tmp[10]; size_t l = 0;
    do { tmp[l++] = (u8)('0' + v % 10); v /= 10; } while (v);
    for (size_t i = 0; i < l; i++) p[i] = tmp[l - 1 - i];
    return l;
}
static u64 run_lines(size_t n) {
    u8 *txt = xalloc(n * 15 + 20, 0);
    size_t len = 0;
    for (size_t i = 0; i < n; i++) {
        u64 r = mix64(0x1357924680ACEBDFULL + i);
        u32 key = (u32)(r >> 44) & 0xFFFFF;
        u32 val = (u32)(((r >> 8) & 0xFFFFFFFFULL) % 1000000ULL);
        len += put_dec(txt + len, key); txt[len++] = ';';
        len += put_dec(txt + len, val); txt[len++] = '\n';
    }
    u64 *tab = xalloc(2097152 * 8, 1);
    int in_val = 0; u32 key = 0, vl = 0;
    for (size_t p = 0; p < len; p++) {
        u8 c = txt[p]; u32 d = (u32)(u8)(c - '0');
        if (in_val) {
            if (c == '\n') {
                size_t k = key & 0xFFFFF;
                tab[2 * k] += vl; tab[2 * k + 1] += 1;
                in_val = 0; key = 0; vl = 0;
            } else vl = vl * 10 + d;
        } else if (c == ';') in_val = 1;
        else key = key * 10 + d;
    }
    u64 ck = CK_INIT;
    for (size_t k = 0; k < 1048576; k++) ck = ckfold(ckfold(ck, tab[2 * k]), tab[2 * k + 1]);
    free(txt); free(tab);
    return ck;
}

/* ---------- hashmap ---------- */
static inline u64 key_of(u64 off, u64 i) { return mix64(0x2468ACE013579BDFULL + off + i) | 1; }
static u64 run_hashmap(size_t n) {
    size_t cap = 2; unsigned lg = 1;
    while (cap < 2 * n && lg < 27 && cap <= 33554432) { cap *= 2; lg++; }
    u64 *t = xalloc(2 * cap * 8, 1);
    for (size_t i = 0; i < n; i++) {
        u64 key = key_of(0, i);
        size_t s = (size_t)((key * 0x9E3779B97F4A7C15ULL) >> (64 - lg));
        if (s >= cap) s = 0;
        for (size_t tries = 0; tries < cap; tries++) {
            u64 k = t[2 * s];
            if (k == 0) { t[2 * s] = key; t[2 * s + 1] = i + 1; break; }
            if (k == key) { t[2 * s + 1] += i + 1; break; }
            if (++s == cap) s = 0;
        }
    }
    u64 ck = CK_INIT;
    for (int phase = 0; phase < 2; phase++)
        for (size_t i = 0; i < n; i++) {
            u64 key = key_of(phase ? n : 0, i), v = 0;
            size_t s = (size_t)((key * 0x9E3779B97F4A7C15ULL) >> (64 - lg));
            if (s >= cap) s = 0;
            for (size_t tries = 0; tries < cap; tries++) {
                u64 k = t[2 * s];
                if (k == 0) break;
                if (k == key) { v = t[2 * s + 1]; break; }
                if (++s == cap) s = 0;
            }
            ck = ckfold(ck, v);
        }
    free(t);
    return ck;
}

int main(int argc, char **argv) {
    if (argc != 3) { fprintf(stderr, "usage: %s <sort|sieve|matmul|lines|hashmap> <n>\n", argv[0]); return 2; }
    size_t n = (size_t)strtoull(argv[2], NULL, 10);
    u64 ck;
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
    printf("RESULT workload=%s n=%llu checksum=%016llx time_ns=%llu\n", argv[1], (unsigned long long)n,
           (unsigned long long)ck, (unsigned long long)ns);
    return 0;
}
