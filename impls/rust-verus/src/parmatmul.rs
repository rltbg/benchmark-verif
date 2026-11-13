//! Workload `parmatmul` (spec/workloads.d/parmatmul.md) : le produit de `matmul`, réparti en blocs de lignes sur
//! T threads (le thread k calcule les lignes [n·k/T, n·(k+1)/T)). 1 <= n <= 16384.
//!
//! Vérifié avec Verus, threads compris (`vstd::thread::spawn` / `JoinHandle::join`) :
//! * le noyau `rows` calcule le bloc de lignes [r0, r1) : `c[i·n+j] == dot(A, B, n, r0+i, j, n) % 2^32`
//!   (même `dot` que `matmul`) ;
//! * chaque thread rend son bloc ; la postcondition de la fermeture passe par `JoinHandle::predicate` jusqu'à `run`,
//!   qui replie les blocs dans l'ordre : le checksum est celui des n² éléments de C = A·B, ligne par ligne
//!   (`run` garantit `result == ck_seq(CK_INIT, product(A, B, n))`, avec A et B donnés par la spécification) ;
//! * absence de panique et de débordement, terminaison.
//!
//! Aucun `assume`/`admit`/`external_body` dans ce module. Hypothèses : les spécifications de `vstd`
//! (`thread::spawn`, `Arc::new`, `Vec`). Différences avec le code de référence, imposées par `vstd::thread::spawn`
//! (fermetures `'static`, pas de threads « scoped ») : A et B sont partagées par `Arc`, chaque thread alloue et rend
//! son bloc de C (même mémoire totale, aucune copie) au lieu d'écrire dans une tranche de C, et le checksum replie
//! les blocs dans l'ordre. Le cas `Err` de `join` (thread paniqué) est inatteignable avec `panic = "abort"` ; il rend 0.
use crate::common::*;
use crate::matmul::{dot, lemma_dot_nonneg, lemma_row_disjoint, lemma_wrap_step, m32};
use std::sync::Arc;
use vstd::prelude::*;
use vstd::thread::{spawn, JoinHandle};

