//! Tri fusion **descendant** (récursif) avec tableau temporaire et recopie : 1 <= n <= 2^30.
//!
//! Prouvé avec Creusot (correction fonctionnelle complète) :
//!   * `msort` : la plage triée est triée *et* est une permutation de la plage d'origine,
//!     rien en dehors de la plage n'est modifié ;
//!   * absence de panique et de débordement partout.
use crate::common::*;
use creusot_std::prelude::*;

const SEED: u64 = 0x0123456789ABCDEF;

/// a[lo..hi) est une permutation de b[lo..hi).
#[logic(open)]
pub fn perm_range(a: Seq<u32>, b: Seq<u32>, lo: Int, hi: Int) -> bool {
    pearlite! { a.subsequence(lo, hi).permutation_of(b.subsequence(lo, hi)) }
}

/// Déplacer la tête x de la file gauche r1 vers la fin de l'accumulateur acc conserve la permutation.
#[logic]
#[ensures(acc.snoc(x).concat(r1).concat(r2).permutation_of(acc.concat(r1.cons(x)).concat(r2)))]
pub fn perm_move_left(acc: Seq<u32>, x: u32, r1: Seq<u32>, r2: Seq<u32>) {}

/// Idem pour la tête de la file droite r2.
#[logic]
#[ensures(acc.snoc(x).concat(r1).concat(r2).permutation_of(acc.concat(r1).concat(r2.cons(x))))]
pub fn perm_move_right(acc: Seq<u32>, x: u32, r1: Seq<u32>, r2: Seq<u32>) {}

/// Permutations de deux paires de segments => permutation des concaténations.
#[logic]
#[requires(a.permutation_of(b) && c.permutation_of(d))]
#[ensures(a.concat(c).permutation_of(b.concat(d)))]
pub fn perm_concat(a: Seq<u32>, b: Seq<u32>, c: Seq<u32>, d: Seq<u32>) {}

#[requires(v@.len() == n@)]
#[ensures((^v)@.len() == n@)]
#[check(terminates)]
pub fn fill(v: &mut Vec<u32>, n: usize) {
    let mut i = 0usize;
    #[invariant(v@.len() == n@ && i@ <= n@)]
    #[variant(n@ - i@)]
    while i < n {
        v[i] = hi32(mix64(SEED.wrapping_add(i as u64)));
        i += 1;
    }
}

/// Fusionne src[lo,mid) et src[mid,hi) (déjà triés) dans dst[lo,hi).
#[requires(src@.len() == dst@.len())]
#[requires(lo@ <= mid@ && mid@ <= hi@ && hi@ <= src@.len())]
#[requires(src@.sorted_range(lo@, mid@) && src@.sorted_range(mid@, hi@))]
#[ensures((^dst)@.len() == dst@.len())]
#[ensures((^dst)@.sorted_range(lo@, hi@))]
#[ensures(perm_range((^dst)@, src@, lo@, hi@))]
#[ensures(forall<p: Int> 0 <= p && p < dst@.len() && (p < lo@ || hi@ <= p) ==> (^dst)@[p] == dst@[p])]
#[check(terminates)]
pub fn merge(src: &Vec<u32>, dst: &mut Vec<u32>, lo: usize, mid: usize, hi: usize) {
    let old = snapshot!(dst@);
    proof_assert!(src@.subsequence(lo@, hi@) == src@.subsequence(lo@, mid@).concat(src@.subsequence(mid@, hi@)));
    proof_assert!(dst@.subsequence(lo@, lo@) == Seq::empty());
    proof_assert!(Seq::<u32>::empty().concat(src@.subsequence(lo@, mid@)) == src@.subsequence(lo@, mid@));
    let mut i = lo;
    let mut j = mid;
    let mut k = lo;
    #[invariant(src@.len() == dst@.len())]
    #[invariant(lo@ <= i@ && i@ <= mid@ && mid@ <= j@ && j@ <= hi@ && hi@ <= src@.len())]
    #[invariant(k@ <= hi@ && k@ + mid@ == i@ + j@)]
    #[invariant(dst@.sorted_range(lo@, k@))]
    #[invariant(forall<p: Int, q: Int> lo@ <= p && p < k@ && i@ <= q && q < mid@ ==> dst@[p] <= src@[q])]
    #[invariant(forall<p: Int, q: Int> lo@ <= p && p < k@ && j@ <= q && q < hi@ ==> dst@[p] <= src@[q])]
    #[invariant(dst@.subsequence(lo@, k@).concat(src@.subsequence(i@, mid@)).concat(src@.subsequence(j@, hi@))
                    .permutation_of(src@.subsequence(lo@, hi@)))]
    #[invariant(forall<p: Int> 0 <= p && p < dst@.len() && (p < lo@ || hi@ <= p) ==> dst@[p] == old[p])]
    #[variant(hi@ - k@)]
    while k < hi {
        let before = snapshot!(dst@);
        if i < mid && (j >= hi || src[i] <= src[j]) {
            proof_assert!(src@.subsequence(i@, mid@) == src@.subsequence(i@ + 1, mid@).cons(src@[i@]));
            dst[k] = src[i];
            proof_assert!(dst@.subsequence(lo@, k@ + 1) == before.subsequence(lo@, k@).snoc(src@[i@]));
            proof_assert!{ perm_move_left(before.subsequence(lo@, k@), src@[i@], src@.subsequence(i@ + 1, mid@), src@.subsequence(j@, hi@)); true };
            i += 1;
        } else {
            proof_assert!(src@.subsequence(j@, hi@) == src@.subsequence(j@ + 1, hi@).cons(src@[j@]));
            dst[k] = src[j];
            proof_assert!(dst@.subsequence(lo@, k@ + 1) == before.subsequence(lo@, k@).snoc(src@[j@]));
            proof_assert!{ perm_move_right(before.subsequence(lo@, k@), src@[j@], src@.subsequence(i@, mid@), src@.subsequence(j@ + 1, hi@)); true };
            j += 1;
        }
        k += 1;
    }
    proof_assert!(i@ == mid@ && j@ == hi@);
    proof_assert!(src@.subsequence(mid@, mid@) == Seq::empty() && src@.subsequence(hi@, hi@) == Seq::empty());
    proof_assert!(dst@.subsequence(lo@, hi@).concat(Seq::empty()).concat(Seq::empty()) == dst@.subsequence(lo@, hi@));
}

