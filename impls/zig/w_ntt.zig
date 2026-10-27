//! Workload `ntt` (voir spec/workloads.d/ntt.md).
const std = @import("std");

const P: u64 = 2013265921;
const G: u64 = 31;
const ck_init: u64 = 0xCBF29CE484222325;

inline fn mix64(x: u64) u64 {
    var z = x +% 0x9E3779B97F4A7C15;
    z = (z ^ (z >> 30)) *% 0xBF58476D1CE4E5B9;
    z = (z ^ (z >> 27)) *% 0x94D049BB133111EB;
    return z ^ (z >> 31);
}
inline fn ckfold(ck: u64, v: u64) u64 {
    return (ck ^ v) *% 0x100000001B3;
}

fn powmod(b0: u64, e0: u64) u64 {
    var b = b0;
    var e = e0;
    var r: u64 = 1;
    while (e > 0) {
        if (e % 2 == 1) r = r * b % P;
        b = b * b % P;
        e /= 2;
    }
    return r;
}

fn transform(a: []u32, tw: []const u32, n: usize, k: usize) void {
    for (0..n) |i| {
        var x = i;
        var rev: usize = 0;
        for (0..k) |_| {
            rev = rev * 2 + x % 2;
            x /= 2;
        }
        if (i < rev) std.mem.swap(u32, &a[i], &a[rev]);
    }
    var len: usize = 2;
    while (len <= n) : (len *= 2) {
        const half = len / 2;
        const stride = n / len;
        for (0..stride) |blk| {
            const start = blk * len;
            for (0..half) |j| {
                const u: u64 = a[start + j];
                const v: u64 = @as(u64, a[start + j + half]) * tw[j * stride] % P;
                a[start + j] = @intCast((u + v) % P);
                a[start + j + half] = @intCast((u + P - v) % P);
            }
        }
    }
}

pub fn run(n: usize, threads: usize) u64 {
    _ = threads;
    if (n > (@as(usize, 1) << 27)) return 0;
    var pw: usize = 1;
    var k: usize = 0;
    while (pw < n) {
        pw *= 2;
        k += 1;
    }
    if (pw != n) return 0;
    const alloc = std.heap.c_allocator;
    const a = alloc.alloc(u32, n) catch std.process.exit(3);
    defer alloc.free(a);
    const tw = alloc.alloc(u32, n / 2 + 1) catch std.process.exit(3);
    defer alloc.free(tw);
    for (a, 0..) |*x, i| x.* = @intCast(mix64(0xA5A5F00D12345678 +% i) % P);
    const w = powmod(G, (P - 1) / n);
    var c: u64 = 1;
    for (0..n / 2) |j| {
        tw[j] = @intCast(c);
        c = c * w % P;
    }
    transform(a, tw, n, k);
    for (a) |*x| x.* = @intCast(@as(u64, x.*) * x.* % P);
    transform(a, tw, n, k);
    var i: usize = 1;
    while (2 * i < n) : (i += 1) std.mem.swap(u32, &a[i], &a[n - i]);
    const ninv = powmod(n, P - 2);
    var ck = ck_init;
    for (a) |x| ck = ckfold(ck, @as(u64, x) * ninv % P);
    return ck;
}
