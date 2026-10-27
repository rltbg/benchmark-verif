//! strsort : tri fusion stable d'indices de chaînes stockées dans un pool contigu (spec/workloads.d/strsort.md).
const std = @import("std");
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
inline fn lenOf(i: u64) u64 {
    return 1 + (mix64(0xC0FFEE0000000001 +% i) >> 59);
}

inline fn le(pool: []const u8, off: []const u64, a: u32, b: u32) bool {
    const pa = pool[off[a]..];
    const pb = pool[off[b]..];
    const la = off[a + 1] - off[a];
    const lb = off[b + 1] - off[b];
    const m = if (la < lb) la else lb;
    var k: usize = 0;
    while (k < m) : (k += 1) {
        if (pa[k] != pb[k]) return pa[k] < pb[k];
    }
    return la <= lb;
}

fn msort(pool: []const u8, off: []const u64, a: []u32, tmp: []u32, lo: usize, hi: usize) void {
    if (hi - lo < 2) return;
    const mid = lo + (hi - lo) / 2;
    msort(pool, off, a, tmp, lo, mid);
    msort(pool, off, a, tmp, mid, hi);
    var i = lo;
    var j = mid;
    var k = lo;
    while (k < hi) : (k += 1) {
        if (i < mid and (j >= hi or le(pool, off, a[i], a[j]))) {
            tmp[k] = a[i];
            i += 1;
        } else {
            tmp[k] = a[j];
            j += 1;
        }
    }
    @memcpy(a[lo..hi], tmp[lo..hi]);
}

pub fn run(n: usize, threads: usize) u64 {
    _ = threads;
    const off = alloc.alloc(u64, n + 1) catch std.process.exit(3);
    defer alloc.free(off);
    off[0] = 0;
    for (0..n) |i| off[i + 1] = off[i] + lenOf(i);
    const total = off[n];
    const pool = alloc.alloc(u8, total) catch std.process.exit(3);
    defer alloc.free(pool);
    for (0..n) |i| {
        const l = off[i + 1] - off[i];
        var k: u64 = 0;
        while (k < l) : (k += 1) {
            pool[off[i] + k] = @as(u8, 'a') + @as(u8, @intCast(mix64(0xC0FFEE0000000002 +% 64 *% @as(u64, i) +% k) & 3));
        }
    }
    const a = alloc.alloc(u32, n) catch std.process.exit(3);
    const tmp = alloc.alloc(u32, n) catch std.process.exit(3);
    defer alloc.free(a);
    defer alloc.free(tmp);
    for (a, 0..) |*x, i| x.* = @intCast(i);
    msort(pool, off, a, tmp, 0, n);
    var ck: u64 = 0xCBF29CE484222325;
    for (a) |x| ck = ckfold(ck, x);
    return ckfold(ck, total);
}
