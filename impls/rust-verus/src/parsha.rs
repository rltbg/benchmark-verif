//! parsha : T segments SHA-256 indépendants, un par thread (spec/workloads.d/parsha.md).
//!
//! **Prouvé (Verus)**, threads compris : le checksum renvoyé par `run` est `ck_segs(T, n/T)`, repliement dans
//! l'ordre s = 0..T-1 des condensés **spec** `sha256(gen_msg(SEED + s·2^40, n/T))` (fonction SHA-256 de `sha256.rs`).
//! Le noyau séquentiel est `sha256::digest` (même code que le workload `sha256`, prouvé égal à la spécification).
//! Les threads passent par `vstd::thread::spawn` / `JoinHandle::join` : le `ensures` de la fermeture lancée devient
//! le prédicat du `JoinHandle`, donc du résultat de `join`. Aucun `assume`/`external_body` dans ce module (ceux de
//! `vstd::thread`, qui enveloppent `std::thread`, font partie de la base de confiance de Verus).
//!
//! Écarts avec `rust-plain`, imposés par les spécifications de vstd :
//! * `vstd::thread::spawn` (fermeture `'static`, elle ne capture que deux entiers) au lieu de `std::thread::scope`,
//!   qui n'a pas de spécification dans vstd ;
//! * `join` peut renvoyer `Err` (le thread a paniqué) et `panic!` exige `requires false` en Verus : dans ce cas,
//!   impossible ici puisque `segment` est prouvé sans panique, le segment est recalculé sur le thread courant au lieu
//!   de `unwrap()`. Le chemin normal est identique (aucun coût) ;
//! * les threads sont lancés du dernier segment au premier pour que `Vec::pop` les rende dans l'ordre 1..T-1.
use crate::common::*;
use crate::sha256::{digest, St, ck_digest, gen_msg, sha256, st_of, SEED};
use vstd::prelude::*;
use vstd::thread::{spawn, JoinHandle};

verus! {

/// Nombre de threads effectif.
pub open spec fn nthreads(threads: int) -> int {
    if threads < 1 {
        1
    } else if threads > 64 {
        64
    } else {
        threads
    }
}

/// Graine du segment `s` : `SEED + s·2^40`.
pub open spec fn seg_base(s: int) -> u64 {
    SEED.wrapping_add((s as u64) << 40u64)
}

/// Condensé spec du segment `s` (m octets). Opaque : `run` n'a besoin que de son nom (contexte SMT réduit) ;
/// `segment` le révèle.
#[verifier::opaque]
pub open spec fn seg_digest(s: int, m: int) -> St {
    sha256(gen_msg(seg_base(s), m))
}

/// Checksum des `t` premiers segments, dans l'ordre.
pub open spec fn ck_segs(t: int, m: int) -> u64
    decreases t,
{
    if t <= 0 {
        CK_INIT
    } else {
        ck_digest(ck_segs(t - 1, m), seg_digest(t - 1, m))
    }
}

/// Condensé du segment `k` (m octets), calculé par le noyau séquentiel prouvé.
fn segment(k: usize, m: usize) -> (h: [u32; 8])
    requires
        m <= 0x100_0000_0000,
    ensures
        st_of(h@) == seg_digest(k as int, m as int),
{
    reveal(seg_digest);
    digest(SEED.wrapping_add((k as u64) << 40), m)
}

/// Repliement du condensé d'un segment dans le checksum.
fn fold_digest(ck: u64, h: &[u32; 8]) -> (r: u64)
    ensures
        r == ck_digest(ck, st_of(h@)),
{
    let mut ck = ckfold(ck, h[0] as u64);
    ck = ckfold(ck, h[1] as u64);
    ck = ckfold(ck, h[2] as u64);
    ck = ckfold(ck, h[3] as u64);
    ck = ckfold(ck, h[4] as u64);
    ck = ckfold(ck, h[5] as u64);
    ck = ckfold(ck, h[6] as u64);
    ckfold(ck, h[7] as u64)
}

pub fn run(n: usize, threads: usize) -> (ck: u64)
    requires
        n <= 0x100_0000_0000,
    ensures
        ck == ck_segs(nthreads(threads as int), n as int / nthreads(threads as int)),
{
    let t = threads.clamp(1, 64);
    assert(t == nthreads(threads as int));
    let m = n / t;
    // Threads T-1, ..., 1 : un segment chacun (hs[j] calcule le segment T-1-j).
    let mut hs: Vec<JoinHandle<[u32; 8]>> = Vec::new();
    let mut k = t;
    while k > 1
        invariant
            1 <= k <= t <= 64,
            m <= 0x100_0000_0000,
            hs.len() == t - k,
            forall|j: int, r: [u32; 8]|
                0 <= j < hs.len() && #[trigger] hs[j].predicate(r) ==> st_of(r@) == seg_digest(t - 1 - j, m as int),
        decreases k,
    {
        k -= 1;
        let s = k;
        let job = move || -> (r: [u32; 8])
            requires
                m <= 0x100_0000_0000,
            ensures
                st_of(r@) == seg_digest(s as int, m as int),
            { segment(s, m) };
        hs.push(spawn(job));
    }
    // Segment 0 sur le thread courant, puis les autres dans l'ordre 1..T-1.
    let mut ck = fold_digest(CK_INIT, &segment(0, m));
    assert(ck_segs(0, m as int) == CK_INIT);
    k = 1;
    while k < t
        invariant
            1 <= k <= t <= 64,
            m <= 0x100_0000_0000,
            hs.len() == t - k,
            ck == ck_segs(k as int, m as int),
            forall|j: int, r: [u32; 8]|
                0 <= j < hs.len() && #[trigger] hs[j].predicate(r) ==> st_of(r@) == seg_digest(t - 1 - j, m as int),
        decreases t - k,
    {
        let h = hs.pop().unwrap();
        let d = match h.join() {
            Ok(d) => d,
            Err(_) => segment(k, m),  // inatteignable : `segment` ne panique pas (voir l'en-tête)
        };
        ck = fold_digest(ck, &d);
        k += 1;
    }
    ck
}

} // verus!
