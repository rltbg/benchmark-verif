//! Workload `ntt` (spec/workloads.d/ntt.md) : produit cyclique par NTT entière modulo p = 2013265921.
//!
//! Prouvé avec Creusot :
//! * toutes les valeurs manipulées sont < p (`below_p`) : entrée, racines, chaque étage de papillons, carré, sortie ;
//! * la permutation bit-reverse : `rev_bits(x, k)` (fonction logique qui suit la boucle `rev = 2·rev + x mod 2`)
//!   est une involution de [0, 2^k) (`rev_involutive`), et `bitrev` permute le tableau : `a'[x] = a[rev_bits(x, k)]` ;
//! * au-delà : `powmod(b, e) = b^e mod p` (carré-et-multiplie ; lemmes `pw_add`, `pw_sq`, `pw_mod`, `mul_mod` écrits
//!   ici avec leur preuve), donc `w = g^((p-1)/n) mod p` et `ninv = n^(p-2) mod p`, et la table `tw[j] = w^j mod p` ;
//! * absence de panique (indices des papillons à stride variable, table de racines) et de débordement ;
//!   terminaison de tout ce qui n'alloue pas.
//! Non prouvé : que la transformée soit la DFT modulaire (ni que w soit une racine primitive n-ième de l'unité, ni la
//! récurrence de Cooley-Tukey des étages) ; c'est vérifié par un test indépendant contre la convolution naïve.
use crate::common::*;
use creusot_std::prelude::*;

const SEED: u64 = 0xA5A5F00D12345678;
const P: u64 = 2013265921;
const G: u64 = 31;

/// 2^k, pour les invariants des boucles bit-reverse.
#[logic(open)]
#[variant(k)]
pub fn pow2(k: Int) -> Int {
    pearlite! { if k <= 0 { 1 } else { 2 * pow2(k - 1) } }
}

#[logic]
#[requires(0 <= k)]
#[ensures(pow2(k) > k && pow2(k) >= 1)]
#[variant(k)]
pub fn pow2_gt(k: Int) {
    pearlite! { if k > 0 { pow2_gt(k - 1) } }
}

#[logic]
#[requires(0 <= a && a <= b)]
#[ensures(pow2(a) <= pow2(b))]
#[variant(b)]
pub fn pow2_le(a: Int, b: Int) {
    pearlite! { if a < b { pow2_gt(b - 1); pow2_le(a, b - 1) } }
}

/// a*b <= x*y pour des naturels.
#[logic]
#[requires(0 <= a && a <= x && 0 <= b && b <= y)]
#[ensures(a * b <= x * y)]
pub fn mul_mono(a: Int, b: Int, x: Int, y: Int) {}

/// Toutes les valeurs de `s` sont < p.
#[logic(open)]
pub fn below_p(s: Seq<u32>) -> bool {
    pearlite! { forall<i> 0 <= i && i < s.len() ==> s[i]@ < 2013265921 }
}

/// x inversé sur k bits, défini comme la boucle du programme : le bit de poids faible de x devient le bit k-1.
#[logic(open)]
#[variant(k)]
pub fn rev_bits(x: Int, k: Int) -> Int {
    pearlite! { if k <= 0 { 0 } else { (x % 2) * pow2(k - 1) + rev_bits(x / 2, k - 1) } }
}

/// 0 <= rev_bits(x, k) < 2^k.
#[logic]
#[requires(0 <= x && 0 <= k)]
#[ensures(0 <= rev_bits(x, k) && rev_bits(x, k) < pow2(k))]
#[variant(k)]
pub fn rev_bound(x: Int, k: Int) {
    pearlite! { if k > 0 { rev_bound(x / 2, k - 1) } }
}

