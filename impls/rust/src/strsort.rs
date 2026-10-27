//! strsort : tri fusion stable (descendant, tampon + recopie) d'indices u32 de chaînes de 1 à 32 octets stockées dans un pool
//! contigu avec des offsets (n+1 entrées, `usize` = 64 bits : mêmes valeurs que les offsets u64 des autres colonnes). 1 <= n <= 2^28.
//! Prouvé : absence de panique (tous les indices du pool et des tableaux sont dans les bornes, y compris `offset + k < taille du
//! pool` dans la comparaison, grâce à des gardes runtime), absence de débordement non voulu, terminaison des noyaux.
//! Pas de correction fonctionnelle du tri.
use crate::common::*;
use creusot_std::prelude::*;

#[check(terminates)]
#[inline(always)]
pub fn len_of(i: u64) -> u64 {
    1 + (mix64(0xC0FFEE0000000001u64.wrapping_add(i)) >> 59)
}

#[check(terminates)]
#[inline(always)]
pub fn chr_of(i: u64, k: u64) -> u8 {
    b'a'.wrapping_add((mix64(0xC0FFEE0000000002u64.wrapping_add(i.wrapping_mul(64)).wrapping_add(k)) & 3) as u8)
}

/// off[i+1] = off[i] + len_of(i) ; renvoie le total (= off[n]).
#[requires(off@.len() == n@ + 1 && n@ <= 268435456)]
#[ensures((^off)@.len() == off@.len())]
#[check(terminates)]
pub fn gen_offsets(off: &mut Vec<usize>, n: usize) -> usize {
    let mut acc = 0usize;
    let mut i = 0usize;
    #[invariant(i@ <= n@ && off@.len() == n@ + 1)]
    #[variant(n@ - i@)]
    while i < n {
        acc = acc.wrapping_add(len_of(i as u64) as usize);
        off[i + 1] = acc;
        i += 1;
    }
    acc
}

/// Octets de la chaîne i à partir de la position cumulée ; garde pos + k < taille du pool.
#[requires(n@ <= 268435456)]
#[ensures((^pool)@.len() == pool@.len())]
#[check(terminates)]
pub fn gen_pool(pool: &mut Vec<u8>, n: usize) {
    let plen = pool.len();
    let mut i = 0usize;
    let mut pos = 0usize;
    #[invariant(i@ <= n@ && pool@.len() == plen@)]
    #[variant(n@ - i@)]
    while i < n {
        let l = len_of(i as u64) as usize;
        let mut k = 0usize;
        #[invariant(k@ <= l@ && pool@.len() == plen@)]
        #[variant(l@ - k@)]
        while k < l {
            let p = pos.wrapping_add(k);
            if p < plen {
                pool[p] = chr_of(i as u64, k as u64);
            }
            k += 1;
        }
        pos = pos.wrapping_add(l);
        i += 1;
    }
}

/// a <= b pour l'ordre (octets lexicographiques, puis longueur) ; indices hors bornes remplacés par 0.
#[requires(1 <= n@ && off@.len() == n@ + 1)]
#[check(terminates)]
pub fn lex_le(pool: &Vec<u8>, off: &Vec<usize>, n: usize, a: u32, b: u32) -> bool {
    let ia = if (a as usize) < n { a as usize } else { 0 };
    let ib = if (b as usize) < n { b as usize } else { 0 };
    let (oa, oa1) = (off[ia], off[ia + 1]);
    let (ob, ob1) = (off[ib], off[ib + 1]);
    let pl = pool.len();
    let la_raw = oa1.wrapping_sub(oa);
    let lb_raw = ob1.wrapping_sub(ob);
    let oka = oa <= pl && la_raw <= pl - oa;
    let okb = ob <= pl && lb_raw <= pl - ob;
    let (sa, la) = if oka { (oa, la_raw) } else { (0, 0) };
    let (sb, lb) = if okb { (ob, lb_raw) } else { (0, 0) };
    let m = if la <= lb { la } else { lb };
    let mut k = 0usize;
    #[invariant(k@ <= m@)]
    #[variant(m@ - k@)]
    while k < m {
        let ca = pool[sa + k];
        let cb = pool[sb + k];
        if ca != cb {
            return ca < cb;
        }
        k += 1;
    }
    la <= lb
}

