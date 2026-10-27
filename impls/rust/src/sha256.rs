//! SHA-256 d'un message de n octets généré à la volée (spec/workloads.d/sha256.md). 0 <= n <= 2^40.
//!
//! Prouvé : absence de panique (tous les indices dans les bornes des tableaux de 16/64/128 éléments),
//! absence de débordement hors `wrapping_*`, terminaison. Pas de correction fonctionnelle (la conformité est
//! contrôlée par comparaison avec `sha256sum` et par le checksum commun). `rotate_right`, `swap_bytes` et
//! `copy_from_slice` ne sont pas utilisés : ils n'ont pas de contrat Creusot, tout est écrit en décalages et boucles.
use crate::common::*;
use creusot_std::prelude::*;

const SEED: u64 = 0x5348413235362D42;

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be,
    0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa,
    0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85,
    0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
    0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f,
    0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

#[requires(0 < k@ && k@ < 32)]
#[check(terminates)]
#[inline(always)]
fn rotr(x: u32, k: u32) -> u32 {
    (x >> k) | (x << (32 - k))
}

#[check(terminates)]
#[inline(always)]
fn bswap(x: u32) -> u32 {
    ((x & 0xff) << 24) | ((x & 0xff00) << 8) | ((x >> 8) & 0xff00) | (x >> 24)
}

#[requires(true)]
#[check(terminates)]
pub fn compress(h: &mut [u32; 8], m: &[u32; 16]) {
    let mut w = [0u32; 64];
    let mut t = 0usize;
    #[invariant(t@ <= 16)]
    #[variant(16 - t@)]
    while t < 16 {
        w[t] = m[t];
        t += 1;
    }
    #[invariant(16 <= t@ && t@ <= 64)]
    #[variant(64 - t@)]
    while t < 64 {
        let s0 = rotr(w[t - 15], 7) ^ rotr(w[t - 15], 18) ^ (w[t - 15] >> 3);
        let s1 = rotr(w[t - 2], 17) ^ rotr(w[t - 2], 19) ^ (w[t - 2] >> 10);
        w[t] = w[t - 16].wrapping_add(s0).wrapping_add(w[t - 7]).wrapping_add(s1);
        t += 1;
    }
    let mut a = h[0];
    let mut b = h[1];
    let mut c = h[2];
    let mut d = h[3];
    let mut e = h[4];
    let mut f = h[5];
    let mut g = h[6];
    let mut hh = h[7];
    t = 0;
    #[invariant(t@ <= 64)]
    #[variant(64 - t@)]
    while t < 64 {
        let t1 = hh
            .wrapping_add(rotr(e, 6) ^ rotr(e, 11) ^ rotr(e, 25))
            .wrapping_add((e & f) ^ (!e & g))
            .wrapping_add(K[t])
            .wrapping_add(w[t]);
        let t2 = (rotr(a, 2) ^ rotr(a, 13) ^ rotr(a, 22)).wrapping_add((a & b) ^ (a & c) ^ (b & c));
        hh = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);
        t += 1;
    }
    h[0] = h[0].wrapping_add(a);
    h[1] = h[1].wrapping_add(b);
    h[2] = h[2].wrapping_add(c);
    h[3] = h[3].wrapping_add(d);
    h[4] = h[4].wrapping_add(e);
    h[5] = h[5].wrapping_add(f);
    h[6] = h[6].wrapping_add(g);
    h[7] = h[7].wrapping_add(hh);
}

#[check(terminates)]
pub fn byte_at(i: u64) -> u8 {
    (mix64(SEED.wrapping_add(i / 8)) >> (8 * (i % 8))) as u8
}

/// Bloc `b` du message (64 octets) : huit mots de mix64, chacun donné en deux mots big-endian.
#[requires(true)]
#[check(terminates)]
pub fn load_block(b: u64, m: &mut [u32; 16]) {
    let mut k = 0usize;
    #[invariant(k@ <= 8)]
    #[variant(8 - k@)]
    while k < 8 {
        let x = mix64(SEED.wrapping_add(b.wrapping_mul(8)).wrapping_add(k as u64));
        m[2 * k] = bswap(x as u32);
        m[2 * k + 1] = bswap((x >> 32) as u32);
        k += 1;
    }
}

#[requires(n@ <= 1099511627776)]
#[check(terminates)]
pub fn digest(n: usize) -> [u32; 8] {
    let mut h: [u32; 8] =
        [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];
    let blocks = (n / 64) as u64;
    let mut m = [0u32; 16];
    let mut b = 0u64;
    #[invariant(b@ <= blocks@)]
    #[variant(blocks@ - b@)]
    while b < blocks {
        load_block(b, &mut m);
        compress(&mut h, &m);
        b += 1;
    }
    let mut tail = [0u8; 128];
    let t = n % 64;
    let base = blocks * 64;
    let mut j = 0usize;
    #[invariant(j@ <= t@ && t@ < 64)]
    #[variant(t@ - j@)]
    while j < t {
        tail[j] = byte_at(base + j as u64);
        j += 1;
    }
    tail[t] = 0x80;
    let total = if t < 56 { 64usize } else { 128usize };
    let bits = (n as u64) * 8;
    let mut q = 0usize;
    #[invariant(q@ <= 8)]
    #[variant(8 - q@)]
    while q < 8 {
        tail[total - 8 + q] = (bits >> (56 - 8 * q)) as u8;
        q += 1;
    }
    let mut off = 0usize;
    #[invariant(off@ % 64 == 0 && off@ <= total@ && total@ <= 128)]
    #[variant(total@ - off@)]
    while off < total {
        let mut i = 0usize;
        #[invariant(i@ <= 16 && off@ % 64 == 0 && off@ + 64 <= total@ && total@ <= 128)]
        #[variant(16 - i@)]
        while i < 16 {
            m[i] = ((tail[off + 4 * i] as u32) << 24)
                | ((tail[off + 4 * i + 1] as u32) << 16)
                | ((tail[off + 4 * i + 2] as u32) << 8)
                | (tail[off + 4 * i + 3] as u32);
            i += 1;
        }
        compress(&mut h, &m);
        off += 64;
    }
    h
}

#[requires(n@ <= 1099511627776)]
#[check(terminates)]
pub fn run(n: usize, _threads: usize) -> u64 {
    let h = digest(n);
    let mut ck = CK_INIT;
    let mut i = 0usize;
    #[invariant(i@ <= 8)]
    #[variant(8 - i@)]
    while i < 8 {
        ck = ckfold(ck, h[i] as u64);
        i += 1;
    }
    ck
}
