//! Workload `ntt` (voir spec/workloads.d/ntt.md). Rust idiomatique, sans preuve.
const P: u64 = 2013265921;
const G: u64 = 31;
const CK_INIT: u64 = 0xCBF29CE484222325;

#[inline(always)]
fn mix64(x: u64) -> u64 {
    let mut z = x.wrapping_add(0x9E3779B97F4A7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}
#[inline(always)]
fn ckfold(ck: u64, v: u64) -> u64 {
    (ck ^ v).wrapping_mul(0x100000001B3)
}

fn powmod(mut b: u64, mut e: u64) -> u64 {
    let mut r = 1u64;
    while e > 0 {
        if e % 2 == 1 {
            r = r * b % P;
        }
        b = b * b % P;
        e /= 2;
    }
    r
}

fn transform(a: &mut [u32], tw: &[u32], n: usize, k: usize) {
    for i in 0..n {
        let (mut x, mut rev) = (i, 0usize);
        for _ in 0..k {
            rev = rev * 2 + x % 2;
            x /= 2;
        }
        if i < rev {
            a.swap(i, rev);
        }
    }
    let mut len = 2;
    while len <= n {
        let half = len / 2;
        let stride = n / len;
        for blk in 0..stride {
            let start = blk * len;
            for j in 0..half {
                let u = a[start + j] as u64;
                let v = a[start + j + half] as u64 * tw[j * stride] as u64 % P;
                a[start + j] = ((u + v) % P) as u32;
                a[start + j + half] = ((u + P - v) % P) as u32;
            }
        }
        len *= 2;
    }
}

pub fn run(n: usize, _threads: usize) -> u64 {
    if n > 1 << 27 {
        return 0;
    }
    let (mut pw, mut k) = (1usize, 0usize);
    while pw < n {
        pw *= 2;
        k += 1;
    }
    if pw != n {
        return 0;
    }
    let mut a: Vec<u32> = (0..n as u64).map(|i| (mix64(0xA5A5F00D12345678_u64.wrapping_add(i)) % P) as u32).collect();
    let mut tw = vec![0u32; n / 2 + 1];
    let w = powmod(G, (P - 1) / n as u64);
    let mut c = 1u64;
    for j in 0..n / 2 {
        tw[j] = c as u32;
        c = c * w % P;
    }
    transform(&mut a, &tw, n, k);
    for x in a.iter_mut() {
        *x = (*x as u64 * *x as u64 % P) as u32;
    }
    transform(&mut a, &tw, n, k);
    let mut i = 1;
    while 2 * i < n {
        a.swap(i, n - i);
        i += 1;
    }
    let ninv = powmod(n as u64, P - 2);
    a.iter().fold(CK_INIT, |ck, &x| ckfold(ck, x as u64 * ninv % P))
}
