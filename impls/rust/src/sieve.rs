//! Crible d'Ératosthène, 1 octet par entier. 2 <= n <= 4_000_000_000.
//!
//! Prouvé avec Creusot (correction fonctionnelle complète, terminaison comprise) :
//!   * après `sieve`, pour 2 <= k <= n : `f[k] == 0 <==> is_prime(k)` ;
//!   * `count` (et donc `run`) renvoie `fold(ck_primes(n), num_primes(n))` : le checksum chaîne
//!     exactement les premiers <= n dans l'ordre, puis leur nombre ;
//!   * absence de panique et de débordement ; aucune garde à l'exécution ajoutée pour la preuve.
//! Aucun `trusted` ni `assume`.
use crate::common::CK_INIT;
use creusot_std::prelude::{vec, *};

// ---------------------------------------------------------------- spécification

/// `p` est premier : p >= 2 et aucun d de [2, p) ne divise p.
#[logic(open)]
pub fn is_prime(p: Int) -> bool {
    pearlite! { 2 <= p && forall<d: Int> 2 <= d && d < p ==> p % d != 0 }
}

/// `k` a un diviseur d avec 2 <= d < b et d*d <= k : « k est barré une fois [2, b) traité ».
#[logic(open)]
pub fn crossed(k: Int, b: Int) -> bool {
    pearlite! { exists<d: Int> 2 <= d && d < b && d * d <= k && k % d == 0 }
}

/// Nombre de premiers dans [2, m].
#[logic(open)]
#[variant(m)]
pub fn num_primes(m: Int) -> Int {
    if m < 2 {
        0
    } else if is_prime(m) {
        num_primes(m - 1) + 1
    } else {
        num_primes(m - 1)
    }
}

/// Repli du checksum (modulo 2^64) : `fold(ck, v) = (ck ^ v) * 0x100000001B3`.
#[logic(open)]
pub fn fold(ck: u64, v: u64) -> u64 {
    (ck ^ v) * 0x100000001B3u64
}

/// Checksum des premiers de [2, m] pris dans l'ordre croissant.
#[logic(open)]
#[variant(m)]
pub fn ck_primes(m: u64) -> u64 {
    pearlite! {
        if m < 2u64 {
            CK_INIT
        } else if is_prime(m@) {
            fold(ck_primes(m - 1u64), m)
        } else {
            ck_primes(m - 1u64)
        }
    }
}

// ---------------------------------------------------------------- lemmes d'arithmétique

/// (d*q + r) % d == r pour 0 <= r < d : seul fait de division dont les autres découlent.
#[logic]
#[requires(1 <= d && 0 <= q && 0 <= r && r < d)]
#[ensures((d * q + r) % d == r)]
pub fn mod_mul_add(d: Int, q: Int, r: Int) {}

/// d | i et i | k  ==>  d | k.
#[logic]
#[requires(1 <= d && 1 <= i && 0 <= k && i % d == 0 && k % i == 0)]
#[ensures(k % d == 0)]
pub fn div_trans(d: Int, i: Int, k: Int) {
    proof_assert!(i == d * (i / d));
    proof_assert!(k == i * (k / i));
    proof_assert!(k == d * ((i / d) * (k / i)) + 0);
    mod_mul_add(d, (i / d) * (k / i), 0)
}

/// Entre deux multiples consécutifs de i, aucun multiple de i.
#[logic]
#[requires(1 <= i && 0 <= j && j % i == 0 && j < k && k < j + i)]
#[ensures(k % i != 0)]
pub fn no_mult_between(i: Int, j: Int, k: Int) {
    proof_assert!(j == i * (j / i));
    proof_assert!(k == i * (j / i) + (k - j));
    mod_mul_add(i, j / i, k - j)
}

/// Un diviseur propre d de k fournit un diviseur <= sqrt(k) (d ou k/d).
#[logic]
#[requires(2 <= d && d < k && k % d == 0)]
#[ensures(crossed(k, k))]
pub fn small_divisor(d: Int, k: Int) {
    let q = k / d;
    proof_assert!(k == d * q);
    proof_assert!(2 <= q && q < k);
    proof_assert!(k % q == 0);
    if d * d > k {
        proof_assert!(q * q <= k);
    }
}

/// Quand tous les d avec d*d <= k sont traités (k < b*b), « barré » équivaut à « composé ».
#[logic]
#[requires(2 <= k && k < b * b && 2 <= b)]
#[ensures(crossed(k, b) == !is_prime(k))]
pub fn crossed_iff_composite(k: Int, b: Int) {
    pearlite! {
        if !is_prime(k) {
            proof_assert!(forall<d: Int> 2 <= d && d < k && k % d == 0 ==> { small_divisor(d, k); crossed(k, k) });
            proof_assert!(forall<d: Int> 2 <= d && d * d <= k ==> d < b);
        }
    }
}

// ---------------------------------------------------------------- code

/// Pas du checksum (même calcul que `common::ckfold`, avec son contrat logique).
#[ensures(result == fold(ck, v))]
#[check(terminates)]
#[inline(always)]
fn ckfold(ck: u64, v: u64) -> u64 {
    (ck ^ v).wrapping_mul(0x100000001B3)
}

