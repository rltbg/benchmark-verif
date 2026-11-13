//! bfs : graphe creux aléatoire (m = 16 n arêtes), CSR par tri par comptage, puis BFS depuis 0.
//! Spécification : spec/workloads.d/bfs.md. 1 <= n <= 100_000_000.
//!
//! Prouvé avec Verus :
//!   * CSR : `off[i] = psum(i)` (somme des degrés sortants des sommets < i ; `src(j) = mix64(SEED + 2j) mod n`,
//!     `dst(j) = mix64(SEED + 2j + 1) mod n`), donc `off[i] <= m` ; `fill` range chaque arête j dans
//!     `[off[src j], off[src j + 1])` (`placed`) et chaque case de cette plage contient la cible d'une arête de
//!     source u (`slot_ok`) : les arcs du CSR sont exactement les arêtes générées ;
//!   * BFS : `dist[0] = 0` ; tout sommet atteint v ≠ 0 a un prédécesseur atteint u (arc u→v) avec
//!     `dist[u] + 1 = dist[v]` ; pour tout arc u→v avec u atteint, v est atteint et `dist[v] <= dist[u] + 1` ;
//!     le résultat est le nombre de sommets atteints. `lemma_bfs_on_edges` transporte ces propriétés aux arêtes
//!     générées (`src(j) → dst(j)`). Elles caractérisent les distances BFS ;
//!   * absence de panique et de débordement, terminaison (`decreases`) de toutes les fonctions.
//! Gardes d'exécution supprimées (`e < m`, `w < n`, `tail < n`, garde de `fill`) : ce sont des invariants prouvés.
//! Aucun `assume`/`admit`/`external_body`.
use crate::common::*;
use vstd::prelude::*;

