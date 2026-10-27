//! stream : CRC-32 + tokeniseur JSON-lines sur un fichier lu par blocs de 1 Mio (spec/workloads.d/stream.md).
//!
//! Prouvé (Creusot) : le noyau `chunk` (indices du tampon et de la table dans les bornes, aucun débordement
//! arithmétique — compteurs en `wrapping_*`, terminaison). **Non prouvé** (`#[trusted]`) : `run`, qui ouvre le
//! fichier et lit les blocs avec l'API standard (les E/S ne sont pas modélisées par Creusot).
use crate::common::*;
use creusot_std::prelude::*;

#[check(terminates)]
#[inline(always)]
fn is_digit(c: u8) -> bool {
    c >= 48 && c <= 57
}

#[check(terminates)]
#[inline(always)]
fn is_lower(c: u8) -> bool {
    c >= 97 && c <= 122
}

/// '.', 'e', 'E', '+', '-'
#[check(terminates)]
#[inline(always)]
fn is_numpunct(c: u8) -> bool {
    c == 46 || c == 101 || c == 69 || c == 43 || c == 45
}

/// Table du CRC-32 (polynôme 0xEDB88320), 256 entrées.
#[ensures(result@.len() == 256)]
pub fn crc_table() -> Vec<u32> {
    let mut t = std::vec![0u32; 256];
    let mut i = 0usize;
    #[invariant(t@.len() == 256 && i@ <= 256)]
    #[variant(256 - i@)]
    while i < 256 {
        let mut x = i as u32;
        let mut k = 0usize;
        #[invariant(k@ <= 8)]
        #[variant(8 - k@)]
        while k < 8 {
            x = if x & 1 != 0 { (x >> 1) ^ 0xEDB88320 } else { x >> 1 };
            k += 1;
        }
        t[i] = x;
        i += 1;
    }
    t
}

/// État initial : 16 u64, slot 0 = registre CRC (0xFFFFFFFF).
#[ensures(result@.len() == 16)]
pub fn new_state() -> Vec<u64> {
    let mut s = std::vec![0u64; 16];
    s[0] = 0xFFFF_FFFF;
    s
}

/// Traite les `len` premiers octets de `buf` ; l'état `st` (16 u64) traverse les frontières de blocs.
#[requires(len@ <= buf@.len() && st@.len() == 16 && tab@.len() == 256)]
#[ensures((^st)@.len() == 16)]
#[check(terminates)]
pub fn chunk(buf: &Vec<u8>, len: usize, st: &mut Vec<u64>, tab: &Vec<u32>) {
    let mut crc = st[0] as u32;
    let mut mode = st[1];
    let mut depth = st[2];
    let mut maxd = st[3];
    let mut objs = st[4];
    let mut strs = st[5];
    let mut nums = st[6];
    let mut arrs = st[7];
    let mut lits = st[8];
    let mut lines = st[9];
    let mut idsum = st[10];
    let mut allsum = st[11];
    let mut acc = st[12];
    let mut isint = st[13];
    let mut gotid = st[14];
    let mut p = 0usize;
    #[invariant(p@ <= len@ && len@ <= buf@.len() && tab@.len() == 256 && st@.len() == 16)]
    #[variant(len@ - p@)]
    while p < len {
        let c = buf[p];
        crc = tab[((crc ^ (c as u32)) % 256) as usize] ^ (crc >> 8);
        let dg = is_digit(c);
        let lo = is_lower(c);
        if mode == 3 && !(dg || is_numpunct(c)) {
            nums = nums.wrapping_add(1);
            if isint == 1 {
                allsum = allsum.wrapping_add(acc);
                if depth == 1 && gotid == 0 {
                    idsum = idsum.wrapping_add(acc);
                }
            }
            if depth == 1 {
                gotid = 1;
            }
            mode = 0;
        } else if mode == 4 && !lo {
            lits = lits.wrapping_add(1);
            mode = 0;
        }
        if mode == 2 {
            mode = 1;
        } else if mode == 1 {
            mode = if c == 92 { 2 } else if c == 34 { 0 } else { 1 };
        } else if mode == 3 {
            if dg {
                acc = acc.wrapping_mul(10).wrapping_add(c.wrapping_sub(48) as u64);
            } else {
                isint = 0;
            }
        } else if mode == 4 {
        } else if c == 34 {
            strs = strs.wrapping_add(1);
            mode = 1;
        } else if c == 123 {
            objs = objs.wrapping_add(1);
            depth = depth.wrapping_add(1);
            if depth > maxd {
                maxd = depth;
            }
        } else if c == 91 {
            arrs = arrs.wrapping_add(1);
            depth = depth.wrapping_add(1);
            if depth > maxd {
                maxd = depth;
            }
        } else if c == 125 || c == 93 {
            if depth > 0 {
                depth -= 1;
            }
        } else if c == 10 {
            lines = lines.wrapping_add(1);
            gotid = 0;
        } else if dg {
            mode = 3;
            acc = c.wrapping_sub(48) as u64;
            isint = 1;
        } else if c == 45 {
            mode = 3;
            acc = 0;
            isint = 1;
        } else if lo {
            mode = 4;
        }
        p += 1;
    }
    st[0] = crc as u64;
    st[1] = mode;
    st[2] = depth;
    st[3] = maxd;
    st[4] = objs;
    st[5] = strs;
    st[6] = nums;
    st[7] = arrs;
    st[8] = lits;
    st[9] = lines;
    st[10] = idsum;
    st[11] = allsum;
    st[12] = acc;
    st[13] = isint;
    st[14] = gotid;
    st[15] = st[15].wrapping_add(len as u64);
}

#[requires(st@.len() == 16)]
#[check(terminates)]
pub fn finish(st: &Vec<u64>) -> u64 {
    let mut ck = CK_INIT;
    ck = ckfold(ck, st[0] ^ 0xFFFF_FFFF);
    ck = ckfold(ck, st[4]);
    ck = ckfold(ck, st[5]);
    ck = ckfold(ck, st[6]);
    ck = ckfold(ck, st[7]);
    ck = ckfold(ck, st[8]);
    ck = ckfold(ck, st[3]);
    ck = ckfold(ck, st[9]);
    ck = ckfold(ck, st[10]);
    ck = ckfold(ck, st[11]);
    ckfold(ck, st[15])
}

/// Lecture du fichier par blocs de 1 Mio puis appel du noyau prouvé (E/S non modélisées : `trusted`).
#[trusted]
pub fn run(n_mio: usize, _threads: usize) -> u64 {
    use std::io::Read;
    let dir = std::env::var("BENCH_DATA_DIR").unwrap_or_else(|_| "/var/tmp/bench-data".to_string());
    let path = format!("{dir}/stream-{n_mio}.json");
    let mut f = std::fs::File::open(&path).unwrap_or_else(|e| {
        eprintln!("{path}: {e}");
        std::process::exit(3)
    });
    let tab = crc_table();
    let mut st = new_state();
    let mut buf = std::vec![0u8; 1 << 20];
    loop {
        let got = f.read(&mut buf).unwrap_or_else(|e| {
            eprintln!("read: {e}");
            std::process::exit(3)
        });
        if got == 0 {
            break;
        }
        chunk(&buf, got, &mut st, &tab);
    }
    finish(&st)
}
