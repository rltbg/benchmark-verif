//! Workload `parmatmul` (spec/workloads.d/parmatmul.md) : le produit de `matmul`, réparti en blocs de lignes sur
//! T threads (le thread k calcule les lignes [n·k/T, n·(k+1)/T)). 1 <= n <= 16384.
//!
//! Prouvé avec Creusot, threads compris :
//! * le noyau séquentiel `rows` calcule le bloc : `c[i·n+j] = dot(a, b, n, i, j, n) mod 2^32` (même `dot` que `matmul`) ;
//! * `blocks` répartit c avec `split_at_mut`, lance un thread par bloc avec `creusot_std::std::thread::Scope::spawn`
//!   et le rejoint avec `join_unwrap` : la postcondition de chaque thread (sur la valeur finale, prophétique, de son
//!   bloc) remonte jusqu'à `run`, qui conclut `is_product(A, B, C, n, n)` ;
//! * absence de panique et de débordement ; terminaison du noyau.
//!
//! Aucun `#[trusted]` dans ce module. Seules hypothèses : les contrats `#[trusted]` de `creusot_std::std::thread`
//! (`scope`, `Scope::spawn`, `join_unwrap`), c'est-à-dire la sémantique des threads de la bibliothèque standard.
//! Différence avec le code de référence : les threads lancés sont rejoints explicitement (`join_unwrap`, dans
//! l'ordre inverse de leur création) avant la fin du `scope`, au lieu de la jonction implicite ; c'est ce qui donne
//! accès à leur postcondition. `blocks` n'a pas de preuve de terminaison (`spawn` n'est pas `#[check(terminates)]`).
use crate::common::*;
#[cfg(creusot)]
use crate::matmul::{dot, dot_nonneg, is_product, mul_le, row_disjoint, wrap_step};
use creusot_std::{
    ghost::invariant::Tokens,
    prelude::*,
    std::thread::{self, JoinHandleExt, Scope},
};

const SEED: u64 = 0x0FEDCBA987654321;

/// Le bloc `c` (m×n) vaut le produit du bloc `a` (m×n) par `b` (n×n), modulo 2^32.
#[logic(open)]
pub fn block_product(a: Seq<u32>, b: Seq<u32>, c: Seq<u32>, n: Int, m: Int) -> bool {
    pearlite! {
        forall<i, j> 0 <= i && i < m && 0 <= j && j < n ==>
            c[i * n + j]@ == dot(a, b, n, i, j, n) % 4294967296
    }
}

/// `c` contient les lignes r0..n de C = A·B mod 2^32 (la ligne i à l'indice (i - r0)·n).
#[logic(open)]
pub fn rows_from(a: Seq<u32>, b: Seq<u32>, c: Seq<u32>, n: Int, r0: Int) -> bool {
    pearlite! {
        forall<i, j> r0 <= i && i < n && 0 <= j && j < n ==>
            c[(i - r0) * n + j]@ == dot(a, b, n, i, j, n) % 4294967296
    }
}

/// Ligne i d'un bloc m×n : i·n + n <= m·n.
#[logic]
#[requires(0 <= i && i < m && 0 <= n)]
#[ensures(i * n + n <= m * n)]
pub fn in_block(n: Int, m: Int, i: Int) {
    mul_le(i + 1, m, n)
}

/// Un bloc de lignes extrait de A : dot sur le bloc = dot sur A, la ligne i du bloc étant la ligne r0 + i de A.
#[logic]
#[requires(0 <= r0 && 0 <= i && 0 <= n && 0 <= k && k <= n)]
#[requires((r0 + i + 1) * n <= e && e <= a.len())]
#[ensures(dot(a.subsequence(r0 * n, e), b, n, i, j, k) == dot(a, b, n, r0 + i, j, k))]
#[variant(k)]
pub fn dot_shift(a: Seq<u32>, b: Seq<u32>, n: Int, r0: Int, e: Int, i: Int, j: Int, k: Int) {
    pearlite! {
        if k > 0 {
            dot_shift(a, b, n, r0, e, i, j, k - 1);
            proof_assert!((r0 + i + 1) * n == (r0 + i) * n + n);
            proof_assert!(r0 * n + (i * n + (k - 1)) == (r0 + i) * n + (k - 1))
        }
    }
}

