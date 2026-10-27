//! parsort : segments triés en parallèle puis fusions par paires (spec/workloads.d/parsort.md).
//!
//! Prouvé avec Creusot : les noyaux séquentiels `fill`, `merge`, `copy_back`, `msort`, `checksum` (absence de panique, indices dans les
//! bornes, terminaison, pas de débordement) — **mais pas** la correction fonctionnelle (le tri complet prouvé est dans `sort.rs`, workload `sort`).
//! **Non prouvé** : `run` (`#[trusted]`) qui découpe les tableaux avec `split_at_mut` et lance les `std::thread`.
use crate::common::*;
use creusot_std::prelude::*;

const SEED: u64 = 0x0123456789ABCDEF;

#[ensures((^v)@.len() == v@.len())]
#[check(terminates)]
pub fn fill(v: &mut [u32]) {
    let n = v.len();
    let mut i = 0usize;
    #[invariant(v@.len() == n@ && i@ <= n@)]
    #[variant(n@ - i@)]
    while i < n {
        v[i] = hi32(mix64(SEED.wrapping_add(i as u64)));
        i += 1;
    }
}

/// Fusionne src[lo,mid) et src[mid,hi) dans dst[lo,hi).
#[requires(src@.len() == dst@.len())]
#[requires(lo@ <= mid@ && mid@ <= hi@ && hi@ <= src@.len())]
#[ensures((^dst)@.len() == dst@.len())]
#[check(terminates)]
pub fn merge(src: &[u32], dst: &mut [u32], lo: usize, mid: usize, hi: usize) {
    let mut i = lo;
    let mut j = mid;
    let mut k = lo;
    #[invariant(src@.len() == dst@.len())]
    #[invariant(lo@ <= i@ && i@ <= mid@ && mid@ <= j@ && j@ <= hi@ && hi@ <= src@.len())]
    #[invariant(k@ <= hi@ && k@ + mid@ == i@ + j@)]
    #[variant(hi@ - k@)]
    while k < hi {
        if i < mid && (j >= hi || src[i] <= src[j]) {
            dst[k] = src[i];
            i += 1;
        } else {
            dst[k] = src[j];
            j += 1;
        }
        k += 1;
    }
}

#[requires(a@.len() == tmp@.len() && lo@ <= hi@ && hi@ <= a@.len())]
#[ensures((^a)@.len() == a@.len())]
#[check(terminates)]
pub fn copy_back(tmp: &[u32], a: &mut [u32], lo: usize, hi: usize) {
    let mut i = lo;
    #[invariant(a@.len() == tmp@.len() && lo@ <= i@ && i@ <= hi@ && hi@ <= a@.len())]
    #[variant(hi@ - i@)]
    while i < hi {
        a[i] = tmp[i];
        i += 1;
    }
}

/// Trie a[lo..hi) en place, en utilisant tmp comme espace de travail.
#[requires(a@.len() == tmp@.len() && lo@ <= hi@ && hi@ <= a@.len())]
#[ensures((^a)@.len() == a@.len() && (^tmp)@.len() == tmp@.len())]
#[check(terminates)]
#[variant(hi@ - lo@)]
pub fn msort(a: &mut [u32], tmp: &mut [u32], lo: usize, hi: usize) {
    if hi - lo < 2 {
        return;
    }
    let mid = lo + (hi - lo) / 2;
    msort(a, tmp, lo, mid);
    msort(a, tmp, mid, hi);
    merge(a, tmp, lo, mid, hi);
    copy_back(tmp, a, lo, hi);
}

/// fold des éléments dans l'ordre ; 0 si l'ordre n'est pas croissant.
#[ensures(true)]
#[check(terminates)]
pub fn checksum(v: &[u32]) -> u64 {
    let n = v.len();
    let mut ck = CK_INIT;
    let mut prev = 0u32;
    let mut ok = true;
    let mut i = 0usize;
    #[invariant(v@.len() == n@ && i@ <= n@)]
    #[variant(n@ - i@)]
    while i < n {
        let x = v[i];
        ok = ok && prev <= x;
        prev = x;
        ck = ckfold(ck, x as u64);
        i += 1;
    }
    if ok { ck } else { 0 }
}

#[trusted]
#[requires(1 <= n@ && n@ <= 1073741824)]
pub fn run(n: usize, threads: usize) -> u64 {
    let t = threads.max(1);
    let mut a = std::vec![0u32; n];
    let mut tmp = std::vec![0u32; n];
    fill(&mut a);
    let mut bounds: Vec<usize> = (0..=t).map(|k| n * k / t).collect();
    std::thread::scope(|s| {
        let (mut ra, mut rt): (&mut [u32], &mut [u32]) = (&mut a, &mut tmp);
        for k in 0..t {
            let len = bounds[k + 1] - bounds[k];
            let (ma, ta) = ra.split_at_mut(len);
            let (mt, tt) = rt.split_at_mut(len);
            ra = ta;
            rt = tt;
            if k + 1 < t {
                s.spawn(move || msort(ma, mt, 0, len));
            } else {
                msort(ma, mt, 0, len);
            }
        }
    });
    while bounds.len() > 2 {
        let runs = bounds.len() - 1;
        let pairs = runs / 2;
        std::thread::scope(|s| {
            let (mut ra, mut rt): (&mut [u32], &mut [u32]) = (&mut a, &mut tmp);
            for p in 0..pairs {
                let (lo, mid, hi) = (bounds[2 * p], bounds[2 * p + 1], bounds[2 * p + 2]);
                let (ma, ta) = ra.split_at_mut(hi - lo);
                let (mt, tt) = rt.split_at_mut(hi - lo);
                ra = ta;
                rt = tt;
                let mut job = move || {
                    merge(ma, mt, 0, mid - lo, hi - lo);
                    copy_back(mt, ma, 0, hi - lo);
                };
                if p + 1 < pairs {
                    s.spawn(job);
                } else {
                    job();
                }
            }
        });
        let mut nb: Vec<usize> = (0..pairs).map(|p| bounds[2 * p]).collect();
        if runs % 2 == 1 {
            nb.push(bounds[runs - 1]);
        }
        nb.push(n);
        bounds = nb;
    }
    checksum(&a)
}