/// Le bit de poids fort t de y + t·2^k devient le bit de poids faible : rev_bits(y + t·2^k, k+1) = 2·rev_bits(y, k) + t.
#[logic]
#[requires(0 <= k && 0 <= y && y < pow2(k) && 0 <= t && t <= 1)]
#[ensures(rev_bits(y + t * pow2(k), k + 1) == 2 * rev_bits(y, k) + t)]
#[variant(k)]
pub fn rev_top(y: Int, t: Int, k: Int) {
    pearlite! {
        if k > 0 {
            // y + t·2^k = 2·(y/2 + t·2^(k-1)) + y mod 2
            proof_assert!(y + t * pow2(k) == 2 * (y / 2 + t * pow2(k - 1)) + y % 2);
            proof_assert!((y + t * pow2(k)) % 2 == y % 2 && (y + t * pow2(k)) / 2 == y / 2 + t * pow2(k - 1));
            rev_top(y / 2, t, k - 1)
        }
    }
}

/// La permutation bit-reverse est une involution de [0, 2^k).
#[logic]
#[requires(0 <= k && 0 <= x && x < pow2(k))]
#[ensures(rev_bits(rev_bits(x, k), k) == x)]
#[variant(k)]
pub fn rev_involutive(x: Int, k: Int) {
    pearlite! {
        if k > 0 {
            rev_bound(x / 2, k - 1);
            rev_top(rev_bits(x / 2, k - 1), x % 2, k - 1);
            rev_involutive(x / 2, k - 1)
        }
    }
}

/// Un pas de la boucle bit-reverse : (2r + x mod 2)·2^(m-1) + rev_bits(x/2, m-1) = r·2^m + rev_bits(x, m).
#[logic]
#[requires(1 <= m)]
#[ensures((2 * r + x % 2) * pow2(m - 1) + rev_bits(x / 2, m - 1) == r * pow2(m) + rev_bits(x, m))]
pub fn rev_step(r: Int, x: Int, m: Int) {
    pearlite! { proof_assert!((2 * r + x % 2) * pow2(m - 1) == r * (2 * pow2(m - 1)) + (x % 2) * pow2(m - 1)) }
}

/// b^e (e >= 0).
#[logic(open)]
#[variant(e)]
pub fn pw(b: Int, e: Int) -> Int {
    pearlite! { if e <= 0 { 1 } else { b * pw(b, e - 1) } }
}

#[logic]
#[requires(0 <= b && 0 <= e)]
#[ensures(pw(b, e) >= 0)]
#[variant(e)]
pub fn pw_nonneg(b: Int, e: Int) {
    pearlite! { if e > 0 { pw_nonneg(b, e - 1); mul_mono(0, 0, b, pw(b, e - 1)) } }
}

/// b^(e+1) = b·b^e (dépliage isolé : dans un grand contexte, la définition récursive de `pw` boucle).
#[logic]
#[requires(0 <= e)]
#[ensures(pw(b, e + 1) == b * pw(b, e))]
pub fn pw_succ(b: Int, e: Int) {}

/// b^(e1 + e2) = b^e1 · b^e2.
#[logic]
#[requires(0 <= e1 && 0 <= e2)]
#[ensures(pw(b, e1 + e2) == pw(b, e1) * pw(b, e2))]
#[variant(e1)]
pub fn pw_add(b: Int, e1: Int, e2: Int) {
    pearlite! {
        if e1 > 0 {
            pw_add(b, e1 - 1, e2);
            proof_assert!(b * (pw(b, e1 - 1) * pw(b, e2)) == (b * pw(b, e1 - 1)) * pw(b, e2))
        }
    }
}

/// b^(2h) = (b·b)^h.
#[logic]
#[requires(0 <= h)]
#[ensures(pw(b, 2 * h) == pw(b * b, h))]
#[variant(h)]
pub fn pw_sq(b: Int, h: Int) {
    pearlite! {
        if h > 0 {
            pw_sq(b, h - 1);
            proof_assert!(pw(b, 2 * h) == b * (b * pw(b, 2 * h - 2)));
            proof_assert!(b * (b * pw(b, 2 * h - 2)) == (b * b) * pw(b, 2 * (h - 1)))
        }
    }
}

/// (q·p + z) mod p = z mod p (q, z >= 0).
#[logic]
#[requires(0 <= q && 0 <= z)]
#[ensures((q * 2013265921 + z) % 2013265921 == z % 2013265921)]
pub fn mod_shift(q: Int, z: Int) {}

