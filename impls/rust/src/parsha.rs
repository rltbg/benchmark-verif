//! parsha : T segments SHA-256 indépendants, un par thread (spec/workloads.d/parsha.md).
//!
//! **Prouvé (Creusot)**, threads compris : le checksum renvoyé par `run` est `ck_segs(T, n/T)`, repliement dans
//! l'ordre s = 0..T-1 des condensés **logiques** `sha256(gen_msg(SEED + s·2^40, n/T))` (fonction SHA-256 de
//! `sha256.rs`). Le noyau séquentiel est `sha256::digest` (même code que le workload `sha256`, prouvé égal à la
//! fonction logique). Les threads passent par les enveloppes spécifiées de creusot-std
//! (`creusot_std::std::thread::spawn` / `JoinHandleExt::join_unwrap`) : le contrat de la fermeture lancée est
//! transmis au résultat de `join_unwrap`. Aucun `trusted`/`assume` dans ce module ; la confiance repose sur ces
//! deux enveloppes `#[trusted]` de la bibliothèque (spécification de `std::thread`).
//!
//! Différence avec `rust-plain` : `std::thread::spawn` (fermeture `'static`, elle ne capture que deux entiers) au
//! lieu de `std::thread::scope`, que creusot-std spécifie aussi mais avec une fermeture de portée plus lourde à
//! prouver ; le travail fait par chaque thread et l'ordre du repliement sont identiques.
use crate::common::CK_INIT;
use crate::sha256::{SEED, ckfold, digest};
#[cfg(creusot)]
use crate::sha256::{St, ck_digest, gen_msg, sha256, st_of, u64_of, u64_of_view};
use creusot_std::ghost::invariant::Tokens;
use creusot_std::prelude::*;
use creusot_std::std::thread::{JoinHandleExt, spawn};
use std::thread::JoinHandle;

/// Nombre de threads effectif.
#[logic(open)]
pub fn nthreads(threads: Int) -> Int {
    if threads < 1 { 1 } else if threads > 64 { 64 } else { threads }
}

/// Graine du segment `s` : `SEED + s·2^40`.
#[logic(open)]
pub fn seg_base(s: Int) -> u64 {
    SEED + (u64_of(s) << 40u64)
}

/// Condensé logique du segment `s` (m octets).
#[logic(open)]
pub fn seg_digest(s: Int, m: Int) -> St {
    sha256(gen_msg(seg_base(s), m))
}

/// Checksum des `t` premiers segments, dans l'ordre.
#[logic(open)]
#[variant(t)]
pub fn ck_segs(t: Int, m: Int) -> u64 {
    if t <= 0 { CK_INIT } else { ck_digest(ck_segs(t - 1, m), seg_digest(t - 1, m)) }
}

/// Condensé du segment `k` (m octets), calculé par le noyau séquentiel prouvé.
#[requires(m@ <= 1099511627776)]
#[ensures(st_of(result@) == seg_digest(k@, m@))]
#[check(terminates)]
fn segment(k: usize, m: usize) -> [u32; 8] {
    proof_assert! { u64_of_view(k as u64); true };
    digest(SEED.wrapping_add((k as u64) << 40), m)
}

/// Repliement du condensé d'un segment dans le checksum.
#[ensures(result == ck_digest(ck, st_of(h@)))]
#[check(terminates)]
fn fold_digest(ck: u64, h: &[u32; 8]) -> u64 {
    let mut ck = ckfold(ck, h[0] as u64);
    ck = ckfold(ck, h[1] as u64);
    ck = ckfold(ck, h[2] as u64);
    ck = ckfold(ck, h[3] as u64);
    ck = ckfold(ck, h[4] as u64);
    ck = ckfold(ck, h[5] as u64);
    ck = ckfold(ck, h[6] as u64);
    ckfold(ck, h[7] as u64)
}

#[requires(n@ <= 1099511627776)]
#[ensures(result == ck_segs(nthreads(threads@), n@ / nthreads(threads@)))]
pub fn run(n: usize, threads: usize) -> u64 {
    let t = threads.clamp(1, 64);
    let m = n / t;
    // Threads 1..t-1 : un segment chacun.
    let mut hs: Vec<JoinHandle<[u32; 8]>> = Vec::new();
    let mut k = 1usize;
    #[invariant(1 <= k@ && k@ <= t@ && hs@.len() == k@ - 1)]
    #[invariant(forall<j, r: [u32; 8]> 0 <= j && j < hs@.len() && hs@[j].valid_result(r)
        ==> r@.len() == 8 && st_of(r@) == seg_digest(j + 1, m@))]
    while k < t {
        // Pas de contrat écrit (attributs sur expression instables en Rust stable) : Creusot infère le contrat
        // de la fermeture à partir de celui de `segment`.
        let job = move |_tokens: Ghost<Tokens>| segment(k, m);
        let hd = spawn(job);
        proof_assert!(forall<r: [u32; 8]> hd.valid_result(r) ==> r@.len() == 8 && st_of(r@) == seg_digest(k@, m@));
        let old = snapshot!(hs@);
        let hds = snapshot!(hd);
        hs.push(hd);
        proof_assert!(hs@.len() == old.len() + 1 && hs@[k@ - 1] == *hds && forall<j> 0 <= j && j < old.len() ==> hs@[j] == old[j]);
        k += 1;
    }
    // Segment 0 sur le thread courant, puis les autres dans l'ordre.
    let mut ck = fold_digest(CK_INIT, &segment(0, m));
    let handles = snapshot!(hs@);
    let mut it = hs.into_iter();
    proof_assert!(handles.subsequence(0, handles.len()).ext_eq(*handles));
    k = 1;
    #[invariant(1 <= k@ && k@ <= t@ && ck == ck_segs(k@, m@))]
    #[invariant(it@ == handles.subsequence(k@ - 1, handles.len()))]
    loop {
        let before = snapshot!(it@);
        match it.next() {
            Some(h) => {
                proof_assert!(h == handles[k@ - 1]);
                proof_assert!(it@.ext_eq(before.subsequence(1, before.len())));
                proof_assert!(it@.ext_eq(handles.subsequence(k@, handles.len())));
                let d = h.join_unwrap();
                ck = fold_digest(ck, &d);
                k += 1;
            }
            None => break,
        }
    }
    proof_assert!(k@ == t@);
    ck
}
