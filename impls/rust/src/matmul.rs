//! Workload `matmul` (spec/WORKLOADS.md §3) : C = A·B, matrices n×n de u32, arithmétique modulo 2^32,
//! boucles i, k, j. 1 <= n <= 4096.
//!
//! Prouvé avec Creusot :
//! * correction fonctionnelle : `matmul` garantit `C[i·n+j] = dot(A, B, n, i, j, n) mod 2^32` pour tous i, j < n,
//!   où `dot` est la somme Σ_l A[i·n+l]·B[l·n+j] dans les entiers mathématiques ;
//! * absence de panique (indices) et de débordement, terminaison de toutes les fonctions sauf `run` (allocation).
//!
//! Aucun `trusted`. Les deux faits arithmétiques délicats sont isolés dans des lemmes :
//! `wrap_step` (congruence : le pas `c + a·b` en u32 suit le pas entier modulo 2^32) et `row_disjoint`
//! (deux lignes différentes n'ont pas d'indice commun : inégalités non linéaires i·n).
use crate::common::*;
use creusot_std::prelude::*;

const SEED: u64 = 0x0FEDCBA987654321;

/// Somme Σ_{l<k} a[i·n+l]·b[l·n+j] dans les entiers (sans réduction modulo 2^32).
#[logic(open)]
#[variant(k)]
pub fn dot(a: Seq<u32>, b: Seq<u32>, n: Int, i: Int, j: Int, k: Int) -> Int {
    pearlite! { if k <= 0 { 0 } else { dot(a, b, n, i, j, k - 1) + a[i * n + (k - 1)]@ * b[(k - 1) * n + j]@ } }
}

/// La matrice `c` (n×n, ligne par ligne) vaut le produit `a·b` modulo 2^32 sur les lignes `0..rows`.
#[logic(open)]
pub fn is_product(a: Seq<u32>, b: Seq<u32>, c: Seq<u32>, n: Int, rows: Int) -> bool {
    pearlite! {
        forall<i, j> 0 <= i && i < rows && 0 <= j && j < n ==>
            c[i * n + j]@ == dot(a, b, n, i, j, n) % 4294967296
    }
}

/// dot est positif (somme de produits de naturels).
#[logic]
#[requires(0 <= k)]
#[ensures(dot(a, b, n, i, j, k) >= 0)]
#[variant(k)]
pub fn dot_nonneg(a: Seq<u32>, b: Seq<u32>, n: Int, i: Int, j: Int, k: Int) {
    pearlite! { if k > 0 { dot_nonneg(a, b, n, i, j, k - 1) } }
}

/// (d mod M + e mod M) mod M = (d + e) mod M pour d, e >= 0, M = 2^32.
#[logic]
#[requires(0 <= d && 0 <= e)]
#[ensures((d % 4294967296 + e % 4294967296) % 4294967296 == (d + e) % 4294967296)]
pub fn mod_add(d: Int, e: Int) {
    pearlite! {
        // d = qd·M + rd, e = qe·M + re, donc d + e = (qd + qe)·M + (rd + re) : même reste que rd + re.
        proof_assert!(d == (d / 4294967296) * 4294967296 + d % 4294967296);
        proof_assert!(e == (e / 4294967296) * 4294967296 + e % 4294967296);
        proof_assert!(d + e == (d / 4294967296 + e / 4294967296) * 4294967296 + (d % 4294967296 + e % 4294967296))
    }
}

/// Congruence modulo 2^32 du pas de la boucle interne : si `c ≡ d`, alors `c ⊕ x ⊗ y ≡ d + x·y`
/// (⊕, ⊗ : opérations u32 de la logique, celles que garantissent `wrapping_add` / `wrapping_mul`).
#[logic]
#[requires(0 <= d && c@ == d % 4294967296)]
#[ensures((c + x * y)@ == (d + x@ * y@) % 4294967296)]
pub fn wrap_step(c: u32, x: u32, y: u32, d: Int) {
    pearlite! {
        // Les axiomes de u32 donnent p@ = x@·y@ + q·2^32 et (c+p)@ = c@ + p@ + q'·2^32 pour des q, q'
        // inconnus ; les bornes 0 <= _ < 2^32 suffisent ensuite au solveur pour réduire modulo 2^32.
        proof_assert!((x * y)@ == (x@ * y@) % 4294967296);
        proof_assert!((c + x * y)@ == (c@ + (x * y)@) % 4294967296);
        mod_add(d, x@ * y@)
    }
}

