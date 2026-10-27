//! strsort : tri fusion stable d'indices de chaînes stockées dans un pool contigu (spec/workloads.d/strsort.md).
use crate::{ckfold, mix64, CK_INIT};

#[inline(always)]
fn len_of(i: u64) -> u64 {
    1 + (mix64(0xC0FFEE0000000001_u64.wrapping_add(i)) >> 59)
}

#[inline(always)]
fn le(pool: &[u8], off: &[u64], a: u32, b: u32) -> bool {
    let (a, b) = (a as usize, b as usize);
    let (oa, ob) = (off[a] as usize, off[b] as usize);
    let (la, lb) = ((off[a + 1] - off[a]) as usize, (off[b + 1] - off[b]) as usize);
    let m = la.min(lb);
    let (pa, pb) = (&pool[oa..oa + m], &pool[ob..ob + m]);
    for k in 0..m {
        if pa[k] != pb[k] {
            return pa[k] < pb[k];
        }
    }
    la <= lb
}

fn msort(pool: &[u8], off: &[u64], a: &mut [u32], tmp: &mut [u32], lo: usize, hi: usize) {
    if hi - lo < 2 {
        return;
    }
    let mid = lo + (hi - lo) / 2;
    msort(pool, off, a, tmp, lo, mid);
    msort(pool, off, a, tmp, mid, hi);
    let (mut i, mut j) = (lo, mid);
    for k in lo..hi {
        if i < mid && (j >= hi || le(pool, off, a[i], a[j])) {
            tmp[k] = a[i];
            i += 1;
        } else {
            tmp[k] = a[j];
            j += 1;
        }
    }
    a[lo..hi].copy_from_slice(&tmp[lo..hi]);
}

pub fn run(n: usize, _threads: usize) -> u64 {
    let mut off = vec![0u64; n + 1];
    for i in 0..n {
        off[i + 1] = off[i] + len_of(i as u64);
    }
    let total = off[n];
    let mut pool = vec![0u8; total as usize];
    for i in 0..n {
        let (o, l) = (off[i], off[i + 1] - off[i]);
        for k in 0..l {
            pool[(o + k) as usize] = b'a' + (mix64(0xC0FFEE0000000002_u64.wrapping_add(64u64.wrapping_mul(i as u64)).wrapping_add(k)) & 3) as u8;
        }
    }
    let mut a: Vec<u32> = (0..n as u32).collect();
    let mut tmp = vec![0u32; n];
    msort(&pool, &off, &mut a, &mut tmp, 0, n);
    let ck = a.iter().fold(CK_INIT, |ck, &x| ckfold(ck, x as u64));
    ckfold(ck, total)
}