/// a[lo..hi) <- tmp[lo..hi).
#[requires(a@.len() == tmp@.len() && lo@ <= hi@ && hi@ <= a@.len())]
#[ensures((^a)@.len() == a@.len())]
#[ensures(forall<p: Int> lo@ <= p && p < hi@ ==> (^a)@[p] == tmp@[p])]
#[ensures(forall<p: Int> 0 <= p && p < a@.len() && (p < lo@ || hi@ <= p) ==> (^a)@[p] == a@[p])]
#[check(terminates)]
pub fn copy_back(tmp: &Vec<u32>, a: &mut Vec<u32>, lo: usize, hi: usize) {
    let old = snapshot!(a@);
    let mut i = lo;
    #[invariant(a@.len() == tmp@.len() && lo@ <= i@ && i@ <= hi@)]
    #[invariant(forall<p: Int> lo@ <= p && p < i@ ==> a@[p] == tmp@[p])]
    #[invariant(forall<p: Int> 0 <= p && p < a@.len() && (p < lo@ || i@ <= p) ==> a@[p] == old[p])]
    #[variant(hi@ - i@)]
    while i < hi {
        a[i] = tmp[i];
        i += 1;
    }
}

/// Trie a[lo..hi) en place, en utilisant tmp comme espace de travail.
#[requires(a@.len() == tmp@.len() && lo@ <= hi@ && hi@ <= a@.len())]
#[ensures((^a)@.len() == a@.len() && (^tmp)@.len() == tmp@.len())]
#[ensures((^a)@.sorted_range(lo@, hi@))]
#[ensures(perm_range((^a)@, a@, lo@, hi@))]
#[ensures(forall<p: Int> 0 <= p && p < a@.len() && (p < lo@ || hi@ <= p) ==> (^a)@[p] == a@[p])]
#[check(terminates)]
#[variant(hi@ - lo@)]
pub fn msort(a: &mut Vec<u32>, tmp: &mut Vec<u32>, lo: usize, hi: usize) {
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
    let t = snapshot!(tmp@);
    copy_back(tmp, a, lo, hi);
    // Les deux moitiés triées sont des permutations des moitiés d'origine...
    proof_assert!(s2.subsequence(lo@, mid@) == s1.subsequence(lo@, mid@));
    proof_assert!(s1.subsequence(mid@, hi@) == orig.subsequence(mid@, hi@));
    proof_assert!(s2.subsequence(lo@, hi@) == s2.subsequence(lo@, mid@).concat(s2.subsequence(mid@, hi@)));
    proof_assert!(orig.subsequence(lo@, hi@) == orig.subsequence(lo@, mid@).concat(orig.subsequence(mid@, hi@)));
    proof_assert!{ perm_concat(s2.subsequence(lo@, mid@), orig.subsequence(lo@, mid@),
                               s2.subsequence(mid@, hi@), orig.subsequence(mid@, hi@)); true };
    // ... la fusion et la recopie préservent la permutation.
    proof_assert!(a@.subsequence(lo@, hi@) == t.subsequence(lo@, hi@));
    proof_assert!(perm_range(a@, *orig, lo@, hi@));
}

/// fold des éléments dans l'ordre ; 0 si l'ordre n'est pas croissant.
#[requires(v@.len() == n@ && n@ <= 1073741824)]
#[check(terminates)]
pub fn checksum(v: &Vec<u32>, n: usize) -> u64 {
    let mut ck = CK_INIT;
    let mut prev = 0u32;
    let mut ok = true;
    let mut i = 0usize;
    #[invariant(v@.len() == n@ && i@ <= n@)]
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
    fill(&mut a, n);
    msort(&mut a, &mut tmp, 0, n);
    checksum(&a, n)
}
