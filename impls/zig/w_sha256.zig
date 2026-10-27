//! Workload sha256 : SHA-256 d'un message de n octets généré à la volée (spec/workloads.d/sha256.md).
const std = @import("std");

const K = [64]u32{
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be,
    0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa,
    0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85,
    0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
    0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f,
    0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
};
const seed: u64 = 0x5348413235362D42;

inline fn mix64(x: u64) u64 {
    var z = x +% 0x9E3779B97F4A7C15;
    z = (z ^ (z >> 30)) *% 0xBF58476D1CE4E5B9;
    z = (z ^ (z >> 27)) *% 0x94D049BB133111EB;
    return z ^ (z >> 31);
}
inline fn ckfold(ck: u64, v: u64) u64 {
    return (ck ^ v) *% 0x100000001B3;
}
inline fn rotr(x: u32, comptime k: u5) u32 {
    return std.math.rotr(u32, x, k);
}

inline fn compress(h: *[8]u32, m: *const [16]u32) void {
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

fn byteAt(i: u64) u8 {
    return @truncate(mix64(seed +% i / 8) >> @intCast(8 * (i % 8)));
}

pub fn run(n: usize, threads: usize) u64 {
    _ = threads;
    var h = [8]u32{ 0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19 };
    const blocks: u64 = n / 64;
    var m: [16]u32 = undefined;
    var b: u64 = 0;
    while (b < blocks) : (b += 1) {
        inline for (0..8) |k| {
            const x = mix64(seed +% 8 *% b +% k);
            m[2 * k] = @byteSwap(@as(u32, @truncate(x)));
            m[2 * k + 1] = @byteSwap(@as(u32, @truncate(x >> 32)));
        }
        compress(&h, &m);
    }
    var tail: [128]u8 = @splat(0);
    const t = n % 64;
    for (0..t) |j| tail[j] = byteAt(blocks * 64 + j);
    tail[t] = 0x80;
    const total: usize = if (t < 56) 64 else 128;
    const bits: u64 = @as(u64, n) * 8;
    for (0..8) |j| tail[total - 8 + j] = @truncate(bits >> @intCast(56 - 8 * j));
    var off: usize = 0;
    while (off < total) : (off += 64) {
        for (0..16) |i| {
            const p = tail[off + 4 * i ..][0..4];
            m[i] = (@as(u32, p[0]) << 24) | (@as(u32, p[1]) << 16) | (@as(u32, p[2]) << 8) | p[3];
        }
        compress(&h, &m);
    }
    var ck: u64 = 0xCBF29CE484222325;
    for (h) |x| ck = ckfold(ck, x);
    return ck;
}
