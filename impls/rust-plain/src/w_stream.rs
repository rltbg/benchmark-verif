//! stream : CRC-32 + tokeniseur JSON-lines sur un fichier lu par blocs de 1 Mio (spec/workloads.d/stream.md).
use std::fs::File;
use std::io::Read;

#[inline(always)]
fn ckfold(ck: u64, v: u64) -> u64 {
    (ck ^ v).wrapping_mul(0x100000001B3)
}

pub fn run(n_mio: usize, _threads: usize) -> u64 {
    let dir = std::env::var("BENCH_DATA_DIR").unwrap_or_else(|_| "/var/tmp/bench-data".to_string());
    let path = format!("{dir}/stream-{n_mio}.json");
    let mut f = File::open(&path).unwrap_or_else(|e| {
        eprintln!("{path}: {e}");
        std::process::exit(3)
    });
    let mut tab = [0u32; 256];
    for (i, t) in tab.iter_mut().enumerate() {
        let mut x = i as u32;
        for _ in 0..8 {
            x = if x & 1 != 0 { (x >> 1) ^ 0xEDB88320 } else { x >> 1 };
        }
        *t = x;
    }
    let mut buf = vec![0u8; 1 << 20];
    let mut crc = 0xFFFF_FFFFu32;
    let (mut mode, mut depth, mut maxd) = (0u8, 0u64, 0u64);
    let (mut objs, mut strs, mut nums, mut arrs, mut lits, mut lines) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let (mut idsum, mut allsum, mut acc, mut total) = (0u64, 0u64, 0u64, 0u64);
    let (mut isint, mut gotid) = (false, false);
    loop {
        let got = f.read(&mut buf).unwrap_or_else(|e| {
            eprintln!("read: {e}");
            std::process::exit(3)
        });
        if got == 0 {
            break;
        }
        total += got as u64;
        for &c in &buf[..got] {
            crc = tab[((crc ^ c as u32) & 0xFF) as usize] ^ (crc >> 8);
            let dg = c.wrapping_sub(b'0') < 10;
            let lo = c.wrapping_sub(b'a') < 26;
            if mode == 3 && !(dg || c == b'.' || c == b'e' || c == b'E' || c == b'+' || c == b'-') {
                nums += 1;
                if isint {
                    allsum = allsum.wrapping_add(acc);
                    if depth == 1 && !gotid {
                        idsum = idsum.wrapping_add(acc);
                    }
                }
                if depth == 1 {
                    gotid = true;
                }
                mode = 0;
            } else if mode == 4 && !lo {
                lits += 1;
                mode = 0;
            }
            match mode {
                2 => mode = 1,
                1 => mode = if c == b'\\' { 2 } else if c == b'"' { 0 } else { 1 },
                3 => {
                    if dg {
                        acc = acc.wrapping_mul(10).wrapping_add((c - b'0') as u64);
                    } else {
                        isint = false;
                    }
                }
                4 => {}
                _ => {
                    if c == b'"' {
                        strs += 1;
                        mode = 1;
                    } else if c == b'{' {
                        objs += 1;
                        depth += 1;
                        maxd = maxd.max(depth);
                    } else if c == b'[' {
                        arrs += 1;
                        depth += 1;
                        maxd = maxd.max(depth);
                    } else if c == b'}' || c == b']' {
                        depth = depth.saturating_sub(1);
                    } else if c == b'\n' {
                        lines += 1;
                        gotid = false;
                    } else if dg {
                        mode = 3;
                        acc = (c - b'0') as u64;
                        isint = true;
                    } else if c == b'-' {
                        mode = 3;
                        acc = 0;
                        isint = true;
                    } else if lo {
                        mode = 4;
                    }
                }
            }
        }
    }
    let mut ck = 0xCBF29CE484222325u64;
    ck = ckfold(ck, (crc ^ 0xFFFF_FFFF) as u64);
    for v in [objs, strs, nums, arrs, lits, maxd, lines, idsum, allsum, total] {
        ck = ckfold(ck, v);
    }
    ck
}
