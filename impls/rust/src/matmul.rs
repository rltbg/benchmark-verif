//! C = A*B, matrices n x n de u32 (arithmétique modulo 2^32), boucles i, k, j. 1 <= n <= 4096.
use crate::common::*;
use creusot_std::prelude::*;

const SEED: u64 = 0x0FEDCBA987654321;

/// Indice i*n+j, strictement inférieur à n*n.
#[requires(n@ <= 4096 && i@ < n@ && j@ < n@)]
#[ensures(result@ == i@ * n@ + j@ && result@ < n@ * n@)]
#[ensures(result@ < 16777216)]
#[check(terminates)]
pub fn idx(n: usize, i: usize, j: usize) -> usize {
    proof_assert!(i@ * n@ + j@ < n@ * n@);
    proof_assert!(n@ * n@ <= 16777216);
    i * n + j
}

#[requires(m@.len() == len@)]
#[ensures((^m)@.len() == len@)]
#[check(terminates)]
pub fn init_mat(m: &mut Vec<u32>, len: usize, off: u64) {
    let mut i = 0usize;
    #[invariant(m@.len() == len@ && i@ <= len@)]
    #[variant(len@ - i@)]
    while i < len {
        m[i] = hi32(mix64(SEED.wrapping_add(off).wrapping_add(i as u64)));
        i += 1;
    }
}

#[requires(n@ <= 4096 && i@ < n@ && k@ < n@)]
#[requires(a@.len() == n@ * n@ && b@.len() == n@ * n@ && c@.len() == n@ * n@)]
#[ensures((^c)@.len() == c@.len())]
#[check(terminates)]
pub fn loop_j(a: &Vec<u32>, b: &Vec<u32>, c: &mut Vec<u32>, n: usize, i: usize, k: usize, aik: u32) {
    let mut j = 0usize;
    #[invariant(c@.len() == n@ * n@ && j@ <= n@)]
    #[variant(n@ - j@)]
    while j < n {
        let ci = idx(n, i, j);
        let bi = idx(n, k, j);
        c[ci] = c[ci].wrapping_add(aik.wrapping_mul(b[bi]));
        j += 1;
    }
}

#[requires(n@ <= 4096 && i@ < n@)]
#[requires(a@.len() == n@ * n@ && b@.len() == n@ * n@ && c@.len() == n@ * n@)]
#[ensures((^c)@.len() == c@.len())]
#[check(terminates)]
pub fn loop_k(a: &Vec<u32>, b: &Vec<u32>, c: &mut Vec<u32>, n: usize, i: usize) {
    let mut k = 0usize;
    #[invariant(c@.len() == n@ * n@ && k@ <= n@)]
    #[variant(n@ - k@)]
    while k < n {
        let aik = a[idx(n, i, k)];
        loop_j(a, b, c, n, i, k, aik);
        k += 1;
    }
}

#[requires(n@ <= 4096)]
#[requires(a@.len() == n@ * n@ && b@.len() == n@ * n@ && c@.len() == n@ * n@)]
#[ensures((^c)@.len() == c@.len())]
#[check(terminates)]
pub fn loop_i(a: &Vec<u32>, b: &Vec<u32>, c: &mut Vec<u32>, n: usize) {
    let mut i = 0usize;
    #[invariant(c@.len() == n@ * n@ && i@ <= n@)]
    #[variant(n@ - i@)]
    while i < n {
        loop_k(a, b, c, n, i);
        i += 1;
    }
}

#[requires(c@.len() == nn@ && nn@ <= 16777216)]
#[check(terminates)]
pub fn checksum(c: &Vec<u32>, nn: usize) -> u64 {
    let mut ck = CK_INIT;
    let mut i = 0usize;
    #[invariant(c@.len() == nn@ && i@ <= nn@)]
    #[variant(nn@ - i@)]
    while i < nn {
        ck = ckfold(ck, c[i] as u64);
        i += 1;
    }
    ck
}

#[requires(1 <= n@ && n@ <= 4096)]
pub fn run(n: usize) -> u64 {
    proof_assert!(n@ * n@ <= 16777216);
    let nn = n * n;
    let mut a = std::vec![0u32; nn];
    let mut b = std::vec![0u32; nn];
    let mut c = std::vec![0u32; nn];
    init_mat(&mut a, nn, 0);
    init_mat(&mut b, nn, nn as u64);
    loop_i(&a, &b, &mut c, n);
    checksum(&c, nn)
}