/// Bornes des blocs : 0 <= n·k/t <= n·(k+1)/t <= n, avec égalité à n pour le dernier bloc.
#[logic]
#[requires(0 <= n && 0 <= k && k < t)]
#[ensures(0 <= n * k / t && n * k / t <= n * (k + 1) / t && n * (k + 1) / t <= n)]
#[ensures(k + 1 == t ==> n * (k + 1) / t == n)]
pub fn block_bounds(n: Int, t: Int, k: Int) {
    pearlite! {
        mul_le(k, k + 1, n);
        mul_le(k + 1, t, n);
        proof_assert!(n * t / t == n);
        div_le(n * k, n * (k + 1), t);
        div_le(n * (k + 1), n * t, t)
    }
}

/// x <= y donne x / t <= y / t (t > 0, x >= 0).
#[logic]
#[requires(0 <= x && x <= y && 0 < t)]
#[ensures(x / t <= y / t)]
pub fn div_le(x: Int, y: Int, t: Int) {}

/// Indices autour de la coupure entre un bloc de lignes [r0, r1) et la suite [r1, n).
#[logic]
#[requires(0 <= r0 && r0 <= r1 && r0 <= i && 0 <= j && j < n)]
#[ensures(i < r1 ==> (i - r0) * n + j < (r1 - r0) * n && (i - r0) * n + n <= (r1 - r0) * n)]
#[ensures(r1 <= i ==> (i - r0) * n + j - (r1 - r0) * n == (i - r1) * n + j)]
pub fn split_index(n: Int, r0: Int, r1: Int, i: Int, j: Int) {
    pearlite! { if i < r1 { mul_le(i - r0 + 1, r1 - r0, n) } }
}

/// Recollement de `split_at_mut` : bloc [r0, r1) (indices du bloc `ablk` de A) puis lignes [r1, n).
#[logic]
#[requires(0 <= r0 && r0 <= r1 && r1 <= n)]
#[requires(cf.len() == (n - r0) * n)]
#[requires(ablk == a.subsequence(r0 * n, r1 * n) && r1 * n <= a.len())]
#[requires(m == r1 - r0)]
#[requires(block_product(ablk, b, cf.subsequence(0, m * n), n, m))]
#[requires(rows_from(a, b, cf.subsequence(m * n, cf.len()), n, r1))]
#[ensures(rows_from(a, b, cf, n, r0))]
pub fn concat_blocks(a: Seq<u32>, b: Seq<u32>, ablk: Seq<u32>, cf: Seq<u32>, n: Int, r0: Int, r1: Int, m: Int) {
    pearlite! {
        proof_assert!(forall<i, j> r0 <= i && i < n && 0 <= j && j < n ==> {
            concat_point(a, b, ablk, cf, n, r0, r1, i, j);
            cf[(i - r0) * n + j]@ == dot(a, b, n, i, j, n) % 4294967296
        })
    }
}

/// Un point (i, j) du recollement, selon que la ligne i est dans le bloc ou après.
#[logic]
#[requires(0 <= r0 && r0 <= r1 && r1 <= n && r0 <= i && i < n && 0 <= j && j < n)]
#[requires(cf.len() == (n - r0) * n)]
#[requires(ablk == a.subsequence(r0 * n, r1 * n) && r1 * n <= a.len())]
#[requires(block_product(ablk, b, cf.subsequence(0, (r1 - r0) * n), n, r1 - r0))]
#[requires(rows_from(a, b, cf.subsequence((r1 - r0) * n, cf.len()), n, r1))]
#[ensures(cf[(i - r0) * n + j]@ == dot(a, b, n, i, j, n) % 4294967296)]
pub fn concat_point(a: Seq<u32>, b: Seq<u32>, ablk: Seq<u32>, cf: Seq<u32>, n: Int, r0: Int, r1: Int, i: Int, j: Int) {
    pearlite! {
        split_index(n, r0, r1, i, j);
        if i < r1 {
            let i2 = i - r0;
            proof_assert!(i2 * n + j < (r1 - r0) * n);
            proof_assert!(cf.subsequence(0, (r1 - r0) * n)[i2 * n + j] == cf[i2 * n + j]);
            proof_assert!(cf.subsequence(0, (r1 - r0) * n)[i2 * n + j]@ == dot(ablk, b, n, i2, j, n) % 4294967296);
            proof_assert! { mul_le(r0 + i2 + 1, r1, n); (r0 + i2 + 1) * n <= r1 * n };
            proof_assert! { dot_shift(a, b, n, r0, r1 * n, i2, j, n); dot(ablk, b, n, i2, j, n) == dot(a, b, n, r0 + i2, j, n) }
        } else {
            let x = (i - r0) * n + j;
            proof_assert!(x - (r1 - r0) * n == (i - r1) * n + j);
            proof_assert!(cf.subsequence((r1 - r0) * n, cf.len())[(i - r1) * n + j] == cf[x]);
            proof_assert!(cf.subsequence((r1 - r0) * n, cf.len())[(i - r1) * n + j]@ == dot(a, b, n, i, j, n) % 4294967296)
        }
    }
}

