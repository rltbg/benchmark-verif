//! parsort : segments triés en parallèle puis fusions par paires (spec/workloads.d/parsort.md).
//!
//! Prouvé avec Verus : noyaux de `sort` (trié ∧ permutation) ; découpage récursif des tranches (`sort_runs`,
//! `merge_pairs`), calcul des bornes, boucle des niveaux : à la fin `a` est trié et est une permutation de l'entrée.
//!
//! **Raccourci (2 `external_body`)** : `fork_sort` et `fork_merge`, qui exécutent un noyau vérifié dans un thread
//! *scopé* (`std::thread::scope`) pendant que le thread courant traite le reste des tranches. `vstd::thread::spawn`
//! exige des clôtures `'static` : elle ne peut pas recevoir les `&mut [u32]` empruntés au tableau (erreur de durée de vie,
//! essai consigné dans docs/verif-compare/parsort.md) et vstd n'a pas d'équivalent de `scope`. La spécification de
//! chacune est exactement la conjonction des contrats des deux appels vérifiés qu'elle fait sur des emprunts disjoints :
//! ce qui est admis est seulement « exécuter en parallèle deux appels sur des `&mut` disjoints = les exécuter en séquence ».
//! (La récursion `sort_runs` → `fork_sort` → `sort_runs` passe par ces fonctions : sa terminaison n'est pas vérifiée.)
use crate::sort::{checksum, fill, merge, msort, perm_range, same_outside, sorted_range};
use vstd::arithmetic::div_mod::{lemma_div_by_multiple, lemma_div_is_ordered};
use vstd::arithmetic::mul::lemma_mul_inequality;
use vstd::prelude::*;
use vstd::seq_lib::lemma_multiset_commutative;

verus! {

/// Bornes croissantes.
pub open spec fn nondecreasing(b: Seq<usize>) -> bool {
    forall|p: int, q: int| 0 <= p <= q < b.len() ==> b[p] <= b[q]
}

/// Bornes de runs : `b[0] = 0 <= b[1] <= … <= b[last] = n`.
pub open spec fn wf_bounds(b: Seq<usize>, n: int) -> bool {
    &&& b.len() >= 2
    &&& b[0] == 0
    &&& b[b.len() - 1] == n
    &&& nondecreasing(b)
}

/// Les runs `[b[j], b[j+1])` pour `j ∈ [k, m)`, décalés de `base`, sont triés dans `s`.
pub open spec fn runs_sorted(s: Seq<u32>, b: Seq<usize>, k: int, m: int, base: int) -> bool {
    forall|j: int| k <= j < m ==> #[trigger] sorted_range(s, b[j] - base, b[j + 1] - base)
}

/// Les paires fusionnées `[b[2q], b[2q+2])` pour `q ∈ [p, m)`, décalées de `base`, sont triées dans `s`.
pub open spec fn pairs_sorted(s: Seq<u32>, b: Seq<usize>, p: int, m: int, base: int) -> bool {
    forall|q: int| p <= q < m ==> #[trigger] sorted_range(s, b[2 * q] - base, b[2 * q + 2] - base)
}

/// Lemme : dans `u + v`, une plage triée de `v` l'est décalée de `|u|`.
pub proof fn sorted_shift(u: Seq<u32>, v: Seq<u32>, x: int, y: int)
    requires
        0 <= x <= y <= v.len(),
        sorted_range(v, x, y),
    ensures
        sorted_range(u + v, x + u.len(), y + u.len()),
{
    assert forall|p: int, q: int| x + u.len() <= p <= q < y + u.len() implies (u + v)[p] <= (u + v)[q] by {
        assert((u + v)[p] == v[p - u.len()] && (u + v)[q] == v[q - u.len()]);
    }
}

/// Lemme : premier run trié dans `u`, suivants dans `v` ⇒ tous les runs `[k, t)` triés dans `u + v`.
pub proof fn runs_concat(u: Seq<u32>, v: Seq<u32>, b: Seq<usize>, k: int, t: int)
    requires
        0 <= k < t < b.len(),
        nondecreasing(b),
        u.len() == b[k + 1] - b[k],
        v.len() == b[t] - b[k + 1],
        sorted_range(u, 0, u.len() as int),
        runs_sorted(v, b, k + 1, t, b[k + 1] as int),
    ensures
        runs_sorted(u + v, b, k, t, b[k] as int),
{
    assert forall|j: int| k <= j < t implies #[trigger] sorted_range(u + v, b[j] - b[k], b[j + 1] - b[k]) by {
        if j == k {
            assert forall|p: int, q: int| 0 <= p <= q < u.len() implies (u + v)[p] <= (u + v)[q] by {}
        } else {
            assert(sorted_range(v, b[j] - b[k + 1], b[j + 1] - b[k + 1]));
            sorted_shift(u, v, b[j] - b[k + 1], b[j + 1] - b[k + 1]);
        }
    }
}

