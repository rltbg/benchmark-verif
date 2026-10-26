//! Table de hachage à adressage ouvert, sondage linéaire, couples (clé, valeur) entrelacés dans un
//! seul tableau de 2*cap u64 (cap = puissance de 2 >= 2n). 1 <= n <= 2^26.
use crate::common::*;
use creusot_std::prelude::*;

const SEED: u64 = 0x2468ACE013579BDF;

#[check(terminates)]
#[inline(always)]
pub fn key_of(off: u64, i: u64) -> u64 {
    mix64(SEED.wrapping_add(off).wrapping_add(i)) | 1
}

/// Position initiale : (key * phi) >> (64 - lg), ramenée dans [0, cap).
#[requires(1 <= lg@ && lg@ <= 27 && 2 <= cap@)]
#[ensures(result@ < cap@)]
#[check(terminates)]
pub fn slot(key: u64, lg: u32, cap: usize) -> usize {
    let s = (key.wrapping_mul(0x9E3779B97F4A7C15) >> (64 - lg)) as usize;
    if s < cap { s } else { 0 }
}

/// Recherche d'un emplacement libre ou portant déjà `key` (au plus cap essais) ; y écrit ou ajoute i1.
#[requires(2 <= cap@ && cap@ <= 134217728 && t@.len() == 2 * cap@ && 1 <= lg@ && lg@ <= 27)]
#[ensures((^t)@.len() == t@.len())]
#[check(terminates)]
pub fn insert(t: &mut Vec<u64>, cap: usize, lg: u32, key: u64, i1: u64) {
    let mut s = slot(key, lg, cap);
    let mut tries = 0usize;
    #[invariant(s@ < cap@ && tries@ <= cap@ && t@.len() == 2 * cap@)]
    #[variant(cap@ - tries@)]
    while tries < cap {
        let k = t[2 * s];
        if k == 0 {
            t[2 * s] = key;
            t[2 * s + 1] = i1;
            return;
        }
        if k == key {
            t[2 * s + 1] = t[2 * s + 1].wrapping_add(i1);
            return;
        }
        s += 1;
        if s == cap {
            s = 0;
        }
        tries += 1;
    }
}

#[requires(2 <= cap@ && cap@ <= 134217728 && t@.len() == 2 * cap@ && 1 <= lg@ && lg@ <= 27)]
#[check(terminates)]
pub fn lookup(t: &Vec<u64>, cap: usize, lg: u32, key: u64) -> u64 {
    let mut s = slot(key, lg, cap);
    let mut tries = 0usize;
    #[invariant(s@ < cap@ && tries@ <= cap@ && t@.len() == 2 * cap@)]
    #[variant(cap@ - tries@)]
    while tries < cap {
        let k = t[2 * s];
        if k == 0 {
            return 0;
        }
        if k == key {
            return t[2 * s + 1];
        }
        s += 1;
        if s == cap {
            s = 0;
        }
        tries += 1;
    }
    0
}

#[requires(2 <= cap@ && cap@ <= 134217728 && t@.len() == 2 * cap@ && 1 <= lg@ && lg@ <= 27 && n@ <= 67108864)]
#[ensures((^t)@.len() == t@.len())]
#[check(terminates)]
pub fn insert_all(t: &mut Vec<u64>, cap: usize, lg: u32, n: usize) {
    let mut i = 0usize;
    #[invariant(i@ <= n@ && t@.len() == 2 * cap@)]
    #[variant(n@ - i@)]
    while i < n {
        insert(t, cap, lg, key_of(0, i as u64), (i as u64).wrapping_add(1));
        i += 1;
    }
}

#[requires(2 <= cap@ && cap@ <= 134217728 && t@.len() == 2 * cap@ && 1 <= lg@ && lg@ <= 27 && n@ <= 67108864)]
#[check(terminates)]
pub fn lookup_all(t: &Vec<u64>, cap: usize, lg: u32, n: usize, off: u64, ck0: u64) -> u64 {
    let mut ck = ck0;
    let mut i = 0usize;
    #[invariant(i@ <= n@ && t@.len() == 2 * cap@)]
    #[variant(n@ - i@)]
    while i < n {
        ck = ckfold(ck, lookup(t, cap, lg, key_of(off, i as u64)));
        i += 1;
    }
    ck
}

#[requires(1 <= n@ && n@ <= 67108864)]
pub fn run(n: usize) -> u64 {
    let n2 = n + n;
    let mut cap = 2usize;
    let mut lg = 1u32;
    #[invariant(2 <= cap@ && cap@ <= 134217728 && 1 <= lg@ && lg@ <= 27)]
    #[variant(27 - lg@)]
    while cap < n2 && lg < 27 && cap <= 33554432 {
        cap += cap;
        lg += 1;
    }
    let mut t = std::vec![0u64; 2 * cap];
    insert_all(&mut t, cap, lg, n);
    let ck1 = lookup_all(&t, cap, lg, n, 0, CK_INIT);
    lookup_all(&t, cap, lg, n, n as u64, ck1)
}
