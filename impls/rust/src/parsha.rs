//! parsha : T segments SHA-256 indépendants, un par thread (spec/workloads.d/parsha.md).
//!
//! Prouvé avec Creusot : le noyau séquentiel `segment` / `compress` (indices des tableaux W[0..63], H[0..7], blocs M[0..15] dans les bornes,
//! absence de panique et de débordement ; les additions sont `wrapping_add`, les rotations sont écrites avec des décalages constants < 32).
//! Pas de correction fonctionnelle (pas de modèle SHA-256 dans la preuve) : elle est contrôlée par le checksum, identique à celui de `sha256sum`
//! pour T = 1 (voir workload `sha256`). **Non prouvé** : `run` (`#[trusted]`) qui lance les `std::thread`.
use crate::common::*;
use creusot_std::prelude::*;

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

/// Une compression : h (8 mots) mis à jour avec le bloc m (16 mots) ; w est un espace de travail de 64 mots.
#[requires(h@.len() == 8 && m@.len() == 16 && w@.len() == 64)]
#[ensures((^h)@.len() == 8 && (^w)@.len() == 64)]
#[check(terminates)]
pub fn compress(h: &mut [u32], m: &[u32], w: &mut [u32]) {
    let mut t = 0usize;
    #[invariant(w@.len() == 64 && t@ <= 16)]
    #[variant(16 - t@)]
    while t < 16 {
        w[t] = m[t];
        t += 1;
    }
    #[invariant(w@.len() == 64 && 16 <= t@ && t@ <= 64)]
    #[variant(64 - t@)]
    while t < 64 {
        let x = w[t - 15];
        let y = w[t - 2];
        let s0 = rotr(x, 7) ^ rotr(x, 18) ^ (x >> 3);
        let s1 = rotr(y, 17) ^ rotr(y, 19) ^ (y >> 10);
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
    #[invariant(w@.len() == 64 && t@ <= 64)]
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

#[inline(always)]
fn bswap(x: u32) -> u32 {
    (x >> 24) | ((x >> 8) & 0xFF00) | ((x << 8) & 0xFF0000) | (x << 24)
}

/// Noyau séquentiel : condensé SHA-256 des m octets (m multiple de 64) du segment de base `seedbase`, écrit dans h (8 mots).
#[requires(h@.len() == 8 && blk@.len() == 16 && w@.len() == 64)]
#[ensures((^h)@.len() == 8 && (^blk)@.len() == 16 && (^w)@.len() == 64)]
pub fn segment(seedbase: u64, m: usize, h: &mut [u32], blk: &mut [u32], w: &mut [u32]) {
    h[0] = 0x6a09e667;
    h[1] = 0xbb67ae85;
    h[2] = 0x3c6ef372;
    h[3] = 0xa54ff53a;
    h[4] = 0x510e527f;
    h[5] = 0x9b05688c;
    h[6] = 0x1f83d9ab;
    h[7] = 0x5be0cd19;
    let blocks = m / 64;
    let mut b = 0usize;
    #[invariant(h@.len() == 8 && blk@.len() == 16 && w@.len() == 64 && b@ <= blocks@)]
    #[variant(blocks@ - b@)]
    while b < blocks {
        let mut k = 0usize;
        #[invariant(blk@.len() == 16 && k@ <= 8)]
        #[variant(8 - k@)]
        while k < 8 {
            let x = mix64(seedbase.wrapping_add((b as u64).wrapping_mul(8)).wrapping_add(k as u64));
            blk[2 * k] = bswap(x as u32);
            blk[2 * k + 1] = bswap((x >> 32) as u32);
            k += 1;
        }
        compress(h, blk, w);
        b += 1;
    }
    let mut k = 0usize;
    #[invariant(blk@.len() == 16 && k@ <= 16)]
    #[variant(16 - k@)]
    while k < 16 {
        blk[k] = 0;
        k += 1;
    }
    blk[0] = 0x8000_0000;
    let bits = (m as u64).wrapping_mul(8);
    blk[14] = (bits >> 32) as u32;
    blk[15] = bits as u32;
    compress(h, blk, w);
}

#[trusted]
#[requires(1 <= n@)]
pub fn run(n: usize, threads: usize) -> u64 {
    let t = threads.clamp(1, 64);
    let m = n / t;
    let digests: Vec<Vec<u32>> = std::thread::scope(|s| {
        let job = move |k: usize| {
            let (mut h, mut blk, mut w) = (std::vec![0u32; 8], std::vec![0u32; 16], std::vec![0u32; 64]);
            segment(0x5348413235362D42_u64.wrapping_add((k as u64) << 40), m, &mut h, &mut blk, &mut w);
            h
        };
        let hs: Vec<_> = (1..t).map(|k| s.spawn(move || job(k))).collect();
        let mut v = std::vec![job(0)];
        v.extend(hs.into_iter().map(|h| h.join().unwrap()));
        v
    });
    let mut ck = CK_INIT;
    for d in &digests {
        for &w in d {
            ck = ckfold(ck, w as u64);
        }
    }
    ck
}