/// Lemme : idem pour les paires fusionnées `[b[2q], b[2q+2])`, `q ∈ [p, m)`.
pub proof fn pairs_concat(u: Seq<u32>, v: Seq<u32>, b: Seq<usize>, p: int, m: int)
    requires
        0 <= p < m,
        2 * m < b.len(),
        nondecreasing(b),
        u.len() == b[2 * p + 2] - b[2 * p],
        v.len() >= b[2 * m] - b[2 * p + 2],
        sorted_range(u, 0, u.len() as int),
        pairs_sorted(v, b, p + 1, m, b[2 * p + 2] as int),
    ensures
        pairs_sorted(u + v, b, p, m, b[2 * p] as int),
{
    assert forall|q: int| p <= q < m implies #[trigger] sorted_range(u + v, b[2 * q] - b[2 * p], b[2 * q + 2] - b[2 * p]) by {
        if q == p {
            assert forall|x: int, y: int| 0 <= x <= y < u.len() implies (u + v)[x] <= (u + v)[y] by {}
        } else {
            assert(sorted_range(v, b[2 * q] - b[2 * p + 2], b[2 * q + 2] - b[2 * p + 2]));
            sorted_shift(u, v, b[2 * q] - b[2 * p + 2], b[2 * q + 2] - b[2 * p + 2]);
        }
    }
}

/// Lemme : une plage complète vue par `perm_range` est une égalité de multiensembles.
pub proof fn perm_whole(s: Seq<u32>, t: Seq<u32>)
    requires
        s.len() == t.len(),
        perm_range(s, t, 0, s.len() as int),
    ensures
        s.to_multiset() == t.to_multiset(),
{
    assert(s.subrange(0, s.len() as int) =~= s && t.subrange(0, t.len() as int) =~= t);
}

/// Travail d'une fusion : `a[..m)` et `a[m..)` triés -> `a` trié (fusion dans `tmp` puis recopie).
#[inline(always)]
pub fn merge_back(a: &mut [u32], tmp: &mut [u32], m: usize)
    requires
        old(a)@.len() == old(tmp)@.len(),
        m <= old(a)@.len(),
        sorted_range(old(a)@, 0, m as int),
        sorted_range(old(a)@, m as int, old(a)@.len() as int),
    ensures
        final(a)@.len() == old(a)@.len(),
        sorted_range(final(a)@, 0, old(a)@.len() as int),
        final(a)@.to_multiset() == old(a)@.to_multiset(),
        final(tmp)@.len() == old(tmp)@.len(),
{
    let ghost a0 = a@;
    let n = a.len();
    merge(a, tmp, 0, m, n);
    a.copy_from_slice(tmp);
    proof { perm_whole(a@, a0); }
}

/// Trie `ma` dans un thread scopé pendant que le thread courant trie les runs suivants (`sort_runs(ra, …, k + 1, t)`).
/// Spécification = contrats de `msort(ma, mt, 0, |ma|)` et de `sort_runs(ra, rt, b, k + 1, t)` (voir l'en-tête).
#[verifier::external_body]
fn fork_sort(ma: &mut [u32], mt: &mut [u32], ra: &mut [u32], rt: &mut [u32], b: &[usize], k: usize, t: usize)
    requires
        old(ma)@.len() == old(mt)@.len(),
        k + 1 < t,
        b@.len() == t + 1,
        nondecreasing(b@),
        old(ra)@.len() == b@[t as int] - b@[k + 1],
        old(rt)@.len() == old(ra)@.len(),
    ensures
        sorted_range(final(ma)@, 0, old(ma)@.len() as int),
        perm_range(final(ma)@, old(ma)@, 0, old(ma)@.len() as int),
        same_outside(final(ma)@, old(ma)@, 0, old(ma)@.len() as int),
        runs_sorted(final(ra)@, b@, k + 1, t as int, b@[k + 1] as int),
        final(ra)@.to_multiset() == old(ra)@.to_multiset(),
        final(ra)@.len() == old(ra)@.len(),
        final(mt)@.len() == old(mt)@.len(),
        final(rt)@.len() == old(rt)@.len(),
    decreases t - k, 0nat,
{
    let len = ma.len();
    std::thread::scope(|s| {
        s.spawn(move || msort(ma, mt, 0, len));
        sort_runs(ra, rt, b, k + 1, t);
    });
}