/// a <= b et 0 <= n donnent a·n <= b·n.
#[logic]
#[requires(a <= b && 0 <= n)]
#[ensures(a * n <= b * n)]
pub fn mul_le(a: Int, b: Int, n: Int) {}

/// Les lignes i et i2 ≠ i sont disjointes : l'indice i2·n+j (j < n) n'est pas dans [i·n, i·n+n).
#[logic]
#[requires(0 <= i && 0 <= i2 && i != i2 && 0 <= j && j < n)]
#[ensures(i2 * n + j < i * n || i * n + n <= i2 * n + j)]
pub fn row_disjoint(n: Int, i: Int, i2: Int, j: Int) {
    pearlite! { if i2 < i { mul_le(i2 + 1, i, n) } else { mul_le(i + 1, i2, n) } }
}

/// L'indice (i, j) d'une matrice n×n : i·n+j < n·n.
#[logic]
#[requires(0 <= i && i < n && 0 <= j && j < n)]
#[ensures(i * n + j < n * n)]
pub fn in_matrix(n: Int, i: Int, j: Int) {
    mul_le(i + 1, n, n)
}

#[requires(m@.len() == len@)]
#[ensures((^m)@.len() == len@)]
#[check(terminates)]
pub fn init_mat(m: &mut Vec<u32>, len: usize, off: u64) {
    let mut i = 0usize;
    #[invariant(m@.len() == len@ && i@ <= len@)]
    #[variant(len@ - i@)]
    while i < len {
        m[i] = hi32(mix64(SEED.wrapping_add(off).wrapping_add(i as u64)));
        i += 1;
    }
}

