/* Générateur déterministe du fichier d'entrée du workload `stream` (voir spec/workloads.d/stream.md).
 * usage : genstream <n_mio> [répertoire]   -> <répertoire>/stream-<n_mio>.json (défaut /var/tmp/bench-data), exactement n_mio Mio,
 * écrit via un fichier temporaire puis renommé ; ne fait rien si le fichier existe déjà avec la bonne taille. */
#define _GNU_SOURCE
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>
typedef uint64_t u64;
static u64 mix64(u64 x) { u64 z = x + 0x9E3779B97F4A7C15ULL; z = (z ^ (z >> 30)) * 0xBF58476D1CE4E5B9ULL; z = (z ^ (z >> 27)) * 0x94D049BB133111EBULL; return z ^ (z >> 31); }
static const u64 SEED = 0xF11E000000000001ULL;
static u64 ctr = 0;
static u64 draw(void) { return mix64(SEED + ctr++); }
static const char *WORDS[16] = {"alpha","bravo","charlie","delta","echo","foxtrot","golf","hotel","india","juliet","kilo","lima","mike","november","oscar","papa"};

static char *put(char *p, const char *s) { size_t l = strlen(s); memcpy(p, s, l); return p + l; }
static char *dec(char *p, u64 v) { char t[24]; int l = 0; do { t[l++] = '0' + v % 10; v /= 10; } while (v); while (l) *p++ = t[--l]; return p; }
static char *pad6(char *p, u64 v) { for (int k = 5; k >= 0; k--) { p[k] = '0' + v % 10; v /= 10; } return p + 6; }

/* écrit un enregistrement, rend la nouvelle fin */
static char *record(char *p) {
    p = put(p, "{\"id\":"); p = dec(p, draw());
    p = put(p, ",\"name\":\"");
    int k = 1 + (int)(draw() % 3);
    for (int w = 0; w < k; w++) { if (w) *p++ = ' '; p = put(p, WORDS[draw() % 16]); }
    if (draw() % 8 == 0) p = put(p, " \\\"q\\\"");
    p = put(p, "\",\"tags\":[");
    int t = (int)(draw() % 4);
    for (int i = 0; i < t; i++) { if (i) *p++ = ','; *p++ = '"'; *p++ = 't'; p = dec(p, draw() % 1000); *p++ = '"'; }
    p = put(p, "],\"v\":");
    u64 d = draw();
    switch (d % 4) {
    case 0: if ((d >> 8) & 1) *p++ = '-'; p = dec(p, (d >> 16) % 1000000007ULL); break;
    case 1: p = dec(p, (d >> 16) % 10000); *p++ = '.'; p = pad6(p, (d >> 40) % 1000000); break;
    case 2: p = dec(p, (d >> 16) % 1000); *p++ = '.'; p = dec(p, (d >> 30) % 1000); *p++ = 'e'; *p++ = ((d >> 9) & 1) ? '-' : '+'; p = dec(p, (d >> 50) % 30); break;
    default: *p++ = '0'; break;
    }
    p = put(p, ",\"ok\":"); u64 o = draw() % 3; p = put(p, o == 0 ? "true" : o == 1 ? "false" : "null");
    *p++ = '}'; *p++ = '\n';
    return p;
}

int main(int argc, char **argv) {
    if (argc < 2) { fprintf(stderr, "usage: %s <n_mio> [répertoire]\n", argv[0]); return 2; }
    u64 mio = strtoull(argv[1], 0, 10), total = mio << 20;
    const char *dir = argc > 2 ? argv[2] : "/var/tmp/bench-data";
    char path[512], tmp[540];
    snprintf(path, sizeof path, "%s/stream-%llu.json", dir, (unsigned long long)mio);
    snprintf(tmp, sizeof tmp, "%s.tmp.%d", path, (int)getpid());
    struct stat sb;
    if (stat(path, &sb) == 0 && (u64)sb.st_size == total) return 0;
    mkdir(dir, 0755);
    FILE *f = fopen(tmp, "wb");
    if (!f) { perror("fopen"); return 1; }
    size_t cap = 1 << 22; char *buf = malloc(cap + 4096); size_t len = 0; u64 written = 0;
    while (written < total) {
        while (len < cap) len = (size_t)(record(buf + len) - buf);
        size_t w = len; if (written + w > total) w = (size_t)(total - written);
        if (fwrite(buf, 1, w, f) != w) { perror("fwrite"); return 1; }
        written += w;
        len = 0;  /* le fichier = les n Mio premiers octets du flux infini d enregistrements : indépendant de cap */
    }
    fclose(f); rename(tmp, path);
    return 0;
}
