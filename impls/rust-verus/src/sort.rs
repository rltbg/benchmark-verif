//! sort : tri fusion **descendant** (récursif) avec tampon et recopie, 1 <= n <= 2^30 (spec/WORKLOADS.md §1).
//!
//! Prouvé avec Verus (correction fonctionnelle complète, terminaison de toutes les fonctions) :
//!   * `msort(a, tmp, lo, hi)` : `a[lo..hi)` est trié, c'est une permutation (égalité des multiensembles) de la plage
//!     d'origine, rien hors de la plage ne change ;
//!   * `merge` : fusion de deux plages triées = plage triée, permutation de l'entrée ;
//!   * absence de panique (indices dans les bornes) et de débordement arithmétique.
//! Les noyaux travaillent sur des tranches pour être réutilisés tels quels par `parsort`.
//! Aucun `assume`, `admit` ni `external_body`.
use crate::common::*;
use vstd::prelude::*;
use vstd::seq_lib::lemma_multiset_commutative;

verus! {

const SEED: u64 = 0x0123456789ABCDEF;

/// `s[lo..hi)` est trié (ordre croissant au sens large).
pub open spec fn sorted_range(s: Seq<u32>, lo: int, hi: int) -> bool {
    forall|p: int, q: int| lo <= p <= q < hi ==> s[p] <= s[q]
}

/// `s[lo..hi)` est une permutation de `t[lo..hi)`.
pub open spec fn perm_range(s: Seq<u32>, t: Seq<u32>, lo: int, hi: int) -> bool {
    s.subrange(lo, hi).to_multiset() == t.subrange(lo, hi).to_multiset()
}

/// `s` et `t` ont la même longueur et coïncident hors de `[lo, hi)`.
pub open spec fn same_outside(s: Seq<u32>, t: Seq<u32>, lo: int, hi: int) -> bool {
    &&& s.len() == t.len()
    &&& forall|p: int| 0 <= p < s.len() && (p < lo || hi <= p) ==> s[p] == t[p]
}

/// `v[i] = hi32(mix64(SEED + i))`.
pub fn fill(v: &mut [u32])
    ensures
        final(v)@.len() == old(v)@.len(),
{
    let n = v.len();
    let mut i: usize = 0;
    while i < n
        invariant
            v@.len() == n,
        decreases n - i,
    {
        v[i] = hi32(mix64(SEED.wrapping_add(i as u64)));
        i += 1;
    }
}

/// Fusionne `src[lo..mid)` et `src[mid..hi)` (triés) dans `dst[lo..hi)` ; stable (à égalité, la gauche d'abord).
pub fn merge(src: &[u32], dst: &mut [u32], lo: usize, mid: usize, hi: usize)
    requires
        src@.len() == old(dst)@.len(),
        lo <= mid <= hi <= src@.len(),
        sorted_range(src@, lo as int, mid as int),
        sorted_range(src@, mid as int, hi as int),
    ensures
        sorted_range(final(dst)@, lo as int, hi as int),
        perm_range(final(dst)@, src@, lo as int, hi as int),
        same_outside(final(dst)@, old(dst)@, lo as int, hi as int),
{
    broadcast use lemma_multiset_commutative, vstd::multiset::group_multiset_axioms;

    let ghost old_dst = dst@;
    let ghost whole = src@.subrange(lo as int, hi as int).to_multiset();
    assert(dst@.subrange(lo as int, lo as int) + src@.subrange(lo as int, mid as int) + src@.subrange(mid as int, hi as int)
        =~= src@.subrange(lo as int, hi as int));
    let mut i = lo;
    let mut j = mid;
    let mut k = lo;
    // dst[lo..k) est trié, ses éléments sont <= aux restes des deux files,
    // et dst[lo..k) + src[i..mid) + src[j..hi) a le même multiensemble que src[lo..hi)
    // (la prise à gauche ne change même pas cette suite ; la prise à droite la permute).
    while k < hi
        invariant
            src@.len() == dst@.len(),
            lo <= i <= mid <= j <= hi <= src@.len(),
            k + mid == i + j,
            sorted_range(src@, lo as int, mid as int),
            sorted_range(src@, mid as int, hi as int),
            sorted_range(dst@, lo as int, k as int),
            forall|p: int, q: int| lo <= p < k && i <= q < mid ==> dst@[p] <= src@[q],
            forall|p: int, q: int| lo <= p < k && j <= q < hi ==> dst@[p] <= src@[q],
            (dst@.subrange(lo as int, k as int) + src@.subrange(i as int, mid as int)
                + src@.subrange(j as int, hi as int)).to_multiset() == whole,
            same_outside(dst@, old_dst, lo as int, hi as int),
        decreases hi - k,
    {
        let ghost before = dst@;
        if i < mid && (j >= hi || src[i] <= src[j]) {
            dst[k] = src[i];
            assert(dst@.subrange(lo as int, k + 1) + src@.subrange(i + 1, mid as int) + src@.subrange(j as int, hi as int)
                =~= before.subrange(lo as int, k as int) + src@.subrange(i as int, mid as int) + src@.subrange(j as int, hi as int));
            i += 1;
        } else {
            dst[k] = src[j];
            proof {
                let (b, x, r1, r2) = (before.subrange(lo as int, k as int), seq![src@[j as int]],
                    src@.subrange(i as int, mid as int), src@.subrange(j + 1, hi as int));
                assert(dst@.subrange(lo as int, k + 1) =~= b + x);
                assert(src@.subrange(j as int, hi as int) =~= x + r2);
                // multiensemble(b + x + r1 + r2) = multiensemble(b + r1 + (x + r2)) : on distribue puis on commute.
                lemma_multiset_commutative(b + x + r1, r2);
                lemma_multiset_commutative(b + x, r1);
                lemma_multiset_commutative(b, x);
                lemma_multiset_commutative(b + r1, x + r2);
                lemma_multiset_commutative(b, r1);
                lemma_multiset_commutative(x, r2);
                assert((b + x + r1 + r2).to_multiset() =~= (b + r1 + (x + r2)).to_multiset());
            }
            j += 1;
        }
        k += 1;
    }
    assert(dst@.subrange(lo as int, hi as int) + src@.subrange(mid as int, mid as int) + src@.subrange(hi as int, hi as int)
        =~= dst@.subrange(lo as int, hi as int));
}

/// Trie `a[lo..hi)` en place, `tmp` servant de tampon de fusion.
pub fn msort(a: &mut [u32], tmp: &mut [u32], lo: usize, hi: usize)
    requires
        old(a)@.len() == old(tmp)@.len(),
        lo <= hi <= old(a)@.len(),
    ensures
        sorted_range(final(a)@, lo as int, hi as int),
        perm_range(final(a)@, old(a)@, lo as int, hi as int),
        same_outside(final(a)@, old(a)@, lo as int, hi as int),
        final(tmp)@.len() == old(tmp)@.len(),
    decreases hi - lo,
{
    broadcast use lemma_multiset_commutative;

    if hi - lo < 2 {
        return;
    }
    let ghost orig = a@;
    let mid = lo + (hi - lo) / 2;
    msort(a, tmp, lo, mid);
    let ghost s1 = a@;
    msort(a, tmp, mid, hi);
    let ghost s2 = a@;
    merge(a, tmp, lo, mid, hi);
    a[lo..hi].copy_from_slice(&tmp[lo..hi]);
    proof {
        // Les deux moitiés triées sont des permutations des moitiés d'origine ...
        assert(s2.subrange(lo as int, mid as int) =~= s1.subrange(lo as int, mid as int));
        assert(s1.subrange(mid as int, hi as int) =~= orig.subrange(mid as int, hi as int));
        assert(s2.subrange(lo as int, hi as int) =~= s2.subrange(lo as int, mid as int) + s2.subrange(mid as int, hi as int));
        assert(orig.subrange(lo as int, hi as int) =~= orig.subrange(lo as int, mid as int) + orig.subrange(mid as int, hi as int));
        // ... et la fusion puis la recopie préservent la permutation.
        assert(a@.subrange(lo as int, hi as int) =~= tmp@.subrange(lo as int, hi as int));
    }
}

/// `fold` des éléments dans l'ordre ; 0 si l'ordre n'est pas croissant.
pub fn checksum(v: &[u32]) -> (r: u64)
    ensures
        !sorted_range(v@, 0, v@.len() as int) ==> r == 0,
{
    let n = v.len();
    let mut ck = CK_INIT;
    let mut prev = 0u32;
    let mut ok = true;
    let mut i: usize = 0;
    while i < n
        invariant
            n == v@.len(),
            i <= n,
            prev == if i == 0 { 0u32 } else { v@[i - 1] },
            ok == sorted_range(v@, 0, i as int),
        decreases n - i,
    {
        let x = v[i];
        ok = ok && prev <= x;
        prev = x;
        ck = ckfold(ck, x as u64);
        i += 1;
    }
    if ok { ck } else { 0 }
}

pub fn run(n: usize, _threads: usize) -> u64
    requires
        1 <= n <= 0x4000_0000,
{
    let mut a = vec![0u32; n];
    let mut tmp = vec![0u32; n];
    fill(a.as_mut_slice());
    let ghost input = a@;
    msort(a.as_mut_slice(), tmp.as_mut_slice(), 0, n);
    assert(sorted_range(a@, 0, n as int) && perm_range(a@, input, 0, n as int));
    checksum(a.as_slice())
}

} // verus!
