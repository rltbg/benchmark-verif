//! parsort : segments triés en parallèle puis fusions par paires (spec/workloads.d/parsort.md).
//!
//! Prouvé avec Creusot, **threads compris**, sans `#[trusted]` dans ce module :
//!   * noyaux séquentiels `fill`, `msort`, `merge`, `checksum` réutilisés de `sort` (trié ∧ permutation) ;
//!   * `sort_level`/`sort_runs` : chaque segment `[b[j], b[j+1])` est trié par son propre thread (le dernier par le
//!     thread courant) ; tous les runs sont triés et le tableau est une permutation de l'entrée ;
//!   * `merge_level`/`merge_pairs` : les paires de runs voisins d'un niveau sont fusionnées en parallèle (`merge_back` =
//!     fusion + recopie), le run impair final est intact ;
//!   * `run` : bornes `n·k/t` croissantes, invariant des niveaux ; à la fin `a` est trié et est une permutation de l'entrée.
//! Terminaison : prouvée pour les noyaux (`#[check(terminates)]`) ; pas pour `sort_runs`/`merge_pairs`/`run`, car
//! `Scope::spawn` et `join_unwrap` ne sont pas déclarés terminants dans creusot-std.
//! Les threads passent par l'API spécifiée de creusot-std (`creusot_std::std::thread::{scope, Scope::spawn}`,
//! `JoinHandleExt::join_unwrap`), elle-même `#[trusted]` dans la bibliothèque (enveloppes fines de `std::thread::scope`).
//! Différence avec rust-plain : les threads sont joints explicitement (`join_unwrap`) au lieu de l'être implicitement à la
//! fin du `scope` (sinon leur postcondition est perdue), et le découpage est récursif (un appel par segment) au lieu d'une
//! boucle : même nombre de threads, mêmes segments, mêmes fusions.
use crate::sort::{checksum, fill, merge, msort};
#[cfg(creusot)]
use crate::sort::perm_concat;
use creusot_std::prelude::*;
use creusot_std::std::thread::{JoinHandleExt, Scope, scope};

/// Bornes de runs : `b[0] = 0 <= b[1] <= … <= b[last] = n`.
#[logic(open)]
pub fn wf_bounds(b: Seq<usize>, n: Int) -> bool {
    pearlite! {
        b.len() >= 2 && b[0]@ == 0 && b[b.len() - 1]@ == n
        && forall<p, q> 0 <= p && p <= q && q < b.len() ==> b[p] <= b[q]
    }
}

/// Les runs `[b[j], b[j+1])` pour `j ∈ [k, m)`, décalés de `base`, sont triés dans `s`.
#[logic(open)]
pub fn runs_sorted(s: Seq<u32>, b: Seq<usize>, k: Int, m: Int, base: Int) -> bool {
    pearlite! { forall<j> k <= j && j < m ==> s.sorted_range(b[j]@ - base, b[j + 1]@ - base) }
}

/// Les paires fusionnées `[b[2q], b[2q+2])` pour `q ∈ [p, m)`, décalées de `base`, sont triées dans `s`.
#[logic(open)]
pub fn pairs_sorted(s: Seq<u32>, b: Seq<usize>, p: Int, m: Int, base: Int) -> bool {
    pearlite! { forall<q> p <= q && q < m ==> s.sorted_range(b[2 * q]@ - base, b[2 * q + 2]@ - base) }
}

/// Lemme : `s = s[..m] ++ s[m..]`.
#[logic]
#[requires(0 <= m && m <= s.len())]
#[ensures(s == s[..m].concat(s[m..]))]
pub fn split_concat(s: Seq<u32>, m: Int) {}

/// Lemme : dans `u ++ v`, une plage triée de `v` l'est décalée de `|u|`.
#[logic]
#[requires(0 <= x && x <= y && y <= v.len() && v.sorted_range(x, y))]
#[ensures(u.concat(v).sorted_range(x + u.len(), y + u.len()))]
pub fn sorted_shift(u: Seq<u32>, v: Seq<u32>, x: Int, y: Int) {}