#[requires(m@.len() == len@)]
#[ensures((^m)@.len() == len@)]
#[check(terminates)]
pub fn init_mat(m: &mut [u32], len: usize, off: u64) {
    let mut i = 0usize;
    #[invariant(m@.len() == len@ && i@ <= len@)]
    #[variant(len@ - i@)]
    while i < len {
        m[i] = hi32(mix64(SEED.wrapping_add(off).wrapping_add(i as u64)));
        i += 1;
    }
}

/// Pas k de la ligne i du bloc : c[i,j] += a[i,k]·b[k,j] pour tout j ; le reste de c ne change pas.
#[requires(n@ <= 16384 && m@ <= n@ && i@ < m@ && k@ < n@ && aik == a@[i@ * n@ + k@])]
#[requires(a@.len() == m@ * n@ && b@.len() == n@ * n@ && c@.len() == m@ * n@)]
#[requires(forall<j> 0 <= j && j < n@ ==> c@[i@ * n@ + j]@ == dot(a@, b@, n@, i@, j, k@) % 4294967296)]
#[ensures((^c)@.len() == m@ * n@)]
#[ensures(forall<j> 0 <= j && j < n@ ==> (^c)@[i@ * n@ + j]@ == dot(a@, b@, n@, i@, j, k@ + 1) % 4294967296)]
#[ensures(forall<x> 0 <= x && x < m@ * n@ && (x < i@ * n@ || i@ * n@ + n@ <= x) ==> (^c)@[x] == c@[x])]
#[check(terminates)]
pub fn row_step(a: &[u32], b: &[u32], c: &mut [u32], n: usize, m: usize, i: usize, k: usize, aik: u32) {
    let old_c = snapshot!(c@);
    proof_assert! { in_block(n@, m@, i@); i@ * n@ + n@ <= m@ * n@ };
    proof_assert! { in_block(n@, n@, k@); k@ * n@ + n@ <= n@ * n@ };
    proof_assert! { mul_le(m@, 16384, n@); mul_le(n@, 16384, 16384); m@ * n@ <= 268435456 && n@ * n@ <= 268435456 };
    let ci = i * n;
    let bk = k * n;
    let mut j = 0usize;
    #[invariant(c@.len() == m@ * n@ && j@ <= n@)]
    #[invariant(forall<j2> 0 <= j2 && j2 < j@ ==> c@[ci@ + j2]@ == dot(a@, b@, n@, i@, j2, k@ + 1) % 4294967296)]
    #[invariant(forall<x> 0 <= x && x < m@ * n@ && !(ci@ <= x && x < ci@ + j@) ==> c@[x] == (*old_c)[x])]
    #[variant(n@ - j@)]
    while j < n {
        proof_assert! {
            dot_nonneg(a@, b@, n@, i@, j@, k@);
            wrap_step(c@[ci@ + j@], aik, b@[bk@ + j@], dot(a@, b@, n@, i@, j@, k@));
            (c@[ci@ + j@] + aik * b@[bk@ + j@])@ == dot(a@, b@, n@, i@, j@, k@ + 1) % 4294967296
        };
        c[ci + j] = c[ci + j].wrapping_add(aik.wrapping_mul(b[bk + j]));
        j += 1;
    }
}

