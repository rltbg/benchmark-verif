//! Les cinq workloads, spécifiés dans spec/WORKLOADS.md.
//!
//! Niveau de preuve visé avec Creusot : absence de panique (indices dans les bornes,
//! pas de débordement arithmétique hors opérations `wrapping_*`), terminaison des
//! boucles `while` par variant implicite des invariants. La correction fonctionnelle
//! complète (tri = permutation triée, etc.) n'est volontairement pas visée : elle
//! n'a aucun impact sur le code généré, seulement sur le temps de preuve.
use creusot_contracts::prelude::*;

pub const CK_INIT: u64 = 0xCBF29CE484222325;

const S_SORT: u64 = 0x0123456789ABCDEF;
const S_MAT: u64 = 0x0FEDCBA987654321;
const S_LINE: u64 = 0x1357924680ACEBDF;
const S_HASH: u64 = 0x2468ACE013579BDF;

pub const LINE_K: usize = 1 << 20;
pub const LINE_MAX: usize = 15;

/// Finaliseur splitmix64. Corps hors du périmètre de preuve (décalages/XOR sur u64).
#[trusted]
#[ensures(true)]
#[inline(always)]
pub fn mix64(x: u64) -> u64 {
    let mut z = x.wrapping_add(0x9E3779B97F4A7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

#[trusted]
#[ensures(true)]
#[inline(always)]
pub fn fold(ck: u64, v: u64) -> u64 {
    (ck ^ v).wrapping_mul(0x100000001B3)
}

// ------------------------------------------------------------------ sort

/// Fusionne buf[so+lo..so+mid] et buf[so+mid..so+hi] vers buf[d+lo..d+hi].
#[requires(n@ <= 1073741824)]
#[requires(buf@.len() == 2 * n@)]
#[requires(so@ == 0 && d@ == n@ || so@ == n@ && d@ == 0)]
#[requires(lo@ <= mid@ && mid@ <= hi@ && hi@ <= n@)]
#[ensures((^buf)@.len() == buf@.len())]
fn merge(buf: &mut [u32], n: usize, so: usize, d: usize, lo: usize, mid: usize, hi: usize) {
    let _ = n; // n n'apparaît que dans les contrats
    let mut i = lo;
    let mut j = mid;
    let mut k = lo;
    #[invariant(lo@ <= i@ && i@ <= mid@)]
    #[invariant(mid@ <= j@ && j@ <= hi@)]
    #[invariant(k@ == i@ + j@ - mid@)]
    #[invariant(buf@.len() == 2 * n@)]
    while i < mid && j < hi {
        let x = buf[so + i];
        let y = buf[so + j];
        if x <= y {
            buf[d + k] = x;
            i += 1;
        } else {
            buf[d + k] = y;
            j += 1;
        }
        k += 1;
    }
    #[invariant(lo@ <= i@ && i@ <= mid@)]
    #[invariant(k@ == i@ + j@ - mid@)]
    #[invariant(mid@ <= j@ && j@ <= hi@)]
    #[invariant(buf@.len() == 2 * n@)]
    while i < mid {
        buf[d + k] = buf[so + i];
        i += 1;
        k += 1;
    }
    #[invariant(mid@ <= j@ && j@ <= hi@)]
    #[invariant(k@ == i@ + j@ - mid@)]
    #[invariant(lo@ <= i@ && i@ <= mid@)]
    #[invariant(buf@.len() == 2 * n@)]
    while j < hi {
        buf[d + k] = buf[so + j];
        j += 1;
        k += 1;
    }
}

#[requires(n@ <= 1073741824)]
#[ensures(true)]
pub fn run_sort(n: usize) -> u64 {
    let mut buf: Vec<u32> = std::vec![0u32; 2 * n];
    #[invariant(i@ <= n@)]
    #[invariant(buf@.len() == 2 * n@)]
    for i in 0..n {
        buf[i] = (mix64(S_SORT.wrapping_add(i as u64)) >> 32) as u32;
    }
    let mut so = 0usize;
    let mut d = n;
    let mut w = 1usize;
    #[invariant(1 <= w@ && w@ <= 2 * n@ + 1)]
    #[invariant(so@ == 0 && d@ == n@ || so@ == n@ && d@ == 0)]
    #[invariant(buf@.len() == 2 * n@)]
    while w < n {
        let mut lo = 0usize;
        #[invariant(lo@ <= n@ + 2 * w@)]
        #[invariant(buf@.len() == 2 * n@)]
        while lo < n {
            let mid = if lo + w < n { lo + w } else { n };
            let hi = if lo + 2 * w < n { lo + 2 * w } else { n };
            merge(&mut buf, n, so, d, lo, mid, hi);
            lo += 2 * w;
        }
        let t = so;
        so = d;
        d = t;
        w *= 2;
    }
    let mut ck = CK_INIT;
    #[invariant(i@ <= n@)]
    #[invariant(buf@.len() == 2 * n@)]
    for i in 0..n {
        let v = buf[so + i];
        if i > 0 && buf[so + i - 1] > v {
            return 0;
        }
        ck = fold(ck, v as u64);
    }
    ck
}

// ------------------------------------------------------------------ sieve

#[requires(n@ <= 4000000000)]
#[ensures(true)]
pub fn run_sieve(n: usize) -> u64 {
    let mut f: Vec<u8> = std::vec![0u8; n + 1];
    let mut i = 2usize;
    #[invariant(2 <= i@ && i@ <= 65536)]
    #[invariant(f@.len() == n@ + 1)]
    while i * i <= n {
        if f[i] == 0 {
            let mut j = i * i;
            #[invariant(j@ <= n@ + i@)]
            #[invariant(f@.len() == n@ + 1)]
            while j <= n {
                f[j] = 1;
                j += i;
            }
        }
        i += 1;
    }
    let mut ck = CK_INIT;
    let mut cnt = 0u64;
    #[invariant(2 <= i@ && i@ <= n@ + 1)]
    #[invariant(cnt@ <= i@)]
    #[invariant(f@.len() == n@ + 1)]
    for i in 2..=n {
        if f[i] == 0 {
            ck = fold(ck, i as u64);
            cnt += 1;
        }
    }
    fold(ck, cnt)
}

// ------------------------------------------------------------------ matmul

#[requires(n@ <= 4096)]
#[ensures(true)]
pub fn run_matmul(n: usize) -> u64 {
    let nn = n * n;
    let mut a: Vec<u32> = std::vec![0u32; nn];
    let mut b: Vec<u32> = std::vec![0u32; nn];
    let mut c: Vec<u32> = std::vec![0u32; nn];
    #[invariant(a@.len() == nn@ && b@.len() == nn@)]
    for i in 0..nn {
        a[i] = (mix64(S_MAT.wrapping_add(i as u64)) >> 32) as u32;
        b[i] = (mix64(S_MAT.wrapping_add((nn + i) as u64)) >> 32) as u32;
    }
    #[invariant(a@.len() == nn@ && b@.len() == nn@ && c@.len() == nn@)]
    for i in 0..n {
        #[invariant(a@.len() == nn@ && b@.len() == nn@ && c@.len() == nn@)]
        for k in 0..n {
            // i < n, k < n  =>  i*n + k < n*n  (arithmétique non linéaire : lemme à fournir au prouveur)
            let aik = a[i * n + k];
            #[invariant(a@.len() == nn@ && b@.len() == nn@ && c@.len() == nn@)]
            for j in 0..n {
                c[i * n + j] = c[i * n + j].wrapping_add(aik.wrapping_mul(b[k * n + j]));
            }
        }
    }
    let mut ck = CK_INIT;
    #[invariant(c@.len() == nn@)]
    for i in 0..nn {
        ck = fold(ck, c[i] as u64);
    }
    ck
}

// ------------------------------------------------------------------ lines

#[requires(buf@.len() >= off@ + 20)]
#[requires(off@ <= 1000000000000)]
#[ensures(result@ >= 1 && result@ <= 20)]
fn put_dec(buf: &mut [u8], off: usize, mut v: u64) -> usize {
    let mut tmp = [0u8; 20];
    let mut l = 0usize;
    #[invariant(l@ <= 19)]
    loop {
        tmp[l] = b'0' + (v % 10) as u8;
        l += 1;
        v /= 10;
        if v == 0 || l == 19 {
            break;
        }
    }
    let mut i = 0usize;
    #[invariant(i@ <= l@ && l@ <= 20)]
    #[invariant(buf@.len() >= off@ + 20)]
    while i < l {
        buf[off + i] = tmp[l - 1 - i];
        i += 1;
    }
    l
}

#[requires(n@ <= 100000000)]
#[ensures(true)]
pub fn run_lines(n: usize) -> u64 {
    let mut txt: Vec<u8> = std::vec![0u8; n * LINE_MAX + 20];
    let mut len = 0usize;
    #[invariant(len@ <= i@ * 15)]
    #[invariant(txt@.len() == n@ * 15 + 20)]
    for i in 0..n {
        let r = mix64(S_LINE.wrapping_add(i as u64));
        let key = r >> 44;
        let val = ((r >> 8) & 0xFFFF_FFFF) % 1_000_000;
        len += put_dec(&mut txt, len, key);
        txt[len] = b';';
        len += 1;
        len += put_dec(&mut txt, len, val);
        txt[len] = b'\n';
        len += 1;
    }
    let mut tab: Vec<u64> = std::vec![0u64; 2 * LINE_K];
    let mut p = 0usize;
    #[invariant(p@ <= len@ && len@ < txt@.len())]
    #[invariant(tab@.len() == 2 * 1048576)]
    while p < len {
        let mut key = 0usize;
        let mut val = 0u64;
        // Les lignes sont bien formées par construction ; l'accès hors bornes est
        // exclu par la présence de ';' et '\n' (non démontré ici sans invariant de forme).
        while txt[p] != b';' {
            key = key.wrapping_mul(10).wrapping_add((txt[p] - b'0') as usize);
            p += 1;
        }
        p += 1;
        while txt[p] != b'\n' {
            val = val.wrapping_mul(10).wrapping_add((txt[p] - b'0') as u64);
            p += 1;
        }
        p += 1;
        let k = key & (LINE_K - 1);
        tab[2 * k] = tab[2 * k].wrapping_add(val);
        tab[2 * k + 1] = tab[2 * k + 1].wrapping_add(1);
    }
    let mut ck = CK_INIT;
    #[invariant(tab@.len() == 2 * 1048576)]
    for k in 0..LINE_K {
        ck = fold(fold(ck, tab[2 * k]), tab[2 * k + 1]);
    }
    ck
}

// ------------------------------------------------------------------ hashmap

#[requires(n@ <= 67108864)]
#[ensures(true)]
pub fn run_hashmap(n: usize) -> u64 {
    let mut lg = 1u32;
    #[invariant(1 <= lg@ && lg@ <= 28)]
    while ((1usize << lg) < 2 * n) && lg < 28 {
        lg += 1;
    }
    let cap = 1usize << lg;
    let mask = cap - 1;
    let mut t: Vec<u64> = std::vec![0u64; 2 * cap];
    #[invariant(t@.len() == 2 * cap@)]
    for i in 0..n {
        let key = mix64(S_HASH.wrapping_add(i as u64)) | 1;
        let mut s = (key.wrapping_mul(0x9E3779B97F4A7C15) >> (64 - lg)) as usize & mask;
        let mut tries = 0usize;
        #[invariant(tries@ <= cap@)]
        #[invariant(t@.len() == 2 * cap@)]
        while tries < cap {
            if t[2 * s] == 0 {
                t[2 * s] = key;
                t[2 * s + 1] = (i as u64) + 1;
                break;
            }
            if t[2 * s] == key {
                t[2 * s + 1] = t[2 * s + 1].wrapping_add((i as u64) + 1);
                break;
            }
            s = (s + 1) & mask;
            tries += 1;
        }
    }
    let mut ck = CK_INIT;
    #[invariant(t@.len() == 2 * cap@)]
    for phase in 0..2usize {
        #[invariant(t@.len() == 2 * cap@)]
        for i in 0..n {
            let off = if phase == 1 { n } else { 0 };
            let key = mix64(S_HASH.wrapping_add((off + i) as u64)) | 1;
            let mut s = (key.wrapping_mul(0x9E3779B97F4A7C15) >> (64 - lg)) as usize & mask;
            let mut v = 0u64;
            let mut tries = 0usize;
            #[invariant(tries@ <= cap@)]
            #[invariant(t@.len() == 2 * cap@)]
            while tries < cap {
                if t[2 * s] == 0 {
                    break;
                }
                if t[2 * s] == key {
                    v = t[2 * s + 1];
                    break;
                }
                s = (s + 1) & mask;
                tries += 1;
            }
            ck = fold(ck, v);
        }
    }
    ck
}
