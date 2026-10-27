//! parmatmul : produit matriciel réparti en blocs de lignes sur T threads (spec/workloads.d/parmatmul.md).
//!
//! Prouvé avec Creusot : le noyau séquentiel `rows` (absence de panique, indices dans les bornes pour 1 <= n <= 16384, absence de
//! débordement). **Non prouvé** : `run` (`#[trusted]`) qui alloue les matrices, répartit les lignes sur des `std::thread` avec
//! `split_at_mut` et enchaîne les noyaux : les threads ne sont pas modélisés par Creusot.
use crate::common::*;
use creusot_std::prelude::*;

const SEED: u64 = 0x0FEDCBA987654321;

/// Indice i*n+j dans une matrice de m lignes et n colonnes, strictement inférieur à m*n.
#[requires(n@ <= 16384 && m@ <= 16384 && i@ < m@ && j@ < n@)]
#[ensures(result@ == i@ * n@ + j@ && result@ < m@ * n@)]
#[check(terminates)]
pub fn idx(n: usize, m: usize, i: usize, j: usize) -> usize {
    proof_assert!((i@ + 1) * n@ <= m@ * n@);
    proof_assert!(m@ * n@ <= 268435456);
    i * n + j
}

#[requires(m@.len() == len@)]
#[ensures((^m)@.len() == len@)]
#[check(terminates)]
pub fn init_mat(m: &mut [u32], len: usize, off: u64) {
    let mut i = 0usize;
    #[invariant(m@.len() == len@ && i@ <= len@)]
    #[variant(len@ - i@)]
    while i < len {
        m[i] = hi32(mix64(SEED.wrapping_add(off).wrapping_add(i as u64)));
        i += 1;
    }
}

/// Une ligne i de C : c[i,:] += a[i,k] * b[k,:] pour tout k.
#[requires(n@ <= 16384 && m@ <= 16384 && i@ < m@)]
#[requires(a@.len() == m@ * n@ && b@.len() == n@ * n@ && c@.len() == m@ * n@)]
#[ensures((^c)@.len() == c@.len())]
#[check(terminates)]
pub fn row(a: &[u32], b: &[u32], c: &mut [u32], n: usize, m: usize, i: usize) {
    let mut k = 0usize;
    #[invariant(c@.len() == m@ * n@ && k@ <= n@)]
    #[variant(n@ - k@)]
    while k < n {
        let aik = a[idx(n, m, i, k)];
        let mut j = 0usize;
        #[invariant(c@.len() == m@ * n@ && j@ <= n@)]
        #[variant(n@ - j@)]
        while j < n {
            let ci = idx(n, m, i, j);
            let bi = idx(n, n, k, j);
            c[ci] = c[ci].wrapping_add(aik.wrapping_mul(b[bi]));
            j += 1;
        }
        k += 1;
    }
}

/// Noyau séquentiel appelé par chaque thread : `a` et `c` sont les m lignes du bloc (m*n éléments), `b` est la matrice entière.
#[requires(n@ <= 16384 && m@ <= 16384)]
#[requires(a@.len() == m@ * n@ && b@.len() == n@ * n@ && c@.len() == m@ * n@)]
#[ensures((^c)@.len() == c@.len())]
#[check(terminates)]
pub fn rows(a: &[u32], b: &[u32], c: &mut [u32], n: usize, m: usize) {
    let mut i = 0usize;
    #[invariant(c@.len() == m@ * n@ && i@ <= m@)]
    #[variant(m@ - i@)]
    while i < m {
        row(a, b, c, n, m, i);
        i += 1;
    }
}

#[requires(c@.len() == nn@)]
#[check(terminates)]
pub fn checksum(c: &[u32], nn: usize) -> u64 {
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

#[trusted]
#[requires(1 <= n@ && n@ <= 16384)]
pub fn run(n: usize, threads: usize) -> u64 {
    let t = threads.max(1);
    let nn = n * n;
    let mut a = std::vec![0u32; nn];
    let mut b = std::vec![0u32; nn];
    let mut c = std::vec![0u32; nn];
    init_mat(&mut a, nn, 0);
    init_mat(&mut b, nn, nn as u64);
    std::thread::scope(|s| {
        let mut rest: &mut [u32] = &mut c;
        for k in 0..t {
            let (r0, r1) = (n * k / t, n * (k + 1) / t);
            let (mine, tail) = rest.split_at_mut((r1 - r0) * n);
            rest = tail;
            let (a, b) = (&a, &b);
            if k + 1 < t {
                s.spawn(move || rows(&a[r0 * n..r1 * n], b, mine, n, r1 - r0));
            } else {
                rows(&a[r0 * n..r1 * n], b, mine, n, r1 - r0);
            }
        }
    });
    checksum(&c, nn)
}
