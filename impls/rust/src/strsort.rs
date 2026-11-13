//! strsort : tri fusion stable (descendant, tampon + recopie) d'indices u32 de chaînes de 1 à 32 octets stockées dans un pool
//! contigu avec des offsets (n+1 entrées, `usize` = 64 bits : mêmes valeurs que les offsets u64 des autres colonnes). 1 <= n <= 2^28.
//! (spec/workloads.d/strsort.md)
//!
//! Prouvé avec Creusot (correction fonctionnelle complète, terminaison de tous les noyaux) :
//!   * `le` calcule exactement l'ordre lexicographique logique `lex_le` (octets, puis longueur) des chaînes pointées ;
//!   * `msort` : le tableau d'indices final est trié pour l'ordre `key_le` = (chaîne, indice), ce qui, l'entrée étant
//!     l'identité, est exactement la sortie d'un tri **stable** ; c'est une permutation de `0..n` ;
//!   * absence de panique **sans garde à l'exécution** : les gardes `ia < n`, `offset + longueur <= taille du pool` et
//!     `p < taille du pool` de l'ancienne version sont remplacées par l'invariant de forme des offsets (`wf_offsets`) et par
//!     la borne des indices (`< n`), prouvés ;
//!   * absence de débordement (`off[n] <= 32 n`).
//! Aucun `#[trusted]`, aucun `assume`.
use crate::common::*;
#[cfg(creusot)]
use crate::sort::{perm_concat, perm_move_left, perm_move_right, perm_range, same_outside};
use creusot_std::prelude::*;

/// L_i = 1 + (mix64(0xC0FFEE0000000001 + i) >> 59), dans 1..=32.
#[bitwise_proof]
#[ensures(1u64 <= result && result <= 32u64)]
#[check(terminates)]
#[inline(always)]
pub fn len_of(i: u64) -> u64 {
    (mix64(0xC0FFEE0000000001u64.wrapping_add(i)) >> 59) + 1
}

#[bitwise_proof]
#[check(terminates)]
#[inline(always)]
pub fn chr_of(i: u64, k: u64) -> u8 {
    b'a' + (mix64(0xC0FFEE0000000002u64.wrapping_add(i.wrapping_mul(64)).wrapping_add(k)) & 3) as u8
}

/// Forme des offsets : croissants, `off[n]` = taille du pool (donc chaque chaîne est dans le pool).
#[logic(open)]
pub fn wf_offsets(pool: Seq<u8>, off: Seq<usize>) -> bool {
    pearlite! {
        off.len() >= 1 && off[off.len() - 1]@ == pool.len()
        && forall<p, q> 0 <= p && p <= q && q < off.len() ==> off[p] <= off[q]
    }
}

/// La chaîne d'indice `x` : `pool[off[x] .. off[x+1])`.
#[logic(open)]
pub fn str_of(pool: Seq<u8>, off: Seq<usize>, x: Int) -> Seq<u8> {
    pearlite! { pool[off[x]@..off[x + 1]@] }
}

/// Ordre lexicographique large sur les octets, la chaîne la plus courte d'abord à préfixe égal.
#[logic(open)]
#[variant(s.len())]
pub fn lex_le(s: Seq<u8>, t: Seq<u8>) -> bool {
    pearlite! {
        if s.len() == 0 { true }
        else if t.len() == 0 { false }
        else if s[0] != t[0] { s[0] < t[0] }
        else { lex_le(s.tail(), t.tail()) }
    }
}

/// Lemme : `lex_le` est transitif.
#[logic]
#[variant(s.len())]
#[requires(lex_le(s, t) && lex_le(t, u))]
#[ensures(lex_le(s, u))]
pub fn lex_trans(s: Seq<u8>, t: Seq<u8>, u: Seq<u8>) {
    if s.len() > 0 && t.len() > 0 && u.len() > 0 && s[0] == t[0] && t[0] == u[0] {
        lex_trans(s.tail(), t.tail(), u.tail())
    }
}

/// Lemme : `lex_le` est total.
#[logic]
#[variant(s.len())]
#[ensures(lex_le(s, t) || lex_le(t, s))]
pub fn lex_total(s: Seq<u8>, t: Seq<u8>) {
    if s.len() > 0 && t.len() > 0 && s[0] == t[0] {
        lex_total(s.tail(), t.tail())
    }
}

/// Ordre de la sortie d'un tri stable de l'identité : par chaîne, puis par indice à chaînes égales.
#[logic(open)]
pub fn key_le(pool: Seq<u8>, off: Seq<usize>, x: Int, y: Int) -> bool {
    pearlite! {
        lex_le(str_of(pool, off, x), str_of(pool, off, y))
        && (lex_le(str_of(pool, off, y), str_of(pool, off, x)) ==> x <= y)
    }
}

