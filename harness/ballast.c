/* Lest mémoire : alloue et verrouille (mlock) N octets pour réduire la RAM disponible des benchmarks, afin de forcer
 * du swap (mode « dépassement de RAM »). Protégé de l'OOM-killer. Usage : ballast <octets> ; écrit "READY" puis dort. */
#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <unistd.h>
int main(int argc, char **argv) {
    if (argc != 2) return 2;
    size_t n = (size_t)strtoull(argv[1], NULL, 10);
    FILE *f = fopen("/proc/self/oom_score_adj", "w"); if (f) { fputs("-1000", f); fclose(f); }
    char *p = mmap(NULL, n, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS | MAP_POPULATE, -1, 0);
    if (p == MAP_FAILED) { perror("mmap"); return 1; }
    if (mlock(p, n)) { perror("mlock"); return 1; }
    memset(p, 0xA5, n);
    puts("READY"); fflush(stdout);
    for (;;) pause();
}
