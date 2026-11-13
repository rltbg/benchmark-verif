//! Workload `ntt` (spec/workloads.d/ntt.md) : produit cyclique par NTT entière modulo p = 2013265921.
//!
//! Vérifié avec Verus :
//! * toutes les valeurs manipulées sont < p (`below_p`) : entrée, racines, chaque étage de papillons, carré, sortie ;
//! * la permutation bit-reverse : `rev_bits(x, k)` (fonction spec qui suit la boucle `rev = 2·rev + x mod 2`)
//!   est une involution de [0, 2^k) (`lemma_rev_involutive`), et `bitrev` permute le tableau : `a'[x] == a[rev_bits(x, k)]` ;
//! * au-delà : `powmod(b, e) == b^e mod p` (carré-et-multiplie, `vstd::arithmetic::power`), donc
//!   `w == g^((p-1)/n) mod p` et `ninv == n^(p-2) mod p`, et la table `tw[j] == w^j mod p` ;
//! * absence de panique (indices des papillons à stride variable, table de racines) et de débordement ; terminaison.
//! Non prouvé : que la transformée soit la DFT modulaire (ni que w soit une racine primitive n-ième de l'unité, ni la
//! récurrence de Cooley-Tukey des étages) ; c'est vérifié par un test indépendant contre la convolution naïve.
//!
//! Aucun `assume`/`admit`/`external_body`.
use crate::common::*;
use vstd::arithmetic::div_mod::lemma_mul_mod_noop_general;
use vstd::arithmetic::power::{
    lemma_pow0, lemma_pow1, lemma_pow_adds, lemma_pow_mod_noop, lemma_pow_multiplies, lemma_square_is_pow2, pow,
};
use vstd::prelude::*;

