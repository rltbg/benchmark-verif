/* stream : CRC-32 + tokeniseur JSON-lines sur un fichier lu par blocs de 1 Mio (spec/workloads.d/stream.md). */
#include <fcntl.h>
#include <unistd.h>

static u64 run_stream(size_t n_mio, int threads) {
    (void)threads;
    const char *dir = getenv("BENCH_DATA_DIR");
    char path[512];
    snprintf(path, sizeof path, "%s/stream-%zu.json", dir ? dir : "/var/tmp/bench-data", n_mio);
    int fd = open(path, O_RDONLY);
    if (fd < 0) { perror(path); exit(3); }
    static u32 tab[256];
    for (u32 i = 0; i < 256; i++) {
        u32 x = i;
        for (int k = 0; k < 8; k++) x = (x & 1) ? (x >> 1) ^ 0xEDB88320u : x >> 1;
        tab[i] = x;
    }
    unsigned char *buf = malloc(1u << 20);
    if (!buf) { perror("malloc"); exit(3); }
    u32 crc = 0xFFFFFFFFu;
    u64 mode = 0, depth = 0, maxd = 0, objs = 0, strs = 0, nums = 0, arrs = 0, lits = 0, lines = 0;
    u64 idsum = 0, allsum = 0, acc = 0, total = 0;
    int isint = 0, gotid = 0;
    for (;;) {
        ssize_t got = read(fd, buf, 1u << 20);
        if (got < 0) { perror("read"); exit(3); }
        if (got == 0) break;
        total += (u64)got;
        for (ssize_t i = 0; i < got; i++) {
            unsigned c = buf[i];
            crc = tab[(crc ^ c) & 0xFF] ^ (crc >> 8);
            int dg = c - '0' < 10u;
            int lo = c - 'a' < 26u;
            if (mode == 3 && !(dg || c == '.' || c == 'e' || c == 'E' || c == '+' || c == '-')) {
                nums++;
                if (isint) { allsum += acc; if (depth == 1 && !gotid) idsum += acc; }
                if (depth == 1) gotid = 1;
                mode = 0;
            } else if (mode == 4 && !lo) {
                lits++;
                mode = 0;
            }
            switch (mode) {
            case 2: mode = 1; break;
            case 1: mode = c == '\\' ? 2 : c == '"' ? 0 : 1; break;
            case 3: if (dg) acc = acc * 10 + (c - '0'); else isint = 0; break;
            case 4: break;
            default:
                if (c == '"') { strs++; mode = 1; }
                else if (c == '{') { objs++; if (++depth > maxd) maxd = depth; }
                else if (c == '[') { arrs++; if (++depth > maxd) maxd = depth; }
                else if (c == '}' || c == ']') { if (depth) depth--; }
                else if (c == '\n') { lines++; gotid = 0; }
                else if (dg) { mode = 3; acc = c - '0'; isint = 1; }
                else if (c == '-') { mode = 3; acc = 0; isint = 1; }
                else if (lo) mode = 4;
            }
        }
    }
    close(fd); free(buf);
    u64 ck = CK_INIT;
    ck = ckfold(ck, crc ^ 0xFFFFFFFFu);
    ck = ckfold(ck, objs); ck = ckfold(ck, strs); ck = ckfold(ck, nums); ck = ckfold(ck, arrs); ck = ckfold(ck, lits);
    ck = ckfold(ck, maxd); ck = ckfold(ck, lines); ck = ckfold(ck, idsum); ck = ckfold(ck, allsum); ck = ckfold(ck, total);
    return ck;
}
