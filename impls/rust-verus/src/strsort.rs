//! strsort : tri fusion stable (descendant, tampon + recopie) d'indices u32 de chaînes de 1 à 32 octets stockées dans un pool
//! contigu avec des offsets (n+1 entrées, `usize` = 64 bits : mêmes valeurs que les offsets u64 des autres colonnes). 1 <= n <= 2^28.
//! (spec/workloads.d/strsort.md)
//!
//! Prouvé avec Verus (correction fonctionnelle complète, terminaison de toutes les fonctions) :
//!   * `le` calcule exactement l'ordre lexicographique logique `lex_le` (octets, puis longueur) des chaînes pointées ;
//!   * `msort` : le tableau d'indices final est trié pour l'ordre `key_le` = (chaîne, indice), ce qui, l'entrée étant
//!     l'identité, est exactement la sortie d'un tri **stable** ; c'est une permutation de `0..n` ;
//!   * absence de panique sans garde à l'exécution (invariant de forme des offsets `wf_offsets`, indices `< n`) ;
//!   * absence de débordement (`off[n] <= 32 n`, bornes de `len_of`/`chr_of` par `bit_vector`).
//! Aucun `assume`, `admit` ni `external_body`.
use crate::common::*;
use crate::sort::{perm_range, same_outside};
use vstd::prelude::*;
use vstd::seq_lib::lemma_multiset_commutative;

