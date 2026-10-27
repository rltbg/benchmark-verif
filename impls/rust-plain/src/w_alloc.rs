//! Workload `alloc` (spec/workloads.d/alloc.md) : blocs réels du tas (`Vec`/boîtes), black_box pour
//! empêcher LLVM d'éliminer les paires allocation/libération.
use std::hint::black_box;

const WMAX: usize = 1 << 25;

#[inline(always)]
fn mix64(x: u64) -> u64 {
    let mut z = x.wrapping_add(0x9E3779B97F4A7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

pub fn run(n: usize, _threads: usize) -> u64 {
    let w = (n >> 6).clamp(1, WMAX);
    let mut ring: Vec<Option<Box<[u8]>>> = (0..w).map(|_| None).collect();
    let mut ck = 0xCBF29CE484222325u64;
    let mut kept = 0usize;
    for i in 0..n {
        let r = mix64(0xA110C00000000001u64.wrapping_add(i as u64));
        let s = 8 + 8 * (r & 31) as usize;
        let mut b = vec![0u8; s].into_boxed_slice();
        black_box(&mut b);
        let mut v = (i % 251) as u8;
        for x in b.iter_mut() {
            *x = v;
            v = if v == 250 { 0 } else { v + 1 };
        }
        let sum: u64 = b.iter().map(|&x| x as u64).sum();
        ck = (ck ^ sum).wrapping_mul(0x100000001B3);
        if i & 15 == 0 {
            ring[kept % w] = Some(b);
            kept += 1;
        } else {
            black_box(&b);
            drop(b);
        }
    }
    drop(ring);
    (ck ^ kept as u64).wrapping_mul(0x100000001B3)
}
