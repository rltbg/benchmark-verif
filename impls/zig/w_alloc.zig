//! Workload `alloc` (spec/workloads.d/alloc.md) : blocs réels du tas via malloc/free de libc, comme le C ;
//! `doNotOptimizeAway` interdit au compilateur d'éliminer les paires allocation/libération.
const std = @import("std");

extern "c" fn malloc(n: usize) ?*anyopaque;
extern "c" fn calloc(n: usize, m: usize) ?*anyopaque;
extern "c" fn free(p: ?*anyopaque) void;

const wmax: usize = 1 << 25;

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
    var w = n >> 6;
    if (w < 1) w = 1;
    if (w > wmax) w = wmax;
    const ring: [*]?[*]u8 = @ptrCast(@alignCast(calloc(w, @sizeOf(?[*]u8)) orelse @panic("oom")));
    var ck: u64 = 0xCBF29CE484222325;
    var kept: usize = 0;
    var i: usize = 0;
    while (i < n) : (i += 1) {
        const r = mix64(0xA110C00000000001 +% i);
        const s: usize = 8 + 8 * @as(usize, @intCast(r & 31));
        const b: [*]u8 = @ptrCast(malloc(s) orelse @panic("oom"));
        @memset(b[0..s], 0);
        std.mem.doNotOptimizeAway(b);
        var v: u8 = @intCast(i % 251);
        var k: usize = 0;
        while (k < s) : (k += 1) {
            b[k] = v;
            v = if (v == 250) 0 else v + 1;
        }
        var sum: u64 = 0;
        k = 0;
        while (k < s) : (k += 1) sum += b[k];
        ck = ckfold(ck, sum);
        if (i & 15 == 0) {
            const slot = kept % w;
            if (kept >= w) free(ring[slot]);
            ring[slot] = b;
            kept += 1;
        } else {
            std.mem.doNotOptimizeAway(b);
            free(b);
        }
    }
    const live = if (kept < w) kept else w;
    const first = if (kept >= w) kept % w else 0;
    var j: usize = 0;
    while (j < live) : (j += 1) free(ring[(first + j) % w]);
    free(@ptrCast(ring));
    return ckfold(ck, kept);
}
