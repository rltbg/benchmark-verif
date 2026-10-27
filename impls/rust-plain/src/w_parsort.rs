//! parsort : segments triés en parallèle puis fusions par paires (spec/workloads.d/parsort.md).
use crate::{ckfold, mix64, CK_INIT};

fn merge(src: &[u32], dst: &mut [u32], lo: usize, mid: usize, hi: usize) {
    let (mut i, mut j) = (lo, mid);
    for k in lo..hi {
        if i < mid && (j >= hi || src[i] <= src[j]) {
            dst[k] = src[i];
            i += 1;
        } else {
            dst[k] = src[j];
            j += 1;
        }
    }
}
fn msort(a: &mut [u32], tmp: &mut [u32], lo: usize, hi: usize) {
    if hi - lo < 2 {
        return;
    }
    let mid = lo + (hi - lo) / 2;
    msort(a, tmp, lo, mid);
    msort(a, tmp, mid, hi);
    merge(a, tmp, lo, mid, hi);
    a[lo..hi].copy_from_slice(&tmp[lo..hi]);
}

pub fn run(n: usize, threads: usize) -> u64 {
    let t = threads.max(1);
    let mut a: Vec<u32> = (0..n as u64).map(|i| (mix64(0x0123456789ABCDEF_u64.wrapping_add(i)) >> 32) as u32).collect();
    let mut tmp = vec![0u32; n];
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
    // fusions par paires, niveau par niveau ; les fusions d'un niveau tournent en parallèle
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
                    ma.copy_from_slice(mt);
                };
                if p + 1 < pairs { s.spawn(job); } else { job(); }
            }
        });
        let mut nb: Vec<usize> = (0..pairs).map(|p| bounds[2 * p]).collect();
        if runs % 2 == 1 {
            nb.push(bounds[runs - 1]);
        }
        nb.push(n);
        bounds = nb;
    }
    let (mut ck, mut prev, mut ok) = (CK_INIT, 0u32, true);
    for &x in &a {
        ok &= prev <= x;
        prev = x;
        ck = ckfold(ck, x as u64);
    }
    if ok { ck } else { 0 }
}