verus! {

pub const SEED: u64 = 0xB5F0000000000001;
pub const INF: u32 = 0xFFFFFFFF;

// ───────────────────────────── Modèle : génération et CSR ─────────────────────────────

/// Extrémité k (0 : source, 1 : cible) de l'arête j, dans [0, n) (opaque : splitmix64 n'est déplié que dans `endp`).
#[verifier::opaque]
pub open spec fn endp_s(j: int, k: int, n: int) -> int {
    spec_mix64((SEED + 2 * j + k) as u64) as int % n
}

pub open spec fn src(j: int, n: int) -> int {
    endp_s(j, 0, n)
}

/// Nombre d'arêtes de source u parmi les j premières.
pub open spec fn cnt(u: int, j: int, n: int) -> nat
    decreases j,
{
    if j <= 0 {
        0
    } else {
        cnt(u, j - 1, n) + if src(j - 1, n) == u { 1nat } else { 0nat }
    }
}

/// Somme des degrés (parmi les j premières arêtes) des sommets < i : début de i dans le CSR.
pub open spec fn psum(i: int, j: int, n: int) -> nat
    decreases i,
{
    if i <= 0 {
        0
    } else {
        psum(i - 1, j, n) + cnt(i - 1, j, n)
    }
}

pub open spec fn dst(j: int, n: int) -> int {
    endp_s(j, 1, n)
}

/// L'arête j (cible dst(j)) est rangée dans adj[lo..hi).
pub open spec fn placed(adj: Seq<u32>, j: int, lo: int, hi: int, n: int) -> bool {
    exists|e: int| lo <= e < hi && #[trigger] adj[e] == dst(j, n)
}

/// adj[e] est la cible d'une arête j < k de source u.
pub open spec fn slot_ok(adj: Seq<u32>, u: int, e: int, k: int, n: int) -> bool {
    exists|j: int| 0 <= j < k && src(j, n) == u && #[trigger] dst(j, n) == adj[e]
}

// ───────────────────────────── Modèle : BFS sur le CSR ─────────────────────────────

/// Arc u→v du graphe CSR.
pub open spec fn arc(off: Seq<u32>, adj: Seq<u32>, u: int, v: int) -> bool {
    exists|e: int| off[u] <= e < off[u + 1] && #[trigger] adj[e] == v
}

/// Nombre de sommets atteints (dist != INF) parmi les k premiers.
pub open spec fn reached(d: Seq<u32>, k: int) -> nat
    decreases k,
{
    if k <= 0 {
        0
    } else {
        reached(d, k - 1) + if d[k - 1] != INF { 1nat } else { 0nat }
    }
}

/// Tout sommet atteint figure dans q[0..tail) (opaque : déplié seulement dans les lemmes de BFS).
#[verifier::opaque]
pub open spec fn queued(q: Seq<u32>, d: Seq<u32>, tail: int, n: int) -> bool {
    forall|v: int| 0 <= v < n && #[trigger] d[v] != INF ==> exists|h: int| 0 <= h < tail && #[trigger] q[h] == v
}

/// Tout sommet atteint v ≠ 0 a un prédécesseur atteint x (arc x→v) avec d[x] + 1 = d[v].
#[verifier::opaque]
pub open spec fn preds(off: Seq<u32>, adj: Seq<u32>, d: Seq<u32>, n: int) -> bool {
    forall|v: int| 0 <= v < n && v != 0 && #[trigger] d[v] != INF ==>
        exists|x: int| 0 <= x < n && d[x] != INF && #[trigger] arc(off, adj, x, v) && d[x] + 1 == d[v]
}

/// Les arcs issus de q[0..head) (sommets traités) mènent à des sommets atteints, à distance au plus +1.
#[verifier::opaque]
pub open spec fn processed(off: Seq<u32>, adj: Seq<u32>, q: Seq<u32>, d: Seq<u32>, head: int) -> bool {
    forall|h: int, w: int| 0 <= h < head && #[trigger] arc(off, adj, q[h] as int, w) ==>
        d[w] != INF && d[w] <= d[q[h] as int] + 1
}

// ───────────────────────────── Lemmes ─────────────────────────────

pub proof fn lemma_endp_bound(j: int, k: int, n: int)
    requires
        0 < n,
    ensures
        0 <= endp_s(j, k, n) < n,
{
    reveal(endp_s);
    vstd::arithmetic::div_mod::lemma_mod_bound(spec_mix64((SEED + 2 * j + k) as u64) as int, n);
}

pub proof fn lemma_cnt_bound(u: int, j: int, k: int, n: int)
    requires
        0 <= j <= k,
    ensures
        cnt(u, j, n) <= j,
        cnt(u, j, n) <= cnt(u, k, n),
    decreases k,
{
    if j < k {
        lemma_cnt_bound(u, j, k - 1, n);
    } else if j > 0 {
        lemma_cnt_bound(u, j - 1, j - 1, n);
    }
}

/// Ajouter l'arête j augmente psum(i) de 1 si sa source est < i.
pub proof fn lemma_psum_step(i: int, j: int, n: int)
    requires
        0 <= i,
        0 <= j,
    ensures
        psum(i, j + 1, n) == psum(i, j, n) + if 0 <= src(j, n) < i { 1nat } else { 0nat },
    decreases i,
{
    if i > 0 {
        lemma_psum_step(i - 1, j, n);
    }
}

/// Toutes les sources sont < n : psum(n, j) = j.
pub proof fn lemma_psum_total(n: int, j: int)
    requires
        0 < n,
        0 <= j,
    ensures
        psum(n, j, n) == j,
    decreases j,
{
    if j > 0 {
        lemma_psum_total(n, j - 1);
        lemma_psum_step(n, j - 1, n);
        lemma_endp_bound(j - 1, 0, n);
    } else {
        lemma_psum_zero(n, n);
    }
}

pub proof fn lemma_psum_zero(i: int, n: int)
    requires
        0 <= i,
    ensures
        psum(i, 0, n) == 0,
    decreases i,
{
    if i > 0 {
        lemma_psum_zero(i - 1, n);
    }
}

/// psum(i) <= psum(k) pour i <= k.
pub proof fn lemma_psum_mono(k: int, j: int, n: int)
    requires
        0 <= k,
    ensures
        forall|i: int| 0 <= i <= k ==> #[trigger] psum(i, j, n) <= psum(k, j, n),
    decreases k,
{
    if k > 0 {
        lemma_psum_mono(k - 1, j, n);
    }
}

pub proof fn lemma_reached_bound(d: Seq<u32>, k: int)
    requires
        0 <= k,
    ensures
        reached(d, k) <= k,
        forall|w: int| 0 <= w < k && d[w] == INF ==> reached(d, k) < k,
    decreases k,
{
    if k > 0 {
        lemma_reached_bound(d, k - 1);
    }
}

/// reached ne dépend que des k premières cases.
pub proof fn lemma_reached_same(d: Seq<u32>, e: Seq<u32>, k: int)
    requires
        0 <= k,
        forall|p: int| 0 <= p < k ==> d[p] == e[p],
    ensures
        reached(d, k) == reached(e, k),
    decreases k,
{
    if k > 0 {
        lemma_reached_same(d, e, k - 1);
    }
}

/// Aucun sommet atteint.
pub proof fn lemma_reached_zero(d: Seq<u32>, k: int)
    requires
        0 <= k,
        forall|p: int| 0 <= p < k ==> d[p] == INF,
    ensures
        reached(d, k) == 0,
    decreases k,
{
    if k > 0 {
        lemma_reached_zero(d, k - 1);
    }
}

/// Marquer un sommet non atteint w augmente le compte de 1.
pub proof fn lemma_reached_set(d: Seq<u32>, w: int, x: u32, k: int)
    requires
        0 <= w < k <= d.len(),
        d[w] == INF,
        x != INF,
    ensures
        reached(d.update(w, x), k) == reached(d, k) + 1,
    decreases k,
{
    if k - 1 > w {
        lemma_reached_set(d, w, x, k - 1);
    } else {
        lemma_reached_same(d, d.update(w, x), w);
    }
}

/// Découverte de w depuis u : les trois invariants opaques de la BFS sont préservés.
pub proof fn lemma_discover(off: Seq<u32>, adj: Seq<u32>, q: Seq<u32>, d: Seq<u32>, n: int, head: int, tail: int,
    u: int, w: int, du: u32)
    requires
        d.len() == n,
        0 <= head <= tail < q.len(),
        forall|h: int| 0 <= h < tail ==> #[trigger] d[q[h] as int] != INF,
        queued(q, d, tail, n),
        preds(off, adj, d, n),
        processed(off, adj, q, d, head),
        0 <= u < n,
        d[u] != INF,
        0 <= w < n,
        d[w] == INF,
        arc(off, adj, u, w),
        du == d[u] + 1,
    ensures
        queued(q.update(tail, w as u32), d.update(w, du), tail + 1, n),
        preds(off, adj, d.update(w, du), n),
        processed(off, adj, q.update(tail, w as u32), d.update(w, du), head),
{
    reveal(queued);
    reveal(preds);
    reveal(processed);
    let (q2, d2) = (q.update(tail, w as u32), d.update(w, du));
    assert forall|v: int| 0 <= v < n && #[trigger] d2[v] != INF implies exists|h: int| 0 <= h < tail + 1 && #[trigger] q2[h] == v by {
        if v == w {
            assert(q2[tail] == v);
        } else {
            let h = choose|h: int| 0 <= h < tail && #[trigger] q[h] == v;
            assert(q2[h] == v);
        }
    }
    assert forall|v: int| 0 <= v < n && v != 0 && #[trigger] d2[v] != INF implies
        exists|x: int| 0 <= x < n && d2[x] != INF && #[trigger] arc(off, adj, x, v) && d2[x] + 1 == d2[v] by {
        if v == w {
            assert(d2[u] != INF && arc(off, adj, u, v) && d2[u] + 1 == d2[v]);
        } else {
            let x = choose|x: int| 0 <= x < n && d[x] != INF && #[trigger] arc(off, adj, x, v) && d[x] + 1 == d[v];
            assert(d2[x] != INF && arc(off, adj, x, v) && d2[x] + 1 == d2[v]);
        }
    }
    assert forall|h: int, x: int| 0 <= h < head && #[trigger] arc(off, adj, q2[h] as int, x) implies
        d2[x] != INF && d2[x] <= d2[q2[h] as int] + 1 by {
        assert(q2[h] == q[h]);
        assert(arc(off, adj, q[h] as int, x));
        assert(d[q[h] as int] != INF);
    }
}

