//! stream : CRC-32 + tokeniseur JSON-lines sur un fichier lu par blocs de 1 Mio (spec/workloads.d/stream.md).
const std = @import("std");

extern "c" fn open(path: [*:0]const u8, flags: c_int, ...) c_int;
extern "c" fn read(fd: c_int, buf: [*]u8, n: usize) isize;
extern "c" fn close(fd: c_int) c_int;
extern "c" fn getenv(name: [*:0]const u8) ?[*:0]const u8;
extern "c" fn exit(code: c_int) noreturn;

const ck_init: u64 = 0xCBF29CE484222325;
inline fn ckfold(ck: u64, v: u64) u64 {
    return (ck ^ v) *% 0x100000001B3;
}

pub fn run(n_mio: usize, threads: usize) u64 {
    _ = threads;
    const dir: []const u8 = if (getenv("BENCH_DATA_DIR")) |d| std.mem.span(d) else "/var/tmp/bench-data";
    var pathbuf: [512]u8 = undefined;
    const path = std.fmt.bufPrint(pathbuf[0..511], "{s}/stream-{d}.json", .{ dir, n_mio }) catch exit(3);
    pathbuf[path.len] = 0;
    const fd = open(@ptrCast(&pathbuf), 0);
    if (fd < 0) exit(3);
    var tab: [256]u32 = undefined;
    for (&tab, 0..) |*t, i| {
        var x: u32 = @intCast(i);
        for (0..8) |_| x = if (x & 1 != 0) (x >> 1) ^ 0xEDB88320 else x >> 1;
        t.* = x;
    }
    const buf = std.heap.c_allocator.alloc(u8, 1 << 20) catch exit(3);
    defer std.heap.c_allocator.free(buf);
    var crc: u32 = 0xFFFFFFFF;
    var mode: u8 = 0;
    var depth: u64 = 0;
    var maxd: u64 = 0;
    var objs: u64 = 0;
    var strs: u64 = 0;
    var nums: u64 = 0;
    var arrs: u64 = 0;
    var lits: u64 = 0;
    var lines: u64 = 0;
    var idsum: u64 = 0;
    var allsum: u64 = 0;
    var acc: u64 = 0;
    var total: u64 = 0;
    var isint = false;
    var gotid = false;
    while (true) {
        const got = read(fd, buf.ptr, buf.len);
        if (got < 0) exit(3);
        if (got == 0) break;
        const len: usize = @intCast(got);
        total += len;
        for (buf[0..len]) |c| {
            crc = tab[(crc ^ c) & 0xFF] ^ (crc >> 8);
            const dg = c -% '0' < 10;
            const lo = c -% 'a' < 26;
            if (mode == 3 and !(dg or c == '.' or c == 'e' or c == 'E' or c == '+' or c == '-')) {
                nums += 1;
                if (isint) {
                    allsum +%= acc;
                    if (depth == 1 and !gotid) idsum +%= acc;
                }
                if (depth == 1) gotid = true;
                mode = 0;
            } else if (mode == 4 and !lo) {
                lits += 1;
                mode = 0;
            }
            switch (mode) {
                2 => mode = 1,
                1 => mode = if (c == '\\') 2 else if (c == '"') 0 else 1,
                3 => if (dg) {
                    acc = acc *% 10 +% (c - '0');
                } else {
                    isint = false;
                },
                4 => {},
                else => {
                    if (c == '"') {
                        strs += 1;
                        mode = 1;
                    } else if (c == '{') {
                        objs += 1;
                        depth += 1;
                        if (depth > maxd) maxd = depth;
                    } else if (c == '[') {
                        arrs += 1;
                        depth += 1;
                        if (depth > maxd) maxd = depth;
                    } else if (c == '}' or c == ']') {
                        if (depth > 0) depth -= 1;
                    } else if (c == '\n') {
                        lines += 1;
                        gotid = false;
                    } else if (dg) {
                        mode = 3;
                        acc = c - '0';
                        isint = true;
                    } else if (c == '-') {
                        mode = 3;
                        acc = 0;
                        isint = true;
                    } else if (lo) mode = 4;
                },
            }
        }
    }
    _ = close(fd);
    var ck = ck_init;
    ck = ckfold(ck, crc ^ 0xFFFFFFFF);
    for ([_]u64{ objs, strs, nums, arrs, lits, maxd, lines, idsum, allsum, total }) |v| ck = ckfold(ck, v);
    return ck;
}