/// (x mod p)·y ≡ x·y et x·(y mod p) ≡ x·y modulo p (x, y >= 0).
#[logic]
#[requires(0 <= x && 0 <= y)]
#[ensures((x % 2013265921 * y) % 2013265921 == (x * y) % 2013265921)]
#[ensures((x * (y % 2013265921)) % 2013265921 == (x * y) % 2013265921)]
pub fn mul_mod(x: Int, y: Int) {
    pearlite! {
        proof_assert!(x == (x / 2013265921) * 2013265921 + x % 2013265921);
        proof_assert!(y == (y / 2013265921) * 2013265921 + y % 2013265921);
        mul_mono(0, 0, x / 2013265921, y);
        mul_mono(0, 0, x % 2013265921, y);
        mul_mono(0, 0, x, y / 2013265921);
        mul_mono(0, 0, x, y % 2013265921);
        proof_assert!(x * y == (x / 2013265921 * y) * 2013265921 + x % 2013265921 * y);
        proof_assert!(x * y == (x * (y / 2013265921)) * 2013265921 + x * (y % 2013265921));
        mod_shift(x / 2013265921 * y, x % 2013265921 * y);
        mod_shift(x * (y / 2013265921), x * (y % 2013265921))
    }
}

/// (b mod p)^h ≡ b^h modulo p.
#[logic]
#[requires(0 <= b && 0 <= h)]
#[ensures(pw(b % 2013265921, h) % 2013265921 == pw(b, h) % 2013265921)]
#[variant(h)]
pub fn pw_mod(b: Int, h: Int) {
    pearlite! {
        if h > 0 {
            pw_mod(b, h - 1);
            pw_nonneg(b % 2013265921, h - 1);
            pw_nonneg(b, h - 1);
            mul_mod(b, pw(b % 2013265921, h - 1));
            mul_mod(b, pw(b, h - 1));
            proof_assert!(pw(b % 2013265921, h) % 2013265921 == (b * pw(b % 2013265921, h - 1)) % 2013265921);
            proof_assert!((b * pw(b % 2013265921, h - 1)) % 2013265921 == (b * (pw(b % 2013265921, h - 1) % 2013265921)) % 2013265921)
        }
    }
}

/// Un tour de carré-et-multiplie conserve r·b^e mod p, e impair : r·b^e ≡ (r·b mod p)·(b² mod p)^(e/2).
#[logic]
#[requires(0 <= r && 0 <= b && 0 < e && e % 2 == 1)]
#[ensures((r * pw(b, e)) % 2013265921 == ((r * b) % 2013265921 * pw((b * b) % 2013265921, e / 2)) % 2013265921)]
pub fn powmod_odd(r: Int, b: Int, e: Int) {
    pearlite! {
        let h = e / 2;
        let q = pw(b * b, h);
        let q2 = pw((b * b) % 2013265921, h);
        pw_add(b, 2 * h, 1);
        pw_sq(b, h);
        pw_succ(b, 0);
        mul_mono(0, 0, b, b);
        mul_mono(0, 0, r, b);
        pw_mod(b * b, h);
        pw_nonneg(b * b, h);
        pw_nonneg((b * b) % 2013265921, h);
        proof_assert!(2 * h + 1 == e);
        proof_assert!(pw(b, 0) == 1);
        proof_assert!(pw(b, 1) == b);
        proof_assert!(pw(b, e) == pw(b, 2 * h) * pw(b, 1));
        proof_assert!(pw(b, e) == q * b);
        proof_assert!(r * (q * b) == (r * b) * q);
        mul_mod(r * b, q2);
        mul_mod(r * b, q)
    }
}

/// Idem, e pair : r·b^e ≡ r·(b² mod p)^(e/2).
#[logic]
#[requires(0 <= r && 0 <= b && 0 < e && e % 2 == 0)]
#[ensures((r * pw(b, e)) % 2013265921 == (r * pw((b * b) % 2013265921, e / 2)) % 2013265921)]
pub fn powmod_even(r: Int, b: Int, e: Int) {
    pearlite! {
        let h = e / 2;
        let q = pw(b * b, h);
        let q2 = pw((b * b) % 2013265921, h);
        pw_sq(b, h);
        mul_mono(0, 0, b, b);
        pw_mod(b * b, h);
        pw_nonneg(b * b, h);
        pw_nonneg((b * b) % 2013265921, h);
        proof_assert!(2 * h == e);
        proof_assert!(r * pw(b, e) == r * q);
        mul_mod(r, q2);
        mul_mod(r, q)
    }
}

