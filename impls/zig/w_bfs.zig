//! bfs : graphe creux aléatoire (m = 16 n arêtes), CSR par tri par comptage, puis BFS depuis 0. Voir spec/workloads.d/bfs.md.
const std = @import("std");

const seed: u64 = 0xB5F0000000000001;
const inf: u32 = 0xFFFFFFFF;
const alloc = std.heap.c_allocator;

inline fn mix64(x: u64) u64 {
    var z = x +% 0x9E3779B97F4A7C15;
    z = (z ^ (z >> 30)) *% 0xBF58476D1CE4E5B9;
    z = (z ^ (z >> 27)) *% 0x94D049BB133111EB;
    return z ^ (z >> 31);
}
inline fn ckfold(ck: u64, v: u64) u64 {
    return (ck ^ v) *% 0x100000001B3;
}

pub fn run(n: usize, threads: usize) u64 {
    _ = threads;
    const m = 16 * n;
    const nn: u64 = n;
    const off = alloc.alloc(u32, n + 1) catch @panic("oom");
    const adj = alloc.alloc(u32, m) catch @panic("oom");
    const cur = alloc.alloc(u32, n) catch @panic("oom");
    @memset(off, 0);
    for (0..m) |j| off[mix64(seed +% 2 *% j) % nn + 1] += 1;
    for (1..n + 1) |i| off[i] += off[i - 1];
    @memcpy(cur, off[0..n]);
    for (0..m) |j| {
        const u: usize = @intCast(mix64(seed +% 2 *% j) % nn);
        const v: u32 = @intCast(mix64(seed +% 2 *% j +% 1) % nn);
        adj[cur[u]] = v;
        cur[u] += 1;
    }
    alloc.free(cur);
    const dist = alloc.alloc(u32, n) catch @panic("oom");
    const q = alloc.alloc(u32, n) catch @panic("oom");
    @memset(dist, inf);
    var head: usize = 0;
    var tail: usize = 1;
    dist[0] = 0;
    q[0] = 0;
    while (head < tail) {
        const u = q[head];
        head += 1;
        const du = dist[u] + 1;
        var e = off[u];
        const end = off[u + 1];
        while (e < end) : (e += 1) {
            const v = adj[e];
            if (dist[v] == inf) {
                dist[v] = du;
                q[tail] = v;
                tail += 1;
            }
        }
    }
    var ck: u64 = 0xCBF29CE484222325;
    var maxd: u64 = 0;
    for (dist) |d| {
        ck = ckfold(ck, d);
        if (d != inf and d > maxd) maxd = d;
    }
    ck = ckfold(ck, tail);
    ck = ckfold(ck, maxd);
    alloc.free(off);
    alloc.free(adj);
    alloc.free(dist);
    alloc.free(q);
    return ck;
}