verus! {

const SEED: u64 = 0xA5A5F00D12345678;

pub const P: u64 = 2013265921;

const G: u64 = 31;

/// 2^k
pub open spec fn pow2(k: int) -> int
    decreases k,
{
    if k <= 0 {
        1
    } else {
        2 * pow2(k - 1)
    }
}

proof fn lemma_pow2_pos(k: int)
    requires
        0 <= k,
    ensures
        pow2(k) >= 1,
        pow2(k) > k,
    decreases k,
{
    if k > 0 {
        lemma_pow2_pos(k - 1);
    }
}

proof fn lemma_pow2_le(a: int, b: int)
    requires
        0 <= a <= b,
    ensures
        pow2(a) <= pow2(b),
    decreases b,
{
    if a < b {
        lemma_pow2_pos(b - 1);
        lemma_pow2_le(a, b - 1);
    }
}

/// Toutes les valeurs de `s` sont < p.
pub open spec fn below_p(s: Seq<u32>) -> bool {
    forall|i: int| 0 <= i < s.len() ==> #[trigger] s[i] < P
}

/// x inversé sur k bits : le bit de poids faible de x devient le bit k-1.
pub open spec fn rev_bits(x: int, k: int) -> int
    decreases k,
{
    if k <= 0 {
        0
    } else {
        (x % 2) * pow2(k - 1) + rev_bits(x / 2, k - 1)
    }
}

/// 0 <= rev_bits(x, k) < 2^k.
proof fn lemma_rev_bound(x: int, k: int)
    requires
        0 <= x,
        0 <= k,
    ensures
        0 <= rev_bits(x, k) < pow2(k),
    decreases k,
{
    if k > 0 {
        lemma_rev_bound(x / 2, k - 1);
        lemma_pow2_pos(k - 1);
    }
}

/// Le bit de poids fort t de y + t·2^k devient le bit de poids faible : rev(y + t·2^k, k+1) == 2·rev(y, k) + t.
proof fn lemma_rev_top(y: int, t: int, k: int)
    requires
        0 <= k,
        0 <= y < pow2(k),
        0 <= t <= 1,
    ensures
        rev_bits(y + t * pow2(k), k + 1) == 2 * rev_bits(y, k) + t,
    decreases k,
{
    if k > 0 {
        let h = pow2(k - 1);
        assert(t * pow2(k) == 2 * (t * h)) by (nonlinear_arith)
            requires
                pow2(k) == 2 * h,
        ;
        assert((y + t * pow2(k)) % 2 == y % 2 && (y + t * pow2(k)) / 2 == y / 2 + t * h);
        lemma_rev_top(y / 2, t, k - 1);
        assert(2 * ((y % 2) * h) == (y % 2) * pow2(k)) by (nonlinear_arith)
            requires
                pow2(k) == 2 * h,
        ;
    } else {
        assert(y == 0);
        reveal_with_fuel(rev_bits, 2);
    }
}

/// La permutation bit-reverse est une involution de [0, 2^k).
proof fn lemma_rev_involutive(x: int, k: int)
    requires
        0 <= k,
        0 <= x < pow2(k),
    ensures
        rev_bits(rev_bits(x, k), k) == x,
    decreases k,
{
    if k > 0 {
        lemma_rev_bound(x / 2, k - 1);
        lemma_rev_top(rev_bits(x / 2, k - 1), x % 2, k - 1);
        lemma_rev_involutive(x / 2, k - 1);
        assert((x % 2) * pow2(k - 1) == (x % 2) * pow2(k - 1));
    }
}

/// Un pas de la boucle bit-reverse : (2r + x mod 2)·2^(m-1) + rev(x/2, m-1) == r·2^m + rev(x, m).
proof fn lemma_rev_step(r: int, x: int, m: int)
    requires
        1 <= m,
    ensures
        (2 * r + x % 2) * pow2(m - 1) + rev_bits(x / 2, m - 1) == r * pow2(m) + rev_bits(x, m),
{
    let h = pow2(m - 1);
    assert((2 * r + x % 2) * h == r * (2 * h) + (x % 2) * h) by (nonlinear_arith);
}

/// a·b <= x·y pour des naturels.
proof fn lemma_mul_mono(a: int, b: int, x: int, y: int)
    requires
        0 <= a <= x,
        0 <= b <= y,
    ensures
        a * b <= x * y,
{
    assert(a * b <= x * y) by (nonlinear_arith)
        requires
            0 <= a <= x,
            0 <= b <= y,
    ;
}

/// Un tour de carré-et-multiplie conserve r·b^e mod p : avec h = e/2,
/// r·b^e ≡ (r·b mod p si e impair, r sinon) · (b² mod p)^h.
proof fn lemma_powmod_step(r: int, b: int, e: nat)
    requires
        e > 0,
    ensures
        (r * pow(b, e)) % (P as int) == ((if e % 2 == 1 {
            (r * b) % (P as int)
        } else {
            r
        }) * pow((b * b) % (P as int), e / 2)) % (P as int),
{
    let m = P as int;
    let h: nat = e / 2;
    let t: nat = e % 2;
    lemma_pow_adds(b, 2 * h, t);
    lemma_pow_multiplies(b, 2, h);
    lemma_square_is_pow2(b);
    lemma_pow_mod_noop(b * b, h, m);
    let q = pow(b * b, h);
    let q2 = pow((b * b) % m, h);
    assert(q2 % m == q % m);
    if t == 1 {
        lemma_pow1(b);
        lemma_mul_mod_noop_general(r * b, q2, m);
        lemma_mul_mod_noop_general(r * b, q, m);
        assert(r * (q * b) == (r * b) * q) by (nonlinear_arith);
    } else {
        lemma_pow0(b);
        lemma_mul_mod_noop_general(r, q2, m);
        lemma_mul_mod_noop_general(r, q, m);
    }
}

/// Exponentiation modulaire (carré-et-multiplie), b < p : r == b^e mod p.
fn powmod(b0: u64, e0: u64) -> (r: u64)
    requires
        b0 < P,
    ensures
        r < P,
        r == pow(b0 as int, e0 as nat) % (P as int),
{
    let mut b = b0;
    let mut e = e0;
    let mut r = 1u64;
    proof {
        lemma_pow1(b0 as int);
        lemma_pow0(b0 as int);
    }
    while e > 0
        invariant
            b < P,
            r < P,
            (r * pow(b as int, e as nat)) % (P as int) == pow(b0 as int, e0 as nat) % (P as int),
        decreases e,
    {
        proof {
            lemma_powmod_step(r as int, b as int, e as nat);
        }
        proof {
            lemma_mul_mono(r as int, b as int, 2013265920, 2013265920);
            lemma_mul_mono(b as int, b as int, 2013265920, 2013265920);
        }
        if e % 2 == 1 {
            r = r * b % P;
        }
        b = b * b % P;
        e /= 2;
    }
    proof {
        lemma_pow0(b as int);
    }
    r
}

fn fill(a: &mut Vec<u32>)
    ensures
        final(a)@.len() == old(a)@.len(),
        below_p(final(a)@),
{
    let n = a.len();
    let mut i = 0usize;
    while i < n
        invariant
            a@.len() == n,
            i <= n,
            forall|x: int| 0 <= x < i ==> #[trigger] a@[x] < P,
        decreases n - i,
    {
        a[i] = (mix64(SEED.wrapping_add(i as u64)) % P) as u32;
        i += 1;
    }
}

/// tw[j] = w^j pour j < n/2 (tw a n/2 + 1 cases).
fn twiddles(tw: &mut Vec<u32>, n: usize, w: u64)
    requires
        w < P,
        old(tw)@.len() == n / 2 + 1,
        below_p(old(tw)@),
    ensures
        final(tw)@.len() == old(tw)@.len(),
        below_p(final(tw)@),
        forall|j: int| 0 <= j < n / 2 ==> #[trigger] final(tw)@[j] == pow(w as int, j as nat) % (P as int),
{
    let mut c = 1u64;
    let mut j = 0usize;
    proof {
        lemma_pow0(w as int);
    }
    while j < n / 2
        invariant
            w < P,
            c < P,
            c == pow(w as int, j as nat) % (P as int),
            tw@.len() == n / 2 + 1,
            j <= n / 2,
            below_p(tw@),
            forall|j2: int| 0 <= j2 < j ==> #[trigger] tw@[j2] == pow(w as int, j2 as nat) % (P as int),
        decreases n / 2 - j,
    {
        tw[j] = c as u32;
        proof {
            lemma_mul_mono(c as int, w as int, 2013265920, 2013265920);
            // w^(j+1) = w^j·w, et (w^j mod p)·w ≡ w^j·w.
            lemma_pow_adds(w as int, j as nat, 1);
            lemma_pow1(w as int);
            lemma_mul_mod_noop_general(pow(w as int, j as nat), w as int, P as int);
        }
        c = c * w % P;
        j += 1;
    }
}

/// Permutation bit-reverse sur k bits ; n = 2^k : a'[x] == a[rev_bits(x, k)].
fn bitrev(a: &mut Vec<u32>, n: usize, k: usize)
    requires
        old(a)@.len() == n,
        n == pow2(k as int),
    ensures
        final(a)@.len() == n,
        forall|x: int| 0 <= x < n ==> #[trigger] final(a)@[x] == old(a)@[rev_bits(x, k as int)],
{
    let ghost a0 = a@;
    proof {
        assert forall|x: int| 0 <= x < n implies 0 <= #[trigger] rev_bits(x, k as int) < n && rev_bits(
            rev_bits(x, k as int),
            k as int,
        ) == x by {
            lemma_rev_bound(x, k as int);
            lemma_rev_involutive(x, k as int);
        }
        lemma_pow2_pos(k as int);
    }
    let mut i = 0usize;
    while i < n
        invariant
            n == pow2(k as int),
            k < n,
            a@.len() == n,
            a0.len() == n,
            i <= n,
            forall|x: int|
                0 <= x < n ==> 0 <= #[trigger] rev_bits(x, k as int) < n && rev_bits(
                    rev_bits(x, k as int),
                    k as int,
                ) == x,
            // Les paires {x, rev x} déjà traitées (x < i ou rev x < i) sont échangées, les autres intactes.
            forall|x: int|
                0 <= x < n ==> #[trigger] a@[x] == if x < i || rev_bits(x, k as int) < i {
                    a0[rev_bits(x, k as int)]
                } else {
                    a0[x]
                },
        decreases n - i,
    {
        let mut x = i;
        let mut rev = 0usize;
        let mut b = 0usize;
        let mut bnd = 1usize;
        while b < k
            invariant
                n == pow2(k as int),
                b <= k,
                bnd == pow2(b as int),
                bnd <= n,
                rev < bnd,
                rev_bits(i as int, k as int) == rev * pow2(k - b) + rev_bits(x as int, k - b),
            decreases k - b,
        {
            proof {
                lemma_pow2_le(b + 1, k as int);
                lemma_rev_step(rev as int, x as int, k - b);
            }
            rev = rev * 2 + x % 2;
            x /= 2;
            bnd *= 2;
            b += 1;
        }
        assert(rev == rev_bits(i as int, k as int)) by {
            assert(pow2(0) == 1 && rev_bits(x as int, 0) == 0);
            assert(rev * pow2(0) == rev) by (nonlinear_arith)
                requires
                    pow2(0) == 1,
            ;
        }
        if i < rev {
            let t = a[i];
            a[i] = a[rev];
            a[rev] = t;
        }
        i += 1;
    }
}

