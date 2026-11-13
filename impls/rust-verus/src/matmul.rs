//! Workload `matmul` (spec/WORKLOADS.md §3) : C = A·B, matrices n×n de u32, arithmétique modulo 2^32,
//! boucles i, k, j. 1 <= n <= 4096.
//!
//! Vérifié avec Verus :
//! * correction fonctionnelle : `matmul` garantit `C[i·n+j] == dot(A, B, n, i, j, n) % 2^32` pour tous i, j < n,
//!   où `dot` est la somme Σ_l A[i·n+l]·B[l·n+j] dans les entiers mathématiques ;
//! * absence de panique et de débordement, terminaison de toutes les fonctions.
//!
//! Aucun `assume`/`admit`/`external_body`. Les faits arithmétiques non linéaires sont isolés dans des lemmes
//! (`lemma_wrap_step` : congruence modulo 2^32 du pas `c + a·b` ; `lemma_row_disjoint`, `lemma_in_matrix` :
//! inégalités sur i·n), prouvés par `by (nonlinear_arith)` ou par les lemmes de `vstd::arithmetic`.
use crate::common::*;
use vstd::arithmetic::div_mod::lemma_add_mod_noop;
use vstd::prelude::*;

verus! {

const SEED: u64 = 0x0FEDCBA987654321;

/// 2^32
pub open spec fn m32() -> int {
    0x1_0000_0000
}

/// Σ_{l<k} a[i·n+l]·b[l·n+j] dans les entiers (sans réduction modulo 2^32).
pub open spec fn dot(a: Seq<u32>, b: Seq<u32>, n: int, i: int, j: int, k: int) -> int
    decreases k,
{
    if k <= 0 {
        0
    } else {
        dot(a, b, n, i, j, k - 1) + a[i * n + (k - 1)] * b[(k - 1) * n + j]
    }
}

/// La matrice `c` (n×n, ligne par ligne) vaut le produit `a·b` modulo 2^32 sur les lignes `0..rows`.
pub open spec fn is_product(a: Seq<u32>, b: Seq<u32>, c: Seq<u32>, n: int, rows: int) -> bool {
    forall|i: int, j: int|
        0 <= i < rows && 0 <= j < n ==> c[i * n + j] == #[trigger] dot(a, b, n, i, j, n) % m32()
}

pub proof fn lemma_dot_nonneg(a: Seq<u32>, b: Seq<u32>, n: int, i: int, j: int, k: int)
    ensures
        dot(a, b, n, i, j, k) >= 0,
    decreases k,
{
    if k > 0 {
        lemma_dot_nonneg(a, b, n, i, j, k - 1);
        assert(a[i * n + (k - 1)] * b[(k - 1) * n + j] >= 0) by (nonlinear_arith);
    }
}

/// Congruence modulo 2^32 du pas de la boucle interne : si c ≡ d, alors `c ⊕ (x ⊗ y) ≡ d + x·y`.
pub proof fn lemma_wrap_step(c: u32, x: u32, y: u32, d: int)
    requires
        0 <= d,
        c == d % m32(),
    ensures
        c.wrapping_add(x.wrapping_mul(y)) == (d + x * y) % m32(),
{
    let p = x.wrapping_mul(y);
    assert(p == (x * y) % m32());
    // wrapping_add est spécifiée par cas (avec ou sans retenue) : c'est (c + p) mod 2^32.
    assert(c.wrapping_add(p) == (c + p) % m32());
    lemma_add_mod_noop(d, x * y, m32());
}

/// i·n + j < n·n pour i, j < n.
pub proof fn lemma_in_matrix(n: int, i: int, j: int)
    requires
        0 <= i < n,
        0 <= j < n,
    ensures
        i * n + j < n * n,
        i * n + n <= n * n,
{
    assert((i + 1) * n <= n * n) by (nonlinear_arith)
        requires
            i + 1 <= n,
            0 <= n,
    ;
    assert((i + 1) * n == i * n + n) by (nonlinear_arith);
}

/// Deux lignes différentes sont disjointes : i2·n + j (j < n) est hors de [i·n, i·n + n).
pub proof fn lemma_row_disjoint(n: int, i: int, i2: int, j: int)
    requires
        0 <= i,
        0 <= i2,
        i != i2,
        0 <= j < n,
    ensures
        i2 * n + j < i * n || i * n + n <= i2 * n + j,
{
    if i2 < i {
        assert(i2 * n + n <= i * n) by (nonlinear_arith)
            requires
                i2 + 1 <= i,
                0 <= n,
        ;
    } else {
        assert(i * n + n <= i2 * n) by (nonlinear_arith)
            requires
                i + 1 <= i2,
                0 <= n,
        ;
    }
}

/// n·n <= 2^24 pour n <= 4096.
pub proof fn lemma_square_bound(n: int)
    requires
        0 <= n <= 4096,
    ensures
        n * n <= 16777216,
{
    assert(n * n <= 4096 * 4096) by (nonlinear_arith)
        requires
            0 <= n <= 4096,
    ;
}

fn init_mat(m: &mut Vec<u32>, off: u64)
    ensures
        final(m)@.len() == old(m)@.len(),
{
    let len = m.len();
    let mut i = 0usize;
    while i < len
        invariant
            m@.len() == len,
            i <= len,
        decreases len - i,
    {
        m[i] = hi32(mix64(SEED.wrapping_add(off).wrapping_add(i as u64)));
        i += 1;
    }
}

/// Pas k de la ligne i : c[i,j] += a[i,k]·b[k,j] pour tout j ; le reste de c ne change pas.
fn row_step(a: &Vec<u32>, b: &Vec<u32>, c: &mut Vec<u32>, n: usize, i: usize, k: usize, aik: u32)
    requires
        n <= 4096,
        i < n,
        k < n,
        a@.len() == n * n,
        b@.len() == n * n,
        old(c)@.len() == n * n,
        aik == a@[i * n + k],
        forall|j: int|
            0 <= j < n ==> old(c)@[i * n + j] == #[trigger] dot(a@, b@, n as int, i as int, j, k as int)
                % m32(),
    ensures
        final(c)@.len() == n * n,
        forall|j: int|
            0 <= j < n ==> final(c)@[i * n + j] == #[trigger] dot(a@, b@, n as int, i as int, j, k + 1)
                % m32(),
        forall|x: int| 0 <= x < n * n && (x < i * n || i * n + n <= x) ==> #[trigger] final(c)@[x] == old(c)@[x],
{
    proof {
        lemma_in_matrix(n as int, i as int, 0);
        lemma_in_matrix(n as int, k as int, 0);
        lemma_square_bound(n as int);
    }
    let ci = i * n;
    let bk = k * n;
    let mut j = 0usize;
    while j < n
        invariant
            n <= 4096,
            i < n,
            k < n,
            ci == i * n,
            bk == k * n,
            ci + n <= n * n,
            n * n <= 16777216,
            bk + n <= n * n,
            a@.len() == n * n,
            b@.len() == n * n,
            c@.len() == n * n,
            aik == a@[i * n + k],
            j <= n,
            forall|j2: int|
                0 <= j2 < j ==> c@[ci + j2] == #[trigger] dot(a@, b@, n as int, i as int, j2, k + 1)
                    % m32(),
            forall|x: int| 0 <= x < n * n && !(ci <= x < ci + j) ==> #[trigger] c@[x] == old(c)@[x],
            forall|j2: int|
                0 <= j2 < n ==> old(c)@[i * n + j2] == #[trigger] dot(
                    a@,
                    b@,
                    n as int,
                    i as int,
                    j2,
                    k as int,
                ) % m32(),
        decreases n - j,
    {
        proof {
            let d = dot(a@, b@, n as int, i as int, j as int, k as int);
            lemma_dot_nonneg(a@, b@, n as int, i as int, j as int, k as int);
            lemma_wrap_step(c@[ci + j], aik, b@[bk + j], d);
            assert(dot(a@, b@, n as int, i as int, j as int, k + 1) == d + aik * b@[bk + j]);
        }
        c[ci + j] = c[ci + j].wrapping_add(aik.wrapping_mul(b[bk + j]));
        j += 1;
    }
}