/// Exponentiation modulaire (carré-et-multiplie), b < p : le résultat est b^e mod p.
#[requires(b0@ < 2013265921)]
#[ensures(result@ < 2013265921)]
#[ensures(result@ == pw(b0@, e0@) % 2013265921)]
#[check(terminates)]
pub fn powmod(b0: u64, e0: u64) -> u64 {
    let mut b = b0;
    let mut e = e0;
    let mut r = 1u64;
    #[invariant(b@ < 2013265921 && r@ < 2013265921)]
    #[invariant((r@ * pw(b@, e@)) % 2013265921 == pw(b0@, e0@) % 2013265921)]
    #[variant(e@)]
    while e > 0 {
        proof_assert!{ mul_mono(r@, b@, 2013265920, 2013265920); mul_mono(b@, b@, 2013265920, 2013265920); true };
        if e % 2 == 1 {
            proof_assert!{ powmod_odd(r@, b@, e@);
                (r@ * pw(b@, e@)) % 2013265921 == ((r@ * b@) % 2013265921 * pw((b@ * b@) % 2013265921, e@ / 2)) % 2013265921 };
            r = r * b % P;
        } else {
            proof_assert!{ powmod_even(r@, b@, e@);
                (r@ * pw(b@, e@)) % 2013265921 == (r@ * pw((b@ * b@) % 2013265921, e@ / 2)) % 2013265921 };
        }
        b = b * b % P;
        e /= 2;
    }
    proof_assert!(pw(b@, e@) == 1 && r@ % 2013265921 == r@);
    r
}

#[requires(a@.len() == n@)]
#[ensures((^a)@.len() == n@ && below_p((^a)@))]
#[check(terminates)]
pub fn fill(a: &mut Vec<u32>, n: usize) {
    let mut i = 0usize;
    #[invariant(a@.len() == n@ && i@ <= n@)]
    #[invariant(forall<x> 0 <= x && x < i@ ==> a@[x]@ < 2013265921)]
    #[variant(n@ - i@)]
    while i < n {
        a[i] = (mix64(SEED.wrapping_add(i as u64)) % P) as u32;
        i += 1;
    }
}

/// tw[j] = w^j pour j < n/2 (tw a n/2 + 1 cases).
#[requires(w@ < 2013265921 && tw@.len() == n@ / 2 + 1)]
#[requires(forall<x> 0 <= x && x < tw@.len() ==> tw@[x]@ == 0)]
#[ensures((^tw)@.len() == tw@.len() && below_p((^tw)@))]
#[ensures(forall<j> 0 <= j && j < n@ / 2 ==> (^tw)@[j]@ == pw(w@, j) % 2013265921)]
#[check(terminates)]
pub fn twiddles(tw: &mut Vec<u32>, n: usize, w: u64) {
    let mut c = 1u64;
    let mut j = 0usize;
    #[invariant(c@ < 2013265921 && tw@.len() == n@ / 2 + 1 && j@ <= n@ / 2)]
    #[invariant(c@ == pw(w@, j@) % 2013265921)]
    #[invariant(below_p(tw@))]
    #[invariant(forall<j2> 0 <= j2 && j2 < j@ ==> tw@[j2]@ == pw(w@, j2) % 2013265921)]
    #[variant(n@ / 2 - j@)]
    while j < n / 2 {
        tw[j] = c as u32;
        proof_assert!{ mul_mono(c@, w@, 2013265920, 2013265920); true };
        // w^(j+1) = w·w^j, et (w^j mod p)·w ≡ w^j·w.
        proof_assert!{ pw_nonneg(w@, j@); mul_mod(pw(w@, j@), w@);
            (c@ * w@) % 2013265921 == pw(w@, j@ + 1) % 2013265921 };
        c = c * w % P;
        j += 1;
    }
}

