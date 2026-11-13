//! sort : tri fusion **descendant** (récursif) avec tampon et recopie, 1 <= n <= 2^30 (spec/WORKLOADS.md §1).
//!
//! Prouvé avec Creusot (correction fonctionnelle complète, terminaison de toutes les fonctions) :
//!   * `msort(a, tmp, lo, hi)` : `a[lo..hi)` est trié, c'est une permutation de la plage d'origine, rien hors de la plage ne change ;
//!   * `merge` : fusion de deux plages triées = plage triée, permutation de l'entrée ;
//!   * absence de panique (indices dans les bornes) et de débordement arithmétique.
//! Les noyaux travaillent sur des tranches pour être réutilisés tels quels par `parsort`.
//! Aucun `#[trusted]`, aucun `assume`.
use crate::common::*;
use creusot_std::prelude::*;

const SEED: u64 = 0x0123456789ABCDEF;

/// `s[lo..hi)` est une permutation de `t[lo..hi)`.
#[logic(open)]
pub fn perm_range(s: Seq<u32>, t: Seq<u32>, lo: Int, hi: Int) -> bool {
    pearlite! { s[lo..hi].permutation_of(t[lo..hi]) }
}

/// `s` et `t` ont la même longueur et coïncident hors de `[lo, hi)`.
#[logic(open)]
pub fn same_outside(s: Seq<u32>, t: Seq<u32>, lo: Int, hi: Int) -> bool {
    pearlite! { s.len() == t.len() && forall<p> 0 <= p && p < s.len() && (p < lo || hi <= p) ==> s[p] == t[p] }
}

/// Lemme : déplacer la tête `x` de la file gauche vers la fin de l'accumulateur conserve la permutation.
#[logic]
#[ensures(acc.snoc(x).concat(r1).concat(r2).permutation_of(acc.concat(r1.cons(x)).concat(r2)))]
pub fn perm_move_left(acc: Seq<u32>, x: u32, r1: Seq<u32>, r2: Seq<u32>) {}

/// Lemme : idem pour la tête de la file droite.
#[logic]
#[ensures(acc.snoc(x).concat(r1).concat(r2).permutation_of(acc.concat(r1).concat(r2.cons(x))))]
pub fn perm_move_right(acc: Seq<u32>, x: u32, r1: Seq<u32>, r2: Seq<u32>) {}

/// Lemme : la concaténation de deux permutations est une permutation de la concaténation.
#[logic]
#[requires(a.permutation_of(b) && c.permutation_of(d))]
#[ensures(a.concat(c).permutation_of(b.concat(d)))]
pub fn perm_concat(a: Seq<u32>, b: Seq<u32>, c: Seq<u32>, d: Seq<u32>) {}

/// `v[i] = hi32(mix64(SEED + i))`.
#[ensures((^v)@.len() == v@.len())]
#[check(terminates)]
pub fn fill(v: &mut [u32]) {
    let n = v.len();
    let mut i = 0usize;
    #[invariant(v@.len() == n@)]
    #[variant(n@ - i@)]
    while i < n {
        v[i] = hi32(mix64(SEED.wrapping_add(i as u64)));
        i += 1;
    }
}

/// Fusionne `src[lo..mid)` et `src[mid..hi)` (triés) dans `dst[lo..hi)` ; stable (à égalité, la gauche d'abord).
#[requires(src@.len() == dst@.len())]
#[requires(lo@ <= mid@ && mid@ <= hi@ && hi@ <= src@.len())]
#[requires(src@.sorted_range(lo@, mid@) && src@.sorted_range(mid@, hi@))]
#[ensures((^dst)@.sorted_range(lo@, hi@))]
#[ensures(perm_range((^dst)@, src@, lo@, hi@))]
#[ensures(same_outside((^dst)@, dst@, lo@, hi@))]
#[check(terminates)]
pub fn merge(src: &[u32], dst: &mut [u32], lo: usize, mid: usize, hi: usize) {
    let old_dst = snapshot!(dst@);
    proof_assert!(src@[lo@..hi@] == src@[lo@..mid@].concat(src@[mid@..hi@]));
    proof_assert!(Seq::<u32>::empty().concat(src@[lo@..mid@]) == src@[lo@..mid@]);
    let mut i = lo;
    let mut j = mid;
    let mut k = lo;
    // dst[lo..k) est trié, ne contient que des éléments <= aux restes des deux files,
    // et dst[lo..k) ++ src[i..mid) ++ src[j..hi) est une permutation de src[lo..hi).
    #[invariant(lo@ <= i@ && i@ <= mid@ && mid@ <= j@ && j@ <= hi@ && k@ + mid@ == i@ + j@)]
    #[invariant(dst@.sorted_range(lo@, k@))]
    #[invariant(forall<p, q> lo@ <= p && p < k@ && i@ <= q && q < mid@ ==> dst@[p] <= src@[q])]
    #[invariant(forall<p, q> lo@ <= p && p < k@ && j@ <= q && q < hi@ ==> dst@[p] <= src@[q])]
    #[invariant(dst@[lo@..k@].concat(src@[i@..mid@]).concat(src@[j@..hi@]).permutation_of(src@[lo@..hi@]))]
    #[invariant(same_outside(dst@, *old_dst, lo@, hi@))]
    #[variant(hi@ - k@)]
    while k < hi {
        let before = snapshot!(dst@);
        if i < mid && (j >= hi || src[i] <= src[j]) {
            dst[k] = src[i];
            proof_assert!(src@[i@..mid@] == src@[i@ + 1..mid@].cons(src@[i@]));
            proof_assert!(dst@[lo@..k@ + 1] == before[lo@..k@].snoc(src@[i@]));
            proof_assert! { perm_move_left(before[lo@..k@], src@[i@], src@[i@ + 1..mid@], src@[j@..hi@]); true };
            i += 1;
        } else {
            dst[k] = src[j];
            proof_assert!(src@[j@..hi@] == src@[j@ + 1..hi@].cons(src@[j@]));
            proof_assert!(dst@[lo@..k@ + 1] == before[lo@..k@].snoc(src@[j@]));
            proof_assert! { perm_move_right(before[lo@..k@], src@[j@], src@[i@..mid@], src@[j@ + 1..hi@]); true };
            j += 1;
        }
        k += 1;
    }
    proof_assert!(dst@[lo@..hi@].concat(src@[mid@..mid@]).concat(src@[hi@..hi@]) == dst@[lo@..hi@]);
}