/// Ligne i du bloc : c[i,j] = Σ_k a[i,k]·b[k,j] mod 2^32, à partir d'une ligne nulle ; le reste de c ne change pas.
#[requires(n@ <= 16384 && m@ <= n@ && i@ < m@)]
#[requires(a@.len() == m@ * n@ && b@.len() == n@ * n@ && c@.len() == m@ * n@)]
#[requires(forall<j> 0 <= j && j < n@ ==> c@[i@ * n@ + j]@ == 0)]
#[ensures((^c)@.len() == m@ * n@)]
#[ensures(forall<j> 0 <= j && j < n@ ==> (^c)@[i@ * n@ + j]@ == dot(a@, b@, n@, i@, j, n@) % 4294967296)]
#[ensures(forall<x> 0 <= x && x < m@ * n@ && (x < i@ * n@ || i@ * n@ + n@ <= x) ==> (^c)@[x] == c@[x])]
#[check(terminates)]
pub fn row(a: &[u32], b: &[u32], c: &mut [u32], n: usize, m: usize, i: usize) {
    let old_c = snapshot!(c@);
    proof_assert! { in_block(n@, m@, i@); i@ * n@ + n@ <= m@ * n@ };
    proof_assert! { mul_le(m@, 16384, n@); mul_le(n@, 16384, 16384); m@ * n@ <= 268435456 };
    let ci = i * n;
    let mut k = 0usize;
    #[invariant(c@.len() == m@ * n@ && k@ <= n@)]
    #[invariant(forall<j> 0 <= j && j < n@ ==> c@[ci@ + j]@ == dot(a@, b@, n@, i@, j, k@) % 4294967296)]
    #[invariant(forall<x> 0 <= x && x < m@ * n@ && (x < ci@ || ci@ + n@ <= x) ==> c@[x] == (*old_c)[x])]
    #[variant(n@ - k@)]
    while k < n {
        let aik = a[ci + k];
        row_step(a, b, c, n, m, i, k, aik);
        k += 1;
    }
}

/// Noyau séquentiel appelé par chaque thread : `a` et `c` sont les m lignes du bloc (m·n éléments, c nul au départ),
/// `b` est la matrice entière.
#[requires(n@ <= 16384 && m@ <= n@)]
#[requires(a@.len() == m@ * n@ && b@.len() == n@ * n@ && c@.len() == m@ * n@)]
#[requires(forall<x> 0 <= x && x < m@ * n@ ==> c@[x]@ == 0)]
#[ensures((^c)@.len() == m@ * n@)]
#[ensures(block_product(a@, b@, (^c)@, n@, m@))]
#[check(terminates)]
pub fn rows(a: &[u32], b: &[u32], c: &mut [u32], n: usize, m: usize) {
    let mut i = 0usize;
    #[invariant(c@.len() == m@ * n@ && i@ <= m@)]
    #[invariant(block_product(a@, b@, c@, n@, i@))]
    #[invariant(forall<i2, j> i@ <= i2 && i2 < m@ && 0 <= j && j < n@ ==> c@[i2 * n@ + j]@ == 0)]
    #[variant(m@ - i@)]
    while i < m {
        proof_assert!(forall<i2, j> 0 <= i2 && i2 < m@ && i2 != i@ && 0 <= j && j < n@ ==> {
            row_disjoint(n@, i@, i2, j);
            in_block(n@, m@, i2);
            (i2 * n@ + j < i@ * n@ || i@ * n@ + n@ <= i2 * n@ + j) && i2 * n@ + j < m@ * n@
        });
        proof_assert!(forall<j> 0 <= j && j < n@ ==> { in_block(n@, m@, i@); i@ * n@ + j < m@ * n@ });
        row(a, b, c, n, m, i);
        i += 1;
    }
}

