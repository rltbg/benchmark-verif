//! parsort : segments triés en parallèle puis fusions par paires (spec/workloads.d/parsort.md).
const std = @import("std");
const pm = @import("w_parmatmul.zig");
const alloc = std.heap.c_allocator;

fn merge(src: []const u32, dst: []u32, lo: usize, mid: usize, hi: usize) void {
    var i = lo;
    var j = mid;
    var k = lo;
    while (k < hi) : (k += 1) {
        if (i < mid and (j >= hi or src[i] <= src[j])) {
            dst[k] = src[i];
            i += 1;
        } else {
            dst[k] = src[j];
            j += 1;
        }
    }
}
fn msort(a: []u32, tmp: []u32, lo: usize, hi: usize) void {
    if (hi - lo < 2) return;
    const mid = lo + (hi - lo) / 2;
    msort(a, tmp, lo, mid);
    msort(a, tmp, mid, hi);
    merge(a, tmp, lo, mid, hi);
    @memcpy(a[lo..hi], tmp[lo..hi]);
}

const Ctx = struct { a: []u32, tmp: []u32, n: usize, T: usize, b: []usize, pairs: usize };
fn sortWorker(x: *const Ctx, t: usize) void {
    msort(x.a, x.tmp, x.n * t / x.T, x.n * (t + 1) / x.T);
}
fn mergeWorker(x: *const Ctx, t: usize) void {
    if (t >= x.pairs) return;
    const lo = x.b[2 * t];
    const mid = x.b[2 * t + 1];
    const hi = x.b[2 * t + 2];
    merge(x.a, x.tmp, lo, mid, hi);
    @memcpy(x.a[lo..hi], x.tmp[lo..hi]);
}

pub fn run(n: usize, threads: usize) u64 {
    const T: usize = @min(@max(threads, 1), 64);
    const a = alloc.alloc(u32, n) catch @panic("oom");
    const tmp = alloc.alloc(u32, n) catch @panic("oom");
    defer alloc.free(a);
    defer alloc.free(tmp);
    for (a, 0..) |*x, i| x.* = @truncate(mixer(0x0123456789ABCDEF +% i) >> 32);
    var bounds: [130]usize = undefined;
    var ctx = Ctx{ .a = a, .tmp = tmp, .n = n, .T = T, .b = bounds[0..], .pairs = 0 };
    pm.parDo(T, sortWorker, &ctx);
    var r: usize = T;
    for (0..T + 1) |t| bounds[t] = n * t / T;
    while (r > 1) {
        ctx.pairs = r / 2;
        pm.parDo(ctx.pairs, mergeWorker, &ctx);
        var nr: usize = 0;
        var i: usize = 0;
        while (i + 1 < r) : (i += 2) {
            bounds[nr] = bounds[i];
            nr += 1;
        }
        if (r % 2 == 1) {
            bounds[nr] = bounds[r - 1];
            nr += 1;
        }
        bounds[nr] = n;
        r = nr;
    }
    var ck: u64 = 0xCBF29CE484222325;
    var prev: u32 = 0;
    var ok = true;
    for (a) |x| {
        ok = ok and prev <= x;
        prev = x;
        ck = (ck ^ x) *% 0x100000001B3;
    }
    return if (ok) ck else 0;
}

inline fn mixer(x: u64) u64 {
    var z = x +% 0x9E3779B97F4A7C15;
    z = (z ^ (z >> 30)) *% 0xBF58476D1CE4E5B9;
    z = (z ^ (z >> 27)) *% 0x94D049BB133111EB;
    return z ^ (z >> 31);
}
