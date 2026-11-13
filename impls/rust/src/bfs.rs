//! bfs : graphe creux aléatoire (m = 16 n arêtes), CSR par tri par comptage, puis BFS depuis 0.
//! Spécification : spec/workloads.d/bfs.md. 1 <= n <= 100_000_000.
//!
//! Prouvé avec Creusot :
//!   * CSR : `off[i] = psum(us, i)` (somme des degrés sortants des sommets < i ; `us`/`vs` = suites fantômes des
//!     sources/cibles, `us[j] = mix64(SEED + 2j) mod n`, `vs[j] = mix64(SEED + 2j + 1) mod n`), donc `off[i] <= m` ;
//!     `fill` range chaque arête j dans `[off[us[j]], off[us[j]+1])` et chaque case de cette plage contient la cible
//!     d'une arête de source u (témoins fantômes `pos`, `who`) : les arcs du CSR sont exactement les arêtes générées ;
//!   * BFS : `dist[0] = 0` ; tout sommet atteint v ≠ 0 a un prédécesseur atteint u (arc u→v) avec
//!     `dist[u] + 1 = dist[v]` ; pour tout arc u→v avec u atteint, v est atteint et `dist[v] <= dist[u] + 1` ;
//!     le résultat est le nombre de sommets atteints. Les fonctions fantômes `bfs_edges_forward/backward`
//!     transportent ces propriétés aux arêtes générées (`us[j] → vs[j]`). Elles caractérisent les distances BFS ;
//!   * absence de panique et de débordement, terminaison de toutes les fonctions.
//! Les gardes d'exécution de l'ancienne version (`e < m`, `w < n`, `tail < n`, `us < n`, garde de `fill`) sont
//! supprimées : ce sont des invariants prouvés. Aucun `trusted`/`assume`.
//! (Les variables fantômes `snapshot!` sont effacées par un `cargo build` normal, d'où le `allow` ci-dessous.)
#![allow(unused_variables, unused_assignments)]
use crate::common::{CK_INIT, ckfold};
use creusot_std::prelude::*;

pub const SEED: u64 = 0xB5F0000000000001;
pub const INF: u32 = 0xFFFFFFFF;

// ───────────────────────────── Modèle : génération et CSR ─────────────────────────────

/// splitmix64 dans la logique (vecteurs de bits modulo 2^64).
#[logic(open)]
pub fn mix64_l(x: u64) -> u64 {
    let z = x + 0x9E3779B97F4A7C15u64;
    let z = (z ^ (z >> 30u64)) * 0xBF58476D1CE4E5B9u64;
    let z = (z ^ (z >> 27u64)) * 0x94D049BB133111EBu64;
    z ^ (z >> 31u64)
}

/// Extrémité k (0 : source, 1 : cible) de l'arête j, dans [0, nn).
#[logic(open)]
pub fn endp_l(j: u64, k: u64, n: Int) -> Int {
    pearlite! { mix64_l(SEED + 2u64 * j + k)@ % n }
}

/// us est la suite des sources des m premières arêtes.
#[logic(open)]
pub fn sources(us: Seq<Int>, m: Int, n: Int) -> bool {
    pearlite! {
        us.len() == m
        && (forall<j: u64> j@ < m ==> us[j@] == endp_l(j, 0u64, n))
        && (forall<j: Int> 0 <= j && j < m ==> 0 <= us[j] && us[j] < n)
    }
}

/// Nombre d'arêtes de source u parmi les j premières.
#[logic(open)]
#[variant(j)]
pub fn cnt(us: Seq<Int>, u: Int, j: Int) -> Int {
    if j <= 0 { 0 } else { cnt(us, u, j - 1) + if us[j - 1] == u { 1 } else { 0 } }
}

/// Somme des degrés (parmi les j premières arêtes) des sommets < i : position de début de i dans le CSR.
#[logic(open)]
#[variant(i)]
pub fn psum(us: Seq<Int>, i: Int, j: Int) -> Int {
    if i <= 0 { 0 } else { psum(us, i - 1, j) + cnt(us, i - 1, j) }
}

/// vs est la suite des cibles des m premières arêtes.
#[logic(open)]
pub fn targets(vs: Seq<Int>, m: Int, n: Int) -> bool {
    pearlite! {
        vs.len() == m
        && (forall<j: u64> j@ < m ==> vs[j@] == endp_l(j, 1u64, n))
        && (forall<j: Int> 0 <= j && j < m ==> 0 <= vs[j] && vs[j] < n)
    }
}

// ───────────────────────────── Modèle : BFS sur le CSR ─────────────────────────────

/// Arc u→v du graphe CSR.
#[logic(open)]
pub fn arc(off: Seq<u32>, adj: Seq<u32>, u: Int, v: Int) -> bool {
    pearlite! { exists<e: Int> off[u]@ <= e && e < off[u + 1]@ && adj[e]@ == v }
}