verus! {

const SEED: u64 = 0x0FEDCBA987654321;

/// Le bloc `c` (m×n) contient les lignes r0..r0+m de A·B modulo 2^32.
pub open spec fn block_ok(a: Seq<u32>, b: Seq<u32>, c: Seq<u32>, n: int, r0: int, m: int) -> bool {
    &&& c.len() == m * n
    &&& forall|i: int, j: int|
        0 <= i < m && 0 <= j < n ==> c[i * n + j] == #[trigger] dot(a, b, n, r0 + i, j, n) % m32()
}

/// Première ligne du bloc k sur t.
pub open spec fn lo(n: int, t: int, k: int) -> int {
    n * k / t
}

/// i·n + n <= m·n pour i < m.
proof fn lemma_in_block(n: int, m: int, i: int)
    requires
        0 <= i < m,
        0 <= n,
    ensures
        i * n + n <= m * n,
{
    assert((i + 1) * n <= m * n) by (nonlinear_arith)
        requires
            i + 1 <= m,
            0 <= n,
    ;
    assert((i + 1) * n == i * n + n) by (nonlinear_arith);
}

/// Bornes des blocs : 0 <= lo(k) <= lo(k+1) <= n, lo(0) = 0, lo(t) = n.
proof fn lemma_block_bounds(n: int, t: int, k: int)
    requires
        0 <= n,
        0 <= k < t,
    ensures
        0 <= lo(n, t, k) <= lo(n, t, k + 1) <= n,
        lo(n, t, 0) == 0,
        lo(n, t, t) == n,
{
    assert(0 <= n * k <= n * (k + 1) <= n * t) by (nonlinear_arith)
        requires
            0 <= n,
            0 <= k < t,
    ;
    vstd::arithmetic::div_mod::lemma_div_is_ordered(n * k, n * (k + 1), t);
    vstd::arithmetic::div_mod::lemma_div_is_ordered(n * (k + 1), n * t, t);
    assert(n * t / t == n) by (nonlinear_arith)
        requires
            0 < t,
    ;
    assert(n * 0 / t == 0) by (nonlinear_arith)
        requires
            0 < t,
    ;
    assert(0 <= n * k / t) by (nonlinear_arith)
        requires
            0 <= n * k,
            0 < t,
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

/// Pas k de la ligne i du bloc (ligne r0+i de A) : c[i,j] += a[r0+i,k]·b[k,j] ; le reste de c ne change pas.
fn row_step(
    a: &Vec<u32>,
    b: &Vec<u32>,
    c: &mut Vec<u32>,
    n: usize,
    m: usize,
    r0: usize,
    i: usize,
    k: usize,
    aik: u32,
)
    requires
        n <= 16384,
        r0 + m <= n,
        i < m,
        k < n,
        a@.len() == n * n,
        b@.len() == n * n,
        old(c)@.len() == m * n,
        aik == a@[(r0 + i) * n + k],
        forall|j: int|
            0 <= j < n ==> old(c)@[i * n + j] == #[trigger] dot(a@, b@, n as int, r0 + i, j, k as int)
                % m32(),
    ensures
        final(c)@.len() == m * n,
        forall|j: int|
            0 <= j < n ==> final(c)@[i * n + j] == #[trigger] dot(a@, b@, n as int, r0 + i, j, k + 1)
                % m32(),
        forall|x: int| 0 <= x < m * n && (x < i * n || i * n + n <= x) ==> #[trigger] final(c)@[x] == old(c)@[x],
{
    proof {
        lemma_in_block(n as int, m as int, i as int);
        lemma_in_block(n as int, n as int, k as int);
        assert(m * n <= n * n <= 16384 * 16384) by (nonlinear_arith)
            requires
                m <= n <= 16384,
        ;
    }
    let ci = i * n;
    let bk = k * n;
    let mut j = 0usize;
    while j < n
        invariant
            n <= 16384,
            r0 + m <= n,
            i < m,
            k < n,
            ci == i * n,
            bk == k * n,
            ci + n <= m * n,
            m * n <= 16384 * 16384,
            a@.len() == n * n,
            aik == a@[(r0 + i) * n + k],
            forall|j2: int|
                0 <= j2 < n ==> old(c)@[i * n + j2] == #[trigger] dot(
                    a@,
                    b@,
                    n as int,
                    r0 + i,
                    j2,
                    k as int,
                ) % m32(),
            bk + n <= n * n,
            n * n <= 16384 * 16384,
            b@.len() == n * n,
            c@.len() == m * n,
            j <= n,
            forall|j2: int|
                0 <= j2 < j ==> c@[ci + j2] == #[trigger] dot(a@, b@, n as int, r0 + i, j2, k + 1)
                    % m32(),
            forall|x: int| 0 <= x < m * n && !(ci <= x < ci + j) ==> #[trigger] c@[x] == old(c)@[x],
        decreases n - j,
    {
        proof {
            let d = dot(a@, b@, n as int, r0 + i, j as int, k as int);
            lemma_dot_nonneg(a@, b@, n as int, r0 + i, j as int, k as int);
            lemma_wrap_step(c@[ci + j], aik, b@[bk + j], d);
            assert(dot(a@, b@, n as int, r0 + i, j as int, k + 1) == d + aik * b@[bk + j]);
        }
        c[ci + j] = c[ci + j].wrapping_add(aik.wrapping_mul(b[bk + j]));
        j += 1;
    }
}

/// Ligne i du bloc (ligne r0+i de C), à partir d'une ligne nulle ; le reste de c ne change pas.
fn row(a: &Vec<u32>, b: &Vec<u32>, c: &mut Vec<u32>, n: usize, m: usize, r0: usize, i: usize)
    requires
        n <= 16384,
        r0 + m <= n,
        i < m,
        a@.len() == n * n,
        b@.len() == n * n,
        old(c)@.len() == m * n,
        forall|j: int| 0 <= j < n ==> #[trigger] old(c)@[i * n + j] == 0,
    ensures
        final(c)@.len() == m * n,
        forall|j: int|
            0 <= j < n ==> final(c)@[i * n + j] == #[trigger] dot(a@, b@, n as int, r0 + i, j, n as int)
                % m32(),
        forall|x: int| 0 <= x < m * n && (x < i * n || i * n + n <= x) ==> #[trigger] final(c)@[x] == old(c)@[x],
{
    proof {
        lemma_in_block(n as int, m as int, i as int);
        lemma_in_block(n as int, n as int, r0 + i);
        assert(n * n <= 16384 * 16384) by (nonlinear_arith)
            requires
                n <= 16384,
        ;
        assert((r0 + i) * n + n <= n * n);
        assert(m * n <= n * n) by (nonlinear_arith)
            requires
                m <= n,
        ;
    }
    let ci = i * n;
    let ai = (r0 + i) * n;
    let mut k = 0usize;
    while k < n
        invariant
            n <= 16384,
            r0 + m <= n,
            i < m,
            ci == i * n,
            ai == (r0 + i) * n,
            ai + n <= n * n,
            n * n <= 16384 * 16384,
            a@.len() == n * n,
            b@.len() == n * n,
            c@.len() == m * n,
            k <= n,
            forall|j: int|
                0 <= j < n ==> c@[ci + j] == #[trigger] dot(a@, b@, n as int, r0 + i, j, k as int)
                    % m32(),
            forall|x: int| 0 <= x < m * n && (x < ci || ci + n <= x) ==> #[trigger] c@[x] == old(c)@[x],
        decreases n - k,
    {
        let aik = a[ai + k];
        row_step(a, b, c, n, m, r0, i, k, aik);
        k += 1;
    }
}

/// Noyau séquentiel d'un thread : le bloc des lignes [r0, r1) de C = A·B, alloué ici.
fn rows(a: &Vec<u32>, b: &Vec<u32>, n: usize, r0: usize, r1: usize) -> (c: Vec<u32>)
    requires
        n <= 16384,
        r0 <= r1 <= n,
        a@.len() == n * n,
        b@.len() == n * n,
    ensures
        block_ok(a@, b@, c@, n as int, r0 as int, r1 - r0),
{
    let m = r1 - r0;
    proof {
        assert(m * n <= n * n <= 16384 * 16384) by (nonlinear_arith)
            requires
                m <= n <= 16384,
        ;
    }
    let mut c = vec![0u32; m * n];
    let mut i = 0usize;
    while i < m
        invariant
            n <= 16384,
            r0 + m <= n,
            a@.len() == n * n,
            b@.len() == n * n,
            c@.len() == m * n,
            i <= m,
            forall|i2: int, j: int|
                0 <= i2 < i && 0 <= j < n ==> c@[i2 * n + j] == #[trigger] dot(
                    a@,
                    b@,
                    n as int,
                    r0 + i2,
                    j,
                    n as int,
                ) % m32(),
            forall|x: int| i * n <= x < m * n ==> #[trigger] c@[x] == 0,
        decreases m - i,
    {
        let ghost c0 = c@;
        proof {
            lemma_in_block(n as int, m as int, i as int);
        }
        row(a, b, &mut c, n, m, r0, i);
        proof {
            assert forall|i2: int, j: int| 0 <= i2 < i + 1 && 0 <= j < n implies c@[i2 * n + j]
                == #[trigger] dot(a@, b@, n as int, r0 + i2, j, n as int) % m32() by {
                if i2 != i {
                    lemma_row_disjoint(n as int, i as int, i2, j);
                    lemma_in_block(n as int, m as int, i2);
                    assert(c@[i2 * n + j] == c0[i2 * n + j]);
                }
            }
            assert forall|x: int| (i + 1) * n <= x < m * n implies #[trigger] c@[x] == 0 by {
                assert((i + 1) * n == i * n + n) by (nonlinear_arith);
            }
        }
        i += 1;
    }
    c
}