/// Fin du traitement de u = q[head] : tous ses arcs mènent à des sommets atteints à distance <= d[u] + 1.
pub proof fn lemma_processed_next(off: Seq<u32>, adj: Seq<u32>, q: Seq<u32>, d: Seq<u32>, head: int)
    requires
        0 <= head < q.len(),
        processed(off, adj, q, d, head),
        forall|w: int| #[trigger] arc(off, adj, q[head] as int, w) ==> d[w] != INF && d[w] <= d[q[head] as int] + 1,
    ensures
        processed(off, adj, q, d, head + 1),
{
    reveal(processed);
}

/// En fin de BFS (file vidée) : propriétés de sortie.
pub proof fn lemma_bfs_done(off: Seq<u32>, adj: Seq<u32>, q: Seq<u32>, d: Seq<u32>, n: int, tail: int)
    requires
        queued(q, d, tail, n),
        processed(off, adj, q, d, tail),
        preds(off, adj, d, n),
    ensures
        forall|v: int| 0 <= v < n && v != 0 && #[trigger] d[v] != INF ==>
            exists|x: int| 0 <= x < n && d[x] != INF && #[trigger] arc(off, adj, x, v) && d[x] + 1 == d[v],
        forall|u: int, v: int| 0 <= u < n && d[u] != INF && #[trigger] arc(off, adj, u, v) ==>
            d[v] != INF && d[v] <= d[u] + 1,
{
    reveal(queued);
    reveal(preds);
    reveal(processed);
    assert forall|u: int, v: int| 0 <= u < n && d[u] != INF && #[trigger] arc(off, adj, u, v) implies
        d[v] != INF && d[v] <= d[u] + 1 by {
        let h = choose|h: int| 0 <= h < tail && #[trigger] q[h] == u;
        assert(arc(off, adj, q[h] as int, v));
    }
}

