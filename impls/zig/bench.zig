//! Implémentation Zig 0.17 écrite à la main, NON vérifiée. Spécification : spec/WORKLOADS.md.
//! Compilée en ReleaseFast (sans contrôles de bornes ni débordements : c'est le mode de performance de Zig).
//! libc n'est utilisée que pour argv, l'horloge et printf.
const std = @import("std");

extern "c" fn printf(fmt: [*:0]const u8, ...) c_int;
extern "c" fn strtoull(s: [*:0]const u8, end: ?*anyopaque, base: c_int) u64;
extern "c" fn clock_gettime(clk: c_int, ts: *Timespec) c_int;
const Timespec = extern struct { sec: i64, nsec: i64 };

const alloc = std.heap.c_allocator;

inline fn mix64(x: u64) u64 {
    var z = x +% 0x9E3779B97F4A7C15;
    z = (z ^ (z >> 30)) *% 0xBF58476D1CE4E5B9;
    z = (z ^ (z >> 27)) *% 0x94D049BB133111EB;
    return z ^ (z >> 31);
}
const ck_init: u64 = 0xCBF29CE484222325;
inline fn ckfold(ck: u64, v: u64) u64 {
    return (ck ^ v) *% 0x100000001B3;
}

fn oom() noreturn {
    _ = printf("oom\n");
    std.process.exit(3);
}

// ---------- sort ----------
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
fn runSort(n: usize) u64 {
    const a = alloc.alloc(u32, n) catch oom();
    const tmp = alloc.alloc(u32, n) catch oom();
    defer alloc.free(a);
    defer alloc.free(tmp);
    for (a, 0..) |*x, i| x.* = @truncate(mix64(0x0123456789ABCDEF +% i) >> 32);
    msort(a, tmp, 0, n);
    var ck = ck_init;
    var prev: u32 = 0;
    var ok = true;
    for (a) |x| {
        ok = ok and prev <= x;
        prev = x;
        ck = ckfold(ck, x);
    }
    return if (ok) ck else 0;
}

// ---------- sieve ----------
fn runSieve(n: usize) u64 {
    const f = alloc.alloc(u8, n + 1) catch oom();
    defer alloc.free(f);
    @memset(f, 0);
    var i: usize = 2;
    while (i <= n / i) : (i += 1) {
        if (f[i] == 0) {
            var j = i * i;
            while (j <= n) : (j += i) f[j] = 1;
        }
    }
    var ck = ck_init;
    var cnt: u64 = 0;
    i = 2;
    while (i <= n) : (i += 1) {
        if (f[i] == 0) {
            ck = ckfold(ck, i);
            cnt += 1;
        }
    }
    return ckfold(ck, cnt);
}

// ---------- matmul ----------
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
fn runMatmul(n: usize) u64 {
    const nn = n * n;
    const a = alloc.alloc(u32, nn) catch oom();
    const b = alloc.alloc(u32, nn) catch oom();
    const c = alloc.alloc(u32, nn) catch oom();
    defer alloc.free(a);
    defer alloc.free(b);
    defer alloc.free(c);
    @memset(c, 0);
    for (a, 0..) |*x, i| x.* = @truncate(mix64(0x0FEDCBA987654321 +% i) >> 32);
    for (b, 0..) |*x, i| x.* = @truncate(mix64(0x0FEDCBA987654321 +% nn +% i) >> 32);
    for (0..n) |i| {
        for (0..n) |k| {
            const aik = a[i * n + k];
            const brow = b[k * n .. k * n + n];
            const crow = c[i * n .. i * n + n];
            axpy(crow, brow, aik);
        }
    }
    var ck = ck_init;
    for (c) |x| ck = ckfold(ck, x);
    return ck;
}