/// Pas k de la ligne i : c[i,j] += a[i,k]·b[k,j] pour tout j ; le reste de c ne change pas.
#[requires(n@ <= 4096 && i@ < n@ && k@ < n@ && aik == a@[i@ * n@ + k@])]
#[requires(a@.len() == n@ * n@ && b@.len() == n@ * n@ && c@.len() == n@ * n@)]
#[requires(forall<j> 0 <= j && j < n@ ==> c@[i@ * n@ + j]@ == dot(a@, b@, n@, i@, j, k@) % 4294967296)]
#[ensures((^c)@.len() == n@ * n@)]
#[ensures(forall<j> 0 <= j && j < n@ ==> (^c)@[i@ * n@ + j]@ == dot(a@, b@, n@, i@, j, k@ + 1) % 4294967296)]
#[ensures(forall<x> 0 <= x && x < n@ * n@ && (x < i@ * n@ || i@ * n@ + n@ <= x) ==> (^c)@[x] == c@[x])]
#[check(terminates)]
pub fn row_step(a: &Vec<u32>, b: &Vec<u32>, c: &mut Vec<u32>, n: usize, i: usize, k: usize, aik: u32) {
    let old_c = snapshot!(*c);
    proof_assert! { in_matrix(n@, i@, n@ - 1); i@ * n@ + n@ <= n@ * n@ };
    proof_assert! { in_matrix(n@, k@, n@ - 1); k@ * n@ + n@ <= n@ * n@ };
    proof_assert! { mul_le(n@, 4096, n@); mul_le(n@, 4096, 4096); n@ * n@ <= 16777216 };
    let ci = i * n;
    let bk = k * n;
    let mut j = 0usize;
    #[invariant(c@.len() == n@ * n@ && j@ <= n@)]
    #[invariant(forall<j2> 0 <= j2 && j2 < j@ ==> c@[ci@ + j2]@ == dot(a@, b@, n@, i@, j2, k@ + 1) % 4294967296)]
    #[invariant(forall<x> 0 <= x && x < n@ * n@ && !(ci@ <= x && x < ci@ + j@) ==> c@[x] == old_c@[x])]
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

/// Ligne i de C : c[i,j] = Σ_k a[i,k]·b[k,j] mod 2^32, à partir d'une ligne nulle ; le reste de c ne change pas.
#[requires(n@ <= 4096 && i@ < n@)]
#[requires(a@.len() == n@ * n@ && b@.len() == n@ * n@ && c@.len() == n@ * n@)]
#[requires(forall<j> 0 <= j && j < n@ ==> c@[i@ * n@ + j]@ == 0)]
#[ensures((^c)@.len() == n@ * n@)]
#[ensures(forall<j> 0 <= j && j < n@ ==> (^c)@[i@ * n@ + j]@ == dot(a@, b@, n@, i@, j, n@) % 4294967296)]
#[ensures(forall<x> 0 <= x && x < n@ * n@ && (x < i@ * n@ || i@ * n@ + n@ <= x) ==> (^c)@[x] == c@[x])]
#[check(terminates)]
pub fn row(a: &Vec<u32>, b: &Vec<u32>, c: &mut Vec<u32>, n: usize, i: usize) {
    let old_c = snapshot!(*c);
    proof_assert! { in_matrix(n@, i@, n@ - 1); i@ * n@ + n@ <= n@ * n@ };
    proof_assert! { mul_le(n@, 4096, n@); mul_le(n@, 4096, 4096); n@ * n@ <= 16777216 };
    let ci = i * n;
    let mut k = 0usize;
    #[invariant(c@.len() == n@ * n@ && k@ <= n@)]
    #[invariant(forall<j> 0 <= j && j < n@ ==> c@[ci@ + j]@ == dot(a@, b@, n@, i@, j, k@) % 4294967296)]
    #[invariant(forall<x> 0 <= x && x < n@ * n@ && (x < ci@ || ci@ + n@ <= x) ==> c@[x] == old_c@[x])]
    #[variant(n@ - k@)]
    while k < n {
        let aik = a[ci + k];
        row_step(a, b, c, n, i, k, aik);
        k += 1;
    }
}

/// C = A·B modulo 2^32, c étant nulle au départ.
#[requires(n@ <= 4096)]
#[requires(a@.len() == n@ * n@ && b@.len() == n@ * n@ && c@.len() == n@ * n@)]
#[requires(forall<x> 0 <= x && x < n@ * n@ ==> c@[x]@ == 0)]
#[ensures((^c)@.len() == n@ * n@)]
#[ensures(is_product(a@, b@, (^c)@, n@, n@))]
#[check(terminates)]
pub fn matmul(a: &Vec<u32>, b: &Vec<u32>, c: &mut Vec<u32>, n: usize) {
    let mut i = 0usize;
    #[invariant(c@.len() == n@ * n@ && i@ <= n@)]
    #[invariant(is_product(a@, b@, c@, n@, i@))]
    #[invariant(forall<i2, j> i@ <= i2 && i2 < n@ && 0 <= j && j < n@ ==> c@[i2 * n@ + j]@ == 0)]
    #[variant(n@ - i@)]
    while i < n {
        proof_assert!(forall<i2, j> 0 <= i2 && i2 < n@ && i2 != i@ && 0 <= j && j < n@ ==> {
            row_disjoint(n@, i@, i2, j);
            in_matrix(n@, i2, j);
            (i2 * n@ + j < i@ * n@ || i@ * n@ + n@ <= i2 * n@ + j) && i2 * n@ + j < n@ * n@
        });
        row(a, b, c, n, i);
        i += 1;
    }
}

#[requires(c@.len() == nn@)]
#[check(terminates)]
pub fn checksum(c: &Vec<u32>, nn: usize) -> u64 {
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

#[requires(1 <= n@ && n@ <= 4096)]
pub fn run(n: usize) -> u64 {
    proof_assert! { mul_le(n@, 4096, n@); mul_le(n@, 4096, 4096); n@ * n@ <= 16777216 };
    let nn = n * n;
    let mut a = std::vec![0u32; nn];
    let mut b = std::vec![0u32; nn];
    let mut c = std::vec![0u32; nn];
    init_mat(&mut a, nn, 0);
    init_mat(&mut b, nn, nn as u64);
    matmul(&a, &b, &mut c, n);
    proof_assert!(is_product(a@, b@, c@, n@, n@));
    checksum(&c, nn)
}