/// Nombre de sommets atteints (dist != INF) parmi les k premiers.
#[logic(open)]
#[variant(k)]
pub fn reached(d: Seq<u32>, k: Int) -> Int {
    if k <= 0 { 0 } else { reached(d, k - 1) + if d[k - 1] != INF { 1 } else { 0 } }
}

// ───────────────────────────── Lemmes ─────────────────────────────

#[logic]
#[requires(0 <= j)]
#[variant(j)]
#[ensures(0 <= cnt(us, u, j) && cnt(us, u, j) <= j)]
pub fn cnt_bounds(us: Seq<Int>, u: Int, j: Int) {
    if j > 0 {
        cnt_bounds(us, u, j - 1)
    }
}

/// cnt est croissant en j.
#[logic]
#[requires(0 <= j && j <= k)]
#[variant(k)]
#[ensures(cnt(us, u, j) <= cnt(us, u, k))]
pub fn cnt_mono(us: Seq<Int>, u: Int, j: Int, k: Int) {
    if j < k {
        cnt_mono(us, u, j, k - 1)
    }
}

/// cnt et psum ne dépendent que des j premières sources.
#[logic]
#[requires(0 <= j && j <= us.len())]
#[variant(j)]
#[ensures(cnt(us.snoc(x), u, j) == cnt(us, u, j))]
pub fn cnt_snoc(us: Seq<Int>, x: Int, u: Int, j: Int) {
    if j > 0 {
        cnt_snoc(us, x, u, j - 1)
    }
}

/// Ajouter l'arête j (de source us[j]) augmente psum(i) de 1 si us[j] < i.
#[logic]
#[requires(0 <= i && 0 <= j)]
#[variant(i)]
#[ensures(psum(us, i, j + 1) == psum(us, i, j) + if 0 <= us[j] && us[j] < i { 1 } else { 0 })]
pub fn psum_step(us: Seq<Int>, i: Int, j: Int) {
    if i > 0 {
        psum_step(us, i - 1, j)
    }
}

/// Toutes les sources sont < n : psum(n, j) = j.
#[logic]
#[requires(0 <= j && j <= us.len() && 0 <= n)]
#[requires(forall<p: Int> 0 <= p && p < us.len() ==> 0 <= us[p] && us[p] < n)]
#[variant(j)]
#[ensures(psum(us, n, j) == j)]
pub fn psum_total(us: Seq<Int>, n: Int, j: Int) {
    if j > 0 {
        psum_total(us, n, j - 1);
        psum_step(us, n, j - 1)
    } else {
        psum_zero(us, n)
    }
}

/// Aucune arête : psum(i, 0) = 0.
#[logic]
#[requires(0 <= i)]
#[variant(i)]
#[ensures(psum(us, i, 0) == 0)]
pub fn psum_zero(us: Seq<Int>, i: Int) {
    if i > 0 {
        psum_zero(us, i - 1)
    }
}

/// psum(i) <= psum(k) pour tout i <= k.
#[logic]
#[requires(0 <= k && 0 <= j)]
#[variant(k)]
#[ensures(forall<i: Int> 0 <= i && i <= k ==> psum(us, i, j) <= psum(us, k, j))]
pub fn psum_mono_all(us: Seq<Int>, k: Int, j: Int) {
    if k > 0 {
        psum_mono_all(us, k - 1, j);
        cnt_bounds(us, k - 1, j)
    }
}

/// psum(i) <= psum(k) pour tous i <= k <= n.
#[logic]
#[requires(0 <= n && 0 <= j)]
#[variant(n)]
#[ensures(forall<i: Int, k: Int> 0 <= i && i <= k && k <= n ==> psum(us, i, j) <= psum(us, k, j))]
pub fn psum_mono_pairs(us: Seq<Int>, n: Int, j: Int) {
    psum_mono_all(us, n, j);
    if n > 0 {
        psum_mono_pairs(us, n - 1, j)
    }
}

/// cnt(u, j) <= cnt(u, k) pour tout u, si j <= k.
#[logic]
#[requires(0 <= j && j <= k)]
#[variant(k)]
#[ensures(forall<u: Int> cnt(us, u, j) <= cnt(us, u, k))]
pub fn cnt_mono_all(us: Seq<Int>, j: Int, k: Int) {
    if j < k {
        cnt_mono_all(us, j, k - 1)
    }
}

#[logic]
#[requires(0 <= k)]
#[variant(k)]
#[ensures(0 <= reached(d, k) && reached(d, k) <= k)]
#[ensures(forall<w: Int> 0 <= w && w < k && d[w] == INF ==> reached(d, k) < k)]
pub fn reached_bound(d: Seq<u32>, k: Int) {
    if k > 0 {
        reached_bound(d, k - 1)
    }
}