/// Un étage : papillons de demi-longueur `half` (len = 2·half <= n), stride = n/len.
fn stage(a: &mut Vec<u32>, tw: &Vec<u32>, n: usize, half: usize)
    requires
        old(a)@.len() == n,
        tw@.len() == n / 2 + 1,
        1 <= half,
        2 * half <= n <= 134217728,
        below_p(old(a)@),
    ensures
        final(a)@.len() == n,
        below_p(final(a)@),
{
    let len = half * 2;
    let stride = n / len;
    proof {
        assert(stride * len <= n) by (nonlinear_arith)
            requires
                stride == n / len,
                len > 0,
        ;
        assert(half * stride * 2 == len * stride) by (nonlinear_arith)
            requires
                len == half * 2,
        ;
    }
    let mut blk = 0usize;
    while blk < stride
        invariant
            a@.len() == n,
            tw@.len() == n / 2 + 1,
            1 <= half,
            len == half * 2,
            len <= n <= 134217728,
            stride * len <= n,
            half * stride * 2 == len * stride,
            blk <= stride,
            below_p(a@),
        decreases stride - blk,
    {
        proof {
            lemma_mul_mono(blk + 1, len as int, stride as int, len as int);
            assert((blk + 1) * len == blk * len + len) by (nonlinear_arith);
        }
        let start = blk * len;
        let mut j = 0usize;
        while j < half
            invariant
                a@.len() == n,
                tw@.len() == n / 2 + 1,
                len == half * 2,
                start + len <= n <= 134217728,
                half * stride * 2 == len * stride,
                stride * len <= n,
                j <= half,
                below_p(a@),
            decreases half - j,
        {
            proof {
                lemma_mul_mono(j as int, stride as int, half - 1, stride as int);
                assert((half - 1) * stride == half * stride - stride) by (nonlinear_arith);
                assert(half * stride <= n / 2) by (nonlinear_arith)
                    requires
                        half * stride * 2 == len * stride,
                        stride * len <= n,
                ;
            }
            let u = a[start + j] as u64;
            let x = a[start + j + half] as u64;
            let t = tw[j * stride] as u64;
            proof {
                lemma_mul_mono(x as int, t as int, u32::MAX as int, u32::MAX as int);
            }
            let v = x * t % P;
            a[start + j] = ((u + v) % P) as u32;
            a[start + j + half] = ((u + P - v) % P) as u32;
            j += 1;
        }
        blk += 1;
    }
}