/// Lemme : `key_le` est transitif.
#[logic]
#[requires(key_le(pool, off, x, y) && key_le(pool, off, y, z))]
#[ensures(key_le(pool, off, x, z))]
pub fn key_trans(pool: Seq<u8>, off: Seq<usize>, x: Int, y: Int, z: Int) {
    let (sx, sy, sz) = (str_of(pool, off, x), str_of(pool, off, y), str_of(pool, off, z));
    lex_trans(sx, sy, sz);
    if lex_le(sz, sx) {
        lex_trans(sy, sz, sx);
        lex_trans(sz, sx, sy);
    }
}

/// `s[lo..hi)` est trié pour `key_le` (paires d'indices distincts : pas besoin de réflexivité).
#[logic(open)]
pub fn sorted_key(pool: Seq<u8>, off: Seq<usize>, s: Seq<u32>, lo: Int, hi: Int) -> bool {
    pearlite! { forall<p, q> lo <= p && p < q && q < hi ==> key_le(pool, off, s[p]@, s[q]@) }
}

/// Toutes les valeurs de `s[lo..hi)` sont dans `[a, b)`.
#[logic(open)]
pub fn vals_in(s: Seq<u32>, lo: Int, hi: Int, a: Int, b: Int) -> bool {
    pearlite! { forall<p> lo <= p && p < hi ==> a <= s[p]@ && s[p]@ < b }
}

/// off[i+1] = off[i] + len_of(i) ; renvoie le total (= off[n]).
#[requires(off@.len() == n@ + 1 && n@ <= 268435456 && off@[0]@ == 0)]
#[ensures((^off)@.len() == off@.len())]
#[ensures(result == (^off)@[n@] && result@ <= 32 * n@)]
#[ensures(forall<p, q> 0 <= p && p <= q && q <= n@ ==> (^off)@[p] <= (^off)@[q])]
#[check(terminates)]
pub fn gen_offsets(off: &mut [usize], n: usize) -> usize {
    let mut acc = 0usize;
    let mut i = 0usize;
    #[invariant(i@ <= n@ && off@.len() == n@ + 1)]
    #[invariant(acc == off@[i@] && acc@ <= 32 * i@)]
    #[invariant(forall<p, q> 0 <= p && p <= q && q <= i@ ==> off@[p] <= off@[q])]
    #[variant(n@ - i@)]
    while i < n {
        acc += len_of(i as u64) as usize;
        off[i + 1] = acc;
        i += 1;
    }
    acc
}

/// Octet k de la chaîne i, à la position off[i] + k.
#[requires(wf_offsets(pool@, off@) && off@.len() == n@ + 1 && n@ <= 268435456)]
#[ensures((^pool)@.len() == pool@.len())]
#[check(terminates)]
pub fn gen_pool(pool: &mut [u8], off: &[usize], n: usize) {
    let mut i = 0usize;
    #[invariant(i@ <= n@ && pool@.len() == off@[n@]@)]
    #[variant(n@ - i@)]
    while i < n {
        let (o, l) = (off[i], off[i + 1] - off[i]);
        let mut k = 0usize;
        #[invariant(k@ <= l@ && pool@.len() == off@[n@]@)]
        #[variant(l@ - k@)]
        while k < l {
            pool[o + k] = chr_of(i as u64, k as u64);
            k += 1;
        }
        i += 1;
    }
}

/// `a <= b` pour l'ordre lexicographique des chaînes (comparaison octet par octet, puis longueurs).
#[requires(wf_offsets(pool@, off@) && a@ + 1 < off@.len() && b@ + 1 < off@.len())]
#[ensures(result == lex_le(str_of(pool@, off@, a@), str_of(pool@, off@, b@)))]
#[check(terminates)]
pub fn le(pool: &[u8], off: &[usize], a: u32, b: u32) -> bool {
    let sa = snapshot!(str_of(pool@, off@, a@));
    let sb = snapshot!(str_of(pool@, off@, b@));
    let (a, b) = (a as usize, b as usize);
    let (oa, ob) = (off[a], off[b]);
    let (la, lb) = (off[a + 1] - oa, off[b + 1] - ob);
    let m = if la <= lb { la } else { lb };
    let (pa, pb) = (&pool[oa..oa + m], &pool[ob..ob + m]);
    let mut k = 0usize;
    proof_assert!(sa[0..] == *sa && sb[0..] == *sb);
    // Les k premiers octets sont égaux : comparer les chaînes revient à comparer leurs suffixes.
    #[invariant(k@ <= m@ && sa.len() == la@ && sb.len() == lb@)]
    #[invariant(lex_le(*sa, *sb) == lex_le(sa[k@..], sb[k@..]))]
    #[variant(m@ - k@)]
    while k < m {
        let (ca, cb) = (pa[k], pb[k]);
        proof_assert!(sa[k@..][0] == ca && sb[k@..][0] == cb);
        if ca != cb {
            return ca < cb;
        }
        proof_assert!(sa[k@..].tail() == sa[k@ + 1..] && sb[k@..].tail() == sb[k@ + 1..]);
        k += 1;
    }
    la <= lb
}