/// Trie chaque segment `[b[j], b[j+1])`, `j ∈ [k, t)`, de `a` (qui commence à `b[k]`) : un thread par segment,
/// le dernier sur le thread courant.
pub fn sort_runs(a: &mut [u32], tmp: &mut [u32], b: &[usize], k: usize, t: usize)
    requires
        k < t,
        b@.len() == t + 1,
        nondecreasing(b@),
        old(a)@.len() == b@[t as int] - b@[k as int],
        old(tmp)@.len() == old(a)@.len(),
    ensures
        runs_sorted(final(a)@, b@, k as int, t as int, b@[k as int] as int),
        final(a)@.to_multiset() == old(a)@.to_multiset(),
        final(a)@.len() == old(a)@.len(),
        final(tmp)@.len() == old(tmp)@.len(),
    decreases t - k, 1nat,
{
    broadcast use lemma_multiset_commutative;

    let len = b[k + 1] - b[k];
    let (ma, ra) = a.split_at_mut(len);
    let (mt, rt) = tmp.split_at_mut(len);
    let ghost (m0, r0) = (ma@, ra@);
    if k + 1 < t {
        fork_sort(ma, mt, ra, rt, b, k, t);
    } else {
        msort(ma, mt, 0, len);
    }
    proof {
        perm_whole(ma@, m0);
        if k + 1 < t {
            runs_concat(ma@, ra@, b@, k as int, t as int);
        } else {
            assert(ra@ =~= Seq::<u32>::empty());
            runs_concat(ma@, ra@, b@, k as int, t as int);
        }
        assert(m0 + r0 =~= old(a)@);
    }
}

/// Fusionne `ma` (deux runs : `[0, m)` et `[m, |ma|)`) dans un thread scopé pendant que le thread courant traite
/// les paires suivantes (`merge_pairs(ra, …, p + 1, pairs)`).
/// Spécification = contrats de `merge_back(ma, mt, m)` et de `merge_pairs(ra, rt, b, p + 1, pairs)`.
#[verifier::external_body]
fn fork_merge(
    ma: &mut [u32], mt: &mut [u32], m: usize, ra: &mut [u32], rt: &mut [u32], b: &[usize], p: usize, pairs: usize,
)
    requires
        old(ma)@.len() == old(mt)@.len(),
        m <= old(ma)@.len(),
        sorted_range(old(ma)@, 0, m as int),
        sorted_range(old(ma)@, m as int, old(ma)@.len() as int),
        p + 1 < pairs,
        2 * pairs + 1 <= b@.len(),
        nondecreasing(b@),
        old(ra)@.len() == b@[b@.len() - 1] - b@[2 * p + 2],
        old(rt)@.len() == old(ra)@.len(),
        runs_sorted(old(ra)@, b@, 2 * p + 2, 2 * pairs as int, b@[2 * p + 2] as int),
    ensures
        final(ma)@.len() == old(ma)@.len(),
        sorted_range(final(ma)@, 0, old(ma)@.len() as int),
        final(ma)@.to_multiset() == old(ma)@.to_multiset(),
        pairs_sorted(final(ra)@, b@, p + 1, pairs as int, b@[2 * p + 2] as int),
        final(ra)@.to_multiset() == old(ra)@.to_multiset(),
        final(ra)@.len() == old(ra)@.len(),
        final(mt)@.len() == old(mt)@.len(),
        final(rt)@.len() == old(rt)@.len(),
        forall|i: int| b@[2 * pairs] - b@[2 * p + 2] <= i < old(ra)@.len() ==> final(ra)@[i] == old(ra)@[i],
    decreases pairs - p, 0nat,
{
    std::thread::scope(|s| {
        s.spawn(move || merge_back(ma, mt, m));
        merge_pairs(ra, rt, b, p + 1, pairs);
    });
}