/// Lemme : si `s[..m)` (le premier run, `m = b[k+1] - b[k]`) est trié et que les runs suivants le sont dans
/// `s[m..)`, tous les runs `[k, t)` sont triés dans `s`. Énoncé sur `s` lui-même (et non sur `u ++ v`) pour que le
/// prouveur l'applique directement à la valeur finale (prophétisée) de la tranche, sans égalité de suites à établir.
#[logic]
#[requires(0 <= k && k < t && t < b.len() && forall<p, q> 0 <= p && p <= q && q < b.len() ==> b[p] <= b[q])]
#[requires(s.len() == b[t]@ - b[k]@ && m == b[k + 1]@ - b[k]@)]
#[requires(s[..m].sorted() && runs_sorted(s[m..], b, k + 1, t, b[k + 1]@))]
#[ensures(runs_sorted(s, b, k, t, b[k]@))]
pub fn runs_split(s: Seq<u32>, b: Seq<usize>, k: Int, t: Int, m: Int) {
    let (u, v) = (s[..m], s[m..]);
    split_concat(s, m);
    proof_assert!(s.sorted_range(0, m));
    proof_assert!(forall<j> k < j && j < t ==> {
        sorted_shift(u, v, b[j]@ - b[k + 1]@, b[j + 1]@ - b[k + 1]@);
        s.sorted_range(b[j]@ - b[k]@, b[j + 1]@ - b[k]@)
    });
}

/// Lemme : idem pour les paires fusionnées `[b[2q], b[2q+2])`, `q ∈ [p, m)`, avec `h = b[2p+2] - b[2p]`.
#[logic]
#[requires(0 <= p && p < m && 2 * m < b.len() && forall<x, y> 0 <= x && x <= y && y < b.len() ==> b[x] <= b[y])]
#[requires(h == b[2 * p + 2]@ - b[2 * p]@ && s.len() >= b[2 * m]@ - b[2 * p]@)]
#[requires(s[..h].sorted() && pairs_sorted(s[h..], b, p + 1, m, b[2 * p + 2]@))]
#[ensures(pairs_sorted(s, b, p, m, b[2 * p]@))]
pub fn pairs_split(s: Seq<u32>, b: Seq<usize>, p: Int, m: Int, h: Int) {
    let (u, v) = (s[..h], s[h..]);
    split_concat(s, h);
    proof_assert!(s.sorted_range(0, h));
    proof_assert!(forall<q> p < q && q < m ==> {
        sorted_shift(u, v, b[2 * q]@ - b[2 * p + 2]@, b[2 * q + 2]@ - b[2 * p + 2]@);
        s.sorted_range(b[2 * q]@ - b[2 * p]@, b[2 * q + 2]@ - b[2 * p]@)
    });
}

/// Trie chaque segment `[b[j], b[j+1])`, `j ∈ [k, t)`, de `a` (qui commence à `b[k]`) : un thread par segment,
/// le dernier sur le thread courant.
#[requires(k@ < t@ && b@.len() == t@ + 1)]
#[requires(forall<p, q> 0 <= p && p <= q && q < b@.len() ==> b@[p] <= b@[q])]
#[requires(a@.len() == b@[t@]@ - b@[k@]@ && tmp@.len() == a@.len())]
#[ensures(runs_sorted((^a)@, b@, k@, t@, b@[k@]@))]
#[ensures((^a)@.permutation_of(a@))]
pub fn sort_runs<'scope, 'env>(
    s: &mut Scope<'scope, 'env>, a: &'scope mut [u32], tmp: &'scope mut [u32], b: &[usize], k: usize, t: usize,
) {
    let len = b[k + 1] - b[k];
    let (ma, ra) = a.split_at_mut(len);
    let (mt, rt) = tmp.split_at_mut(len);
    let (bm, br) = (snapshot!(ma), snapshot!(ra));
    let _ = snapshot!(split_concat);
    if k + 1 < t {
        let h = s.spawn(move |_| msort(ma, mt, 0, len));
        sort_runs(s, ra, rt, b, k + 1, t);
        h.join_unwrap();
    } else {
        msort(ma, mt, 0, len);
        proof_assert!(br@ == Seq::empty() && (^*br)@ == Seq::empty());
    }
    // a = ma ++ ra, et chaque moitié finale est une permutation de l'initiale.
    proof_assert!((^*bm)@[0..len@] == (^*bm)@ && bm@[0..len@] == bm@);
    proof_assert! { perm_concat((^*bm)@, bm@, (^*br)@, br@); true };
    // Runs triés : le premier dans ma, les suivants dans ra (décalés de len) ; le lemme `runs_split`, rendu
    // disponible ici, s'applique à la valeur finale de `a` lors de la preuve de la postcondition.
    let _ = snapshot!(runs_split);
}

