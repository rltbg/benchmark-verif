//! parmatmul : produit matriciel réparti en blocs de lignes sur T threads (spec/workloads.d/parmatmul.md).
//! Définit aussi `parDo`, utilisé par w_parsort.zig et w_parsha.zig.
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

/// Exécute `func(ctx, t)` pour t = 0..T-1 : le thread appelant fait t = 0, les autres sont des std.Thread.
pub fn parDo(threads: usize, comptime func: anytype, ctx: anytype) void {
    const T: usize = @min(@max(threads, 1), 64);
    var th: [64]std.Thread = undefined;
    var spawned: usize = 1;
    while (spawned < T) : (spawned += 1) {
        th[spawned] = std.Thread.spawn(.{}, func, .{ ctx, spawned }) catch @panic("thread");
    }
    func(ctx, 0);
    var t: usize = 1;
    while (t < T) : (t += 1) th[t].join();
}

fn axpy(noalias crow: []u32, noalias brow: []const u32, aik: u32) void {
    const V = 16;
    const Vec = @Vector(V, u32);
    const av: Vec = @splat(aik);
    var j: usize = 0;
    while (j + V <= crow.len) : (j += V) {
        const bv: Vec = brow[j..][0..V].*;
        const cv: Vec = crow[j..][0..V].*;
        crow[j..][0..V].* = cv +% av *% bv;
    }
    while (j < crow.len) : (j += 1) crow[j] +%= aik *% brow[j];
}

/// Noyau séquentiel : lignes ra..rb de C = A*B.
fn rows(a: []const u32, b: []const u32, c: []u32, n: usize, ra: usize, rb: usize) void {
    for (ra..rb) |i| {
        for (0..n) |k| {
            const aik = a[i * n + k];
            axpy(c[i * n .. i * n + n], b[k * n .. k * n + n], aik);
        }
    }
}

const Ctx = struct { a: []const u32, b: []const u32, c: []u32, n: usize, T: usize };
fn worker(x: *const Ctx, t: usize) void {
    rows(x.a, x.b, x.c, x.n, x.n * t / x.T, x.n * (t + 1) / x.T);
}

pub fn run(n: usize, threads: usize) u64 {
    const nn = n * n;
    const a = alloc.alloc(u32, nn) catch @panic("oom");
    const b = alloc.alloc(u32, nn) catch @panic("oom");
    const c = alloc.alloc(u32, nn) catch @panic("oom");
    defer alloc.free(a);
    defer alloc.free(b);
    defer alloc.free(c);
    @memset(c, 0);
    for (a, 0..) |*x, i| x.* = @truncate(mix64(0x0FEDCBA987654321 +% i) >> 32);
    for (b, 0..) |*x, i| x.* = @truncate(mix64(0x0FEDCBA987654321 +% nn +% i) >> 32);
    const T = @max(threads, 1);
    const ctx = Ctx{ .a = a, .b = b, .c = c, .n = n, .T = T };
    parDo(T, worker, &ctx);
    var ck: u64 = 0xCBF29CE484222325;
    for (c) |x| ck = ckfold(ck, x);
    return ck;
}
