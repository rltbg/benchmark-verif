//! ptrchase : voir spec/workloads.d/ptrchase.md (arène par indices, BST + recherches + chasse de pointeurs).
const NIL: u64 = 0xFFFF_FFFF;

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
const CK_INIT: u64 = 0xCBF29CE484222325;

pub fn run(n: usize, _threads: usize) -> u64 {
    let mut a = vec![0u64; 2 * n];
    let mut next = vec![0u32; n];
    // phase 1 : insertions
    let mut cnt: u64 = 1;
    let mut plen: u64 = 0;
    a[0] = mix64(0xD00D000000000001) | 1;
    a[1] = NIL | (NIL << 32);
    for i in 1..n as u64 {
        let nk = mix64(0xD00D000000000001u64.wrapping_add(i)) | 1;
        let mut cur = 0usize;
        let mut v = 0u64;
        loop {
            v += 1;
            let key = a[2 * cur];
            if nk == key {
                break;
            }
            let lnk = a[2 * cur + 1];
            let left = nk < key;
            let c = if left { lnk & 0xFFFF_FFFF } else { lnk >> 32 };
            if c < cnt {
                cur = c as usize;
                continue;
            }
            a[2 * cnt as usize] = nk;
            a[2 * cnt as usize + 1] = NIL | (NIL << 32);
            a[2 * cur + 1] = if left { (lnk & !0xFFFF_FFFFu64) | cnt } else { (lnk & 0xFFFF_FFFF) | (cnt << 32) };
            cnt += 1;
            break;
        }
        plen = plen.wrapping_add(v);
    }
    let r1 = ckfold(ckfold(CK_INIT, cnt), plen);
    // phase 2 : recherches
    let mut r2 = CK_INIT;
    let total = n as u64 + n as u64 / 2;
    for k in 0..total {
        let key = if k < n as u64 {
            mix64(0xD00D000000000001u64.wrapping_add(3 * k % n as u64)) | 1
        } else {
            mix64(0xD00D000000000003u64.wrapping_add(k - n as u64)) & !1u64
        };
        let mut cur = 0usize;
        let mut v = 0u64;
        let mut found = 0u64;
        while v < cnt {
            v += 1;
            let kc = a[2 * cur];
            if key == kc {
                found = 1;
                break;
            }
            let lnk = a[2 * cur + 1];
            let c = if key < kc { lnk & 0xFFFF_FFFF } else { lnk >> 32 };
            if c < cnt {
                cur = c as usize;
            } else {
                break;
            }
        }
        r2 = ckfold(r2, (v << 1) | found);
    }
    // phase 3 : permutation de Sattolo puis chasse
    for (i, x) in next.iter_mut().enumerate() {
        *x = i as u32;
    }
    for i in (1..n as u64).rev() {
        let j = (mix64(0xD00D000000000002u64.wrapping_add(i)) % i) as usize;
        next.swap(i as usize, j);
    }
    let mut cur = 0u64;
    let mut sum = 0u64;
    for _ in 0..4 * n as u64 {
        cur = next[cur as usize] as u64;
        sum = sum.wrapping_add(cur);
    }
    let r3 = ckfold(ckfold(CK_INIT, cur), sum);
    ckfold(ckfold(ckfold(CK_INIT, r1), r2), r3)
}