/// Marquer un sommet non atteint w augmente le compte de 1.
#[logic]
#[requires(0 <= w && w < k && k <= d.len() && d[w] == INF && x != INF)]
#[variant(k)]
#[ensures(reached(d.set(w, x), k) == reached(d, k) + 1)]
pub fn reached_set(d: Seq<u32>, w: Int, x: u32, k: Int) {
    if k - 1 > w {
        reached_set(d, w, x, k - 1)
    } else {
        reached_same(d, d.set(w, x), w)
    }
}

/// Aucun sommet atteint.
#[logic]
#[requires(0 <= k && forall<p: Int> 0 <= p && p < k ==> d[p] == INF)]
#[variant(k)]
#[ensures(reached(d, k) == 0)]
pub fn reached_zero(d: Seq<u32>, k: Int) {
    if k > 0 {
        reached_zero(d, k - 1)
    }
}

/// reached ne dépend que des k premières cases.
#[logic]
#[requires(0 <= k)]
#[requires(forall<p: Int> 0 <= p && p < k ==> d[p] == e[p])]
#[variant(k)]
#[ensures(reached(d, k) == reached(e, k))]
pub fn reached_same(d: Seq<u32>, e: Seq<u32>, k: Int) {
    if k > 0 {
        reached_same(d, e, k - 1)
    }
}

// ───────────────────────────── Code exécutable ─────────────────────────────

/// splitmix64 (copie spécifiée de `common::mix64`, même code machine).
#[ensures(result == mix64_l(x))]
#[check(terminates)]
#[inline(always)]
fn mix64(x: u64) -> u64 {
    let z = x.wrapping_add(0x9E3779B97F4A7C15);
    let z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    let z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

/// Extrémité `k` (0 : source, 1 : cible) de l'arête `j`, ramenée dans [0, nn).
#[requires(0 < nn@)]
#[ensures(result@ == endp_l(j, k, nn@) && result@ < nn@)]
#[check(terminates)]
#[inline(always)]
pub fn endp(j: u64, k: u64, nn: u64) -> u64 {
    mix64(SEED.wrapping_add(2u64.wrapping_mul(j)).wrapping_add(k)) % nn
}

/// Passe 1 : off[u+1] = nombre d'arêtes de source u ; rend (fantôme) la suite des sources.
#[requires(1 <= n@ && n@ <= 100000000 && m@ == 16 * n@ && off@.len() == n@ + 1)]
#[requires(forall<i: Int> 0 <= i && i <= n@ ==> off@[i]@ == 0)]
#[ensures(sources(*result, m@, n@))]
#[ensures((^off)@.len() == n@ + 1 && (^off)@[0]@ == 0)]
#[ensures(forall<u: Int> 0 <= u && u < n@ ==> (^off)@[u + 1]@ == cnt(*result, u, m@))]
#[check(terminates)]
pub fn count_deg(off: &mut Vec<u32>, n: usize, m: usize) -> Snapshot<Seq<Int>> {
    let nn = n as u64;
    let mut us: Snapshot<Seq<Int>> = snapshot!(Seq::empty());
    let mut j = 0usize;
    #[invariant(j@ <= m@ && off@.len() == n@ + 1 && off@[0]@ == 0)]
    #[invariant(sources(*us, j@, n@))]
    #[invariant(forall<u: Int> 0 <= u && u < n@ ==> off@[u + 1]@ == cnt(*us, u, j@))]
    #[variant(m@ - j@)]
    while j < m {
        let u = endp(j as u64, 0, nn) as usize;
        proof_assert! { cnt_bounds(*us, u@, j@); cnt(*us, u@, j@) <= j@ };
        off[u + 1] += 1;
        let us0 = us;
        us = snapshot!(us0.snoc(u@));
        proof_assert!((j as u64)@ == j@ && u@ == endp_l(j as u64, 0u64, n@) && us[j@] == u@);
        proof_assert!(forall<x: u64> x@ == j@ ==> x == j as u64);
        proof_assert!(forall<x: u64> x@ < j@ + 1 ==> us[x@] == endp_l(x, 0u64, n@));
        proof_assert!(forall<w: Int> 0 <= w && w < n@ ==> { cnt_snoc(*us0, u@, w, j@); cnt(*us, w, j@ + 1) == cnt(*us0, w, j@) + if w == u@ { 1 } else { 0 } });
        j += 1;
    }
    us
}

/// Passe 2 : sommes préfixes, off[i] = psum(us, i, m) <= m.
#[requires(1 <= n@ && n@ <= 100000000 && m@ == 16 * n@ && off@.len() == n@ + 1 && sources(*us, m@, n@))]
#[requires(off@[0]@ == 0 && forall<u: Int> 0 <= u && u < n@ ==> off@[u + 1]@ == cnt(*us, u, m@))]
#[ensures((^off)@.len() == n@ + 1)]
#[ensures(forall<i: Int> 0 <= i && i <= n@ ==> (^off)@[i]@ == psum(*us, i, m@) && (^off)@[i]@ <= m@)]
#[check(terminates)]
pub fn prefix(off: &mut Vec<u32>, n: usize, m: Snapshot<usize>, us: Snapshot<Seq<Int>>) {
    proof_assert! { psum_total(*us, n@, m@); psum(*us, n@, m@) == m@ };
    proof_assert! { psum_mono_all(*us, n@, m@); forall<k: Int> 0 <= k && k <= n@ ==> psum(*us, k, m@) <= m@ };
    let mut i = 1usize;
    #[invariant(1 <= i@ && i@ <= n@ + 1 && off@.len() == n@ + 1)]
    #[invariant(forall<k: Int> 0 <= k && k < i@ ==> off@[k]@ == psum(*us, k, m@))]
    #[invariant(forall<k: Int> i@ <= k && k <= n@ ==> off@[k]@ == cnt(*us, k - 1, m@))]
    #[variant(n@ + 1 - i@)]
    while i <= n {
        off[i] += off[i - 1];
        i += 1;
    }
}

/// cur[i] <- off[i] pour i < n.
#[requires(off@.len() == n@ + 1 && cur@.len() == n@)]
#[ensures((^cur)@.len() == n@)]
#[ensures(forall<i: Int> 0 <= i && i < n@ ==> (^cur)@[i] == off@[i])]
#[check(terminates)]
pub fn copy_cur(off: &Vec<u32>, cur: &mut Vec<u32>, n: usize) {
    let mut i = 0usize;
    #[invariant(i@ <= n@ && cur@.len() == n@)]
    #[invariant(forall<k: Int> 0 <= k && k < i@ ==> cur@[k] == off@[k])]
    #[variant(n@ - i@)]
    while i < n {
        cur[i] = off[i];
        i += 1;
    }
}

/// Passe 3 : adj[cur[u]++] = v. L'arête j de source u va dans [off[u], off[u+1]) ⊆ [0, m).
/// Rend (fantômes) la suite des cibles vs et deux témoins : l'arête j est rangée en pos[j], la case e contient
/// l'arête who[e] ; les arcs du CSR sont donc exactement les arêtes générées.
#[requires(1 <= n@ && n@ <= 100000000 && m@ == 16 * n@ && cur@.len() == n@ && adj@.len() == m@)]
#[requires(sources(*us, m@, n@))]
#[requires(forall<u: Int> 0 <= u && u < n@ ==> cur@[u]@ == psum(*us, u, m@))]
#[requires(forall<e: Int> 0 <= e && e < m@ ==> adj@[e]@ < n@)]
#[ensures(targets(*result.0, m@, n@) && result.1.len() == m@ && result.2.len() == m@)]
#[ensures((^adj)@.len() == m@)]
#[ensures(forall<e: Int> 0 <= e && e < m@ ==> (^adj)@[e]@ < n@)]
#[ensures(forall<j: Int> 0 <= j && j < m@ ==> psum(*us, us[j], m@) <= result.1[j]
    && result.1[j] < psum(*us, us[j] + 1, m@) && (^adj)@[result.1[j]]@ == result.0[j])]
