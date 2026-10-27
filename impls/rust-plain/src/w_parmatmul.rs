//! parmatmul : produit matriciel réparti en blocs de lignes sur T threads (spec/workloads.d/parmatmul.md).
use crate::{ckfold, mix64, CK_INIT};

/// Noyau séquentiel : `c` = lignes i0..i1 de C (taille (i1-i0)*n), `a` = lignes i0..i1 de A.
fn rows(a: &[u32], b: &[u32], c: &mut [u32], n: usize) {
    for (arow, crow) in a.chunks_exact(n).zip(c.chunks_exact_mut(n)) {
        for (k, &aik) in arow.iter().enumerate() {
            let brow = &b[k * n..(k + 1) * n];
            for (cv, &bv) in crow.iter_mut().zip(brow) {
                *cv = cv.wrapping_add(aik.wrapping_mul(bv));
            }
        }
    }
}

pub fn run(n: usize, threads: usize) -> u64 {
    let t = threads.max(1);
    let nn = n * n;
    let seed = 0x0FEDCBA987654321_u64;
    let a: Vec<u32> = (0..nn as u64).map(|i| (mix64(seed.wrapping_add(i)) >> 32) as u32).collect();
    let b: Vec<u32> = (0..nn as u64).map(|i| (mix64(seed.wrapping_add(nn as u64).wrapping_add(i)) >> 32) as u32).collect();
    let mut c = vec![0u32; nn];
    std::thread::scope(|s| {
        let mut rest: &mut [u32] = &mut c;
        for k in 0..t {
            let (r0, r1) = (n * k / t, n * (k + 1) / t);
            let (mine, tail) = rest.split_at_mut((r1 - r0) * n);
            rest = tail;
            let (a, b) = (&a, &b);
            if k + 1 < t {
                s.spawn(move || rows(&a[r0 * n..r1 * n], b, mine, n));
            } else {
                rows(&a[r0 * n..r1 * n], b, mine, n);
            }
        }
    });
    c.iter().fold(CK_INIT, |ck, &x| ckfold(ck, x as u64))
}