/// Trie `a[lo..hi)` en place, `tmp` servant de tampon de fusion.
#[requires(a@.len() == tmp@.len() && lo@ <= hi@ && hi@ <= a@.len())]
#[ensures((^a)@.sorted_range(lo@, hi@))]
#[ensures(perm_range((^a)@, a@, lo@, hi@))]
#[ensures(same_outside((^a)@, a@, lo@, hi@))]
#[ensures((^tmp)@.len() == tmp@.len())]
#[check(terminates)]
#[variant(hi@ - lo@)]
pub fn msort(a: &mut [u32], tmp: &mut [u32], lo: usize, hi: usize) {
    if hi - lo < 2 {
        return;
    }
    let orig = snapshot!(a@);
    let mid = lo + (hi - lo) / 2;
    msort(a, tmp, lo, mid);
    let s1 = snapshot!(a@);
    msort(a, tmp, mid, hi);
    let s2 = snapshot!(a@);
    merge(a, tmp, lo, mid, hi);
    a[lo..hi].copy_from_slice(&tmp[lo..hi]);
    // Les deux moitiés triées sont des permutations des moitiés d'origine ...
    proof_assert!(s2[lo@..mid@] == s1[lo@..mid@] && s1[mid@..hi@] == orig[mid@..hi@]);
    proof_assert!(s2[lo@..hi@] == s2[lo@..mid@].concat(s2[mid@..hi@]));
    proof_assert!(orig[lo@..hi@] == orig[lo@..mid@].concat(orig[mid@..hi@]));
    proof_assert! { perm_concat(s2[lo@..mid@], orig[lo@..mid@], s2[mid@..hi@], orig[mid@..hi@]); true };
    // ... et la fusion puis la recopie préservent la permutation.
    proof_assert!(a@[lo@..hi@] == tmp@[lo@..hi@]);
    proof_assert!(forall<p> lo@ <= p && p < hi@ ==> a@[p] == a@[lo@..hi@][p - lo@] && tmp@[p] == tmp@[lo@..hi@][p - lo@]);
    proof_assert!(perm_range(a@, *s2, lo@, hi@));
}

/// `fold` des éléments dans l'ordre ; 0 si l'ordre n'est pas croissant.
#[ensures(!v@.sorted() ==> result == 0u64)]
#[check(terminates)]
pub fn checksum(v: &[u32]) -> u64 {
    let n = v.len();
    let mut ck = CK_INIT;
    let mut prev = 0u32;
    let mut ok = true;
    let mut i = 0usize;
    #[invariant(i@ <= n@ && prev == if i@ == 0 { 0u32 } else { v@[i@ - 1] })]
    #[invariant(ok == v@.sorted_range(0, i@))]
    #[variant(n@ - i@)]
    while i < n {
        let x = v[i];
        ok = ok && prev <= x;
        prev = x;
        ck = ckfold(ck, x as u64);
        i += 1;
    }
    if ok { ck } else { 0 }
}

#[requires(1 <= n@ && n@ <= 1073741824)]
pub fn run(n: usize) -> u64 {
    let mut a = std::vec![0u32; n];
    let mut tmp = std::vec![0u32; n];
    fill(&mut a);
    let input = snapshot!(a@);
    msort(&mut a, &mut tmp, 0, n);
    proof_assert!(a@[0..n@] == a@ && input[0..n@] == *input);
    proof_assert!(a@.sorted() && a@.permutation_of(*input));
    checksum(&a)
}