#[ensures(forall<u: Int, e: Int> 0 <= u && u < n@ && psum(*us, u, m@) <= e && e < psum(*us, u + 1, m@) ==>
    0 <= result.2[e] && result.2[e] < m@ && us[result.2[e]] == u && result.0[result.2[e]] == (^adj)@[e]@)]
#[check(terminates)]
pub fn fill(cur: &mut Vec<u32>, adj: &mut Vec<u32>, n: usize, m: usize, us: Snapshot<Seq<Int>>)
    -> (Snapshot<Seq<Int>>, Snapshot<Seq<Int>>, Snapshot<Seq<Int>>) {
    let nn = n as u64;
    proof_assert! { psum_total(*us, n@, m@); psum(*us, n@, m@) == m@ };
    proof_assert! { psum_mono_pairs(*us, n@, m@); forall<i: Int, k: Int> 0 <= i && i <= k && k <= n@ ==> psum(*us, i, m@) <= psum(*us, k, m@) };
    proof_assert!(psum(*us, 0, m@) == 0);
    proof_assert! { cnt_mono_all(*us, 0, m@); forall<w: Int> 0 <= w && w < n@ ==> psum(*us, w + 1, m@) == psum(*us, w, m@) + cnt(*us, w, m@) };
    let mut vs: Snapshot<Seq<Int>> = snapshot!(Seq::empty());
    let mut pos: Snapshot<Seq<Int>> = snapshot!(Seq::empty());
    let mut who: Snapshot<Seq<Int>> = snapshot!(Seq::create(m@, |_| 0));
    let mut j = 0usize;
    #[invariant(j@ <= m@ && cur@.len() == n@ && adj@.len() == m@)]
    #[invariant(vs.len() == j@ && (forall<x: u64> x@ < j@ ==> vs[x@] == endp_l(x, 1u64, n@))
        && (forall<x: Int> 0 <= x && x < j@ ==> 0 <= vs[x] && vs[x] < n@))]
    #[invariant(forall<u: Int> 0 <= u && u < n@ ==> cur@[u]@ == psum(*us, u, m@) + cnt(*us, u, j@))]
    #[invariant(forall<e: Int> 0 <= e && e < m@ ==> adj@[e]@ < n@)]
    // Témoins fantômes : l'arête jj est rangée en pos[jj] ; la case e contient l'arête who[e].
    #[invariant(pos.len() == j@ && forall<jj: Int> 0 <= jj && jj < j@ ==>
        psum(*us, us[jj], m@) <= pos[jj] && pos[jj] < cur@[us[jj]]@ && adj@[pos[jj]]@ == vs[jj])]
    #[invariant(who.len() == m@ && forall<w: Int, e: Int> 0 <= w && w < n@ && psum(*us, w, m@) <= e && e < cur@[w]@ ==>
        0 <= who[e] && who[e] < j@ && us[who[e]] == w && vs[who[e]] == adj@[e]@)]
    #[variant(m@ - j@)]
    while j < m {
        let u = endp(j as u64, 0, nn) as usize;
        let v = endp(j as u64, 1, nn) as u32;
        proof_assert!(us[(j as u64)@] == endp_l(j as u64, 0u64, n@) && (j as u64)@ == j@ && u@ == us[j@]);
        // cur[u] = off[u] + cnt(u, j) < off[u] + cnt(u, m) = off[u+1] <= m
        proof_assert!(cnt(*us, u@, j@ + 1) == cnt(*us, u@, j@) + 1);
        proof_assert! { cnt_mono(*us, u@, j@ + 1, m@); cnt(*us, u@, j@ + 1) <= cnt(*us, u@, m@) };
        proof_assert!(cur@[u@]@ < psum(*us, u@ + 1, m@) && psum(*us, u@ + 1, m@) <= m@);
        // Les plages [psum(w), cur[w]) des autres sommets w sont disjointes de celle de u.
        proof_assert! { cnt_mono_all(*us, j@, m@);
            forall<w: Int> 0 <= w && w < n@ ==> 0 <= psum(*us, w, m@) && cur@[w]@ <= psum(*us, w + 1, m@) };
        proof_assert!(forall<w: Int> 0 <= w && w < n@ && w != u@ ==>
            cur@[u@]@ < psum(*us, w, m@) || psum(*us, w + 1, m@) <= psum(*us, u@, m@));
        let adj0 = snapshot!(adj@);
        let cur0 = snapshot!(cur@);
        let (vs0, pos0, who0) = (vs, pos, who);
        let p = cur[u];
        adj[p as usize] = v;
        cur[u] = p + 1;
        vs = snapshot!(vs0.snoc(v@));
        pos = snapshot!(pos0.snoc(p@));
        who = snapshot!(who0.set(p@, j@));
        proof_assert!(forall<x: u64> x@ == j@ ==> x == j as u64);
        proof_assert!(v@ == endp_l(j as u64, 1u64, n@) && vs[j@] == v@ && adj@[p@] == v);
        proof_assert!(forall<e: Int> 0 <= e && e < m@ && e != p@ ==> adj@[e] == adj0[e]);
        proof_assert!(forall<w: Int> 0 <= w && w < n@ && w != u@ ==> cur@[w] == cur0[w]);
        proof_assert!(forall<jj: Int> 0 <= jj && jj < j@ ==> pos0[jj] != p@);
        proof_assert!(forall<jj: Int> 0 <= jj && jj < j@ ==> pos[jj] == pos0[jj] && vs[jj] == vs0[jj]
            && adj@[pos[jj]] == adj0[pos0[jj]] && cur0[us[jj]]@ <= cur@[us[jj]]@);
        proof_assert!(pos[j@] == p@ && adj@[pos[j@]]@ == vs[j@]);
        proof_assert!(forall<w: Int> 0 <= w && w < n@ && w != u@ ==> !(psum(*us, w, m@) <= p@ && p@ < cur@[w]@));
        proof_assert!(who[p@] == j@ && us[j@] == u@ && vs[who[p@]] == adj@[p@]@);
        proof_assert!(forall<w: Int, e: Int> 0 <= w && w < n@ && psum(*us, w, m@) <= e && e < cur@[w]@ && e != p@ ==>
            e < cur0[w]@ && who[e] == who0[e] && adj@[e] == adj0[e] && 0 <= who0[e] && who0[e] < j@
            && vs[who0[e]] == vs0[who0[e]]);
        j += 1;
    }
    proof_assert!(forall<u: Int> 0 <= u && u < n@ ==> cur@[u]@ == psum(*us, u + 1, m@));
    proof_assert!(forall<jj: Int> 0 <= jj && jj < m@ ==> cur@[us[jj]]@ == psum(*us, us[jj] + 1, m@));
    proof_assert!(forall<jj: Int> 0 <= jj && jj < m@ ==>
        psum(*us, us[jj], m@) <= pos[jj] && pos[jj] < psum(*us, us[jj] + 1, m@) && adj@[pos[jj]]@ == vs[jj]);
    proof_assert!(forall<w: Int, e: Int> 0 <= w && w < n@ && psum(*us, w, m@) <= e && e < psum(*us, w + 1, m@) ==>
        0 <= who[e] && who[e] < m@ && us[who[e]] == w && vs[who[e]] == adj@[e]@);
    (vs, pos, who)
}