/// Fusionne les paires de runs `[b[2q], b[2q+1])` + `[b[2q+1], b[2q+2])`, `q ∈ [p, pairs)`, de `a` (qui commence à
/// `b[2p]` et va jusqu'à la fin du tableau) : une fusion par thread, la dernière sur le thread courant ; le reste est intact.
pub fn merge_pairs(a: &mut [u32], tmp: &mut [u32], b: &[usize], p: usize, pairs: usize)
    requires
        p < pairs,
        2 * pairs + 1 <= b@.len(),
        nondecreasing(b@),
        old(a)@.len() == b@[b@.len() - 1] - b@[2 * p as int],
        old(tmp)@.len() == old(a)@.len(),
        runs_sorted(old(a)@, b@, 2 * p as int, 2 * pairs as int, b@[2 * p as int] as int),
    ensures
        pairs_sorted(final(a)@, b@, p as int, pairs as int, b@[2 * p as int] as int),
        final(a)@.to_multiset() == old(a)@.to_multiset(),
        final(a)@.len() == old(a)@.len(),
        final(tmp)@.len() == old(tmp)@.len(),
        forall|i: int| b@[2 * pairs] - b@[2 * p as int] <= i < old(a)@.len() ==> final(a)@[i] == old(a)@[i],
    decreases pairs - p, 1nat,
{
    broadcast use lemma_multiset_commutative;

    let (lo, mid, hi) = (b[2 * p], b[2 * p + 1], b[2 * p + 2]);
    let (ma, ra) = a.split_at_mut(hi - lo);
    let (mt, rt) = tmp.split_at_mut(hi - lo);
    let ghost (m0, r0) = (ma@, ra@);
    assert(sorted_range(old(a)@, b@[2 * p as int] - lo, b@[2 * p + 1] - lo));
    assert(sorted_range(old(a)@, b@[2 * p + 1] - lo, b@[(2 * p + 1) + 1] - lo));
    if p + 1 < pairs {
        assert forall|j: int| 2 * p + 2 <= j < 2 * pairs implies #[trigger] sorted_range(ra@, b@[j] - b@[2 * p + 2], b@[j + 1] - b@[2 * p + 2]) by {
            assert(sorted_range(old(a)@, b@[j] - lo, b@[j + 1] - lo));
        }
        fork_merge(ma, mt, mid - lo, ra, rt, b, p, pairs);
    } else {
        merge_back(ma, mt, mid - lo);
    }
    proof {
        if p + 1 < pairs {
            pairs_concat(ma@, ra@, b@, p as int, pairs as int);
        } else {
            pairs_concat(ma@, ra@, b@, p as int, pairs as int);
        }
        assert(m0 + r0 =~= old(a)@);
        assert(ma@ + ra@ == final(a)@);
    }
}

/// Phase 1 : les `t` segments triés en parallèle (`t - 1` threads lancés).
pub fn sort_level(a: &mut [u32], tmp: &mut [u32], b: &[usize], t: usize)
    requires
        old(a)@.len() == old(tmp)@.len(),
        wf_bounds(b@, old(a)@.len() as int),
        b@.len() == t + 1,
        1 <= t,
    ensures
        runs_sorted(final(a)@, b@, 0, t as int, 0),
        final(a)@.to_multiset() == old(a)@.to_multiset(),
        final(a)@.len() == old(a)@.len(),
        final(tmp)@.len() == old(tmp)@.len(),
{
    sort_runs(a, tmp, b, 0, t);
}