/// Fusionne `src[lo..mid)` et `src[mid..hi)` dans `dst[lo..hi)` ; stable : à égalité on prend la gauche.
/// Les indices de gauche sont tous < ceux de droite, d'où le tri pour `key_le`.
#[requires(wf_offsets(pool@, off@) && src@.len() + 1 == off@.len() && src@.len() == dst@.len())]
#[requires(lo@ <= mid@ && mid@ <= hi@ && hi@ <= src@.len())]
#[requires(vals_in(src@, lo@, mid@, lo@, mid@) && vals_in(src@, mid@, hi@, mid@, hi@))]
#[requires(sorted_key(pool@, off@, src@, lo@, mid@) && sorted_key(pool@, off@, src@, mid@, hi@))]
#[ensures(sorted_key(pool@, off@, (^dst)@, lo@, hi@))]
#[ensures(vals_in((^dst)@, lo@, hi@, lo@, hi@))]
#[ensures(perm_range((^dst)@, src@, lo@, hi@))]
#[ensures(same_outside((^dst)@, dst@, lo@, hi@))]
#[check(terminates)]
pub fn merge(pool: &[u8], off: &[usize], src: &[u32], dst: &mut [u32], lo: usize, mid: usize, hi: usize) {
    let _ = snapshot!(key_trans);
    let _ = snapshot!(lex_total);
    let old_dst = snapshot!(dst@);
    proof_assert!(src@[lo@..hi@] == src@[lo@..mid@].concat(src@[mid@..hi@]));
    proof_assert!(Seq::<u32>::empty().concat(src@[lo@..mid@]) == src@[lo@..mid@]);
    let mut i = lo;
    let mut j = mid;
    let mut k = lo;
    #[invariant(lo@ <= i@ && i@ <= mid@ && mid@ <= j@ && j@ <= hi@ && k@ + mid@ == i@ + j@)]
    #[invariant(sorted_key(pool@, off@, dst@, lo@, k@) && vals_in(dst@, lo@, k@, lo@, hi@))]
    #[invariant(forall<p, q> lo@ <= p && p < k@ && i@ <= q && q < mid@ ==> key_le(pool@, off@, dst@[p]@, src@[q]@))]
    #[invariant(forall<p, q> lo@ <= p && p < k@ && j@ <= q && q < hi@ ==> key_le(pool@, off@, dst@[p]@, src@[q]@))]
    #[invariant(dst@[lo@..k@].concat(src@[i@..mid@]).concat(src@[j@..hi@]).permutation_of(src@[lo@..hi@]))]
    #[invariant(same_outside(dst@, *old_dst, lo@, hi@))]
    #[variant(hi@ - k@)]
    while k < hi {
        let before = snapshot!(dst@);
        if i < mid && (j >= hi || le(pool, off, src[i], src[j])) {
            dst[k] = src[i];
            // à chaînes égales, l'indice de gauche (< mid) précède celui de droite (>= mid) ; puis transitivité
            proof_assert!(j@ < hi@ ==> key_le(pool@, off@, src@[i@]@, src@[j@]@));
            proof_assert!(forall<q> j@ < q && q < hi@ ==> {
                key_trans(pool@, off@, src@[i@]@, src@[j@]@, src@[q]@);
                key_le(pool@, off@, src@[i@]@, src@[q]@)
            });
            proof_assert!(src@[i@..mid@] == src@[i@ + 1..mid@].cons(src@[i@]));
            proof_assert!(dst@[lo@..k@ + 1] == before[lo@..k@].snoc(src@[i@]));
            proof_assert! { perm_move_left(before[lo@..k@], src@[i@], src@[i@ + 1..mid@], src@[j@..hi@]); true };
            i += 1;
        } else {
            dst[k] = src[j];
            proof_assert!(i@ < mid@ ==> {
                lex_total(str_of(pool@, off@, src@[i@]@), str_of(pool@, off@, src@[j@]@));
                key_le(pool@, off@, src@[j@]@, src@[i@]@)
            });
            proof_assert!(forall<q> i@ < q && q < mid@ ==> {
                key_trans(pool@, off@, src@[j@]@, src@[i@]@, src@[q]@);
                key_le(pool@, off@, src@[j@]@, src@[q]@)
            });
            proof_assert!(src@[j@..hi@] == src@[j@ + 1..hi@].cons(src@[j@]));
            proof_assert!(dst@[lo@..k@ + 1] == before[lo@..k@].snoc(src@[j@]));
            proof_assert! { perm_move_right(before[lo@..k@], src@[j@], src@[i@..mid@], src@[j@ + 1..hi@]); true };
            j += 1;
        }
        k += 1;
    }
    proof_assert!(dst@[lo@..hi@].concat(src@[mid@..mid@]).concat(src@[hi@..hi@]) == dst@[lo@..hi@]);
}