/// BFS depuis 0 sur le CSR (off, adj) ; rend le nombre de sommets atteints.
#[requires(1 <= n@ && n@ <= 100000000 && off@.len() == n@ + 1 && dist@.len() == n@ && q@.len() == n@)]
#[requires(forall<i: Int> 0 <= i && i <= n@ ==> off@[i]@ <= adj@.len())]
#[requires(forall<e: Int> 0 <= e && e < adj@.len() ==> adj@[e]@ < n@)]
#[requires(forall<v: Int> 0 <= v && v < n@ ==> dist@[v] == INF)]
#[ensures((^dist)@.len() == n@ && (^q)@.len() == n@)]
#[ensures((^dist)@[0]@ == 0)]
#[ensures(forall<v: Int> 0 <= v && v < n@ && v != 0 && (^dist)@[v] != INF ==>
    exists<u: Int> 0 <= u && u < n@ && (^dist)@[u] != INF && arc(off@, adj@, u, v) && (^dist)@[u]@ + 1 == (^dist)@[v]@)]
#[ensures(forall<u: Int, v: Int> 0 <= u && u < n@ && (^dist)@[u] != INF && arc(off@, adj@, u, v) ==>
    (^dist)@[v] != INF && (^dist)@[v]@ <= (^dist)@[u]@ + 1)]
