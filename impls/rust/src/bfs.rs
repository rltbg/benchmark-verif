//! bfs : graphe creux aléatoire (m = 16 n arêtes), CSR par tri par comptage, puis BFS depuis 0.
//! Spécification : spec/workloads.d/bfs.md. 1 <= n <= 100_000_000.
//! Prouvé : absence de panique (indices, division par n > 0) et de débordement. Les invariants « offsets <= m »,
//! « sommet < n », « file <= n » ne sont pas prouvés mais gardés à l'exécution (gardes toujours vraies sur ces entrées).
use crate::common::*;
use creusot_std::prelude::*;

const SEED: u64 = 0xB5F0000000000001;
const INF: u32 = 0xFFFFFFFF;

/// Extrémité `k` (0 : source, 1 : cible) de l'arête `j`, ramenée dans [0, nn).
#[requires(0 < nn@)]
#[ensures(result@ < nn@)]
#[check(terminates)]
#[inline(always)]
pub fn endp(j: u64, k: u64, nn: u64) -> u64 {
    mix64(SEED.wrapping_add(2u64.wrapping_mul(j)).wrapping_add(k)) % nn
}

/// Passe 1 : off[u+1] += 1 pour chaque arête.
#[requires(1 <= n@ && n@ <= 100000000 && off@.len() == n@ + 1)]
#[ensures((^off)@.len() == off@.len())]
#[check(terminates)]
pub fn count_deg(off: &mut Vec<u32>, n: usize, m: usize) {
    let nn = n as u64;
    let mut j = 0usize;
    #[invariant(j@ <= m@ && off@.len() == n@ + 1)]
    #[variant(m@ - j@)]
    while j < m {
        let u = endp(j as u64, 0, nn) as usize;
        off[u + 1] = off[u + 1].wrapping_add(1);
        j += 1;
    }
}

/// Passe 2 : sommes préfixes (modulo 2^32).
#[requires(off@.len() == n@ + 1)]
#[ensures((^off)@.len() == off@.len())]
#[check(terminates)]
pub fn prefix(off: &mut Vec<u32>, n: usize) {
    let mut i = 1usize;
    #[invariant(1 <= i@ && i@ <= n@ + 1 && off@.len() == n@ + 1)]
    #[variant(n@ + 1 - i@)]
    while i <= n {
        off[i] = off[i].wrapping_add(off[i - 1]);
        i += 1;
    }
}

/// cur[i] <- off[i] pour i < n.
#[requires(off@.len() == n@ + 1 && cur@.len() == n@)]
#[ensures((^cur)@.len() == cur@.len())]
#[check(terminates)]
pub fn copy_cur(off: &Vec<u32>, cur: &mut Vec<u32>, n: usize) {
    let mut i = 0usize;
    #[invariant(i@ <= n@ && off@.len() == n@ + 1 && cur@.len() == n@)]
    #[variant(n@ - i@)]
    while i < n {
        cur[i] = off[i];
        i += 1;
    }
}

/// Passe 3 : adj[cur[u]++] = v ; garde `cur[u] < m` (toujours vraie).
#[requires(1 <= n@ && n@ <= 100000000 && cur@.len() == n@ && adj@.len() == m@)]
#[ensures((^cur)@.len() == cur@.len() && (^adj)@.len() == adj@.len())]
#[check(terminates)]
pub fn fill(cur: &mut Vec<u32>, adj: &mut Vec<u32>, n: usize, m: usize) {
    let nn = n as u64;
    let mut j = 0usize;
    #[invariant(j@ <= m@ && cur@.len() == n@ && adj@.len() == m@)]
    #[variant(m@ - j@)]
    while j < m {
        let u = endp(j as u64, 0, nn) as usize;
        let v = endp(j as u64, 1, nn) as u32;
        let p = cur[u];
        if (p as usize) < m {
            adj[p as usize] = v;
        }
        cur[u] = p.wrapping_add(1);
        j += 1;
    }
}

/// Relaxe les arêtes e0..e1 (bornées par m) du sommet courant ; renvoie le nouveau `tail` (<= n).
#[requires(adj@.len() == m@ && dist@.len() == n@ && q@.len() == n@ && t0@ <= n@)]
#[ensures((^dist)@.len() == dist@.len() && (^q)@.len() == q@.len() && result@ <= n@ && t0@ <= result@)]
#[check(terminates)]
pub fn relax(adj: &Vec<u32>, dist: &mut Vec<u32>, q: &mut Vec<u32>, n: usize, m: usize, e0: usize, e1: usize, t0: usize, du: u32) -> usize {
    let mut e = if e0 <= m { e0 } else { m };
    let mut tail = t0;
    #[invariant(e@ <= m@ && tail@ <= n@ && t0@ <= tail@ && adj@.len() == m@ && dist@.len() == n@ && q@.len() == n@)]
    #[variant(m@ - e@)]
    while e < e1 && e < m {
        let w32 = adj[e];
        let w = w32 as usize;
        if w < n && dist[w] == INF && tail < n {
            dist[w] = du;
            q[tail] = w32;
            tail += 1;
        }
        e += 1;
    }
    tail
}

/// BFS depuis 0 ; renvoie le nombre de sommets atteints (<= n).
#[requires(1 <= n@ && off@.len() == n@ + 1 && adj@.len() == m@ && dist@.len() == n@ && q@.len() == n@)]
#[ensures((^dist)@.len() == dist@.len() && (^q)@.len() == q@.len() && result@ <= n@)]
#[check(terminates)]
pub fn bfs(off: &Vec<u32>, adj: &Vec<u32>, dist: &mut Vec<u32>, q: &mut Vec<u32>, n: usize, m: usize) -> usize {
    dist[0] = 0;
    q[0] = 0;
    let mut head = 0usize;
    let mut tail = 1usize;
    #[invariant(head@ <= tail@ && tail@ <= n@ && off@.len() == n@ + 1 && adj@.len() == m@ && dist@.len() == n@ && q@.len() == n@)]
    #[variant(n@ - head@)]
    while head < tail {
        let us = q[head] as usize;
        let usafe = if us < n { us } else { 0 };
        let du = dist[usafe].wrapping_add(1);
        let e0 = off[usafe] as usize;
        let e1 = off[usafe + 1] as usize;
        tail = relax(adj, dist, q, n, m, e0, e1, tail, du);
        head += 1;
    }
    tail
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
    count_deg(&mut off, n, m);
    prefix(&mut off, n);
    {
        let mut cur = std::vec![0u32; n];
        copy_cur(&off, &mut cur, n);
        fill(&mut cur, &mut adj, n, m);
    }
    let mut dist = std::vec![INF; n];
    let mut q = std::vec![0u32; n];
    let t = bfs(&off, &adj, &mut dist, &mut q, n, m);
    checksum(&dist, n, t)
}