/// Checksum logique : repli de `ckfold` sur une séquence.
pub open spec fn ck_seq(ck: u64, s: Seq<u32>) -> u64
    decreases s.len(),
{
    if s.len() == 0 {
        ck
    } else {
        ck_seq(spec_ckfold(ck, s[0] as u64), s.drop_first())
    }
}

proof fn lemma_ck_seq_push(ck: u64, s: Seq<u32>, x: u32)
    ensures
        ck_seq(ck, s.push(x)) == spec_ckfold(ck_seq(ck, s), x as u64),
    decreases s.len(),
{
    if s.len() > 0 {
        assert(s.push(x).drop_first() =~= s.drop_first().push(x));
        lemma_ck_seq_push(spec_ckfold(ck, s[0] as u64), s.drop_first(), x);
    } else {
        assert(s.push(x).drop_first() =~= Seq::<u32>::empty());
        reveal_with_fuel(ck_seq, 2);
    }
}

/// C = A·B modulo 2^32, matrice n×n ligne par ligne.
pub open spec fn product(a: Seq<u32>, b: Seq<u32>, n: int) -> Seq<u32> {
    Seq::new((n * n) as nat, |x: int| (dot(a, b, n, x / n, x % n, n) % m32()) as u32)
}

/// Continue le checksum `ck` avec les éléments du bloc.
fn fold_block(ck: u64, blk: &Vec<u32>) -> (r: u64)
    ensures
        r == ck_seq(ck, blk@),
{
    let ghost ck0 = ck;
    let mut ck = ck;
    let mut i = 0usize;
    while i < blk.len()
        invariant
            i <= blk@.len(),
            ck == ck_seq(ck0, blk@.subrange(0, i as int)),
        decreases blk@.len() - i,
    {
        proof {
            lemma_ck_seq_push(ck0, blk@.subrange(0, i as int), blk@[i as int]);
            assert(blk@.subrange(0, i + 1) == blk@.subrange(0, i as int).push(blk@[i as int]));
        }
        ck = ckfold(ck, blk[i] as u64);
        i += 1;
    }
    proof {
        assert(blk@.subrange(0, blk@.len() as int) == blk@);
    }
    ck
}

