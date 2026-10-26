/* Driver C pour les modules extraits par KaRaMeL. Même interface CLI que les autres
 * implémentations ; aucune logique de benchmark ici, seulement le chronométrage. */
#define _POSIX_C_SOURCE 200809L
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "Bench_Sort.h"
#include "Bench_Sieve.h"
#include "Bench_Matmul.h"
#include "Bench_Lines.h"
#include "Bench_Hashmap.h"

int main(int argc, char **argv) {
    if (argc != 3) { fprintf(stderr, "usage: %s <sort|sieve|matmul|lines|hashmap> <n>\n", argv[0]); return 2; }
    unsigned long long n64 = strtoull(argv[2], NULL, 10);
    if (n64 > 0xFFFFFFFFULL) { fprintf(stderr, "n doit tenir dans un u32\n"); return 2; }
    uint32_t n = (uint32_t)n64;
    uint64_t ck;
    struct timespec t0, t1;
    clock_gettime(CLOCK_MONOTONIC, &t0);
    if (!strcmp(argv[1], "sort")) ck = Bench_Sort_run(n);
    else if (!strcmp(argv[1], "sieve")) ck = Bench_Sieve_run(n);
    else if (!strcmp(argv[1], "matmul")) ck = Bench_Matmul_run(n);
    else if (!strcmp(argv[1], "lines")) ck = Bench_Lines_run(n);
    else if (!strcmp(argv[1], "hashmap")) ck = Bench_Hashmap_run(n);
    else { fprintf(stderr, "workload inconnu\n"); return 2; }
    clock_gettime(CLOCK_MONOTONIC, &t1);
    uint64_t ns = (uint64_t)(t1.tv_sec - t0.tv_sec) * 1000000000ULL + (uint64_t)(t1.tv_nsec - t0.tv_nsec);
    printf("RESULT workload=%s n=%llu checksum=%016llx time_ns=%llu\n", argv[1],
           (unsigned long long)n, (unsigned long long)ck, (unsigned long long)ns);
    return 0;
}
