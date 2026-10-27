//! Rust idiomatique, sans Creusot ni aucune preuve. Spécification : spec/WORKLOADS.md.
use std::time::Instant;

#[inline(always)]
fn mix64(x: u64) -> u64 {
    let mut z = x.wrapping_add(0x9E3779B97F4A7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}
const CK_INIT: u64 = 0xCBF29CE484222325;
#[inline(always)]
fn ckfold(ck: u64, v: u64) -> u64 {
    (ck ^ v).wrapping_mul(0x100000001B3)
}

fn merge(src: &[u32], dst: &mut [u32], lo: usize, mid: usize, hi: usize) {
    let (mut i, mut j) = (lo, mid);
    for k in lo..hi {
        if i < mid && (j >= hi || src[i] <= src[j]) {
            dst[k] = src[i];
            i += 1;
        } else {
            dst[k] = src[j];
            j += 1;
        }
    }
}
fn msort(a: &mut [u32], tmp: &mut [u32], lo: usize, hi: usize) {
    if hi - lo < 2 {
        return;
    }
    let mid = lo + (hi - lo) / 2;
    msort(a, tmp, lo, mid);
    msort(a, tmp, mid, hi);
    merge(a, tmp, lo, mid, hi);
    a[lo..hi].copy_from_slice(&tmp[lo..hi]);
}
fn run_sort(n: usize) -> u64 {
    let mut a: Vec<u32> = (0..n as u64).map(|i| (mix64(0x0123456789ABCDEF_u64.wrapping_add(i)) >> 32) as u32).collect();
    let mut tmp = vec![0u32; n];
    msort(&mut a, &mut tmp, 0, n);
    let mut ck = CK_INIT;
    let (mut prev, mut ok) = (0u32, true);
    for &x in &a {
        ok &= prev <= x;
        prev = x;
        ck = ckfold(ck, x as u64);
    }
    if ok { ck } else { 0 }
}

fn run_sieve(n: usize) -> u64 {
    let mut f = vec![0u8; n + 1];
    let mut i = 2;
    while i <= n / i {
        if f[i] == 0 {
            let mut j = i * i;
            while j <= n {
                f[j] = 1;
                j += i;
            }
        }
        i += 1;
    }
    let (mut ck, mut cnt) = (CK_INIT, 0u64);
    for i in 2..=n {
        if f[i] == 0 {
            ck = ckfold(ck, i as u64);
            cnt += 1;
        }
    }
    ckfold(ck, cnt)
}

fn run_matmul(n: usize) -> u64 {
    let nn = n * n;
    let seed = 0x0FEDCBA987654321_u64;
    let a: Vec<u32> = (0..nn as u64).map(|i| (mix64(seed.wrapping_add(i)) >> 32) as u32).collect();
    let b: Vec<u32> = (0..nn as u64).map(|i| (mix64(seed.wrapping_add(nn as u64).wrapping_add(i)) >> 32) as u32).collect();
    let mut c = vec![0u32; nn];
    for i in 0..n {
        let crow = &mut c[i * n..(i + 1) * n];
        for k in 0..n {
            let aik = a[i * n + k];
            let brow = &b[k * n..(k + 1) * n];
            for (cv, &bv) in crow.iter_mut().zip(brow) {
                *cv = cv.wrapping_add(aik.wrapping_mul(bv));
            }
        }
    }
    c.iter().fold(CK_INIT, |ck, &x| ckfold(ck, x as u64))
}

fn put_dec(out: &mut Vec<u8>, mut v: u32) {
    let mut tmp = [0u8; 10];
    let mut l = 0;
    loop {
        tmp[l] = b'0' + (v % 10) as u8;
        l += 1;
        v /= 10;
        if v == 0 {
            break;
        }
    }
    out.extend(tmp[..l].iter().rev());
}
fn run_lines(n: usize) -> u64 {
    let mut txt = Vec::with_capacity(n * 15 + 20);
    for i in 0..n as u64 {
        let r = mix64(0x1357924680ACEBDF_u64.wrapping_add(i));
        let key = ((r >> 44) as u32) & 0xFFFFF;
        let val = (((r >> 8) & 0xFFFFFFFF) % 1_000_000) as u32;
        put_dec(&mut txt, key);
        txt.push(b';');
        put_dec(&mut txt, val);
        txt.push(b'\n');
    }
    let mut tab = vec![0u64; 2097152];
    let (mut in_val, mut key, mut vl) = (false, 0u32, 0u32);
    for &c in &txt {
        let d = (c as u32).wrapping_sub(b'0' as u32);
        if in_val {
            if c == b'\n' {
                let k = (key & 0xFFFFF) as usize;
                tab[2 * k] = tab[2 * k].wrapping_add(vl as u64);
                tab[2 * k + 1] = tab[2 * k + 1].wrapping_add(1);
                in_val = false;
                key = 0;
                vl = 0;
            } else {
                vl = vl.wrapping_mul(10).wrapping_add(d);
            }
        } else if c == b';' {
            in_val = true;
        } else {
            key = key.wrapping_mul(10).wrapping_add(d);
        }
    }
    (0..1048576).fold(CK_INIT, |ck, k| ckfold(ckfold(ck, tab[2 * k]), tab[2 * k + 1]))
}

#[inline(always)]
fn key_of(off: u64, i: u64) -> u64 {
    mix64(0x2468ACE013579BDF_u64.wrapping_add(off).wrapping_add(i)) | 1
}
fn run_hashmap(n: usize) -> u64 {
    let (mut cap, mut lg) = (2usize, 1u32);
    while cap < 2 * n && lg < 27 && cap <= 33554432 {
        cap *= 2;
        lg += 1;
    }
    let slot0 = |key: u64| -> usize {
        let s = (key.wrapping_mul(0x9E3779B97F4A7C15) >> (64 - lg)) as usize;
        if s >= cap { 0 } else { s }
    };
    let mut t = vec![0u64; 2 * cap];
    for i in 0..n as u64 {
        let key = key_of(0, i);
        let mut s = slot0(key);
        for _ in 0..cap {
            let k = t[2 * s];
            if k == 0 {
                t[2 * s] = key;
                t[2 * s + 1] = i + 1;
                break;
            }
            if k == key {
                t[2 * s + 1] = t[2 * s + 1].wrapping_add(i + 1);
                break;
            }
            s += 1;
            if s == cap {
                s = 0;
            }
        }
    }
    let mut ck = CK_INIT;
    for off in [0u64, n as u64] {
        for i in 0..n as u64 {
            let key = key_of(off, i);
            let mut s = slot0(key);
            let mut v = 0u64;
            for _ in 0..cap {
                let k = t[2 * s];
                if k == 0 {
                    break;
                }
                if k == key {
                    v = t[2 * s + 1];
                    break;
                }
                s += 1;
                if s == cap {
                    s = 0;
                }
            }
            ck = ckfold(ck, v);
        }
    }
    ck
}

mod w_sha256;
mod w_bfs;
mod w_ptrchase;
mod w_ntt;
mod w_stream;
mod w_parmatmul;
mod w_parsort;
mod w_parsha;
mod w_strsort;
// @@MODS@@ (une ligne `mod w_<nom>;` ; chaque module expose `pub fn run(n: usize, threads: usize) -> u64`)

fn extended(w: &str, n: usize, threads: usize) -> Option<u64> {
    let _ = (n, threads);
    match w {
        "sha256" => Some(w_sha256::run(n, threads)),
        "bfs" => Some(w_bfs::run(n, threads)),
        "ptrchase" => Some(w_ptrchase::run(n, threads)),
        "ntt" => Some(w_ntt::run(n, threads)),
        "stream" => Some(w_stream::run(n, threads)),
        "parmatmul" => Some(w_parmatmul::run(n, threads)),
        "parsort" => Some(w_parsort::run(n, threads)),
        "parsha" => Some(w_parsha::run(n, threads)),
        "strsort" => Some(w_strsort::run(n, threads)),
        // @@DISPATCH@@
        _ => None,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let threads: usize = args.get(3).map_or(1, |t| t.parse().expect("threads"));
    if args.len() < 3 || args.len() > 4 {
        eprintln!("usage: bench-rust-plain <workload> <n> [threads]");
        std::process::exit(2);
    }
    let n: usize = args[2].parse().expect("n");
    let t0 = Instant::now();
    let ck = match args[1].as_str() {
        "sort" => run_sort(n),
        "sieve" => run_sieve(n),
        "matmul" => run_matmul(n),
        "lines" => run_lines(n),
        "hashmap" => run_hashmap(n),
        w => extended(w, n, threads).unwrap_or_else(|| {
            eprintln!("workload inconnu");
            std::process::exit(2);
        }),
    };
    println!("RESULT workload={} n={} checksum={:016x} time_ns={}", args[1], n, ck, t0.elapsed().as_nanos());
}