/// Théorème final : les propriétés de la BFS valent pour les arêtes *générées* (src(j) → dst(j)), pas seulement
/// pour les arcs du CSR, grâce à la correspondance établie par `prefix` et `fill`.
pub proof fn lemma_bfs_on_edges(off: Seq<u32>, adj: Seq<u32>, d: Seq<u32>, n: int, m: int)
    requires
        1 <= n,
        off.len() == n + 1,
        forall|i: int| 0 <= i <= n ==> #[trigger] off[i] == psum(i, m, n),
        forall|j: int| 0 <= j < m ==> #[trigger] placed(adj, j, psum(src(j, n), m, n) as int, psum(src(j, n) + 1, m, n) as int, n),
        forall|u: int, e: int| 0 <= u < n && psum(u, m, n) <= e < psum(u + 1, m, n) ==> #[trigger] slot_ok(adj, u, e, m, n),
        forall|v: int| 0 <= v < n && v != 0 && #[trigger] d[v] != INF ==>
            exists|x: int| 0 <= x < n && d[x] != INF && #[trigger] arc(off, adj, x, v) && d[x] + 1 == d[v],
        forall|u: int, v: int| 0 <= u < n && d[u] != INF && #[trigger] arc(off, adj, u, v) ==> d[v] != INF && d[v] <= d[u] + 1,
    ensures
        forall|j: int| 0 <= j < m && d[src(j, n)] != INF ==>
            d[#[trigger] dst(j, n)] != INF && d[dst(j, n)] <= d[src(j, n)] + 1,
        forall|v: int| 0 <= v < n && v != 0 && #[trigger] d[v] != INF ==>
            exists|j: int| 0 <= j < m && #[trigger] dst(j, n) == v && d[src(j, n)] != INF && d[src(j, n)] + 1 == d[v],
{
    assert forall|j: int| 0 <= j < m && d[src(j, n)] != INF implies
        d[#[trigger] dst(j, n)] != INF && d[dst(j, n)] <= d[src(j, n)] + 1 by {
        let u = src(j, n);
        lemma_endp_bound(j, 0, n);
        assert(placed(adj, j, psum(u, m, n) as int, psum(u + 1, m, n) as int, n));
        let e = choose|e: int| psum(u, m, n) <= e < psum(u + 1, m, n) && #[trigger] adj[e] == dst(j, n);
        assert(off[u] == psum(u, m, n) && off[u + 1] == psum(u + 1, m, n));
        assert(arc(off, adj, u, dst(j, n)));
    }
    assert forall|v: int| 0 <= v < n && v != 0 && #[trigger] d[v] != INF implies
        exists|j: int| 0 <= j < m && #[trigger] dst(j, n) == v && d[src(j, n)] != INF && d[src(j, n)] + 1 == d[v] by {
        let x = choose|x: int| 0 <= x < n && d[x] != INF && #[trigger] arc(off, adj, x, v) && d[x] + 1 == d[v];
        let e = choose|e: int| off[x] <= e < off[x + 1] && #[trigger] adj[e] == v;
        assert(off[x] == psum(x, m, n) && off[x + 1] == psum(x + 1, m, n));
        assert(slot_ok(adj, x, e, m, n));
        let j = choose|j: int| 0 <= j < m && src(j, n) == x && #[trigger] dst(j, n) == adj[e];
        assert(dst(j, n) == v);
    }
}

// ───────────────────────────── Code exécutable ─────────────────────────────

/// Extrémité `k` (0 : source, 1 : cible) de l'arête `j`, ramenée dans [0, nn).
#[inline(always)]
pub fn endp(j: u64, k: u64, nn: u64) -> (r: u64)
    requires
        0 < nn,
        j < 2000000000,
        k <= 1,
    ensures
        r == endp_s(j as int, k as int, nn as int),
        r < nn,
{
    proof {
        reveal(endp_s);
    }
    mix64(SEED + 2 * j + k) % nn
}