#[ensures(result@ == reached((^dist)@, n@))]
#[check(terminates)]
pub fn bfs(off: &Vec<u32>, adj: &Vec<u32>, dist: &mut Vec<u32>, q: &mut Vec<u32>, n: usize) -> usize {
    let d0 = snapshot!(dist@);
    proof_assert! { reached_zero(*d0, n@); reached(*d0, n@) == 0 };
    proof_assert! { reached_set(*d0, 0, 0u32, n@); reached(d0.set(0, 0u32), n@) == 1 };
    dist[0] = 0;
    q[0] = 0;
    proof_assert! { reached_same(dist@, d0.set(0, 0u32), n@); reached(dist@, n@) == 1 };
    let mut head = 0usize;
    let mut tail = 1usize;
    #[invariant(head@ <= tail@ && tail@ <= n@ && dist@.len() == n@ && q@.len() == n@)]
    #[invariant(dist@[0]@ == 0)]
    #[invariant(tail@ == reached(dist@, n@))]
    #[invariant(forall<h: Int> 0 <= h && h < tail@ ==> q@[h]@ < n@ && dist@[q@[h]@] != INF && dist@[q@[h]@]@ <= h)]
    #[invariant(forall<v: Int> 0 <= v && v < n@ && dist@[v] != INF ==> exists<h: Int> 0 <= h && h < tail@ && q@[h]@ == v)]
    #[invariant(forall<h1: Int, h2: Int> 0 <= h1 && h1 <= h2 && h2 < tail@ ==> dist@[q@[h1]@]@ <= dist@[q@[h2]@]@)]
    #[invariant(head@ < tail@ ==> dist@[q@[tail@ - 1]@]@ <= dist@[q@[head@]@]@ + 1)]
    #[invariant(forall<v: Int> 0 <= v && v < n@ && v != 0 && dist@[v] != INF ==>
        exists<u: Int> 0 <= u && u < n@ && dist@[u] != INF && arc(off@, adj@, u, v) && dist@[u]@ + 1 == dist@[v]@)]
    #[invariant(forall<h: Int, w: Int> 0 <= h && h < head@ && arc(off@, adj@, q@[h]@, w) ==>
        dist@[w] != INF && dist@[w]@ <= dist@[q@[h]@]@ + 1)]
    #[variant(n@ - head@)]
    while head < tail {
        let u = q[head] as usize;
        proof_assert!(dist@[u@]@ <= head@);
        let du = dist[u] + 1;
        let mut e = off[u] as usize;
        let e1 = off[u + 1] as usize;
        proof_assert!(u@ == q@[head@]@ && du@ == dist@[q@[head@]@]@ + 1);
        proof_assert!(dist@[q@[tail@ - 1]@]@ <= du@);
        let dist_h = snapshot!(dist@);
        let q_h = snapshot!(q@);
        let tail0 = snapshot!(tail);
        #[invariant(off@[u@]@ <= e@)]
        #[invariant(head@ < tail0@ && tail0@ <= tail@ && tail@ <= n@ && dist@.len() == n@ && q@.len() == n@)]
        #[invariant(forall<h: Int> 0 <= h && h < tail0@ ==> q@[h] == q_h[h])]
        #[invariant(dist@[0]@ == 0)]
        #[invariant(tail@ == reached(dist@, n@))]
        #[invariant(forall<h: Int> 0 <= h && h < tail@ ==> q@[h]@ < n@ && dist@[q@[h]@] != INF && dist@[q@[h]@]@ <= h)]
        #[invariant(forall<v: Int> 0 <= v && v < n@ && dist@[v] != INF ==> exists<h: Int> 0 <= h && h < tail@ && q@[h]@ == v)]
        #[invariant(forall<h1: Int, h2: Int> 0 <= h1 && h1 <= h2 && h2 < tail@ ==> dist@[q@[h1]@]@ <= dist@[q@[h2]@]@)]
        #[invariant(dist@[q@[tail@ - 1]@]@ <= du@)]
        #[invariant(forall<v: Int> 0 <= v && v < n@ && dist_h[v] != INF ==> dist@[v] == dist_h[v])]
        #[invariant(forall<v: Int> 0 <= v && v < n@ && v != 0 && dist@[v] != INF ==>
            exists<x: Int> 0 <= x && x < n@ && dist@[x] != INF && arc(off@, adj@, x, v) && dist@[x]@ + 1 == dist@[v]@)]
        #[invariant(forall<f: Int> off@[u@]@ <= f && f < e@ ==> dist@[adj@[f]@] != INF && dist@[adj@[f]@]@ <= du@)]
        #[variant(e1@ - e@)]
        while e < e1 {
            let w = adj[e] as usize;
            if dist[w] == INF {
                let db = snapshot!(dist@);
                proof_assert! { reached_bound(*db, n@); tail@ < n@ };
                proof_assert! { reached_set(*db, w@, du, n@); reached(db.set(w@, du), n@) == tail@ + 1 };
                proof_assert!(off@[u@]@ <= e@ && e@ < off@[u@ + 1]@ && adj@[e@]@ == w@);
                dist[w] = du;
                proof_assert! { reached_same(dist@, db.set(w@, du), n@); reached(dist@, n@) == tail@ + 1 };
                proof_assert!(arc(off@, adj@, u@, w@) && dist@[u@]@ + 1 == dist@[w@]@ && dist@[u@] != INF);
                q[tail] = w as u32;
                tail += 1;
                proof_assert!(q@[tail@ - 1]@ == w@ && dist@[w@] == du);
            }
            e += 1;
        }
        proof_assert!(q@[head@]@ == u@);
        proof_assert!(forall<w: Int> arc(off@, adj@, u@, w) ==> dist@[w] != INF && dist@[w]@ <= du@);
        head += 1;
    }
    tail
}