/// Trie la plage identité `a[lo..hi) = lo, lo+1, …, hi-1` de manière stable.
#[requires(wf_offsets(pool@, off@) && a@.len() + 1 == off@.len() && a@.len() == tmp@.len())]
#[requires(lo@ <= hi@ && hi@ <= a@.len())]
#[requires(forall<p> lo@ <= p && p < hi@ ==> a@[p]@ == p)]
#[ensures(sorted_key(pool@, off@, (^a)@, lo@, hi@))]
#[ensures(vals_in((^a)@, lo@, hi@, lo@, hi@))]
#[ensures(perm_range((^a)@, a@, lo@, hi@))]
#[ensures(same_outside((^a)@, a@, lo@, hi@))]
#[ensures((^tmp)@.len() == tmp@.len())]
#[check(terminates)]
#[variant(hi@ - lo@)]
pub fn msort(pool: &[u8], off: &[usize], a: &mut [u32], tmp: &mut [u32], lo: usize, hi: usize) {
    if hi - lo < 2 {
        return;
    }
    let orig = snapshot!(a@);
    let mid = lo + (hi - lo) / 2;
    msort(pool, off, a, tmp, lo, mid);
    let s1 = snapshot!(a@);
    msort(pool, off, a, tmp, mid, hi);
    let s2 = snapshot!(a@);
    merge(pool, off, a, tmp, lo, mid, hi);
    a[lo..hi].copy_from_slice(&tmp[lo..hi]);
    proof_assert!(s2[lo@..mid@] == s1[lo@..mid@] && s1[mid@..hi@] == orig[mid@..hi@]);
    proof_assert!(s2[lo@..hi@] == s2[lo@..mid@].concat(s2[mid@..hi@]));
    proof_assert!(orig[lo@..hi@] == orig[lo@..mid@].concat(orig[mid@..hi@]));
    proof_assert! { perm_concat(s2[lo@..mid@], orig[lo@..mid@], s2[mid@..hi@], orig[mid@..hi@]); true };
    proof_assert!(a@[lo@..hi@] == tmp@[lo@..hi@]);
    proof_assert!(forall<p> lo@ <= p && p < hi@ ==> a@[p] == a@[lo@..hi@][p - lo@] && tmp@[p] == tmp@[lo@..hi@][p - lo@]);
    proof_assert!(perm_range(a@, *s2, lo@, hi@));
}

#[requires(1 <= n@ && n@ <= 268435456)]
pub fn run(n: usize, _threads: usize) -> u64 {
    let mut off = std::vec![0usize; n + 1];
    let total = gen_offsets(&mut off, n);
    let mut pool = std::vec![0u8; total];
    gen_pool(&mut pool, &off, n);
    let mut a = std::vec![0u32; n];
    let mut i = 0usize;
    #[invariant(i@ <= n@ && a@.len() == n@)]
    #[invariant(forall<p> 0 <= p && p < i@ ==> a@[p]@ == p)]
    while i < n {
        a[i] = i as u32;
        i += 1;
    }
    let input = snapshot!(a@);
    let mut tmp = std::vec![0u32; n];
    msort(&pool, &off, &mut a, &mut tmp, 0, n);
    // Le résultat : indices triés par (chaîne, indice), permutation de 0..n.
    proof_assert!(a@[0..n@] == a@ && input[0..n@] == *input);
    proof_assert!(sorted_key(pool@, off@, a@, 0, n@) && a@.permutation_of(*input));
    let mut ck = CK_INIT;
    let mut i = 0usize;
    #[invariant(i@ <= n@ && a@.len() == n@)]
    while i < n {
        ck = ckfold(ck, a[i] as u64);
        i += 1;
    }
    ckfold(ck, total as u64)
}
