//! ptrchase : arène par indices, BST non équilibré + recherches + chasse de pointeurs
//! (voir spec/workloads.d/ptrchase.md). 1 <= n <= 2^31 - 2.
//!
//! Prouvé : absence de panique (indices dans les bornes), absence de débordement, terminaison des noyaux.
//! Les liens lus dans l'arène sont gardés (`c < cnt`, `v < n`) : pas d'invariant quantifié sur l'arène.
//! Aucune correction fonctionnelle (arbre bien formé, cycle unique) n'est prouvée.
use crate::common::*;
use creusot_std::prelude::*;

const NIL: u64 = 0xFFFF_FFFF;
const SEED1: u64 = 0xD00D000000000001;
const SEED2: u64 = 0xD00D000000000002;
const SEED3: u64 = 0xD00D000000000003;

/// Phase 1 : n insertions dans l'arène (2n u64) ; renvoie (nombre de nœuds, somme des chemins).
#[requires(1 <= n@ && n@ <= 2147483646 && a@.len() == 2 * n@)]
#[ensures((^a)@.len() == a@.len() && 1 <= result.0@ && result.0@ <= n@)]
#[check(terminates)]
pub fn build(a: &mut Vec<u64>, n: usize) -> (usize, u64) {
    a[0] = mix64(SEED1) | 1;
    a[1] = NIL | (NIL << 32);
    let mut cnt = 1usize;
    let mut plen = 0u64;
    let mut i = 1usize;
    #[invariant(1 <= i@ && i@ <= n@ && 1 <= cnt@ && cnt@ <= i@ && a@.len() == 2 * n@)]
    #[variant(n@ - i@)]
    while i < n {
        let nk = mix64(SEED1.wrapping_add(i as u64)) | 1;
        let mut cur = 0usize;
        let mut v = 0usize;
        let mut placed = false;
        #[invariant(cur@ < cnt@ && v@ <= cnt@ && cnt@ <= i@ + 1 && (placed || cnt@ <= i@) && a@.len() == 2 * n@)]
        #[variant(n@ - v@)]
        while !placed && v < cnt {
            v += 1;
            let key = a[2 * cur];
            if nk == key {
                placed = true;
            } else {
                let lnk = a[2 * cur + 1];
                let left = nk < key;
                let c = if left { lnk & 0xFFFF_FFFF } else { lnk >> 32 };
                if c < cnt as u64 {
                    cur = c as usize;
                } else {
                    a[2 * cnt] = nk;
                    a[2 * cnt + 1] = NIL | (NIL << 32);
                    a[2 * cur + 1] = if left {
                        (lnk & !0xFFFF_FFFFu64) | (cnt as u64)
                    } else {
                        (lnk & 0xFFFF_FFFF) | ((cnt as u64) << 32)
                    };
                    cnt += 1;
                    placed = true;
                }
            }
        }
        plen = plen.wrapping_add(v as u64);
        i += 1;
    }
    (cnt, plen)
}

/// Une recherche : valeur `(visites << 1) | trouvée`.
#[requires(1 <= cnt@ && cnt@ <= n@ && n@ <= 2147483646 && a@.len() == 2 * n@)]
#[check(terminates)]
pub fn search(a: &Vec<u64>, cnt: usize, n: usize, key: u64) -> u64 {
    let mut cur = 0usize;
    let mut v = 0usize;
    let mut found = 0u64;
    let mut fin = false;
    #[invariant(cur@ < cnt@ && v@ <= cnt@)]
    #[variant(cnt@ - v@)]
    while !fin && v < cnt {
        v += 1;
        let kc = a[2 * cur];
        if key == kc {
            found = 1;
            fin = true;
        } else {
            let lnk = a[2 * cur + 1];
            let c = if key < kc { lnk & 0xFFFF_FFFF } else { lnk >> 32 };
            if c < cnt as u64 {
                cur = c as usize;
            } else {
                fin = true;
            }
        }
    }
    ((v as u64) << 1) | found
}

/// Phase 2 : n succès (clés insérées) puis n/2 échecs (clés paires).
#[requires(1 <= cnt@ && cnt@ <= n@ && n@ <= 2147483646 && a@.len() == 2 * n@)]
#[check(terminates)]
pub fn search_all(a: &Vec<u64>, cnt: usize, n: usize) -> u64 {
    let mut ck = CK_INIT;
    let mut k = 0usize;
    #[invariant(k@ <= n@)]
    #[variant(n@ - k@)]
    while k < n {
        let j = (3 * (k as u64)) % (n as u64);
        ck = ckfold(ck, search(a, cnt, n, mix64(SEED1.wrapping_add(j)) | 1));
        k += 1;
    }
    let half = n / 2;
    let mut m = 0usize;
    #[invariant(m@ <= half@)]
    #[variant(half@ - m@)]
    while m < half {
        ck = ckfold(ck, search(a, cnt, n, mix64(SEED3.wrapping_add(m as u64)) & !1u64));
        m += 1;
    }
    ck
}

/// Phase 3a : permutation de Sattolo (cycle unique) de [0, n).
#[requires(1 <= n@ && n@ <= 2147483646 && next@.len() == n@)]
#[ensures((^next)@.len() == next@.len())]
#[check(terminates)]
pub fn sattolo(next: &mut Vec<u32>, n: usize) {
    let mut i = 0usize;
    #[invariant(i@ <= n@ && next@.len() == n@)]
    #[variant(n@ - i@)]
    while i < n {
        next[i] = i as u32;
        i += 1;
    }
    let mut i = n - 1;
    #[invariant(i@ < n@ && next@.len() == n@)]
    #[variant(i@)]
    while i >= 1 {
        let j = (mix64(SEED2.wrapping_add(i as u64)) % (i as u64)) as usize;
        let t = next[i];
        next[i] = next[j];
        next[j] = t;
        i -= 1;
    }
}

/// Phase 3b : 4n sauts ; renvoie r3.
#[requires(1 <= n@ && n@ <= 2147483646 && next@.len() == n@)]
#[check(terminates)]
pub fn chase(next: &Vec<u32>, n: usize) -> u64 {
    let total = 4 * (n as u64);
    let mut cur = 0usize;
    let mut sum = 0u64;
    let mut h = 0u64;
    #[invariant(cur@ < n@ && h@ <= total@ && next@.len() == n@)]
    #[variant(total@ - h@)]
    while h < total {
        let v = next[cur] as usize;
        cur = if v < n { v } else { cur };
        sum = sum.wrapping_add(cur as u64);
        h += 1;
    }
    ckfold(ckfold(CK_INIT, cur as u64), sum)
}

#[requires(1 <= n@ && n@ <= 2147483646)]
pub fn run(n: usize, threads: usize) -> u64 {
    let _ = threads;
    let mut a = std::vec![0u64; 2 * n];
    let mut next = std::vec![0u32; n];
    let (cnt, plen) = build(&mut a, n);
    let r1 = ckfold(ckfold(CK_INIT, cnt as u64), plen);
    let r2 = search_all(&a, cnt, n);
    sattolo(&mut next, n);
    let r3 = chase(&next, n);
    ckfold(ckfold(ckfold(CK_INIT, r1), r2), r3)
}