/// Barre les multiples de `i` à partir de `i*i`.
#[requires(f@.len() == n@ + 1 && 2 <= i@ && i@ * i@ <= n@ && n@ <= 4000000000)]
#[ensures((^f)@.len() == f@.len())]
#[ensures(forall<k: Int> 0 <= k && k <= n@ ==>
    ((^f)[k]@ == 0) == (f[k]@ == 0 && !(i@ * i@ <= k && k % i@ == 0)))]
#[check(terminates)]
pub fn mark(f: &mut Vec<u8>, n: usize, i: usize) {
    let old = snapshot!(*f);
    let mut j = i * i;
    proof_assert!({ mod_mul_add(i@, i@, 0); j@ % i@ == 0 });
    #[invariant(f@.len() == n@ + 1)]
    #[invariant(i@ * i@ <= j@ && j@ <= n@ + i@ && j@ % i@ == 0)]
    #[invariant(forall<k: Int> 0 <= k && k <= n@ ==>
        (f[k]@ == 0) == (old[k]@ == 0 && !(i@ * i@ <= k && k < j@ && k % i@ == 0)))]
    #[variant(n@ + i@ - j@)]
    while j <= n {
        f[j] = 1;
        proof_assert!(forall<k: Int> j@ < k && k < j@ + i@ ==> { no_mult_between(i@, j@, k); k % i@ != 0 });
        proof_assert!(j@ == i@ * (j@ / i@));
        proof_assert!(j@ + i@ == i@ * (j@ / i@ + 1) + 0);
        proof_assert!({ mod_mul_add(i@, j@ / i@ + 1, 0); (i@ * (j@ / i@ + 1) + 0) % i@ == 0 });
        j += i;
    }
}

/// Crible : à la sortie, pour 2 <= k <= n, f[k] == 0 ssi k est premier.
#[requires(f@.len() == n@ + 1 && 2 <= n@ && n@ <= 4000000000)]
#[requires(forall<k: Int> 0 <= k && k <= n@ ==> f[k]@ == 0)]
#[ensures((^f)@.len() == f@.len())]
#[ensures(forall<k: Int> 2 <= k && k <= n@ ==> ((^f)[k]@ == 0) == is_prime(k))]
#[check(terminates)]
pub fn sieve(f: &mut Vec<u8>, n: usize) {
    let mut i = 2usize;
    #[invariant(f@.len() == n@ + 1)]
    #[invariant(2 <= i@ && (i@ - 1) * (i@ - 1) <= n@)]
    #[invariant(forall<k: Int> 2 <= k && k <= n@ ==> (f[k]@ == 0) == !crossed(k, i@))]
    #[variant(n@ - i@)]
    while i <= n / i {
        proof_assert!(i@ * i@ <= n@);
        if f[i] == 0 {
            mark(f, n, i);
        } else {
            // i est barré par un d < i : tout multiple k >= i*i de i l'est déjà par d.
            proof_assert!(forall<k: Int> i@ * i@ <= k && k <= n@ && k % i@ == 0 ==>
                forall<d: Int> 2 <= d && d < i@ && d * d <= i@ && i@ % d == 0 ==> { div_trans(d, i@, k); crossed(k, i@) });
        }
        i += 1;
    }
    proof_assert!(n@ < i@ * i@);
    proof_assert!(forall<k: Int> 2 <= k && k <= n@ ==> { crossed_iff_composite(k, i@); true });
}

/// Checksum : repli des premiers dans l'ordre, puis de leur nombre.
#[requires(f@.len() == n@ + 1 && 2 <= n@ && n@ <= 4000000000)]
#[requires(forall<k: Int> 2 <= k && k <= n@ ==> (f[k]@ == 0) == is_prime(k))]
#[ensures(exists<c: u64> c@ == num_primes(n@) && result == fold(ck_primes(n as u64), c))]
#[check(terminates)]
pub fn count(f: &Vec<u8>, n: usize) -> u64 {
    let mut i = 2usize;
    let mut ck = CK_INIT;
    let mut cnt = 0u64;
    #[invariant(2 <= i@ && i@ <= n@ + 1)]
    #[invariant(ck == ck_primes((i - 1usize) as u64) && cnt@ == num_primes(i@ - 1) && cnt@ <= i@)]
    #[variant(n@ + 1 - i@)]
    while i <= n {
        if f[i] == 0 {
            ck = ckfold(ck, i as u64);
            cnt += 1;
        }
        i += 1;
    }
    ckfold(ck, cnt)
}

#[requires(2 <= n@ && n@ <= 4000000000)]
#[ensures(exists<c: u64> c@ == num_primes(n@) && result == fold(ck_primes(n as u64), c))]
// Pas de `#[check(terminates)]` : `std::vec::from_elem` n'est pas spécifié `terminates` dans creusot-std.
pub fn run(n: usize) -> u64 {
    let mut f = vec![0u8; n + 1];
    sieve(&mut f, n);
    count(&f, n)
}

