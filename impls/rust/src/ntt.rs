//! Workload `ntt` (spec/workloads.d/ntt.md) : produit cyclique par NTT entière modulo p = 2013265921.
//!
//! Prouvé avec Creusot : absence de panique (tous les indices dans les bornes : bit-reverse, étages de
//! papillons à stride variable, table de racines) et de débordement arithmétique (produits `u32 x u32` en `u64`,
//! indices non linéaires `blk*len`, `j*stride`) ; terminaison de tout ce qui n'alloue pas.
//! Non prouvé : la correction fonctionnelle de la transformée (vérifiée par un test indépendant contre la
//! convolution naïve, voir la spec).
use crate::common::*;
use creusot_std::prelude::*;

const SEED: u64 = 0xA5A5F00D12345678;
const P: u64 = 2013265921;
const G: u64 = 31;

/// 2^k, pour les invariants des boucles bit-reverse.
#[logic(open)]
#[variant(k)]
pub fn pow2(k: Int) -> Int {
    pearlite! { if k <= 0 { 1 } else { 2 * pow2(k - 1) } }
}

#[logic]
#[requires(0 <= k)]
#[ensures(pow2(k) > k && pow2(k) >= 1)]
#[variant(k)]
pub fn pow2_gt(k: Int) {
    pearlite! { if k > 0 { pow2_gt(k - 1) } }
}

#[logic]
#[requires(0 <= a && a <= b)]
#[ensures(pow2(a) <= pow2(b))]
#[variant(b)]
pub fn pow2_le(a: Int, b: Int) {
    pearlite! { if a < b { pow2_gt(b - 1); pow2_le(a, b - 1) } }
}

/// a*b <= x*y pour des naturels.
#[logic]
#[requires(0 <= a && a <= x && 0 <= b && b <= y)]
#[ensures(a * b <= x * y)]
pub fn mul_mono(a: Int, b: Int, x: Int, y: Int) {}

/// Exponentiation modulaire (carré-et-multiplie), b < p.
#[requires(b0@ < 2013265921)]
#[ensures(result@ < 2013265921)]
#[check(terminates)]
pub fn powmod(b0: u64, e0: u64) -> u64 {
    let mut b = b0;
    let mut e = e0;
    let mut r = 1u64;
    #[invariant(b@ < 2013265921 && r@ < 2013265921)]
    #[variant(e@)]
    while e > 0 {
        proof_assert!{ mul_mono(r@, b@, 2013265920, 2013265920); mul_mono(b@, b@, 2013265920, 2013265920); true };
        if e % 2 == 1 {
            r = r * b % P;
        }
        b = b * b % P;
        e /= 2;
    }
    r
}

#[requires(a@.len() == n@)]
#[ensures((^a)@.len() == n@)]
#[check(terminates)]
pub fn fill(a: &mut Vec<u32>, n: usize) {
    let mut i = 0usize;
    #[invariant(a@.len() == n@ && i@ <= n@)]
    #[variant(n@ - i@)]
    while i < n {
        a[i] = (mix64(SEED.wrapping_add(i as u64)) % P) as u32;
        i += 1;
    }
}

/// tw[j] = w^j pour j < n/2 (tw a n/2 + 1 cases).
#[requires(w@ < 2013265921 && tw@.len() == n@ / 2 + 1)]
#[ensures((^tw)@.len() == tw@.len())]
#[check(terminates)]
pub fn twiddles(tw: &mut Vec<u32>, n: usize, w: u64) {
    let mut c = 1u64;
    let mut j = 0usize;
    #[invariant(c@ < 2013265921 && tw@.len() == n@ / 2 + 1 && j@ <= n@ / 2)]
    #[variant(n@ / 2 - j@)]
    while j < n / 2 {
        tw[j] = c as u32;
        proof_assert!{ mul_mono(c@, w@, 2013265920, 2013265920); true };
        c = c * w % P;
        j += 1;
    }
}

/// Permutation bit-reverse sur k bits ; n = 2^k.
#[requires(a@.len() == n@ && n@ == pow2(k@) && k@ <= n@)]
#[ensures((^a)@.len() == a@.len())]
#[check(terminates)]
pub fn bitrev(a: &mut Vec<u32>, n: usize, k: usize) {
    let mut i = 0usize;
    #[invariant(a@.len() == n@ && i@ <= n@)]
    #[variant(n@ - i@)]
    while i < n {
        let mut x = i;
        let mut rev = 0usize;
        let mut b = 0usize;
        let mut bnd = 1usize;
        #[invariant(rev@ < bnd@ && bnd@ == pow2(b@) && b@ <= k@ && bnd@ <= n@)]
        #[variant(k@ - b@)]
        while b < k {
            proof_assert!{ pow2_le(b@ + 1, k@); pow2_gt(b@); true };
            rev = rev * 2 + x % 2;
            x /= 2;
            bnd *= 2;
            b += 1;
        }
        if i < rev {
            let t = a[i];
            a[i] = a[rev];
            a[rev] = t;
        }
        i += 1;
    }
}