/// Un bloc correct [r0, r1) prolonge le préfixe correct des r0 premières lignes de C.
proof fn lemma_append_block(a: Seq<u32>, b: Seq<u32>, n: int, r0: int, r1: int, pre: Seq<u32>, blk: Seq<u32>)
    requires
        0 < n,
        0 <= r0 <= r1 <= n,
        pre == product(a, b, n).subrange(0, r0 * n),
        block_ok(a, b, blk, n, r0, r1 - r0),
    ensures
        pre + blk == product(a, b, n).subrange(0, r1 * n),
{
    assert(r0 * n <= r1 * n <= n * n) by (nonlinear_arith)
        requires
            0 <= r0 <= r1 <= n,
    ;
    assert((r1 - r0) * n == r1 * n - r0 * n) by (nonlinear_arith);
    let p = product(a, b, n);
    assert forall|x: int| 0 <= x < r1 * n implies #[trigger] (pre + blk)[x] == p.subrange(0, r1 * n)[x] by {
        if x >= r0 * n {
            let y = x - r0 * n;
            let i = y / n;
            let j = y % n;
            assert(y == i * n + j && 0 <= j < n) by {
                vstd::arithmetic::div_mod::lemma_fundamental_div_mod(y, n);
                vstd::arithmetic::div_mod::lemma_mod_bound(y, n);
                vstd::arithmetic::mul::lemma_mul_is_commutative(n, i);
            }
            assert(0 <= i < r1 - r0) by (nonlinear_arith)
                requires
                    y == i * n + j,
                    0 <= j < n,
                    0 <= y < (r1 - r0) * n,
                    0 < n,
            ;
            assert(x / n == r0 + i && x % n == j) by {
                assert(x == (r0 + i) * n + j) by (nonlinear_arith)
                    requires
                        x == r0 * n + y,
                        y == i * n + j,
                ;
                vstd::arithmetic::div_mod::lemma_div_multiples_vanish_fancy(r0 + i, j, n);
                vstd::arithmetic::div_mod::lemma_mod_multiples_vanish(r0 + i, j, n);
                vstd::arithmetic::div_mod::lemma_small_mod(j as nat, n as nat);
                vstd::arithmetic::mul::lemma_mul_is_commutative(n, r0 + i);
            }
            assert(blk[i * n + j] == dot(a, b, n, r0 + i, j, n) % m32());
            lemma_dot_nonneg(a, b, n, r0 + i, j, n);
        }
    }
    assert(pre + blk =~= p.subrange(0, r1 * n));
}