/// Passe 1 : off[u+1] = nombre d'arêtes de source u.
pub fn count_deg(off: &mut Vec<u32>, n: usize, m: usize)
    requires
        1 <= n <= 100000000,
        m == 16 * n,
        old(off)@.len() == n + 1,
        forall|i: int| 0 <= i <= n ==> old(off)@[i] == 0,
    ensures
        final(off)@.len() == n + 1,
        final(off)@[0] == 0,
        forall|u: int| 0 <= u < n ==> #[trigger] final(off)@[u + 1] == cnt(u, m as int, n as int),
{
    let nn = n as u64;
    let mut j: usize = 0;
    while j < m
        invariant
            1 <= n <= 100000000,
            m == 16 * n,
            j <= m,
            nn == n,
            off@.len() == n + 1,
            off@[0] == 0,
            forall|u: int| 0 <= u < n ==> #[trigger] off@[u + 1] == cnt(u, j as int, n as int),
        decreases m - j,
    {
        let u = endp(j as u64, 0, nn) as usize;
        proof {
            lemma_cnt_bound(u as int, j as int, j as int, n as int);
        }
        off[u + 1] += 1;
        assert forall|w: int| 0 <= w < n implies #[trigger] off@[w + 1] == cnt(w, j + 1, n as int) by {}
        j += 1;
    }
}

/// Passe 2 : sommes préfixes, off[i] = psum(i, m) <= m.
pub fn prefix(off: &mut Vec<u32>, n: usize, m: Ghost<int>)
    requires
        1 <= n <= 100000000,
        m@ == 16 * n,
        old(off)@.len() == n + 1,
        old(off)@[0] == 0,
        forall|u: int| 0 <= u < n ==> #[trigger] old(off)@[u + 1] == cnt(u, m@, n as int),
    ensures
        final(off)@.len() == n + 1,
        forall|i: int| 0 <= i <= n ==> #[trigger] final(off)@[i] == psum(i, m@, n as int) && final(off)@[i] <= m@,
{
    proof {
        lemma_psum_total(n as int, m@);
        lemma_psum_mono(n as int, m@, n as int);
        assert(psum(0, m@, n as int) == 0);
    }
    assert forall|k: int| 1 <= k <= n implies #[trigger] off@[k] == cnt(k - 1, m@, n as int) by {
        assert(off@[(k - 1) + 1] == cnt(k - 1, m@, n as int));
    }
    let mut i: usize = 1;
    while i <= n
        invariant
            1 <= n <= 100000000,
            m@ == 16 * n,
            1 <= i <= n + 1,
            off@.len() == n + 1,
            psum(n as int, m@, n as int) == m@,
            forall|k: int| 0 <= k <= n ==> #[trigger] psum(k, m@, n as int) <= psum(n as int, m@, n as int),
            forall|k: int| 0 <= k < i ==> #[trigger] off@[k] == psum(k, m@, n as int),
            forall|k: int| i <= k <= n ==> #[trigger] off@[k] == cnt(k - 1, m@, n as int),
        decreases n + 1 - i,
    {
        assert(psum(i as int, m@, n as int) == psum(i - 1, m@, n as int) + cnt(i - 1, m@, n as int));
        assert(psum(i as int, m@, n as int) <= m@);
        off[i] += off[i - 1];
        i += 1;
    }
}

/// cur[i] <- off[i] pour i < n.
pub fn copy_cur(off: &Vec<u32>, cur: &mut Vec<u32>, n: usize)
    requires
        off@.len() == n + 1,
        old(cur)@.len() == n,
    ensures
        final(cur)@.len() == n,
        forall|i: int| 0 <= i < n ==> #[trigger] final(cur)@[i] == off@[i],
{
    let mut i: usize = 0;
    while i < n
        invariant
            i <= n,
            off@.len() == n + 1,
            cur@.len() == n,
            forall|k: int| 0 <= k < i ==> #[trigger] cur@[k] == off@[k],
        decreases n - i,
    {
        cur[i] = off[i];
        i += 1;
    }
}