/// Permutation bit-reverse sur k bits ; n = 2^k : a'[x] = a[rev_bits(x, k)].
#[requires(a@.len() == n@ && n@ == pow2(k@) && k@ <= n@)]
#[ensures((^a)@.len() == a@.len())]
#[ensures(forall<x> 0 <= x && x < n@ ==> (^a)@[x] == a@[rev_bits(x, k@)])]
#[check(terminates)]
pub fn bitrev(a: &mut Vec<u32>, n: usize, k: usize) {
    let a0 = snapshot!(a@);
    proof_assert!(forall<x> 0 <= x && x < n@ ==> {
        rev_bound(x, k@);
        rev_involutive(x, k@);
        0 <= rev_bits(x, k@) && rev_bits(x, k@) < n@ && rev_bits(rev_bits(x, k@), k@) == x
    });
    let mut i = 0usize;
    #[invariant(a@.len() == n@ && i@ <= n@)]
    // Les paires {x, rev x} déjà traitées (x < i ou rev x < i) sont échangées, les autres intactes.
    #[invariant(forall<x> 0 <= x && x < n@ ==>
        a@[x] == if x < i@ || rev_bits(x, k@) < i@ { a0[rev_bits(x, k@)] } else { a0[x] })]
    #[variant(n@ - i@)]
    while i < n {
        let mut x = i;
        let mut rev = 0usize;
        let mut b = 0usize;
        let mut bnd = 1usize;
        #[invariant(rev@ < bnd@ && bnd@ == pow2(b@) && b@ <= k@ && bnd@ <= n@ && 0 <= x@)]
        #[invariant(rev_bits(i@, k@) == rev@ * pow2(k@ - b@) + rev_bits(x@, k@ - b@))]
        #[variant(k@ - b@)]
        while b < k {
            proof_assert!{ pow2_le(b@ + 1, k@); pow2_gt(b@); true };
            proof_assert!{ rev_step(rev@, x@, k@ - b@);
                (2 * rev@ + x@ % 2) * pow2(k@ - b@ - 1) + rev_bits(x@ / 2, k@ - b@ - 1) == rev@ * pow2(k@ - b@) + rev_bits(x@, k@ - b@) };
            rev = rev * 2 + x % 2;
            x /= 2;
            bnd *= 2;
            b += 1;
        }
        proof_assert!(rev@ == rev_bits(i@, k@));
        if i < rev {
            let t = a[i];
            a[i] = a[rev];
            a[rev] = t;
        }
        i += 1;
    }
}

/// Un étage : papillons de demi-longueur `half` (len = 2 half <= n), stride = n/len.
#[requires(a@.len() == n@ && tw@.len() == n@ / 2 + 1 && 1 <= half@ && 2 * half@ <= n@ && n@ <= 134217728)]
#[requires(below_p(a@))]
#[ensures((^a)@.len() == a@.len() && below_p((^a)@))]
#[check(terminates)]
pub fn stage(a: &mut Vec<u32>, tw: &Vec<u32>, n: usize, half: usize) {
    let len = half * 2;
    let stride = n / len;
    proof_assert!(stride@ * len@ <= n@);
    proof_assert!(half@ * stride@ * 2 == len@ * stride@);
    let mut blk = 0usize;
    #[invariant(a@.len() == n@ && blk@ <= stride@ && below_p(a@))]
    #[variant(stride@ - blk@)]
    while blk < stride {
        proof_assert!{ mul_mono(blk@ + 1, len@, stride@, len@); true };
        let start = blk * len;
        let mut j = 0usize;
        #[invariant(a@.len() == n@ && j@ <= half@ && below_p(a@))]
        #[variant(half@ - j@)]
        while j < half {
            proof_assert!{ mul_mono(j@, stride@, half@ - 1, stride@); mul_mono(half@, stride@, half@, stride@); true };
            proof_assert!(j@ * stride@ <= half@ * stride@ - stride@);
            let u = a[start + j] as u64;
            let x = a[start + j + half] as u64;
            let t = tw[j * stride] as u64;
            proof_assert!{ mul_mono(x@, t@, 4294967295, 4294967295); true };
            let v = x * t % P;
            a[start + j] = ((u + v) % P) as u32;
            a[start + j + half] = ((u + P - v) % P) as u32;
            j += 1;
        }
        blk += 1;
    }
}