// ---------- lines ----------
fn putDec(p: []u8, v0: u32) usize {
    var tmp: [10]u8 = undefined;
    var l: usize = 0;
    var v = v0;
    while (true) {
        tmp[l] = '0' + @as(u8, @intCast(v % 10));
        l += 1;
        v /= 10;
        if (v == 0) break;
    }
    for (0..l) |i| p[i] = tmp[l - 1 - i];
    return l;
}
fn runLines(n: usize) u64 {
    const txt = alloc.alloc(u8, n * 15 + 20) catch oom();
    defer alloc.free(txt);
    var len: usize = 0;
    for (0..n) |i| {
        const r = mix64(0x1357924680ACEBDF +% i);
        const key: u32 = @as(u32, @truncate(r >> 44)) & 0xFFFFF;
        const val: u32 = @intCast(((r >> 8) & 0xFFFFFFFF) % 1000000);
        len += putDec(txt[len..], key);
        txt[len] = ';';
        len += 1;
        len += putDec(txt[len..], val);
        txt[len] = '\n';
        len += 1;
    }
    const tab = alloc.alloc(u64, 2097152) catch oom();
    defer alloc.free(tab);
    @memset(tab, 0);
    var in_val = false;
    var key: u32 = 0;
    var vl: u32 = 0;
    for (txt[0..len]) |c| {
        const d: u32 = c -% '0';
        if (in_val) {
            if (c == '\n') {
                const k: usize = key & 0xFFFFF;
                tab[2 * k] +%= vl;
                tab[2 * k + 1] +%= 1;
                in_val = false;
                key = 0;
                vl = 0;
            } else vl = vl *% 10 +% d;
        } else if (c == ';') {
            in_val = true;
        } else key = key *% 10 +% d;
    }
    var ck = ck_init;
    for (0..1048576) |k| ck = ckfold(ckfold(ck, tab[2 * k]), tab[2 * k + 1]);
    return ck;
}

// ---------- hashmap ----------
inline fn keyOf(off: u64, i: u64) u64 {
    return mix64(0x2468ACE013579BDF +% off +% i) | 1;
}
fn runHashmap(n: usize) u64 {
    var cap: usize = 2;
    var lg: u7 = 1;
    while (cap < 2 * n and lg < 27 and cap <= 33554432) {
        cap *= 2;
        lg += 1;
    }
    const t = alloc.alloc(u64, 2 * cap) catch oom();
    defer alloc.free(t);
    @memset(t, 0);
    for (0..n) |i| {
        const key = keyOf(0, i);
        var s: usize = @intCast((key *% 0x9E3779B97F4A7C15) >> @as(u6, @intCast(64 - lg)));
        if (s >= cap) s = 0;
        var tries: usize = 0;
        while (tries < cap) : (tries += 1) {
            const k = t[2 * s];
            if (k == 0) {
                t[2 * s] = key;
                t[2 * s + 1] = i + 1;
                break;
            }
            if (k == key) {
                t[2 * s + 1] +%= i + 1;
                break;
            }
            s += 1;
            if (s == cap) s = 0;
        }
    }
    var ck = ck_init;
    for (0..2) |phase| {
        for (0..n) |i| {
            const key = keyOf(if (phase == 1) n else 0, i);
            var v: u64 = 0;
            var s: usize = @intCast((key *% 0x9E3779B97F4A7C15) >> @as(u6, @intCast(64 - lg)));
            if (s >= cap) s = 0;
            var tries: usize = 0;
            while (tries < cap) : (tries += 1) {
                const k = t[2 * s];
                if (k == 0) break;
                if (k == key) {
                    v = t[2 * s + 1];
                    break;
                }
                s += 1;
                if (s == cap) s = 0;
            }
            ck = ckfold(ck, v);
        }
    }
    return ck;
}

pub export fn main(argc: c_int, argv: [*][*:0]u8) c_int {
    if (argc != 3) {
        _ = printf("usage: bench <sort|sieve|matmul|lines|hashmap> <n>\n");
        return 2;
    }
    const w = std.mem.span(argv[1]);
    const n: usize = @intCast(strtoull(argv[2], null, 10));
    var t0: Timespec = undefined;
    var t1: Timespec = undefined;
    _ = clock_gettime(1, &t0); // CLOCK_MONOTONIC
    const ck: u64 = if (std.mem.eql(u8, w, "sort")) runSort(n) else if (std.mem.eql(u8, w, "sieve")) runSieve(n) else if (std.mem.eql(u8, w, "matmul")) runMatmul(n) else if (std.mem.eql(u8, w, "lines")) runLines(n) else if (std.mem.eql(u8, w, "hashmap")) runHashmap(n) else {
        _ = printf("workload inconnu\n");
        return 2;
    };
    _ = clock_gettime(1, &t1);
    const ns: u64 = @intCast((t1.sec - t0.sec) * 1_000_000_000 + (t1.nsec - t0.nsec));
    _ = printf("RESULT workload=%s n=%llu checksum=%016llx time_ns=%llu\n", argv[1], @as(u64, n), ck, ns);
    return 0;
}
