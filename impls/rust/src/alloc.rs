//! Allocation en masse (spec/workloads.d/alloc.md) : n blocs de 8..256 octets, un bloc sur 16 conservé dans une
//! fenêtre circulaire de W = min(max(1, n/64), 2^25) blocs.
//!
//! * Prouvé (Creusot) : absence de panique et de débordement ; le remplissage préserve la taille du bloc ;
//!   l'indice de l'anneau est toujours dans les bornes (`kept % w < w`, `ring@.len() == w`).
//! * `#[trusted]` : la barrière d'optimisation `barrier` (black_box, sans effet logique).
//! * Non vérifié : la terminaison de `run` (elle alloue : `from_elem`, `push` ne sont pas `check(terminates)`) ;
//!   la libération des blocs est faite par `Drop`, que Creusot modélise sans la spécifier.
use crate::common::*;
use creusot_std::prelude::*;

const WMAX: usize = 1 << 25;

/// Barrière d'optimisation : le compilateur ne peut pas éliminer l'allocation du bloc (non vérifié, sans effet logique).
#[trusted]
#[check(terminates)]
#[inline(never)]
pub fn barrier(b: &Vec<u8>) {
    std::hint::black_box(b);
}

/// Remplit `b[k] = (i + k) mod 251`.
#[requires(b@.len() <= 256)]
#[ensures((^b)@.len() == b@.len())]
#[check(terminates)]
pub fn fill(b: &mut Vec<u8>, i: u64) {
    let mut v = (i % 251) as u8;
    let mut k = 0usize;
    let n = b.len();
    #[invariant(b@.len() == n@)]
    #[invariant(k@ <= n@)]
    #[invariant(v@ <= 250)]
    #[variant(n@ - k@)]
    while k < n {
        b[k] = v;
        v = if v == 250 { 0 } else { v + 1 };
        k += 1;
    }
}

/// Somme des octets du bloc (modulo 2^64).
#[requires(b@.len() <= 256)]
#[check(terminates)]
pub fn sum(b: &Vec<u8>) -> u64 {
    let mut s = 0u64;
    let mut k = 0usize;
    let n = b.len();
    #[invariant(b@.len() == n@)]
    #[invariant(k@ <= n@)]
    #[variant(n@ - k@)]
    while k < n {
        s = s.wrapping_add(b[k] as u64);
        k += 1;
    }
    s
}

#[requires(1 <= n@ && n@ <= 4000000000)]
pub fn run(n: usize) -> u64 {
    let mut w = n >> 6;
    if w < 1 {
        w = 1;
    }
    if w > WMAX {
        w = WMAX;
    }
    let mut ring: Vec<Option<Vec<u8>>> = Vec::new();
    let mut j = 0usize;
    #[invariant(ring@.len() == j@)]
    #[invariant(j@ <= w@)]
    while j < w {
        ring.push(None);
        j += 1;
    }
    let mut ck = CK_INIT;
    let mut kept = 0usize;
    let mut i = 0usize;
    #[invariant(ring@.len() == w@)]
    #[invariant(1 <= w@)]
    #[invariant(i@ <= n@)]
    while i < n {
        let r = mix64(0xA110C00000000001u64.wrapping_add(i as u64));
        let s = 8 + 8 * ((r & 31) as usize);
        let mut b = std::vec![0u8; s];
        barrier(&b);
        fill(&mut b, i as u64);
        ck = ckfold(ck, sum(&b));
        if i % 16 == 0 {
            let slot = kept % w;
            ring[slot] = Some(b);
            kept = kept.wrapping_add(1);
        } else {
            barrier(&b);
        }
        i += 1;
    }
    ckfold(ck, kept as u64)
}