/// Blocs k..t : `c` (lignes n·k/t..n de C, nulles au départ) est découpé avec `split_at_mut` ; le bloc k est calculé
/// par un nouveau thread (le dernier par le thread courant), les suivants récursivement, puis le thread est rejoint.
#[requires(1 <= n@ && n@ <= 16384 && k@ < t@ && t@ <= 4294967296)]
#[requires(a@.len() == n@ * n@ && b@.len() == n@ * n@)]
#[requires(c@.len() == (n@ - n@ * k@ / t@) * n@)]
#[requires(forall<x> 0 <= x && x < c@.len() ==> c@[x]@ == 0)]
#[ensures((^c)@.len() == c@.len())]
#[ensures(rows_from(a@, b@, (^c)@, n@, n@ * k@ / t@))]
pub fn blocks<'scope, 'env>(
    s: &mut Scope<'scope, 'env>,
    a: &'env [u32],
    b: &'env [u32],
    c: &'scope mut [u32],
    n: usize,
    t: usize,
    k: usize,
) {
    let (r0, r1) = (n * k / t, n * (k + 1) / t);
    proof_assert! { block_bounds(n@, t@, k@); 0 <= r0@ && r0@ <= r1@ && r1@ <= n@ && (k@ + 1 == t@ ==> r1@ == n@) };
    proof_assert! { in_block(n@, n@, r1@ - 1); mul_le(r1@ - r0@, n@, n@); mul_le(n@ - r1@, n@, n@);
        r1@ * n@ <= n@ * n@ && (r1@ - r0@) * n@ <= (n@ - r0@) * n@ };
    proof_assert!((n@ - r0@) * n@ - (r1@ - r0@) * n@ == (n@ - r1@) * n@);
    let m = r1 - r0;
    let (mine, tail) = c.split_at_mut(m * n);
    proof_assert!(mine@.len() == m@ * n@ && tail@.len() == (n@ - r1@) * n@);
    proof_assert!(forall<x> 0 <= x && x < m@ * n@ ==> mine@[x]@ == 0);
    proof_assert!(forall<x> 0 <= x && x < tail@.len() ==> tail@[x]@ == 0);
    proof_assert! { mul_le(r0@, r1@, n@); r0@ * n@ <= r1@ * n@ && (r1@ - r0@) * n@ == r1@ * n@ - r0@ * n@ };
    let ablk = &a[r0 * n..r1 * n];
    proof_assert!(ablk@ == a@.subsequence(r0@ * n@, r1@ * n@) && ablk@.len() == m@ * n@);
    // Recollement : un bloc correct suivi des lignes r1..n correctes donne les lignes r0..n correctes.
    proof_assert!(forall<cf: Seq<u32>> cf.len() == (n@ - r0@) * n@
        && block_product(ablk@, b@, cf.subsequence(0, m@ * n@), n@, m@)
        && rows_from(a@, b@, cf.subsequence(m@ * n@, cf.len()), n@, r1@) ==> {
            concat_blocks(a@, b@, ablk@, cf, n@, r0@, r1@, m@);
            rows_from(a@, b@, cf, n@, r0@)
        });
    if k + 1 < t {
        let h = s.spawn(move |_: Ghost<Tokens>| rows(ablk, b, mine, n, m));
        blocks(s, a, b, tail, n, t, k + 1);
        h.join_unwrap();
    } else {
        rows(ablk, b, mine, n, m);
    }
}

#[requires(c@.len() == nn@)]
#[check(terminates)]
pub fn checksum(c: &[u32], nn: usize) -> u64 {
    let mut ck = CK_INIT;
    let mut i = 0usize;
    #[invariant(c@.len() == nn@ && i@ <= nn@)]
    #[variant(nn@ - i@)]
    while i < nn {
        ck = ckfold(ck, c[i] as u64);
        i += 1;
    }
    ck
}

/// 1 <= n <= 16384 (spécification) ; threads <= 2^32 évite le débordement de n·threads.
#[requires(1 <= n@ && n@ <= 16384 && threads@ <= 4294967296)]
pub fn run(n: usize, threads: usize) -> u64 {
    let t = threads.max(1);
    proof_assert! { mul_le(n@, 16384, n@); mul_le(n@, 16384, 16384); n@ * n@ <= 268435456 };
    let nn = n * n;
    let mut a = std::vec![0u32; nn];
    let mut b = std::vec![0u32; nn];
    let mut c = std::vec![0u32; nn];
    init_mat(&mut a, nn, 0);
    init_mat(&mut b, nn, nn as u64);
    let (ra, rb, rc): (&[u32], &[u32], &mut [u32]) = (&a, &b, &mut c);
    thread::scope(move |s| blocks(s, ra, rb, rc, n, t, 0));
    proof_assert!(is_product(a@, b@, c@, n@, n@));
    checksum(&c, nn)
}