verus! {

/// L_i = 1 + (mix64(0xC0FFEE0000000001 + i) >> 59), dans 1..=32.
#[inline(always)]
pub fn len_of(i: u64) -> (r: u64)
    ensures
        1 <= r <= 32,
{
    let x = mix64(0xC0FFEE0000000001u64.wrapping_add(i));
    assert(x >> 59u64 <= 31u64) by (bit_vector);
    (x >> 59) + 1
}

#[inline(always)]
pub fn chr_of(i: u64, k: u64) -> u8 {
    let x = mix64(0xC0FFEE0000000002u64.wrapping_add(i.wrapping_mul(64)).wrapping_add(k));
    assert(x & 3u64 <= 3u64) by (bit_vector);
    b'a' + (x & 3) as u8
}

/// Forme des offsets : croissants, `off[n]` = taille du pool (donc chaque chaîne est dans le pool).
pub open spec fn wf_offsets(pool: Seq<u8>, off: Seq<usize>) -> bool {
    &&& off.len() >= 1
    &&& off[off.len() - 1] == pool.len()
    &&& forall|p: int, q: int| 0 <= p <= q < off.len() ==> off[p] <= off[q]
}

/// La chaîne d'indice `x` : `pool[off[x] .. off[x+1])`.
pub open spec fn str_of(pool: Seq<u8>, off: Seq<usize>, x: int) -> Seq<u8> {
    pool.subrange(off[x] as int, off[x + 1] as int)
}

/// Ordre lexicographique large sur les octets, la chaîne la plus courte d'abord à préfixe égal.
pub open spec fn lex_le(s: Seq<u8>, t: Seq<u8>) -> bool
    decreases s.len(),
{
    if s.len() == 0 {
        true
    } else if t.len() == 0 {
        false
    } else if s[0] != t[0] {
        s[0] < t[0]
    } else {
        lex_le(s.drop_first(), t.drop_first())
    }
}

/// Lemme : `lex_le` est transitif.
pub proof fn lex_trans(s: Seq<u8>, t: Seq<u8>, u: Seq<u8>)
    requires
        lex_le(s, t),
        lex_le(t, u),
    ensures
        lex_le(s, u),
    decreases s.len(),
{
    if s.len() > 0 && t.len() > 0 && u.len() > 0 && s[0] == t[0] && t[0] == u[0] {
        lex_trans(s.drop_first(), t.drop_first(), u.drop_first());
    }
}

/// Lemme : `lex_le` est total.
pub proof fn lex_total(s: Seq<u8>, t: Seq<u8>)
    ensures
        lex_le(s, t) || lex_le(t, s),
    decreases s.len(),
{
    if s.len() > 0 && t.len() > 0 && s[0] == t[0] {
        lex_total(s.drop_first(), t.drop_first());
    }
}

/// Ordre de la sortie d'un tri stable de l'identité : par chaîne, puis par indice à chaînes égales.
pub open spec fn key_le(pool: Seq<u8>, off: Seq<usize>, x: int, y: int) -> bool {
    &&& lex_le(str_of(pool, off, x), str_of(pool, off, y))
    &&& lex_le(str_of(pool, off, y), str_of(pool, off, x)) ==> x <= y
}

/// Lemme : `key_le` est transitif.
pub broadcast proof fn key_trans(pool: Seq<u8>, off: Seq<usize>, x: int, y: int, z: int)
    requires
        #[trigger] key_le(pool, off, x, y),
        #[trigger] key_le(pool, off, y, z),
    ensures
        key_le(pool, off, x, z),
{
    let (sx, sy, sz) = (str_of(pool, off, x), str_of(pool, off, y), str_of(pool, off, z));
    lex_trans(sx, sy, sz);
    if lex_le(sz, sx) {
        lex_trans(sy, sz, sx);
        lex_trans(sz, sx, sy);
    }
}

/// `s[lo..hi)` est trié pour `key_le`.
pub open spec fn sorted_key(pool: Seq<u8>, off: Seq<usize>, s: Seq<u32>, lo: int, hi: int) -> bool {
    forall|p: int, q: int| lo <= p < q < hi ==> #[trigger] key_le(pool, off, s[p] as int, s[q] as int)
}

/// Toutes les valeurs de `s[lo..hi)` sont dans `[a, b)`.
pub open spec fn vals_in(s: Seq<u32>, lo: int, hi: int, a: int, b: int) -> bool {
    forall|p: int| lo <= p < hi ==> a <= #[trigger] s[p] < b
}

/// off[i+1] = off[i] + len_of(i) ; renvoie le total (= off[n]).
pub fn gen_offsets(off: &mut [usize], n: usize) -> (total: usize)
    requires
        old(off)@.len() == n + 1,
        32 * n < usize::MAX,
        old(off)@[0] == 0,
    ensures
        final(off)@.len() == n + 1,
        total == final(off)@[n as int],
        total <= 32 * n,
        forall|p: int, q: int| 0 <= p <= q <= n ==> final(off)@[p] <= final(off)@[q],
{
    let mut acc: usize = 0;
    let mut i: usize = 0;
    while i < n
        invariant
            i <= n,
            32 * n < usize::MAX,
            off@.len() == n + 1,
            acc == off@[i as int],
            acc <= 32 * i,
            forall|p: int, q: int| 0 <= p <= q <= i ==> off@[p] <= off@[q],
        decreases n - i,
    {
        acc += len_of(i as u64) as usize;
        off[i + 1] = acc;
        i += 1;
    }
    acc
}

/// Octet k de la chaîne i, à la position off[i] + k.
pub fn gen_pool(pool: &mut [u8], off: &[usize], n: usize)
    requires
        wf_offsets(old(pool)@, off@),
        off@.len() == n + 1,
    ensures
        final(pool)@.len() == old(pool)@.len(),
{
    let mut i: usize = 0;
    while i < n
        invariant
            i <= n,
            off@.len() == n + 1,
            wf_offsets(pool@, off@),
        decreases n - i,
    {
        let (o, l) = (off[i], off[i + 1] - off[i]);
        let mut k: usize = 0;
        while k < l
            invariant
                i < n,
                off@.len() == n + 1,
                o == off@[i as int],
                o + l == off@[i + 1],
                k <= l,
                wf_offsets(pool@, off@),
            decreases l - k,
        {
            pool[o + k] = chr_of(i as u64, k as u64);
            k += 1;
        }
        i += 1;
    }
}

/// `a <= b` pour l'ordre lexicographique des chaînes (comparaison octet par octet, puis longueurs).
pub fn le(pool: &[u8], off: &[usize], a: u32, b: u32) -> (r: bool)
    requires
        wf_offsets(pool@, off@),
        a + 1 < off@.len(),
        b + 1 < off@.len(),
    ensures
        r == lex_le(str_of(pool@, off@, a as int), str_of(pool@, off@, b as int)),
{
    let ghost sa = str_of(pool@, off@, a as int);
    let ghost sb = str_of(pool@, off@, b as int);
    let (ia, ib) = (a as usize, b as usize);
    let (oa, ob) = (off[ia], off[ib]);
    let (la, lb) = (off[ia + 1] - oa, off[ib + 1] - ob);
    let m = if la <= lb { la } else { lb };
    let (pa, pb) = (&pool[oa..oa + m], &pool[ob..ob + m]);
    let mut k: usize = 0;
    assert(sa.subrange(0, sa.len() as int) =~= sa && sb.subrange(0, sb.len() as int) =~= sb);
    // Les k premiers octets sont égaux : comparer les chaînes revient à comparer leurs suffixes.
    while k < m
        invariant
            sa == str_of(pool@, off@, a as int),
            sb == str_of(pool@, off@, b as int),
            k <= m <= la,
            m <= lb,
            sa.len() == la,
            sb.len() == lb,
            pa@ == sa.subrange(0, m as int),
            pb@ == sb.subrange(0, m as int),
            lex_le(sa, sb) == lex_le(sa.subrange(k as int, la as int), sb.subrange(k as int, lb as int)),
        decreases m - k,
    {
        let (ca, cb) = (pa[k], pb[k]);
        assert(sa.subrange(k as int, la as int)[0] == ca && sb.subrange(k as int, lb as int)[0] == cb);
        if ca != cb {
            return ca < cb;
        }
        assert(sa.subrange(k as int, la as int).drop_first() =~= sa.subrange(k + 1, la as int));
        assert(sb.subrange(k as int, lb as int).drop_first() =~= sb.subrange(k + 1, lb as int));
        k += 1;
    }
    la <= lb
}

/// Fusionne `src[lo..mid)` et `src[mid..hi)` dans `dst[lo..hi)` ; stable : à égalité on prend la gauche.
/// Les indices de gauche sont tous < ceux de droite, d'où le tri pour `key_le`.
pub fn merge(pool: &[u8], off: &[usize], src: &[u32], dst: &mut [u32], lo: usize, mid: usize, hi: usize)
    requires
        wf_offsets(pool@, off@),
        src@.len() + 1 == off@.len(),
        src@.len() == old(dst)@.len(),
        lo <= mid <= hi <= src@.len(),
        vals_in(src@, lo as int, mid as int, lo as int, mid as int),
        vals_in(src@, mid as int, hi as int, mid as int, hi as int),
        sorted_key(pool@, off@, src@, lo as int, mid as int),
        sorted_key(pool@, off@, src@, mid as int, hi as int),
    ensures
        sorted_key(pool@, off@, final(dst)@, lo as int, hi as int),
        vals_in(final(dst)@, lo as int, hi as int, lo as int, hi as int),
        perm_range(final(dst)@, src@, lo as int, hi as int),
        same_outside(final(dst)@, old(dst)@, lo as int, hi as int),
{
    broadcast use key_trans, lemma_multiset_commutative, vstd::multiset::group_multiset_axioms;

    let ghost old_dst = dst@;
    let ghost whole = src@.subrange(lo as int, hi as int).to_multiset();
    assert(dst@.subrange(lo as int, lo as int) + src@.subrange(lo as int, mid as int) + src@.subrange(mid as int, hi as int)
        =~= src@.subrange(lo as int, hi as int));
    let mut i = lo;
    let mut j = mid;
    let mut k = lo;
    while k < hi
        invariant
            wf_offsets(pool@, off@),
            src@.len() + 1 == off@.len(),
            src@.len() == dst@.len(),
            lo <= i <= mid <= j <= hi <= src@.len(),
            k + mid == i + j,
            vals_in(src@, lo as int, mid as int, lo as int, mid as int),
            vals_in(src@, mid as int, hi as int, mid as int, hi as int),
            sorted_key(pool@, off@, src@, lo as int, mid as int),
            sorted_key(pool@, off@, src@, mid as int, hi as int),
            sorted_key(pool@, off@, dst@, lo as int, k as int),
            vals_in(dst@, lo as int, k as int, lo as int, hi as int),
            forall|p: int, q: int| lo <= p < k && i <= q < mid ==> #[trigger] key_le(pool@, off@, dst@[p] as int, src@[q] as int),
            forall|p: int, q: int| lo <= p < k && j <= q < hi ==> #[trigger] key_le(pool@, off@, dst@[p] as int, src@[q] as int),
            (dst@.subrange(lo as int, k as int) + src@.subrange(i as int, mid as int)
                + src@.subrange(j as int, hi as int)).to_multiset() == whole,
            same_outside(dst@, old_dst, lo as int, hi as int),
        decreases hi - k,
    {
        let ghost before = dst@;
        if i < mid && (j >= hi || le(pool, off, src[i], src[j])) {
            let x = src[i];
            dst[k] = x;
            proof {
                if j < hi {
                    // à chaînes égales, l'indice de gauche est le plus petit : x < mid <= src[j]
                    assert(key_le(pool@, off@, x as int, src@[j as int] as int));
                }
                assert forall|p: int, q: int| lo <= p < k + 1 && i + 1 <= q < mid
                    implies #[trigger] key_le(pool@, off@, dst@[p] as int, src@[q] as int) by {
                    if p < k { assert(dst@[p] == before[p]); }
                    else { assert(key_le(pool@, off@, src@[i as int] as int, src@[q] as int)); }
                }
                assert forall|p: int, q: int| lo <= p < k + 1 && j <= q < hi
                    implies #[trigger] key_le(pool@, off@, dst@[p] as int, src@[q] as int) by {
                    if p < k { assert(dst@[p] == before[p]); }
                    else if q > j {
                        assert(key_le(pool@, off@, src@[j as int] as int, src@[q] as int));
                        key_trans(pool@, off@, x as int, src@[j as int] as int, src@[q] as int);
                    }
                }
                assert forall|p: int, q: int| lo <= p < q < k + 1
                    implies #[trigger] key_le(pool@, off@, dst@[p] as int, dst@[q] as int) by {
                    assert(dst@[p] == before[p]);
                    if q < k { assert(dst@[q] == before[q]); }
                    else { assert(key_le(pool@, off@, before[p] as int, src@[i as int] as int)); }
                }
            }
            assert(dst@.subrange(lo as int, k + 1) + src@.subrange(i + 1, mid as int) + src@.subrange(j as int, hi as int)
                =~= before.subrange(lo as int, k as int) + src@.subrange(i as int, mid as int) + src@.subrange(j as int, hi as int));
            i += 1;
        } else {
            let y = src[j];
            dst[k] = y;
            proof {
                if i < mid {
                    lex_total(str_of(pool@, off@, src@[i as int] as int), str_of(pool@, off@, y as int));
                    assert(key_le(pool@, off@, y as int, src@[i as int] as int));
                }
                assert forall|p: int, q: int| lo <= p < k + 1 && i <= q < mid
                    implies #[trigger] key_le(pool@, off@, dst@[p] as int, src@[q] as int) by {
                    if p < k { assert(dst@[p] == before[p]); }
                    else if q > i {
                        assert(key_le(pool@, off@, src@[i as int] as int, src@[q] as int));
                        key_trans(pool@, off@, y as int, src@[i as int] as int, src@[q as int] as int);
                    }
                }
                assert forall|p: int, q: int| lo <= p < k + 1 && j + 1 <= q < hi
                    implies #[trigger] key_le(pool@, off@, dst@[p] as int, src@[q] as int) by {
                    if p < k { assert(dst@[p] == before[p]); }
                    else { assert(key_le(pool@, off@, src@[j as int] as int, src@[q] as int)); }
                }
                assert forall|p: int, q: int| lo <= p < q < k + 1
                    implies #[trigger] key_le(pool@, off@, dst@[p] as int, dst@[q] as int) by {
                    assert(dst@[p] == before[p]);
                    if q < k { assert(dst@[q] == before[q]); }
                    else { assert(key_le(pool@, off@, before[p] as int, src@[j as int] as int)); }
                }
            }
            proof {
                let (b, x, r1, r2) = (before.subrange(lo as int, k as int), seq![y],
                    src@.subrange(i as int, mid as int), src@.subrange(j + 1, hi as int));
                assert(dst@.subrange(lo as int, k + 1) =~= b + x);
                assert(src@.subrange(j as int, hi as int) =~= x + r2);
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

/// Trie la plage identité `a[lo..hi) = lo, lo+1, …, hi-1` de manière stable.
pub fn msort(pool: &[u8], off: &[usize], a: &mut [u32], tmp: &mut [u32], lo: usize, hi: usize)
    requires
        wf_offsets(pool@, off@),
        old(a)@.len() + 1 == off@.len(),
        old(a)@.len() == old(tmp)@.len(),
        lo <= hi <= old(a)@.len(),
        forall|p: int| lo <= p < hi ==> #[trigger] old(a)@[p] == p,
    ensures
        sorted_key(pool@, off@, final(a)@, lo as int, hi as int),
        vals_in(final(a)@, lo as int, hi as int, lo as int, hi as int),
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
    msort(pool, off, a, tmp, lo, mid);
    let ghost s1 = a@;
    msort(pool, off, a, tmp, mid, hi);
    let ghost s2 = a@;
    assert(sorted_key(pool@, off@, s2, lo as int, mid as int)) by {
        assert forall|p: int, q: int| lo <= p < q < mid implies #[trigger] key_le(pool@, off@, s2[p] as int, s2[q] as int) by {
            assert(s2[p] == s1[p] && s2[q] == s1[q]);
        }
    }
    merge(pool, off, a, tmp, lo, mid, hi);
    a[lo..hi].copy_from_slice(&tmp[lo..hi]);
    proof {
        assert(s2.subrange(lo as int, mid as int) =~= s1.subrange(lo as int, mid as int));
        assert(s1.subrange(mid as int, hi as int) =~= orig.subrange(mid as int, hi as int));
        assert(s2.subrange(lo as int, hi as int) =~= s2.subrange(lo as int, mid as int) + s2.subrange(mid as int, hi as int));
        assert(orig.subrange(lo as int, hi as int) =~= orig.subrange(lo as int, mid as int) + orig.subrange(mid as int, hi as int));
        assert(a@.subrange(lo as int, hi as int) =~= tmp@.subrange(lo as int, hi as int));
        assert forall|p: int| lo <= p < hi implies a@[p] == tmp@[p] by {
            assert(a@[p] == a@.subrange(lo as int, hi as int)[p - lo]);
        }
    }
}