/// Travail d'une fusion : `a[..m)` et `a[m..)` triés -> `a` trié (fusion dans `tmp` puis recopie).
#[requires(a@.len() == tmp@.len() && m@ <= a@.len())]
#[requires(a@.sorted_range(0, m@) && a@.sorted_range(m@, a@.len()))]
#[ensures((^a)@.sorted() && (^a)@.permutation_of(a@))]
#[check(terminates)]
#[inline(always)]
pub fn merge_back(a: &mut [u32], tmp: &mut [u32], m: usize) {
    let a0 = snapshot!(a@);
    let n = a.len();
    merge(a, tmp, 0, m, n);
    a.copy_from_slice(tmp);
    proof_assert!(a@[0..n@] == a@ && a0[0..n@] == *a0);
}

/// Fusionne les paires de runs `[b[2q], b[2q+1])` + `[b[2q+1], b[2q+2])`, `q ∈ [p, pairs)`, de `a` (qui commence à
/// `b[2p]` et va jusqu'à la fin du tableau) : une fusion par thread, la dernière sur le thread courant ; le reste est intact.
#[requires(p@ < pairs@ && 2 * pairs@ + 1 <= b@.len())]
#[requires(forall<x, y> 0 <= x && x <= y && y < b@.len() ==> b@[x] <= b@[y])]
#[requires(a@.len() == b@[b@.len() - 1]@ - b@[2 * p@]@ && tmp@.len() == a@.len())]
#[requires(runs_sorted(a@, b@, 2 * p@, 2 * pairs@, b@[2 * p@]@))]
#[ensures(pairs_sorted((^a)@, b@, p@, pairs@, b@[2 * p@]@))]
#[ensures((^a)@.permutation_of(a@))]
#[ensures(forall<i> b@[2 * pairs@]@ - b@[2 * p@]@ <= i && i < a@.len() ==> (^a)@[i] == a@[i])]
pub fn merge_pairs<'scope, 'env>(
    s: &mut Scope<'scope, 'env>, a: &'scope mut [u32], tmp: &'scope mut [u32], b: &[usize], p: usize, pairs: usize,
) {
    let (lo, mid, hi) = (b[2 * p], b[2 * p + 1], b[2 * p + 2]);
    let (ma, ra) = a.split_at_mut(hi - lo);
    let (mt, rt) = tmp.split_at_mut(hi - lo);
    let (bm, br) = (snapshot!(ma), snapshot!(ra));
    let _ = snapshot!(split_concat);
    proof_assert!(ma@.sorted_range(0, mid@ - lo@) && ma@.sorted_range(mid@ - lo@, hi@ - lo@));
    if p + 1 < pairs {
        let h = s.spawn(move |_| merge_back(ma, mt, mid - lo));
        merge_pairs(s, ra, rt, b, p + 1, pairs);
        h.join_unwrap();
    } else {
        merge_back(ma, mt, mid - lo);
    }
    proof_assert! { perm_concat((^*bm)@, bm@, (^*br)@, br@); true };
    let _ = snapshot!(pairs_split);
}

/// Phase 1 : les `t` segments triés en parallèle (un `scope`, `t - 1` threads lancés).
#[requires(a@.len() == tmp@.len() && wf_bounds(b@, a@.len()) && b@.len() == t@ + 1 && 1 <= t@)]
#[ensures(runs_sorted((^a)@, b@, 0, t@, 0) && (^a)@.permutation_of(a@))]
pub fn sort_level(a: &mut [u32], tmp: &mut [u32], b: &[usize], t: usize) {
    scope(move |s| sort_runs(s, a, tmp, b, 0, t))
}

