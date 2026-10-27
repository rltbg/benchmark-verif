//! parsha : T segments SHA-256 indépendants, un par thread (spec/workloads.d/parsha.md).
const std = @import("std");
const pm = @import("w_parmatmul.zig");

const K = [64]u32{
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be,
    0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa,
    0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85,
    0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
    0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f,
    0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
};

inline fn rotr(x: u32, comptime k: u5) u32 {
    return std.math.rotr(u32, x, k);
}

fn compress(h: *[8]u32, m: *const [16]u32) void {
    var w: [64]u32 = undefined;
    @memcpy(w[0..16], m);
    var t: usize = 16;
    while (t < 64) : (t += 1) {
        const s0 = rotr(w[t - 15], 7) ^ rotr(w[t - 15], 18) ^ (w[t - 15] >> 3);
        const s1 = rotr(w[t - 2], 17) ^ rotr(w[t - 2], 19) ^ (w[t - 2] >> 10);
        w[t] = w[t - 16] +% s0 +% w[t - 7] +% s1;
    }
    var a = h[0];
    var b = h[1];
    var c = h[2];
    var d = h[3];
    var e = h[4];
    var f = h[5];
    var g = h[6];
    var hh = h[7];
    t = 0;
    while (t < 64) : (t += 1) {
        const t1 = hh +% (rotr(e, 6) ^ rotr(e, 11) ^ rotr(e, 25)) +% ((e & f) ^ (~e & g)) +% K[t] +% w[t];
        const t2 = (rotr(a, 2) ^ rotr(a, 13) ^ rotr(a, 22)) +% ((a & b) ^ (a & c) ^ (b & c));
        hh = g;
        g = f;
        f = e;
        e = d +% t1;
        d = c;
        c = b;
        b = a;
        a = t1 +% t2;
    }
    h[0] +%= a;
    h[1] +%= b;
    h[2] +%= c;
    h[3] +%= d;
    h[4] +%= e;
    h[5] +%= f;
    h[6] +%= g;
    h[7] +%= hh;
}

inline fn mix64(x: u64) u64 {
    var z = x +% 0x9E3779B97F4A7C15;
    z = (z ^ (z >> 30)) *% 0xBF58476D1CE4E5B9;
    z = (z ^ (z >> 27)) *% 0x94D049BB133111EB;
    return z ^ (z >> 31);
}

/// Noyau séquentiel : condensé SHA-256 des m octets (m multiple de 64) du segment de base `seedbase`.
fn segment(seedbase: u64, m: usize, h: *[8]u32) void {
    h.* = .{ 0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19 };
    var blk: [16]u32 = undefined;
    var b: u64 = 0;
    while (b < m / 64) : (b += 1) {
        for (0..8) |k| {
            const x = mix64(seedbase +% 8 *% b +% k);
            blk[2 * k] = @byteSwap(@as(u32, @truncate(x)));
            blk[2 * k + 1] = @byteSwap(@as(u32, @truncate(x >> 32)));
        }
        compress(h, &blk);
    }
    @memset(&blk, 0);
    blk[0] = 0x80000000;
    const bits: u64 = @as(u64, m) * 8;
    blk[14] = @truncate(bits >> 32);
    blk[15] = @truncate(bits);
    compress(h, &blk);
}

const Ctx = struct { m: usize, h: *[64][8]u32 };
fn worker(x: *const Ctx, t: usize) void {
    segment(0x5348413235362D42 +% (@as(u64, t) << 40), x.m, &x.h[t]);
}

pub fn run(n: usize, threads: usize) u64 {
    const T: usize = @min(@max(threads, 1), 64);
    var h: [64][8]u32 = undefined;
    const ctx = Ctx{ .m = n / T, .h = &h };
    pm.parDo(T, worker, &ctx);
    var ck: u64 = 0xCBF29CE484222325;
    for (0..T) |s| for (h[s]) |w| {
        ck = (ck ^ w) *% 0x100000001B3;
    };
    return ck;
}