/// Ligne i de C : c[i,j] = Σ_k a[i,k]·b[k,j] mod 2^32, à partir d'une ligne nulle ; le reste de c ne change pas.
fn row(a: &Vec<u32>, b: &Vec<u32>, c: &mut Vec<u32>, n: usize, i: usize)
    requires
        n <= 4096,
        i < n,
        a@.len() == n * n,
        b@.len() == n * n,
        old(c)@.len() == n * n,
        forall|j: int| 0 <= j < n ==> #[trigger] old(c)@[i * n + j] == 0,
    ensures
        final(c)@.len() == n * n,
        forall|j: int|
            0 <= j < n ==> final(c)@[i * n + j] == #[trigger] dot(a@, b@, n as int, i as int, j, n as int)
                % m32(),
        forall|x: int| 0 <= x < n * n && (x < i * n || i * n + n <= x) ==> #[trigger] final(c)@[x] == old(c)@[x],
{
    proof {
        lemma_in_matrix(n as int, i as int, 0);
        lemma_square_bound(n as int);
    }
    let ci = i * n;
    let mut k = 0usize;
    while k < n
        invariant
            n <= 4096,
            i < n,
            ci == i * n,
            ci + n <= n * n,
            n * n <= 16777216,
            a@.len() == n * n,
            b@.len() == n * n,
            c@.len() == n * n,
            k <= n,
            forall|j: int|
                0 <= j < n ==> c@[ci + j] == #[trigger] dot(a@, b@, n as int, i as int, j, k as int)
                    % m32(),
            forall|x: int| 0 <= x < n * n && (x < ci || ci + n <= x) ==> #[trigger] c@[x] == old(c)@[x],
        decreases n - k,
    {
        let aik = a[ci + k];
        row_step(a, b, c, n, i, k, aik);
        k += 1;
    }
}