fn transform(a: &mut Vec<u32>, tw: &Vec<u32>, n: usize, k: usize)
    requires
        old(a)@.len() == n,
        tw@.len() == n / 2 + 1,
        n == pow2(k as int),
        n <= 134217728,
        below_p(old(a)@),
    ensures
        final(a)@.len() == n,
        below_p(final(a)@),
{
    bitrev(a, n, k);
    proof {
        lemma_pow2_pos(k as int);
        assert forall|x: int| 0 <= x < n implies #[trigger] a@[x] < P by {
            lemma_rev_bound(x, k as int);
        }
    }
    let mut half = 1usize;
    while half * 2 <= n
        invariant
            a@.len() == n,
            tw@.len() == n / 2 + 1,
            n <= 134217728,
            1 <= half <= n,
            below_p(a@),
        decreases n + 1 - half,
    {
        stage(a, tw, n, half);
        half *= 2;
    }
}

fn square(a: &mut Vec<u32>)
    requires
        below_p(old(a)@),
    ensures
        final(a)@.len() == old(a)@.len(),
        below_p(final(a)@),
{
    let n = a.len();
    let mut i = 0usize;
    while i < n
        invariant
            a@.len() == n,
            i <= n,
            below_p(a@),
        decreases n - i,
    {
        let x = a[i] as u64;
        proof {
            lemma_mul_mono(x as int, x as int, u32::MAX as int, u32::MAX as int);
        }
        a[i] = (x * x % P) as u32;
        i += 1;
    }
}