/// Un étage : papillons de demi-longueur `half` (len = 2 half <= n), stride = n/len.
#[requires(a@.len() == n@ && tw@.len() == n@ / 2 + 1 && 1 <= half@ && 2 * half@ <= n@ && n@ <= 134217728)]
#[ensures((^a)@.len() == a@.len())]
#[check(terminates)]
pub fn stage(a: &mut Vec<u32>, tw: &Vec<u32>, n: usize, half: usize) {
    let len = half * 2;
    let stride = n / len;
    proof_assert!(stride@ * len@ <= n@);
    proof_assert!(half@ * stride@ * 2 == len@ * stride@);
    let mut blk = 0usize;
    #[invariant(a@.len() == n@ && blk@ <= stride@)]
    #[variant(stride@ - blk@)]
    while blk < stride {
        proof_assert!{ mul_mono(blk@ + 1, len@, stride@, len@); true };
        let start = blk * len;
        let mut j = 0usize;
        #[invariant(a@.len() == n@ && j@ <= half@)]
        #[variant(half@ - j@)]
        while j < half {
            proof_assert!{ mul_mono(j@, stride@, half@ - 1, stride@); mul_mono(half@, stride@, half@, stride@); true };
            proof_assert!(j@ * stride@ <= half@ * stride@ - stride@);
            let u = a[start + j] as u64;
            let x = a[start + j + half] as u64;
            let t = tw[j * stride] as u64;
            proof_assert!{ mul_mono(x@, t@, 4294967295, 4294967295); true };
            let v = x * t % P;
            a[start + j] = ((u + v) % P) as u32;
            a[start + j + half] = ((u + P - v) % P) as u32;
            j += 1;
        }
        blk += 1;
    }
}

#[requires(a@.len() == n@ && tw@.len() == n@ / 2 + 1 && n@ == pow2(k@) && k@ <= n@ && n@ <= 134217728)]
#[ensures((^a)@.len() == a@.len())]
#[check(terminates)]
pub fn transform(a: &mut Vec<u32>, tw: &Vec<u32>, n: usize, k: usize) {
    bitrev(a, n, k);
    let mut half = 1usize;
    #[invariant(a@.len() == n@ && 1 <= half@ && half@ <= n@)]
    #[variant(n@ + 1 - half@)]
    while half * 2 <= n {
        stage(a, tw, n, half);
        half *= 2;
    }
}

#[requires(a@.len() == n@ && n@ <= 134217728)]
#[ensures((^a)@.len() == a@.len())]
#[check(terminates)]
pub fn square(a: &mut Vec<u32>, n: usize) {
    let mut i = 0usize;
    #[invariant(a@.len() == n@ && i@ <= n@)]
    #[variant(n@ - i@)]
    while i < n {
        let x = a[i] as u64;
        proof_assert!{ mul_mono(x@, x@, 4294967295, 4294967295); true };
        a[i] = (x * x % P) as u32;
        i += 1;
    }
}

/// Renverse a[1..n-1] (échange a[i] et a[n-i], 1 <= i, 2i < n).
#[requires(a@.len() == n@ && 1 <= n@ && n@ <= 134217728)]
#[ensures((^a)@.len() == a@.len())]
#[check(terminates)]
pub fn reverse(a: &mut Vec<u32>, n: usize) {
    let mut i = 1usize;
    #[invariant(a@.len() == n@ && 1 <= i@ && 2 * i@ <= n@ + 2)]
    #[variant(n@ - i@)]
    while 2 * i < n {
        let t = a[i];
        a[i] = a[n - i];
        a[n - i] = t;
        i += 1;
    }
}

#[requires(a@.len() == n@ && ninv@ < 2013265921 && n@ <= 134217728)]
#[check(terminates)]
pub fn checksum(a: &Vec<u32>, n: usize, ninv: u64) -> u64 {
    let mut ck = CK_INIT;
    let mut i = 0usize;
    #[invariant(a@.len() == n@ && i@ <= n@)]
    #[variant(n@ - i@)]
    while i < n {
        let x = a[i] as u64;
        proof_assert!{ mul_mono(x@, ninv@, 4294967295, 2013265920); true };
        ck = ckfold(ck, x * ninv % P);
        i += 1;
    }
    ck
}

#[ensures(result@ >= 0)]
pub fn run(n: usize, _threads: usize) -> u64 {
    if n > 134217728 {
        return 0;
    }
    let mut pw = 1usize;
    let mut k = 0usize;
    #[invariant(pw@ == pow2(k@) && pw@ <= 2 * n@ && k@ < pw@)]
    #[variant(2 * n@ + 1 - pw@)]
    while pw < n {
        proof_assert!{ pow2_gt(k@ + 1); true };
        pw *= 2;
        k += 1;
    }
    if pw != n {
        return 0;
    }
    let mut a = std::vec![0u32; n];
    let mut tw = std::vec![0u32; n / 2 + 1];
    fill(&mut a, n);
    let w = powmod(G, (P - 1) / (n as u64));
    twiddles(&mut tw, n, w);
    transform(&mut a, &tw, n, k);
    square(&mut a, n);
    transform(&mut a, &tw, n, k);
    reverse(&mut a, n);
    let ninv = powmod(n as u64, P - 2);
    checksum(&a, n, ninv)
}