/// Passe 3 : adj[cur[u]++] = v. L'arête j de source u va dans [off[u], off[u+1]) ⊆ [0, m).
pub fn fill(cur: &mut Vec<u32>, adj: &mut Vec<u32>, n: usize, m: usize)
    requires
        1 <= n <= 100000000,
        m == 16 * n,
        old(cur)@.len() == n,
        old(adj)@.len() == m,
        forall|u: int| 0 <= u < n ==> #[trigger] old(cur)@[u] == psum(u, m as int, n as int),
        forall|e: int| 0 <= e < m ==> #[trigger] old(adj)@[e] < n,
    ensures
        final(adj)@.len() == m,
        forall|e: int| 0 <= e < m ==> #[trigger] final(adj)@[e] < n,
        // Le CSR range exactement les arêtes générées : chaque arête j est dans la plage de sa source,
        // chaque case de la plage de u est la cible d'une arête de source u.
        forall|j: int| 0 <= j < m ==> #[trigger] placed(final(adj)@, j, psum(src(j, n as int), m as int, n as int) as int,
            psum(src(j, n as int) + 1, m as int, n as int) as int, n as int),
        forall|u: int, e: int| 0 <= u < n && psum(u, m as int, n as int) <= e < psum(u + 1, m as int, n as int) ==>
            #[trigger] slot_ok(final(adj)@, u, e, m as int, n as int),
{
    let nn = n as u64;
    proof {
        lemma_psum_total(n as int, m as int);
        lemma_psum_mono(n as int, m as int, n as int);
    }
    let mut j: usize = 0;
    while j < m
        invariant
            1 <= n <= 100000000,
            m == 16 * n,
            nn == n,
            j <= m,
            cur@.len() == n,
            adj@.len() == m,
            psum(n as int, m as int, n as int) == m,
            forall|k: int| 0 <= k <= n ==> #[trigger] psum(k, m as int, n as int) <= psum(n as int, m as int, n as int),
            forall|u: int| 0 <= u < n ==> #[trigger] cur@[u] == psum(u, m as int, n as int) + cnt(u, j as int, n as int),
            forall|e: int| 0 <= e < m ==> #[trigger] adj@[e] < n,
            forall|jj: int| 0 <= jj < j ==> #[trigger] placed(adj@, jj, psum(src(jj, n as int), m as int, n as int) as int,
                cur@[src(jj, n as int)] as int, n as int),
            forall|uu: int, e: int| 0 <= uu < n && psum(uu, m as int, n as int) <= e < cur@[uu] ==>
                #[trigger] slot_ok(adj@, uu, e, j as int, n as int),
        decreases m - j,
    {
        let u = endp(j as u64, 0, nn) as usize;
        let v = endp(j as u64, 1, nn) as u32;
        // cur[u] = off[u] + cnt(u, j) < off[u] + cnt(u, m) = off[u+1] <= m
        proof {
            lemma_cnt_bound(u as int, j + 1, m as int, n as int);
            assert(cnt(u as int, j + 1, n as int) == cnt(u as int, j as int, n as int) + 1);
            assert(psum(u + 1, m as int, n as int) == psum(u as int, m as int, n as int) + cnt(u as int, m as int, n as int));
            assert(psum(u + 1, m as int, n as int) <= m);
        }
        let ghost adj0 = adj@;
        let ghost cur0 = cur@;
        let p = cur[u];
        adj[p as usize] = v;
        cur[u] = p + 1;
        proof {
            let (nI, mI, jI, uI, pI) = (n as int, m as int, j as int, u as int, p as int);
            // Les plages [psum(w), cur[w]) sont disjointes de la case écrite p (sauf pour w = u, où p = cur[u]).
            assert forall|w: int| 0 <= w < n && w != uI implies #[trigger] cur0[w] <= psum(w + 1, mI, nI)
                && (pI < psum(w, mI, nI) || psum(w + 1, mI, nI) <= pI) by {
                lemma_cnt_bound(w, jI, mI, nI);
                assert(psum(w + 1, mI, nI) == psum(w, mI, nI) + cnt(w, mI, nI));
                if w < uI {
                    lemma_psum_mono(uI, mI, nI);
                } else {
                    lemma_psum_mono(w, mI, nI);
                }
            }
            assert forall|jj: int| 0 <= jj < jI + 1 implies #[trigger] placed(adj@, jj, psum(src(jj, nI), mI, nI) as int,
                cur@[src(jj, nI)] as int, nI) by {
                if jj == jI {
                    assert(adj@[pI] == dst(jI, nI));
                } else {
                    let w = src(jj, nI);
                    lemma_endp_bound(jj, 0, nI);
                    assert(placed(adj0, jj, psum(w, mI, nI) as int, cur0[w] as int, nI));
                    let e = choose|e: int| psum(w, mI, nI) <= e < cur0[w] && #[trigger] adj0[e] == dst(jj, nI);
                    assert(e != pI);
                    assert(adj@[e] == dst(jj, nI));
                }
            }
            assert forall|uu: int, e: int| 0 <= uu < n && psum(uu, mI, nI) <= e < cur@[uu]
                implies #[trigger] slot_ok(adj@, uu, e, jI + 1, nI) by {
                if e == pI {
                    assert(uu == uI);
                    assert(src(jI, nI) == uu && dst(jI, nI) == adj@[e]);
                } else {
                    assert(slot_ok(adj0, uu, e, jI, nI));
                    let jj = choose|jj: int| 0 <= jj < jI && src(jj, nI) == uu && #[trigger] dst(jj, nI) == adj0[e];
                    assert(dst(jj, nI) == adj@[e]);
                }
            }
        }
        j += 1;
    }
    proof {
        let (nI, mI) = (n as int, m as int);
        assert forall|u: int| 0 <= u < n implies #[trigger] cur@[u] == psum(u + 1, mI, nI) by {
            assert(psum(u + 1, mI, nI) == psum(u, mI, nI) + cnt(u, mI, nI));
        }
        assert forall|j: int| 0 <= j < m implies #[trigger] placed(adj@, j, psum(src(j, nI), mI, nI) as int,
            psum(src(j, nI) + 1, mI, nI) as int, nI) by {
            lemma_endp_bound(j, 0, nI);
            assert(cur@[src(j, nI)] == psum(src(j, nI) + 1, mI, nI));
        }
    }
}