/// Théorème final, sens « arêtes → BFS » (fonction fantôme) : pour toute arête *générée* j (us[j] → vs[j], soit
/// mix64(SEED+2j) mod n → mix64(SEED+2j+1) mod n) de source atteinte, la cible est atteinte et dist[v] <= dist[u] + 1.
#[check(ghost)]
#[requires(1 <= *n && off.len() == *n + 1 && sources(*us, *m, *n) && pos.len() == *m)]
#[requires(forall<i: Int> 0 <= i && i <= *n ==> off[i]@ == psum(*us, i, *m))]
#[requires(forall<j: Int> 0 <= j && j < *m ==> psum(*us, us[j], *m) <= pos[j]
    && pos[j] < psum(*us, us[j] + 1, *m) && adj[pos[j]]@ == vs[j])]
#[requires(forall<x: Int, v: Int> 0 <= x && x < *n && d[x] != INF && arc(*off, *adj, x, v) ==>
    d[v] != INF && d[v]@ <= d[x]@ + 1)]
#[ensures(forall<j: Int> 0 <= j && j < *m && d[us[j]] != INF ==> d[vs[j]] != INF && d[vs[j]]@ <= d[us[j]]@ + 1)]
pub fn bfs_edges_forward(off: Snapshot<Seq<u32>>, adj: Snapshot<Seq<u32>>, d: Snapshot<Seq<u32>>,
    us: Snapshot<Seq<Int>>, vs: Snapshot<Seq<Int>>, pos: Snapshot<Seq<Int>>, n: Snapshot<Int>, m: Snapshot<Int>) {
    proof_assert!(forall<j: Int> 0 <= j && j < *m ==> 0 <= us[j] && us[j] < *n
        && off[us[j]]@ <= pos[j] && pos[j] < off[us[j] + 1]@ && adj[pos[j]]@ == vs[j]);
    proof_assert!(forall<j: Int> 0 <= j && j < *m ==> arc(*off, *adj, us[j], vs[j]));
}

