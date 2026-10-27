/* Pilote du workload `stream` pour le C extrait : lit /var/tmp/bench-data/stream-<n>.json (ou $BENCH_DATA_DIR) par blocs de 1 Mio avec
 * read(2) et appelle le noyau Pulse `Bench_Stream_chunk` (CRC-32 + tokeniseur JSON). Non prouvé : lecture, boucle, ouverture du fichier. */
#include <fcntl.h>
#include <unistd.h>

static uint64_t drv_stream(size_t n_mio) {
    const char *dir = getenv("BENCH_DATA_DIR");
    char path[512];
    snprintf(path, sizeof path, "%s/stream-%zu.json", dir ? dir : "/var/tmp/bench-data", n_mio);
    int fd = open(path, O_RDONLY);
    if (fd < 0) { perror(path); exit(3); }
    uint8_t *buf = Bench_Stream_new_buf();
    uint64_t *st = Bench_Stream_new_state();
    uint32_t *tab = Bench_Stream_new_table();
    for (;;) {
        ssize_t got = read(fd, buf, 1u << 20);
        if (got < 0) { perror("read"); exit(3); }
        if (got == 0) break;
        Bench_Stream_chunk(buf, (size_t)got, st, tab);
    }
    close(fd);
    return Bench_Stream_finish(st);
}