/// Renverse a[1..n-1] (échange a[i] et a[n-i], 1 <= i, 2i < n).
fn reverse(a: &mut Vec<u32>)
    requires
        old(a)@.len() <= 134217728,
        below_p(old(a)@),
    ensures
        final(a)@.len() == old(a)@.len(),
        below_p(final(a)@),
{
    let n = a.len();
    let mut i = 1usize;
    while 2 * i < n
        invariant
            a@.len() == n,
            n <= 134217728,
            1 <= i,
            2 * i <= n + 2,
            below_p(a@),
        decreases n - i,
    {
        let t = a[i];
        a[i] = a[n - i];
        a[n - i] = t;
        i += 1;
    }
}

/// Repli des r[i] = a[i]·ninv mod p (chacun < p).
fn checksum(a: &Vec<u32>, ninv: u64) -> u64
    requires
        ninv < P,
{
    let mut ck = CK_INIT;
    let mut i = 0usize;
    while i < a.len()
        invariant
            ninv < P,
            i <= a@.len(),
        decreases a@.len() - i,
    {
        let x = a[i] as u64;
        proof {
            lemma_mul_mono(x as int, ninv as int, u32::MAX as int, 2013265920);
        }
        ck = ckfold(ck, x * ninv % P);
        i += 1;
    }
    ck
}

pub fn run(n: usize, _threads: usize) -> u64 {
    // Rejets exigés par la spécification : n > 2^27 ou n pas une puissance de 2 (n = 0 compris) donnent 0.
    if n > 134217728 {
        return 0;
    }
    let mut pw = 1usize;
    let mut k = 0usize;
    while pw < n
        invariant
            n <= 134217728,
            pw == pow2(k as int),
            pw <= 2 * n || pw == 1,
            k < pw,
        decreases 2 * n + 1 - pw,
    {
        proof {
            lemma_pow2_pos(k + 1);
        }
        pw *= 2;
        k += 1;
    }
    if pw != n {
        return 0;
    }
    let mut a = vec![0u32; n];
    let mut tw = vec![0u32; n / 2 + 1];
    fill(&mut a);
    let w = powmod(G, (P - 1) / (n as u64));
    twiddles(&mut tw, n, w);
    transform(&mut a, &tw, n, k);
    square(&mut a);
    transform(&mut a, &tw, n, k);
    reverse(&mut a);
    let ninv = powmod(n as u64, P - 2);
    checksum(&a, ninv)
}

} // verus!