/// BFS depuis 0 sur le CSR (off, adj) ; rend le nombre de sommets atteints.
pub fn bfs(off: &Vec<u32>, adj: &Vec<u32>, dist: &mut Vec<u32>, q: &mut Vec<u32>, n: usize) -> (tail: usize)
    requires
        1 <= n <= 100000000,
        off@.len() == n + 1,
        old(dist)@.len() == n,
        old(q)@.len() == n,
        forall|i: int| 0 <= i <= n ==> #[trigger] off@[i] <= adj@.len(),
        forall|e: int| 0 <= e < adj@.len() ==> #[trigger] adj@[e] < n,
        forall|v: int| 0 <= v < n ==> #[trigger] old(dist)@[v] == INF,
    ensures
        final(dist)@.len() == n,
        final(q)@.len() == n,
        final(dist)@[0] == 0,
        forall|v: int| 0 <= v < n && v != 0 && #[trigger] final(dist)@[v] != INF ==>
            exists|x: int| 0 <= x < n && final(dist)@[x] != INF && #[trigger] arc(off@, adj@, x, v)
                && final(dist)@[x] + 1 == final(dist)@[v],
        forall|u: int, v: int| 0 <= u < n && final(dist)@[u] != INF && #[trigger] arc(off@, adj@, u, v) ==>
            final(dist)@[v] != INF && final(dist)@[v] <= final(dist)@[u] + 1,
        tail == reached(final(dist)@, n as int),
{
    let ghost d0 = dist@;
    proof {
        lemma_reached_zero(d0, n as int);
        lemma_reached_set(d0, 0, 0u32, n as int);
    }
    dist[0] = 0;
    q[0] = 0;
    proof {
        lemma_reached_same(dist@, d0.update(0, 0u32), n as int);
        reveal(queued);
        reveal(preds);
        reveal(processed);
        assert(q@[0] == 0);
    }
    let mut head: usize = 0;
    let mut tail: usize = 1;
    let ghost mut lvl: int = 0;  // niveau courant
    let ghost mut s: int = 1;    // début du niveau lvl + 1 dans la file
    while head < tail
        invariant
            1 <= n <= 100000000,
            off@.len() == n + 1,
            forall|i: int| 0 <= i <= n ==> #[trigger] off@[i] <= adj@.len(),
            forall|e: int| 0 <= e < adj@.len() ==> #[trigger] adj@[e] < n,
            head <= tail <= n,
            dist@.len() == n,
            q@.len() == n,
            dist@[0] == 0,
            tail == reached(dist@, n as int),
            forall|h: int| 0 <= h < tail ==> #[trigger] q@[h] < n && dist@[q@[h] as int] != INF && dist@[q@[h] as int] <= h,
            // File par niveaux : q[0..head) <= lvl, q[head..s) = lvl, q[s..tail) = lvl + 1.
            head <= s <= tail,
            lvl < n,
            forall|h: int| 0 <= h < head ==> #[trigger] dist@[q@[h] as int] <= lvl,
            forall|h: int| head <= h < s ==> #[trigger] dist@[q@[h] as int] == lvl,
            forall|h: int| s <= h < tail ==> #[trigger] dist@[q@[h] as int] == lvl + 1,
            queued(q@, dist@, tail as int, n as int),
            preds(off@, adj@, dist@, n as int),
            processed(off@, adj@, q@, dist@, head as int),
        decreases n - head,
    {
        proof {
            if head == s {
                lvl = lvl + 1;
                s = tail as int;
            }
        }
        let u = q[head] as usize;
        assert(dist@[u as int] <= head && dist@[u as int] == lvl);
        let du = dist[u] + 1;
        let mut e = off[u] as usize;
        let e1 = off[u + 1] as usize;
        let ghost q_h = q@;
        let ghost tail0 = tail;
        while e < e1
            invariant
                1 <= n <= 100000000,
                off@.len() == n + 1,
                forall|i: int| 0 <= i <= n ==> #[trigger] off@[i] <= adj@.len(),
                forall|e: int| 0 <= e < adj@.len() ==> #[trigger] adj@[e] < n,
                u < n,
                u == q@[head as int],
                e1 == off@[u + 1],
                du == dist@[u as int] + 1,
                off@[u as int] <= e,
                head < tail0 <= tail <= n,
                dist@.len() == n,
                q@.len() == n,
                forall|h: int| 0 <= h < tail0 ==> #[trigger] q@[h] == q_h[h],
                dist@[0] == 0,
                tail == reached(dist@, n as int),
                forall|h: int| 0 <= h < tail ==> #[trigger] q@[h] < n && dist@[q@[h] as int] != INF && dist@[q@[h] as int] <= h,
                head < s <= tail0,
                du == lvl + 1,
                lvl < n,
                forall|h: int| 0 <= h < head ==> #[trigger] dist@[q@[h] as int] <= lvl,
                forall|h: int| head <= h < s ==> #[trigger] dist@[q@[h] as int] == lvl,
                forall|h: int| s <= h < tail ==> #[trigger] dist@[q@[h] as int] == lvl + 1,
                queued(q@, dist@, tail as int, n as int),
                preds(off@, adj@, dist@, n as int),
                processed(off@, adj@, q@, dist@, head as int),
                forall|f: int| off@[u as int] <= f < e ==> dist@[#[trigger] adj@[f] as int] != INF && dist@[adj@[f] as int] <= du,
            decreases e1 - e,
        {
            let w = adj[e] as usize;
            if dist[w] == INF {
                let ghost (db, qb) = (dist@, q@);
                proof {
                    lemma_reached_bound(db, n as int);
                    lemma_reached_set(db, w as int, du, n as int);
                    assert(arc(off@, adj@, u as int, w as int));
                    lemma_discover(off@, adj@, qb, db, n as int, head as int, tail as int, u as int, w as int, du);
                }
                dist[w] = du;
                q[tail] = w as u32;
                tail += 1;
                proof {
                    lemma_reached_same(dist@, db.update(w as int, du), n as int);
                    assert(dist@ == db.update(w as int, du) && q@ == qb.update(tail - 1, w as u32));
                }
            } else {
                proof {
                    // w déjà atteint : il est dans la file, donc à distance <= lvl + 1 = du.
                    reveal(queued);
                    let h = choose|h: int| 0 <= h < tail && #[trigger] q@[h] == w;
                    assert(dist@[q@[h] as int] <= lvl + 1);
                }
            }
            e += 1;
        }
        proof {
            lemma_processed_next(off@, adj@, q@, dist@, head as int);
        }
        head += 1;
    }
    proof {
        lemma_bfs_done(off@, adj@, q@, dist@, n as int, tail as int);
    }
    tail
}

/// ck = fold(dist[i]) puis fold(atteints) puis fold(profondeur max).
pub fn checksum(dist: &Vec<u32>, n: usize, t: usize) -> u64
    requires
        dist@.len() == n,
{
    let mut i: usize = 0;
    let mut ck = CK_INIT;
    let mut maxd: u32 = 0;
    while i < n
        invariant
            i <= n,
            dist@.len() == n,
        decreases n - i,
    {
        let d = dist[i];
        ck = ckfold(ck, d as u64);
        if d != INF && d > maxd {
            maxd = d;
        }
        i += 1;
    }
    ckfold(ckfold(ck, t as u64), maxd as u64)
}

pub fn run(n: usize, _threads: usize) -> u64
    requires
        1 <= n <= 100000000,
{
    let m = 16 * n;
    let mut off = vec![0u32; n + 1];
    let mut adj = vec![0u32; m];
    count_deg(&mut off, n, m);
    prefix(&mut off, n, Ghost(m as int));
    {
        let mut cur = vec![0u32; n];
        copy_cur(&off, &mut cur, n);
        fill(&mut cur, &mut adj, n, m);
    }
    let mut dist = vec![INF; n];
    let mut q = vec![0u32; n];
    let t = bfs(&off, &adj, &mut dist, &mut q, n);
    proof {
        lemma_bfs_on_edges(off@, adj@, dist@, n as int, m as int);
    }
    checksum(&dist, n, t)
}

} // verus!
