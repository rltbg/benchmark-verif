//! Crible d'Ératosthène, 1 octet par entier. 2 <= n <= 4_000_000_000.
use crate::common::*;
use creusot_std::prelude::*;

/// Marque les multiples de `i` à partir de `j0`.
#[requires(f@.len() == n@ + 1 && 1 <= i@ && i@ <= n@ && n@ <= 4000000000 && j0@ <= n@ + i@)]
#[ensures((^f)@.len() == f@.len())]
#[check(terminates)]
pub fn mark(f: &mut Vec<u8>, n: usize, i: usize, j0: usize) {
    let mut j = j0;
    #[invariant(f@.len() == n@ + 1)]
    #[invariant(j@ <= n@ + i@)]
    #[variant(n@ + i@ - j@)]
    while j <= n {
        f[j] = 1;
        j += i;
    }
}

#[requires(f@.len() == n@ + 1 && 2 <= n@ && n@ <= 4000000000)]
#[ensures((^f)@.len() == f@.len())]
#[check(terminates)]
pub fn sieve_outer(f: &mut Vec<u8>, n: usize) {
    let mut i = 2usize;
    #[invariant(f@.len() == n@ + 1)]
    #[invariant(2 <= i@ && i@ <= n@ + 1)]
    #[variant(n@ + 1 - i@)]
    while i <= n && i <= n / i {
        if f[i] == 0 {
            mark(f, n, i, i * i);
        }
        i += 1;
    }
}

#[requires(f@.len() == n@ + 1 && 2 <= n@ && n@ <= 4000000000)]
#[check(terminates)]
pub fn count(f: &Vec<u8>, n: usize) -> u64 {
    let mut i = 2usize;
    let mut ck = CK_INIT;
    let mut cnt = 0u64;
    #[invariant(2 <= i@ && i@ <= n@ + 1)]
    #[variant(n@ + 1 - i@)]
    while i <= n {
        if f[i] == 0 {
            ck = ckfold(ck, i as u64);
            cnt = cnt.wrapping_add(1);
        }
        i += 1;
    }
    ckfold(ck, cnt)
}

#[requires(2 <= n@ && n@ <= 4000000000)]
pub fn run(n: usize) -> u64 {
    let mut f = std::vec![0u8; n + 1];
    sieve_outer(&mut f, n);
    count(&f, n)
}