/// `32 n < usize::MAX` (offsets cumulés <= 32 n) est vrai pour n <= 2^28 sur 64 bits ; on l'exige plutôt que de fixer
/// `global size_of usize == 8`, qui ne peut être déclaré qu'une fois par crate.
pub fn run(n: usize, _threads: usize) -> u64
    requires
        1 <= n <= 0x1000_0000,
        32 * n < usize::MAX,
{
    let mut off = vec![0usize; n + 1];
    let total = gen_offsets(off.as_mut_slice(), n);
    let mut pool = vec![0u8; total];
    gen_pool(pool.as_mut_slice(), off.as_slice(), n);
    let mut a = vec![0u32; n];
    let mut i: usize = 0;
    while i < n
        invariant
            i <= n <= 0x1000_0000,
            a@.len() == n,
            forall|p: int| 0 <= p < i ==> #[trigger] a@[p] == p,
        decreases n - i,
    {
        a[i] = i as u32;
        i += 1;
    }
    let ghost input = a@;
    let mut tmp = vec![0u32; n];
    msort(pool.as_slice(), off.as_slice(), a.as_mut_slice(), tmp.as_mut_slice(), 0, n);
    // Le résultat : indices triés par (chaîne, indice), permutation de 0..n.
    assert(sorted_key(pool@, off@, a@, 0, n as int) && perm_range(a@, input, 0, n as int));
    let mut ck = CK_INIT;
    let mut i: usize = 0;
    while i < n
        invariant
            i <= n,
            a@.len() == n,
        decreases n - i,
    {
        ck = ckfold(ck, a[i] as u64);
        i += 1;
    }
    ckfold(ck, total as u64)
}

} // verus!