/// Fusionne src[lo,mid) et src[mid,hi) dans dst[lo,hi) ; stable : à égalité on prend la gauche.
#[requires(1 <= n@ && off@.len() == n@ + 1)]
#[requires(src@.len() == dst@.len())]
#[requires(lo@ <= mid@ && mid@ <= hi@ && hi@ <= src@.len())]
#[ensures((^dst)@.len() == dst@.len())]
#[check(terminates)]
pub fn merge(pool: &Vec<u8>, off: &Vec<usize>, n: usize, src: &Vec<u32>, dst: &mut Vec<u32>, lo: usize, mid: usize, hi: usize) {
    let mut i = lo;
    let mut j = mid;
    let mut k = lo;
    #[invariant(src@.len() == dst@.len())]
    #[invariant(lo@ <= i@ && i@ <= mid@ && mid@ <= j@ && j@ <= hi@ && hi@ <= src@.len())]
    #[invariant(k@ <= hi@ && k@ + mid@ == i@ + j@)]
    #[variant(hi@ - k@)]
    while k < hi {
        if i < mid && (j >= hi || lex_le(pool, off, n, src[i], src[j])) {
            dst[k] = src[i];
            i += 1;
        } else {
            dst[k] = src[j];
            j += 1;
        }
        k += 1;
    }
}

/// a[lo..hi) <- tmp[lo..hi).
#[requires(tmp@.len() == a@.len() && lo@ <= hi@ && hi@ <= a@.len())]
#[ensures((^a)@.len() == a@.len())]
#[check(terminates)]
pub fn copy_back(tmp: &Vec<u32>, a: &mut Vec<u32>, lo: usize, hi: usize) {
    let mut i = lo;
    #[invariant(lo@ <= i@ && i@ <= hi@ && hi@ <= a@.len() && tmp@.len() == a@.len())]
    #[variant(hi@ - i@)]
    while i < hi {
        a[i] = tmp[i];
        i += 1;
    }
}

#[requires(1 <= n@ && off@.len() == n@ + 1)]
#[requires(a@.len() == tmp@.len() && lo@ <= hi@ && hi@ <= a@.len())]
#[ensures((^a)@.len() == a@.len() && (^tmp)@.len() == tmp@.len())]
#[check(terminates)]
#[variant(hi@ - lo@)]
pub fn msort(pool: &Vec<u8>, off: &Vec<usize>, n: usize, a: &mut Vec<u32>, tmp: &mut Vec<u32>, lo: usize, hi: usize) {
    if hi - lo < 2 {
        return;
    }
    let mid = lo + (hi - lo) / 2;
    msort(pool, off, n, a, tmp, lo, mid);
    msort(pool, off, n, a, tmp, mid, hi);
    merge(pool, off, n, a, tmp, lo, mid, hi);
    copy_back(tmp, a, lo, hi);
}

#[requires(a@.len() == n@)]
#[check(terminates)]
pub fn checksum(a: &Vec<u32>, n: usize, total: u64) -> u64 {
    let mut ck = CK_INIT;
    let mut i = 0usize;
    #[invariant(i@ <= n@ && a@.len() == n@)]
    #[variant(n@ - i@)]
    while i < n {
        ck = ckfold(ck, a[i] as u64);
        i += 1;
    }
    ckfold(ck, total)
}

#[requires(1 <= n@ && n@ <= 268435456)]
pub fn run(n: usize, _threads: usize) -> u64 {
    let mut off = std::vec![0usize; n + 1];
    let total = gen_offsets(&mut off, n);
    let mut pool = std::vec![0u8; total];
    gen_pool(&mut pool, n);
    let mut a = std::vec![0u32; n];
    let mut i = 0usize;
    #[invariant(i@ <= n@ && a@.len() == n@)]
    #[variant(n@ - i@)]
    while i < n {
        a[i] = i as u32;
        i += 1;
    }
    let mut tmp = std::vec![0u32; n];
    msort(&pool, &off, n, &mut a, &mut tmp, 0, n);
    checksum(&a, n, total as u64)
}