#[requires(a@.len() == n@ && tw@.len() == n@ / 2 + 1 && n@ == pow2(k@) && k@ <= n@ && n@ <= 134217728)]
#[requires(below_p(a@))]
#[ensures((^a)@.len() == a@.len() && below_p((^a)@))]
#[check(terminates)]
pub fn transform(a: &mut Vec<u32>, tw: &Vec<u32>, n: usize, k: usize) {
    bitrev(a, n, k);
    proof_assert!(forall<x> 0 <= x && x < n@ ==> { rev_bound(x, k@); rev_bits(x, k@) < n@ });
    let mut half = 1usize;
    #[invariant(a@.len() == n@ && 1 <= half@ && half@ <= n@ && below_p(a@))]
    #[variant(n@ + 1 - half@)]
    while half * 2 <= n {
        stage(a, tw, n, half);
        half *= 2;
    }
}

#[requires(a@.len() == n@ && n@ <= 134217728 && below_p(a@))]
#[ensures((^a)@.len() == a@.len() && below_p((^a)@))]
#[check(terminates)]
pub fn square(a: &mut Vec<u32>, n: usize) {
    let mut i = 0usize;
    #[invariant(a@.len() == n@ && i@ <= n@ && below_p(a@))]
    #[variant(n@ - i@)]
    while i < n {
        let x = a[i] as u64;
        proof_assert!{ mul_mono(x@, x@, 4294967295, 4294967295); true };
        a[i] = (x * x % P) as u32;
        i += 1;
    }
}

/// Renverse a[1..n-1] (échange a[i] et a[n-i], 1 <= i, 2i < n).
#[requires(a@.len() == n@ && 1 <= n@ && n@ <= 134217728 && below_p(a@))]
#[ensures((^a)@.len() == a@.len() && below_p((^a)@))]
#[check(terminates)]
pub fn reverse(a: &mut Vec<u32>, n: usize) {
    let mut i = 1usize;
    #[invariant(a@.len() == n@ && 1 <= i@ && 2 * i@ <= n@ + 2 && below_p(a@))]
    #[variant(n@ - i@)]
    while 2 * i < n {
        let t = a[i];
        a[i] = a[n - i];
        a[n - i] = t;
        i += 1;
    }
}

/// Repli des r[i] = a[i]·ninv mod p (chacun < p).
#[requires(a@.len() == n@ && ninv@ < 2013265921 && n@ <= 134217728)]
#[check(terminates)]
pub fn checksum(a: &Vec<u32>, n: usize, ninv: u64) -> u64 {
    let mut ck = CK_INIT;
    let mut i = 0usize;
    #[invariant(a@.len() == n@ && i@ <= n@)]
    #[variant(n@ - i@)]
    while i < n {
        let x = a[i] as u64;
        proof_assert!{ mul_mono(x@, ninv@, 4294967295, 2013265920); true };
        ck = ckfold(ck, x * ninv % P);
        i += 1;
    }
    ck
}

#[ensures(result@ >= 0)]
pub fn run(n: usize, _threads: usize) -> u64 {
    // Rejets exigés par la spécification : n > 2^27 ou n pas une puissance de 2 (n = 0 compris) donnent 0.
    if n > 134217728 {
        return 0;
    }
    let mut pw = 1usize;
    let mut k = 0usize;
    #[invariant(pw@ == pow2(k@) && (pw@ <= 2 * n@ || pw@ == 1) && k@ < pw@)]
    #[variant(2 * n@ + 1 - pw@)]
    while pw < n {
        proof_assert!{ pow2_gt(k@ + 1); true };
        pw *= 2;
        k += 1;
    }
    if pw != n {
        return 0;
    }
    let mut a = std::vec![0u32; n];
    let mut tw = std::vec![0u32; n / 2 + 1];
    fill(&mut a, n);
    let w = powmod(G, (P - 1) / (n as u64));
    twiddles(&mut tw, n, w);
    transform(&mut a, &tw, n, k);
    square(&mut a, n);
    transform(&mut a, &tw, n, k);
    reverse(&mut a, n);
    let ninv = powmod(n as u64, P - 2);
    checksum(&a, n, ninv)
}