/// Phase 2, un niveau : les `pairs` paires de runs fusionnées en parallèle ; le run impair final est intact.
pub fn merge_level(a: &mut [u32], tmp: &mut [u32], b: &[usize], pairs: usize)
    requires
        old(a)@.len() == old(tmp)@.len(),
        wf_bounds(b@, old(a)@.len() as int),
        1 <= pairs,
        2 * pairs + 1 <= b@.len(),
        runs_sorted(old(a)@, b@, 0, b@.len() - 1, 0),
    ensures
        pairs_sorted(final(a)@, b@, 0, pairs as int, 0),
        final(a)@.to_multiset() == old(a)@.to_multiset(),
        final(a)@.len() == old(a)@.len(),
        final(tmp)@.len() == old(tmp)@.len(),
        forall|i: int| b@[2 * pairs] <= i < old(a)@.len() ==> final(a)@[i] == old(a)@[i],
{
    merge_pairs(a, tmp, b, 0, pairs);
}

pub fn run(n: usize, threads: usize) -> u64
    requires
        1 <= n <= 0x4000_0000,
        threads <= 0x4000_0000,
        n * (threads + 1) <= usize::MAX,
{
    let t = threads.max(1);
    let mut a = vec![0u32; n];
    let mut tmp = vec![0u32; n];
    fill(a.as_mut_slice());
    let ghost input = a@;
    let mut bounds: Vec<usize> = Vec::new();
    let mut k: usize = 0;
    while k <= t
        invariant
            1 <= t <= threads + 1,
            threads <= 0x4000_0000,
            n * (threads + 1) <= usize::MAX,
            k <= t + 1,
            bounds@.len() == k,
            forall|p: int| 0 <= p < k ==> #[trigger] bounds@[p] == (n * p) / (t as int),
        decreases t + 1 - k,
    {
        proof { lemma_mul_inequality(k as int, threads + 1, n as int); }
        bounds.push(n * k / t);
        k += 1;
    }
    proof {
        assert forall|p: int, q: int| 0 <= p <= q < bounds@.len() implies bounds@[p] <= bounds@[q] by {
            lemma_mul_inequality(p, q, n as int);
            lemma_div_is_ordered(n * p, n * q, t as int);
        }
        lemma_div_by_multiple(n as int, t as int);
        assert(bounds@[t as int] == n);
    }
    sort_level(a.as_mut_slice(), tmp.as_mut_slice(), bounds.as_slice(), t);
    // Fusions par paires, niveau par niveau.
    while bounds.len() > 2
        invariant
            wf_bounds(bounds@, n as int),
            a@.len() == n,
            tmp@.len() == n,
            runs_sorted(a@, bounds@, 0, bounds@.len() - 1, 0),
            a@.to_multiset() == input.to_multiset(),
        decreases bounds@.len(),
    {
        let runs = bounds.len() - 1;
        let pairs = runs / 2;
        let ghost before = a@;
        merge_level(a.as_mut_slice(), tmp.as_mut_slice(), bounds.as_slice(), pairs);
        let mut nb: Vec<usize> = Vec::new();
        let mut q: usize = 0;
        while q < pairs
            invariant
                pairs == runs / 2,
                runs == bounds@.len() - 1,
                q <= pairs,
                nb@.len() == q,
                forall|x: int| 0 <= x < q ==> #[trigger] nb@[x] == bounds@[2 * x],
            decreases pairs - q,
        {
            nb.push(bounds[2 * q]);
            q += 1;
        }
        if runs % 2 == 1 {
            nb.push(bounds[runs - 1]);
        }
        nb.push(n);
        proof {
            // Nouvelles bornes : nb[x] = b[2x] pour x <= pairs, puis n si le nombre de runs est impair.
            assert(forall|x: int| 0 <= x <= pairs ==> #[trigger] nb@[x] == bounds@[2 * x]);
            assert(nb@.len() == runs - pairs + 1);
            assert forall|j: int| 0 <= j < nb@.len() - 1 implies #[trigger] sorted_range(a@, nb@[j] - 0, nb@[j + 1] - 0) by {
                if j < pairs {
                    assert(sorted_range(a@, bounds@[2 * j] - 0, bounds@[2 * j + 2] - 0));
                } else {
                    // le dernier run (nombre impair) n'a pas bougé
                    assert(sorted_range(before, bounds@[runs - 1] - 0, bounds@[(runs - 1) + 1] - 0));
                }
            }
        }
        bounds = nb;
    }
    assert(sorted_range(a@, bounds@[0] - 0, bounds@[0int + 1] - 0));
    assert(sorted_range(a@, 0, n as int) && a@.to_multiset() == input.to_multiset());
    checksum(a.as_slice())
}

} // verus!
