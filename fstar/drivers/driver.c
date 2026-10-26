/* Pilote C pour le programme extrait par `fstar.exe --codegen C`. Aucune logique de benchmark ici :
 * analyse des arguments, chronométrage, affichage au format commun (voir spec/WORKLOADS.md). */
#define _POSIX_C_SOURCE 200809L
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include "Custard.h"

int main(int argc, char **argv) {
    if (argc != 3) { fprintf(stderr, "usage: %s <sort|sieve|matmul|lines|hashmap> <n>\n", argv[0]); return 2; }
    size_t n = (size_t)strtoull(argv[2], NULL, 10);
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
