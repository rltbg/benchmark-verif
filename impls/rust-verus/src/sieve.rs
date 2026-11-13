//! Crible d'Ératosthène, 1 octet par entier. 2 <= n <= 4_000_000_000.
//!
//! Prouvé avec Verus (correction fonctionnelle complète, terminaison comprise) :
//!   * après `sieve`, pour 2 <= k <= n : `f[k] == 0 <==> is_prime(k)` ;
//!   * `count` (donc `run`) renvoie `fold(ck_primes(n), num_primes(n))` : le checksum chaîne
//!     exactement les premiers <= n dans l'ordre, puis leur nombre ;
//!   * absence de panique et de débordement ; aucune garde à l'exécution ajoutée pour la preuve.
//! Aucun `assume`, `admit` ni `external_body`.
use crate::common::*;
#[cfg(verus_only)]
use vstd::arithmetic::div_mod::{
    lemma_fundamental_div_mod, lemma_fundamental_div_mod_converse, lemma_mod_multiples_basic,
};
use vstd::prelude::*;

verus! {

// ---------------------------------------------------------------- spécification

/// d divise k.
pub open spec fn divides(d: int, k: int) -> bool {
    k % d == 0
}

/// p est premier : p >= 2 et aucun d de [2, p) ne le divise.
pub open spec fn is_prime(p: int) -> bool {
    2 <= p && forall|d: int| 2 <= d < p ==> !#[trigger] divides(d, p)
}

/// k a un diviseur d avec 2 <= d < b et d*d <= k : « k est barré une fois [2, b) traité ».
pub open spec fn crossed(k: int, b: int) -> bool {
    exists|d: int| 2 <= d < b && d * d <= k && #[trigger] divides(d, k)
}

/// Nombre de premiers dans [2, m].
pub open spec fn num_primes(m: int) -> nat
    decreases m,
{
    if m < 2 {
        0
    } else if is_prime(m) {
        num_primes(m - 1) + 1
    } else {
        num_primes(m - 1)
    }
}

/// Checksum des premiers de [2, m] dans l'ordre croissant.
pub open spec fn ck_primes(m: int) -> u64
    decreases m,
{
    if m < 2 {
        CK_INIT
    } else if is_prime(m) {
        spec_ckfold(ck_primes(m - 1), m as u64)
    } else {
        ck_primes(m - 1)
    }
}

// ---------------------------------------------------------------- lemmes d'arithmétique

/// d | i et i | k  ==>  d | k.
proof fn lemma_div_trans(d: int, i: int, k: int)
    requires
        1 <= d,
        1 <= i,
        0 <= k,
        divides(d, i),
        divides(i, k),
    ensures
        divides(d, k),
{
    lemma_fundamental_div_mod(i, d);
    lemma_fundamental_div_mod(k, i);
    let (a, b) = (i / d, k / i);
    assert(k == (a * b) * d) by (nonlinear_arith)
        requires
            i == d * a,
            k == i * b,
    ;
    lemma_mod_multiples_basic(a * b, d);
}

/// Entre deux multiples consécutifs de i, aucun multiple de i.
proof fn lemma_no_mult_between(i: int, j: int, k: int)
    requires
        1 <= i,
        0 <= j < k < j + i,
        divides(i, j),
    ensures
        !divides(i, k),
{
    lemma_fundamental_div_mod(j, i);
    assert(k == (j / i) * i + (k - j)) by (nonlinear_arith)
        requires
            j == i * (j / i),
    ;
    lemma_fundamental_div_mod_converse(k, i, j / i, k - j);
}

/// Un diviseur propre d de k fournit un diviseur <= sqrt(k) (d ou k/d).
proof fn lemma_small_divisor(d: int, k: int)
    requires
        2 <= d < k,
        divides(d, k),
    ensures
        crossed(k, k),
{
    lemma_fundamental_div_mod(k, d);
    let q = k / d;
    assert(2 <= q < k && k == q * d + 0) by (nonlinear_arith)
        requires
            k == d * q,
            2 <= d < k,
    ;
    lemma_fundamental_div_mod_converse(k, d, q, 0);
    lemma_fundamental_div_mod_converse(k, q, d, 0);
    assert(divides(q, k) && divides(d, k));
    if d * d > k {
        assert(q < d) by (nonlinear_arith)
            requires
                k == q * d,
                d * d > k,
                d >= 2,
        ;
        assert(q * q <= k) by (nonlinear_arith)
            requires
                k == q * d,
                q < d,
                q >= 2,
        ;
    }
}

/// Quand tous les d avec d*d <= k sont traités (k < b*b), « barré » équivaut à « composé ».
proof fn lemma_crossed_iff_composite(k: int, b: int)
    requires
        2 <= k < b * b,
        2 <= b,
    ensures
        crossed(k, b) == !is_prime(k),
{
    if crossed(k, b) {
        let d = choose|d: int| 2 <= d < b && d * d <= k && #[trigger] divides(d, k);
        assert(d < k) by (nonlinear_arith)
            requires
                2 <= d,
                d * d <= k,
        ;
    }
    if !is_prime(k) {
        let d = choose|d: int| 2 <= d < k && #[trigger] divides(d, k);
        lemma_small_divisor(d, k);
        let e = choose|e: int| 2 <= e < k && e * e <= k && #[trigger] divides(e, k);
        assert(e < b) by (nonlinear_arith)
            requires
                e * e <= k,
                k < b * b,
                2 <= e,
                2 <= b,
        ;
    }
}

// ---------------------------------------------------------------- code

/// Barre les multiples de i à partir de i*i.
fn mark(f: &mut Vec<u8>, n: usize, i: usize)
    requires
        old(f).len() == n + 1,
        2 <= i,
        i * i <= n <= 4_000_000_000,
    ensures
        final(f).len() == n + 1,
        forall|k: int|
            0 <= k <= n ==> (#[trigger] final(f)[k] == 0) == (old(f)[k] == 0 && !(i * i <= k && divides(
                i as int,
                k,
            ))),
{
    let mut j = i * i;
    proof {
        lemma_mod_multiples_basic(i as int, i as int);
    }
    // i <= sqrt(4e9) < 2^16 : j + i ne déborde pas (même si usize faisait 32 bits).
    assert(i <= 65536) by (nonlinear_arith)
        requires
            i * i <= 4_000_000_000,
            2 <= i,
    ;
    while j <= n
        invariant
            f.len() == n + 1,
            2 <= i <= 65536,
            i * i <= j <= n + i,
            n <= 4_000_000_000,
            divides(i as int, j as int),
            forall|k: int|
                0 <= k <= n ==> (#[trigger] f[k] == 0) == (old(f)[k] == 0 && !(i * i <= k < j
                    && divides(i as int, k))),
        decreases n + i - j,
    {
        f[j] = 1;
        proof {
            assert forall|k: int| j < k < j + i implies !divides(i as int, k) by {
                lemma_no_mult_between(i as int, j as int, k);
            }
            lemma_fundamental_div_mod(j as int, i as int);
            assert(j + i == (j / i + 1) * i + 0) by (nonlinear_arith)
                requires
                    j == i * (j / i),
            ;
            lemma_fundamental_div_mod_converse(j + i, i as int, j / i + 1, 0);
        }
        j += i;
    }
}