/// Théorème final, sens « BFS → arêtes » : tout sommet atteint v ≠ 0 est la cible d'une arête *générée* j
/// dont la source est atteinte avec dist[us[j]] + 1 = dist[v].
#[check(ghost)]
#[requires(1 <= *n && off.len() == *n + 1)]
#[requires(forall<i: Int> 0 <= i && i <= *n ==> off[i]@ == psum(*us, i, *m))]
#[requires(forall<u: Int, e: Int> 0 <= u && u < *n && psum(*us, u, *m) <= e && e < psum(*us, u + 1, *m) ==>
    0 <= who[e] && who[e] < *m && us[who[e]] == u && vs[who[e]] == adj[e]@)]
#[requires(forall<v: Int> 0 <= v && v < *n && v != 0 && d[v] != INF ==>
    exists<x: Int> 0 <= x && x < *n && d[x] != INF && arc(*off, *adj, x, v) && d[x]@ + 1 == d[v]@)]
#[ensures(forall<v: Int> 0 <= v && v < *n && v != 0 && d[v] != INF ==>
    exists<j: Int> 0 <= j && j < *m && vs[j] == v && d[us[j]] != INF && d[us[j]]@ + 1 == d[v]@)]
pub fn bfs_edges_backward(off: Snapshot<Seq<u32>>, adj: Snapshot<Seq<u32>>, d: Snapshot<Seq<u32>>,
    us: Snapshot<Seq<Int>>, vs: Snapshot<Seq<Int>>, who: Snapshot<Seq<Int>>, n: Snapshot<Int>, m: Snapshot<Int>) {
    proof_assert!(forall<x: Int, e: Int> 0 <= x && x < *n && off[x]@ <= e && e < off[x + 1]@ ==>
        0 <= who[e] && who[e] < *m && us[who[e]] == x && vs[who[e]] == adj[e]@);
    proof_assert!(forall<v: Int> 0 <= v && v < *n && v != 0 && d[v] != INF ==>
        exists<x: Int, e: Int> 0 <= x && x < *n && d[x] != INF && off[x]@ <= e && e < off[x + 1]@ && adj[e]@ == v
            && d[x]@ + 1 == d[v]@);
    proof_assert!(forall<v: Int> 0 <= v && v < *n && v != 0 && d[v] != INF ==>
        exists<x: Int, e: Int> 0 <= x && x < *n && d[x] != INF && d[x]@ + 1 == d[v]@
            && 0 <= who[e] && who[e] < *m && us[who[e]] == x && vs[who[e]] == v);
}

/// ck = fold(dist[i]) puis fold(atteints) puis fold(profondeur max).
#[requires(dist@.len() == n@)]
#[check(terminates)]
pub fn checksum(dist: &Vec<u32>, n: usize, t: usize) -> u64 {
    let mut i = 0usize;
    let mut ck = CK_INIT;
    let mut maxd = 0u32;
    #[invariant(i@ <= n@ && dist@.len() == n@)]
    #[variant(n@ - i@)]
    while i < n {
        let d = dist[i];
        ck = ckfold(ck, d as u64);
        if d != INF && d > maxd {
            maxd = d;
        }
        i += 1;
    }
    ckfold(ckfold(ck, t as u64), maxd as u64)
}

#[requires(1 <= n@ && n@ <= 100000000)]
pub fn run(n: usize, _threads: usize) -> u64 {
    let m = 16 * n;
    let mut off = std::vec![0u32; n + 1];
    let mut adj = std::vec![0u32; m];
    let us = count_deg(&mut off, n, m);
    prefix(&mut off, n, snapshot!(m), us);
    let (vs, pos, who);
    {
        let mut cur = std::vec![0u32; n];
        copy_cur(&off, &mut cur, n);
        (vs, pos, who) = fill(&mut cur, &mut adj, n, m, us);
    }
    let mut dist = std::vec![INF; n];
    let mut q = std::vec![0u32; n];
    let t = bfs(&off, &adj, &mut dist, &mut q, n);
    let (g_off, g_adj, g_dist, g_n, g_m) = (snapshot!(off@), snapshot!(adj@), snapshot!(dist@), snapshot!(n@), snapshot!(m@));
    ghost! { bfs_edges_forward(g_off, g_adj, g_dist, us, vs, pos, g_n, g_m) };
    ghost! { bfs_edges_backward(g_off, g_adj, g_dist, us, vs, who, g_n, g_m) };
    checksum(&dist, n, t)
}
