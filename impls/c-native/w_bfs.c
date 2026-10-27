/* bfs : graphe creux aléatoire (m = 16 n arêtes), CSR par tri par comptage, puis BFS depuis 0. Voir spec/workloads.d/bfs.md. */
#define BFS_SEED 0xB5F0000000000001ULL
static u64 run_bfs(size_t n, int threads) {
    (void)threads;
    size_t m = 16 * n;
    u32 *off = calloc(n + 1, sizeof(u32));
    u32 *adj = malloc(m * sizeof(u32));
    u32 *cur = malloc(n * sizeof(u32));
    if (!off || !adj || !cur) { fprintf(stderr, "oom\n"); exit(3); }
    for (size_t j = 0; j < m; j++) off[mix64(BFS_SEED + 2 * j) % n + 1]++;
    for (size_t i = 1; i <= n; i++) off[i] += off[i - 1];
    memcpy(cur, off, n * sizeof(u32));
    for (size_t j = 0; j < m; j++) {
        u64 u = mix64(BFS_SEED + 2 * j) % n, v = mix64(BFS_SEED + 2 * j + 1) % n;
        adj[cur[u]++] = (u32)v;
    }
    free(cur);
    u32 *dist = malloc(n * sizeof(u32));
    u32 *q = malloc(n * sizeof(u32));
    if (!dist || !q) { fprintf(stderr, "oom\n"); exit(3); }
    memset(dist, 0xFF, n * sizeof(u32));
    size_t head = 0, tail = 1;
    dist[0] = 0; q[0] = 0;
    while (head < tail) {
        u32 u = q[head++], du = dist[u] + 1;
        for (u32 e = off[u]; e < off[u + 1]; e++) {
            u32 v = adj[e];
            if (dist[v] == 0xFFFFFFFFu) { dist[v] = du; q[tail++] = v; }
        }
    }
    u64 ck = CK_INIT, maxd = 0;
    for (size_t i = 0; i < n; i++) {
        ck = ckfold(ck, dist[i]);
        if (dist[i] != 0xFFFFFFFFu && dist[i] > maxd) maxd = dist[i];
    }
    ck = ckfold(ck, tail);
    ck = ckfold(ck, maxd);
    free(off); free(adj); free(dist); free(q);
    return ck;
}