/// 1 <= n <= 16384 (spécification) ; threads <= 2^16 évite le débordement de n·threads (usize de 32 bits compris).
pub fn run(n: usize, threads: usize) -> (ck: u64)
    requires
        1 <= n <= 16384,
        threads <= 65536,
{
    let t = if threads == 0 { 1 } else { threads };
    proof {
        assert(n * n <= 16384 * 16384) by (nonlinear_arith)
            requires
                n <= 16384,
        ;
    }
    let nn = n * n;
    let mut a = vec![0u32; nn];
    let mut b = vec![0u32; nn];
    init_mat(&mut a, 0);
    init_mat(&mut b, nn as u64);
    let ghost (av, bv) = (a@, b@);
    let a = Arc::new(a);
    let b = Arc::new(b);
    // Lancement des threads 0..t-1 ; le dernier bloc est calculé par le thread courant.
    let mut handles: Vec<JoinHandle<Vec<u32>>> = Vec::new();
    let mut k = 0usize;
    while k + 1 < t
        invariant
            1 <= n <= 16384,
            1 <= t <= 65536,
            k < t,
            a@ == av,
            b@ == bv,
            av.len() == n * n,
            bv.len() == n * n,
            handles@.len() == k,
            forall|kk: int, blk: Vec<u32>|
                0 <= kk < k && #[trigger] handles@[kk].predicate(blk) ==> block_ok(
                    av,
                    bv,
                    blk@,
                    n as int,
                    lo(n as int, t as int, kk),
                    lo(n as int, t as int, kk + 1) - lo(n as int, t as int, kk),
                ),
        decreases t - k,
    {
        proof {
            lemma_block_bounds(n as int, t as int, k as int);
            assert(n * k <= n * (k + 1) <= 16384 * 65536) by (nonlinear_arith)
                requires
                    n <= 16384,
                    k + 1 <= 65536,
            ;
        }
        let (r0, r1) = (n * k / t, n * (k + 1) / t);
        let (ta, tb) = (a.clone(), b.clone());
        let h = spawn(
            move || -> (blk: Vec<u32>)
                requires
                    r0 <= r1 <= n <= 16384,
                    ta@.len() == n * n,
                    tb@.len() == n * n,
                ensures
                    block_ok(ta@, tb@, blk@, n as int, r0 as int, r1 - r0),
                { rows(&ta, &tb, n, r0, r1) },
        );
        handles.push(h);
        k += 1;
    }
    proof {
        lemma_block_bounds(n as int, t as int, k as int);
        assert(n * k <= n * (k + 1) <= 16384 * 65536) by (nonlinear_arith)
            requires
                n <= 16384,
                k + 1 <= 65536,
        ;
    }
    let last = rows(&a, &b, n, n * k / t, n);
    // Jonction dans l'ordre des blocs ; le checksum replie C = concaténation des blocs (`cg`, fantôme).
    let ghost mut cg = Seq::<u32>::empty();
    let mut ck = CK_INIT;
    let mut kk = 0usize;
    proof {
        lemma_block_bounds(n as int, t as int, 0);
        assert(cg == product(av, bv, n as int).subrange(0, 0));
    }
    while kk < k
        invariant
            1 <= n <= 16384,
            k + 1 == t,
            kk <= k,
            av.len() == n * n,
            bv.len() == n * n,
            handles@.len() == k - kk,
            forall|k2: int, blk: Vec<u32>|
                0 <= k2 < k - kk && #[trigger] handles@[k2].predicate(blk) ==> block_ok(
                    av,
                    bv,
                    blk@,
                    n as int,
                    lo(n as int, t as int, kk + k2),
                    lo(n as int, t as int, kk + k2 + 1) - lo(n as int, t as int, kk + k2),
                ),
            cg == product(av, bv, n as int).subrange(0, lo(n as int, t as int, kk as int) * n),
            ck == ck_seq(CK_INIT, cg),
        decreases k - kk,
    {
        let ghost hs = handles@;
        let h = handles.remove(0);
        let blk = match h.join() {
            Result::Ok(blk) => blk,
            // Inatteignable : avec `panic = "abort"`, un thread qui panique arrête le processus.
            Result::Err(_) => return 0,
        };
        proof {
            assert(h == hs[0]);
            assert forall|k2: int, b2: Vec<u32>|
                0 <= k2 < k - (kk + 1) && #[trigger] handles@[k2].predicate(b2) implies block_ok(
                    av,
                    bv,
                    b2@,
                    n as int,
                    lo(n as int, t as int, kk + 1 + k2),
                    lo(n as int, t as int, kk + 1 + k2 + 1) - lo(n as int, t as int, kk + 1 + k2),
                ) by {
                assert(handles@[k2] == hs[k2 + 1]);
            }
            lemma_block_bounds(n as int, t as int, kk as int);
            lemma_append_block(
                av,
                bv,
                n as int,
                lo(n as int, t as int, kk as int),
                lo(n as int, t as int, kk + 1),
                cg,
                blk@,
            );
            lemma_ck_seq_concat(CK_INIT, cg, blk@);
        }
        ck = fold_block(ck, &blk);
        proof {
            cg = cg + blk@;
        }
        kk += 1;
    }
    proof {
        lemma_block_bounds(n as int, t as int, k as int);
        lemma_append_block(av, bv, n as int, lo(n as int, t as int, k as int), n as int, cg, last@);
        lemma_ck_seq_concat(CK_INIT, cg, last@);
    }
    ck = fold_block(ck, &last);
    proof {
        cg = cg + last@;
        assert(cg =~= product(av, bv, n as int));
        // Résultat : le checksum des n² éléments de C = A·B (mod 2^32), en ordre ligne-majeur.
        assert(ck == ck_seq(CK_INIT, product(av, bv, n as int)));
    }
    ck
}

proof fn lemma_ck_seq_concat(ck: u64, s1: Seq<u32>, s2: Seq<u32>)
    ensures
        ck_seq(ck, s1 + s2) == ck_seq(ck_seq(ck, s1), s2),
    decreases s1.len(),
{
    if s1.len() > 0 {
        assert((s1 + s2).drop_first() =~= s1.drop_first() + s2);
        lemma_ck_seq_concat(spec_ckfold(ck, s1[0] as u64), s1.drop_first(), s2);
    } else {
        assert(s1 + s2 =~= s2);
    }
}

} // verus!