/// Phase 2, un niveau : les `pairs` paires de runs fusionnées en parallèle ; le run impair final est intact.
#[requires(a@.len() == tmp@.len() && wf_bounds(b@, a@.len()) && 1 <= pairs@ && 2 * pairs@ + 1 <= b@.len())]
#[requires(runs_sorted(a@, b@, 0, b@.len() - 1, 0))]
#[ensures(pairs_sorted((^a)@, b@, 0, pairs@, 0) && (^a)@.permutation_of(a@))]
#[ensures(forall<i> b@[2 * pairs@]@ <= i && i < a@.len() ==> (^a)@[i] == a@[i])]
pub fn merge_level(a: &mut [u32], tmp: &mut [u32], b: &[usize], pairs: usize) {
    scope(move |s| merge_pairs(s, a, tmp, b, 0, pairs))
}

#[requires(1 <= n@ && n@ <= 1073741824 && threads@ <= 1073741824)]
pub fn run(n: usize, threads: usize) -> u64 {
    let t = threads.max(1);
    let mut a = std::vec![0u32; n];
    let mut tmp = std::vec![0u32; n];
    fill(&mut a);
    let input = snapshot!(a@);
    let mut bounds: Vec<usize> = Vec::new();
    let mut k = 0usize;
    #[invariant(k@ <= t@ + 1 && bounds@.len() == k@)]
    #[invariant(forall<p> 0 <= p && p < k@ ==> bounds@[p]@ == n@ * p / t@)]
    while k <= t {
        bounds.push(n * k / t);
        k += 1;
    }
    proof_assert!(forall<p, q> 0 <= p && p <= q && q <= t@ ==> n@ * p / t@ <= n@ * q / t@);
    proof_assert!(n@ * t@ / t@ == n@);
    sort_level(&mut a, &mut tmp, &bounds, t);
    // Fusions par paires, niveau par niveau.
    #[invariant(wf_bounds(bounds@, n@) && a@.len() == n@ && tmp@.len() == n@)]
    #[invariant(runs_sorted(a@, bounds@, 0, bounds@.len() - 1, 0) && a@.permutation_of(*input))]
    while bounds.len() > 2 {
        let runs = bounds.len() - 1;
        let pairs = runs / 2;
        let before = snapshot!(a@);
        merge_level(&mut a, &mut tmp, &bounds, pairs);
        let mut nb: Vec<usize> = Vec::new();
        let mut q = 0usize;
        #[invariant(q@ <= pairs@ && nb@.len() == q@)]
        #[invariant(forall<x> 0 <= x && x < q@ ==> nb@[x] == bounds@[2 * x])]
        while q < pairs {
            nb.push(bounds[2 * q]);
            q += 1;
        }
        if runs % 2 == 1 {
            nb.push(bounds[runs - 1]);
        }
        nb.push(n);
        // Nouvelles bornes : nb[x] = b[2x] pour x <= pairs, puis n si le nombre de runs est impair.
        proof_assert!(runs@ == 2 * pairs@ || runs@ == 2 * pairs@ + 1);
        proof_assert!(forall<x> 0 <= x && x <= pairs@ ==> nb@[x] == bounds@[2 * x]);
        proof_assert!(nb@.len() == runs@ - pairs@ + 1 && nb@[nb@.len() - 1]@ == n@);
        proof_assert!(forall<x, y> 0 <= x && x <= y && y < nb@.len() ==> nb@[x] <= nb@[y]);
        // Le dernier run (nombre impair) n'a pas bougé : il est toujours trié.
        proof_assert!(forall<i> bounds@[2 * pairs@]@ <= i && i < n@ ==> a@[i] == before[i]);
        proof_assert!(runs@ == 2 * pairs@ + 1 ==> a@.sorted_range(bounds@[runs@ - 1]@, n@));
        proof_assert!(runs@ == 2 * pairs@ + 1 ==> a@.sorted_range(nb@[pairs@]@, nb@[pairs@ + 1]@));
        proof_assert!(forall<x> 0 <= x && x < pairs@ ==> a@.sorted_range(nb@[x]@, nb@[x + 1]@));
        proof_assert!(runs_sorted(a@, nb@, 0, nb@.len() - 1, 0));
        bounds = nb;
    }
    proof_assert!(a@.sorted_range(0, n@) && a@.permutation_of(*input));
    checksum(&a)
}
