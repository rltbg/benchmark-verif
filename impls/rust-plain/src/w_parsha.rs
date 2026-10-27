//! parsha : T segments SHA-256 indépendants, un par thread (spec/workloads.d/parsha.md).
use crate::{ckfold, mix64, CK_INIT};

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be,
    0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa,
    0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85,
    0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
    0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f,
    0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

#[inline(always)]
fn compress(h: &mut [u32; 8], m: &[u32; 16]) {
    let mut w = [0u32; 64];
    w[..16].copy_from_slice(m);
    for t in 16..64 {
        let s0 = w[t - 15].rotate_right(7) ^ w[t - 15].rotate_right(18) ^ (w[t - 15] >> 3);
        let s1 = w[t - 2].rotate_right(17) ^ w[t - 2].rotate_right(19) ^ (w[t - 2] >> 10);
        w[t] = w[t - 16].wrapping_add(s0).wrapping_add(w[t - 7]).wrapping_add(s1);
    }
    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = *h;
    for t in 0..64 {
        let t1 = hh
            .wrapping_add(e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25))
            .wrapping_add((e & f) ^ (!e & g))
            .wrapping_add(K[t])
            .wrapping_add(w[t]);
        let t2 = (a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22)).wrapping_add((a & b) ^ (a & c) ^ (b & c));
        hh = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);
    }
    for (x, y) in h.iter_mut().zip([a, b, c, d, e, f, g, hh]) {
        *x = x.wrapping_add(y);
    }
}

/// Noyau séquentiel : condensé SHA-256 des m octets (m multiple de 64) du segment de base `seedbase`.
fn segment(seedbase: u64, m: usize) -> [u32; 8] {
    let mut h: [u32; 8] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];
    let mut blk = [0u32; 16];
    for b in 0..(m / 64) as u64 {
        for k in 0..8u64 {
            let x = mix64(seedbase.wrapping_add(8u64.wrapping_mul(b)).wrapping_add(k));
            blk[2 * k as usize] = (x as u32).swap_bytes();
            blk[2 * k as usize + 1] = ((x >> 32) as u32).swap_bytes();
        }
        compress(&mut h, &blk);
    }
    blk = [0u32; 16];
    blk[0] = 0x8000_0000;
    let bits = m as u64 * 8;
    blk[14] = (bits >> 32) as u32;
    blk[15] = bits as u32;
    compress(&mut h, &blk);
    h
}

pub fn run(n: usize, threads: usize) -> u64 {
    let t = threads.clamp(1, 64);
    let m = n / t;
    let digests: Vec<[u32; 8]> = std::thread::scope(|s| {
        let hs: Vec<_> = (1..t).map(|k| s.spawn(move || segment(0x5348413235362D42_u64.wrapping_add((k as u64) << 40), m))).collect();
        let mut v = vec![segment(0x5348413235362D42_u64, m)];
        v.extend(hs.into_iter().map(|h| h.join().unwrap()));
        v
    });
    digests.iter().flatten().fold(CK_INIT, |ck, &w| ckfold(ck, w as u64))
}
