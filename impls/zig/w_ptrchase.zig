//! ptrchase : voir spec/workloads.d/ptrchase.md (arène par indices, BST + recherches + chasse de pointeurs).
const std = @import("std");

const alloc = std.heap.c_allocator;
const NIL: u64 = 0xFFFFFFFF;

inline fn mix64(x: u64) u64 {
    var z = x +% 0x9E3779B97F4A7C15;
    z = (z ^ (z >> 30)) *% 0xBF58476D1CE4E5B9;
    z = (z ^ (z >> 27)) *% 0x94D049BB133111EB;
    return z ^ (z >> 31);
}
inline fn ckfold(ck: u64, v: u64) u64 {
    return (ck ^ v) *% 0x100000001B3;
}
const ck_init: u64 = 0xCBF29CE484222325;

pub fn run(n: usize, threads: usize) u64 {
    _ = threads;
    const a = alloc.alloc(u64, 2 * n) catch std.process.exit(3);
    defer alloc.free(a);
    const next = alloc.alloc(u32, n) catch std.process.exit(3);
    defer alloc.free(next);
    // phase 1 : insertions
    var cnt: u64 = 1;
    var plen: u64 = 0;
    a[0] = mix64(0xD00D000000000001) | 1;
    a[1] = NIL | (NIL << 32);
    var i: u64 = 1;
    while (i < n) : (i += 1) {
        const nk = mix64(0xD00D000000000001 +% i) | 1;
        var cur: u64 = 0;
        var v: u64 = 0;
        while (true) {
            v += 1;
            const key = a[2 * cur];
            if (nk == key) break;
            const lnk = a[2 * cur + 1];
            const left = nk < key;
            const c = if (left) (lnk & 0xFFFFFFFF) else (lnk >> 32);
            if (c < cnt) {
                cur = c;
                continue;
            }
            a[2 * cnt] = nk;
            a[2 * cnt + 1] = NIL | (NIL << 32);
            a[2 * cur + 1] = if (left) ((lnk & ~@as(u64, 0xFFFFFFFF)) | cnt) else ((lnk & 0xFFFFFFFF) | (cnt << 32));
            cnt += 1;
            break;
        }
        plen +%= v;
    }
    const r1 = ckfold(ckfold(ck_init, cnt), plen);
    // phase 2 : recherches
    var r2 = ck_init;
    const total: u64 = n + n / 2;
    var k: u64 = 0;
    while (k < total) : (k += 1) {
        const key = if (k < n) (mix64(0xD00D000000000001 +% (3 * k) % n) | 1) else (mix64(0xD00D000000000003 +% (k - n)) & ~@as(u64, 1));
        var cur: u64 = 0;
        var v: u64 = 0;
        var found: u64 = 0;
        while (v < cnt) {
            v += 1;
            const kc = a[2 * cur];
            if (key == kc) {
                found = 1;
                break;
            }
            const lnk = a[2 * cur + 1];
            const c = if (key < kc) (lnk & 0xFFFFFFFF) else (lnk >> 32);
            if (c < cnt) cur = c else break;
        }
        r2 = ckfold(r2, (v << 1) | found);
    }
    // phase 3 : permutation de Sattolo puis chasse
    for (next, 0..) |*x, idx| x.* = @intCast(idx);
    var ii: u64 = n - 1;
    while (ii >= 1) : (ii -= 1) {
        const j: usize = @intCast(mix64(0xD00D000000000002 +% ii) % ii);
        const t = next[ii];
        next[ii] = next[j];
        next[j] = t;
    }
    var cur: u64 = 0;
    var sum: u64 = 0;
    var h: u64 = 0;
    while (h < 4 * @as(u64, n)) : (h += 1) {
        cur = next[cur];
        sum +%= cur;
    }
    const r3 = ckfold(ckfold(ck_init, cur), sum);
    return ckfold(ckfold(ckfold(ck_init, r1), r2), r3);
}