/// Crible : à la sortie, pour 2 <= k <= n, f[k] == 0 ssi k est premier.
fn sieve(f: &mut Vec<u8>, n: usize)
    requires
        old(f).len() == n + 1,
        2 <= n <= 4_000_000_000,
        forall|k: int| 0 <= k <= n ==> old(f)[k] == 0,
    ensures
        final(f).len() == n + 1,
        forall|k: int| 2 <= k <= n ==> (#[trigger] final(f)[k] == 0) == is_prime(k),
{
    let mut i: usize = 2;
    while i <= n / i
        invariant
            f.len() == n + 1,
            2 <= n <= 4_000_000_000,
            2 <= i <= n,
            forall|k: int| 2 <= k <= n ==> (#[trigger] f[k] == 0) == !crossed(k, i as int),
        decreases n - i,
    {
        proof {
            lemma_fundamental_div_mod(n as int, i as int);
        }
        assert(i * i <= n && i + 1 <= n) by (nonlinear_arith)
            requires
                i <= n / i,
                n == i * (n / i) + n % i,
                0 <= n % i,
                2 <= i,
        ;
        if f[i] == 0 {
            mark(f, n, i);
            assert forall|k: int| 2 <= k <= n implies (#[trigger] f[k] == 0) == !crossed(
                k,
                i + 1,
            ) by {
                if i * i <= k && divides(i as int, k) {
                    assert(crossed(k, i + 1));
                }
                if crossed(k, i + 1) && !crossed(k, i as int) {
                    let d = choose|d: int| 2 <= d < i + 1 && d * d <= k && #[trigger] divides(d, k);
                    assert(d == i);
                }
            }
        } else {
            // i est barré par un d < i : tout multiple k >= i*i de i l'est déjà par d.
            let ghost d = choose|d: int| 2 <= d < i && d * d <= i && #[trigger] divides(d, i as int);
            assert forall|k: int| 2 <= k <= n implies (#[trigger] f[k] == 0) == !crossed(
                k,
                i + 1,
            ) by {
                if i * i <= k && divides(i as int, k) {
                    lemma_div_trans(d, i as int, k);
                    assert(i <= k) by (nonlinear_arith)
                        requires
                            i * i <= k,
                            2 <= i,
                    ;
                    assert(crossed(k, i as int));
                }
                if crossed(k, i + 1) && !crossed(k, i as int) {
                    let e = choose|e: int| 2 <= e < i + 1 && e * e <= k && #[trigger] divides(e, k);
                    assert(e == i);
                    lemma_div_trans(d, i as int, k);
                    assert(d * d <= k);
                }
            }
        }
        i += 1;
    }
    proof {
        lemma_fundamental_div_mod(n as int, i as int);
        assert(n < i * i) by (nonlinear_arith)
            requires
                i > n / i,
                n == i * (n / i) + n % i,
                n % i < i,
        ;
        assert forall|k: int| 2 <= k <= n implies (#[trigger] f[k] == 0) == is_prime(k) by {
            lemma_crossed_iff_composite(k, i as int);
        }
    }
}

/// Checksum : repli des premiers dans l'ordre, puis de leur nombre.
fn count(f: &Vec<u8>, n: usize) -> (r: u64)
    requires
        f.len() == n + 1,
        2 <= n <= 4_000_000_000,
        forall|k: int| 2 <= k <= n ==> (#[trigger] f[k] == 0) == is_prime(k),
    ensures
        r == spec_ckfold(ck_primes(n as int), num_primes(n as int) as u64),
{
    let mut i: usize = 2;
    let mut ck = CK_INIT;
    let mut cnt: u64 = 0;
    while i <= n
        invariant
            2 <= i <= n + 1,
            f.len() == n + 1,
            n <= 4_000_000_000,
            forall|k: int| 2 <= k <= n ==> (#[trigger] f[k] == 0) == is_prime(k),
            ck == ck_primes(i - 1),
            cnt == num_primes(i - 1),
            cnt <= i,
        decreases n + 1 - i,
    {
        if f[i] == 0 {
            ck = ckfold(ck, i as u64);
            cnt += 1;
        }
        i += 1;
    }
    ckfold(ck, cnt)
}

pub fn run(n: usize, _threads: usize) -> (r: u64)
    requires
        2 <= n <= 4_000_000_000,
    ensures
        r == spec_ckfold(ck_primes(n as int), num_primes(n as int) as u64),
{
    let mut f = vec![0u8; n + 1];
    sieve(&mut f, n);
    count(&f, n)
}

} // verus!