/// C = A·B modulo 2^32, c étant nulle au départ.
pub fn matmul(a: &Vec<u32>, b: &Vec<u32>, c: &mut Vec<u32>, n: usize)
    requires
        n <= 4096,
        a@.len() == n * n,
        b@.len() == n * n,
        old(c)@.len() == n * n,
        forall|x: int| 0 <= x < n * n ==> #[trigger] old(c)@[x] == 0,
    ensures
        final(c)@.len() == n * n,
        is_product(a@, b@, final(c)@, n as int, n as int),
{
    let mut i = 0usize;
    while i < n
        invariant
            n <= 4096,
            a@.len() == n * n,
            b@.len() == n * n,
            c@.len() == n * n,
            i <= n,
            is_product(a@, b@, c@, n as int, i as int),
            forall|x: int| i * n <= x < n * n ==> #[trigger] c@[x] == 0,
        decreases n - i,
    {
        let ghost c0 = c@;
        proof {
            lemma_in_matrix(n as int, i as int, 0);
            assert forall|j: int| 0 <= j < n implies #[trigger] c@[i * n + j] == 0 by {
                lemma_in_matrix(n as int, i as int, j);
            }
        }
        row(a, b, c, n, i);
        proof {
            assert forall|i2: int, j: int| 0 <= i2 < i + 1 && 0 <= j < n implies c@[i2 * n + j]
                == #[trigger] dot(a@, b@, n as int, i2, j, n as int) % m32() by {
                if i2 != i {
                    lemma_row_disjoint(n as int, i as int, i2, j);
                    lemma_in_matrix(n as int, i2, j);
                    assert(c@[i2 * n + j] == c0[i2 * n + j]);
                }
            }
            assert forall|x: int| (i + 1) * n <= x < n * n implies #[trigger] c@[x] == 0 by {
                assert((i + 1) * n == i * n + n) by (nonlinear_arith);
            }
        }
        i += 1;
    }
}

fn checksum(c: &Vec<u32>) -> u64 {
    let mut ck = CK_INIT;
    let mut i = 0usize;
    while i < c.len()
        invariant
            i <= c@.len(),
        decreases c@.len() - i,
    {
        ck = ckfold(ck, c[i] as u64);
        i += 1;
    }
    ck
}

/// Contrainte de la spécification : 1 <= n <= 4096 (précondition, pas de garde à l'exécution ; `main` n'est pas vérifié).
pub fn run(n: usize, _threads: usize) -> u64
    requires
        1 <= n <= 4096,
{
    proof {
        lemma_square_bound(n as int);
    }
    let nn = n * n;
    let mut a = vec![0u32; nn];
    let mut b = vec![0u32; nn];
    let mut c = vec![0u32; nn];
    init_mat(&mut a, 0);
    init_mat(&mut b, nn as u64);
    matmul(&a, &b, &mut c, n);
    checksum(&c)
}

} // verus!
