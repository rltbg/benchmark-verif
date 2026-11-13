//! SHA-256 d'un message de n octets généré à la volée (spec/workloads.d/sha256.md). 0 <= n <= 2^40.
//!
//! **Prouvé (Creusot)** : le condensé calculé est `sha256(gen_msg(SEED, n))`, où `sha256` est une fonction logique
//! écrite d'après FIPS 180-4 sur une séquence d'octets *quelconque* (bourrage §5.1.1, mots big-endian §5.2.1,
//! échéancier et 64 rondes §6.2.2, addition finale), et `gen_msg` le message du workload. Le checksum renvoyé par
//! `run` est le repliement `ckfold` de ce condensé logique. Plus : absence de panique, de débordement, terminaison.
//!
//! Aucun `trusted` / `assume`. Le noyau (`compress`, `digest`) est réutilisé par `parsha`.
//! En logique, les entiers machine de Creusot sont des vecteurs de bits : `+` y est l'addition modulo 2^32 et
//! `>> << & | ^ !` ont leur sens Rust. Les rotations sont écrites en décalages (`rotate_right` n'a pas de contrat
//! dans creusot-std) ; LLVM les compile en `ror`. `mix64`/`ckfold` sont redéfinies ici avec un contrat
//! (celles de `common` n'en ont pas et `common.rs` ne doit pas changer).
use crate::common::CK_INIT;
use creusot_std::prelude::*;

pub const SEED: u64 = 0x5348413235362D42;

pub const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be,
    0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa,
    0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85,
    0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
    0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f,
    0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

pub const IV: [u32; 8] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];

// =====================================================================================================
// Spécification : SHA-256 (FIPS 180-4) d'une séquence d'octets quelconque
// =====================================================================================================

/// Fonction de compression : spécification (FIPS 180-4 §6.2.2) et code, dans un module à part pour que le
/// corps de `compress_l` (64 rondes récursives) n'encombre pas les preuves de `digest`.
pub mod compression {
    use super::K;
    use creusot_std::prelude::*;

    /// État de hachage (H0..H7) ou variables de travail (a..h).
    pub type St = (u32, u32, u32, u32, u32, u32, u32, u32);

    #[logic(open)]
    pub fn st_of(s: Seq<u32>) -> St {
        (s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7])
    }

    #[logic(open)]
    pub fn rotr_l(x: u32, k: u32) -> u32 {
        (x >> k) | (x << (32u32 - k))
    }
    #[logic(open)]
    pub fn bsig0_l(x: u32) -> u32 {
        rotr_l(x, 2u32) ^ rotr_l(x, 13u32) ^ rotr_l(x, 22u32)
    }
    #[logic(open)]
    pub fn bsig1_l(x: u32) -> u32 {
        rotr_l(x, 6u32) ^ rotr_l(x, 11u32) ^ rotr_l(x, 25u32)
    }
    #[logic(open)]
    pub fn ssig0_l(x: u32) -> u32 {
        rotr_l(x, 7u32) ^ rotr_l(x, 18u32) ^ (x >> 3u32)
    }
    #[logic(open)]
    pub fn ssig1_l(x: u32) -> u32 {
        rotr_l(x, 17u32) ^ rotr_l(x, 19u32) ^ (x >> 10u32)
    }
    #[logic(open)]
    pub fn ch_l(e: u32, f: u32, g: u32) -> u32 {
        (e & f) ^ (!e & g)
    }
    #[logic(open)]
    pub fn maj_l(a: u32, b: u32, c: u32) -> u32 {
        (a & b) ^ (a & c) ^ (b & c)
    }

    /// Échéancier W_t du bloc `m` (16 mots).
    #[logic(open)]
    #[variant(t)]
    pub fn sched(m: Seq<u32>, t: Int) -> u32 {
        if t < 16 {
            m[t]
        } else {
            sched(m, t - 16) + ssig0_l(sched(m, t - 15)) + sched(m, t - 7) + ssig1_l(sched(m, t - 2))
        }
    }

    /// Une ronde (FIPS 180-4 §6.2.2, étape 3).
    #[logic(open)]
    pub fn round(s: St, k: u32, w: u32) -> St {
        let t1 = s.7 + bsig1_l(s.4) + ch_l(s.4, s.5, s.6) + k + w;
        let t2 = bsig0_l(s.0) + maj_l(s.0, s.1, s.2);
        (t1 + t2, s.0, s.1, s.2, s.3 + t1, s.4, s.5, s.6)
    }

    /// Variables de travail après les `t` premières rondes.
    #[logic(open)]
    #[variant(t)]
    pub fn rounds(h: St, m: Seq<u32>, t: Int) -> St {
        pearlite! { if t <= 0 { h } else { round(rounds(h, m, t - 1), K@[t - 1], sched(m, t - 1)) } }
    }

    /// Fonction de compression : H + rondes(H, M). Corps fermé hors de ce module : les preuves de `digest` n'ont
    /// besoin que du contrat de `compress` (contexte plus petit pour les solveurs).
    #[logic]
    pub fn compress_l(h: St, m: Seq<u32>) -> St {
        let r = rounds(h, m, 64);
        (h.0 + r.0, h.1 + r.1, h.2 + r.2, h.3 + r.3, h.4 + r.4, h.5 + r.5, h.6 + r.6, h.7 + r.7)
    }

    #[requires(0u32 < k && k < 32u32)]
    #[ensures(result == rotr_l(x, k))]
    #[check(terminates)]
    #[inline(always)]
    fn rotr(x: u32, k: u32) -> u32 {
        (x >> k) | (x << (32 - k))
    }

    #[ensures(result == bsig0_l(x))]
    #[check(terminates)]
    #[inline(always)]
    fn bsig0(x: u32) -> u32 {
        rotr(x, 2) ^ rotr(x, 13) ^ rotr(x, 22)
    }
    #[ensures(result == bsig1_l(x))]
    #[check(terminates)]
    #[inline(always)]
    fn bsig1(x: u32) -> u32 {
        rotr(x, 6) ^ rotr(x, 11) ^ rotr(x, 25)
    }
    #[ensures(result == ssig0_l(x))]
    #[check(terminates)]
    #[inline(always)]
    fn ssig0(x: u32) -> u32 {
        rotr(x, 7) ^ rotr(x, 18) ^ (x >> 3)
    }
    #[ensures(result == ssig1_l(x))]
    #[check(terminates)]
    #[inline(always)]
    fn ssig1(x: u32) -> u32 {
        rotr(x, 17) ^ rotr(x, 19) ^ (x >> 10)
    }

    /// Compression d'un bloc : `h ← compress(h, m)`.
    #[ensures(st_of((^h)@) == compress_l(st_of(h@), m@))]
    #[check(terminates)]
    #[inline(always)]
    pub fn compress(h: &mut [u32; 8], m: &[u32; 16]) {
        let mut w = [0u32; 64];
        let mut t = 0usize;
        #[invariant(t@ <= 16)]
        #[invariant(forall<i> 0 <= i && i < t@ ==> w@[i] == sched(m@, i))]
        #[variant(16 - t@)]
        while t < 16 {
            w[t] = m[t];
            t += 1;
        }
        #[invariant(16 <= t@ && t@ <= 64)]
        #[invariant(forall<i> 0 <= i && i < t@ ==> w@[i] == sched(m@, i))]
        #[variant(64 - t@)]
        while t < 64 {
            w[t] = w[t - 16].wrapping_add(ssig0(w[t - 15])).wrapping_add(w[t - 7]).wrapping_add(ssig1(w[t - 2]));
            t += 1;
        }
        let h0 = snapshot!(st_of(h@));
        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];
        let mut f = h[5];
        let mut g = h[6];
        let mut hh = h[7];
        t = 0;
        #[invariant(t@ <= 64)]
        #[invariant((a, b, c, d, e, f, g, hh) == rounds(*h0, m@, t@))]
        #[variant(64 - t@)]
        while t < 64 {
            let t1 = hh.wrapping_add(bsig1(e)).wrapping_add((e & f) ^ (!e & g)).wrapping_add(K[t]).wrapping_add(w[t]);
            let t2 = bsig0(a).wrapping_add((a & b) ^ (a & c) ^ (b & c));
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
            t += 1;
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }
}
pub use compression::{St, compress};
#[cfg(creusot)]
pub use compression::{compress_l, st_of};

/// Mot big-endian de 4 octets.
#[logic(open)]
pub fn be_word(b0: u8, b1: u8, b2: u8, b3: u8) -> u32 {
    ((b0 as u32) << 24u32) | ((b1 as u32) << 16u32) | ((b2 as u32) << 8u32) | (b3 as u32)
}

/// Nombre de blocs du message bourré (`len` octets + 0x80 + zéros + 8 octets de longueur).
#[logic(open)]
pub fn nblocks(len: Int) -> Int {
    len / 64 + if len % 64 < 56 { 1 } else { 2 }
}

/// Conversion `Int -> u64` : Creusot n'en fournit pas en logique, on la définit par récurrence. Le corps est
/// fermé hors du sous-module (une définition récursive ouverte fait boucler l'instanciation des solveurs) ;
/// le seul fait exporté est le lemme `u64_of_view` : `u64_of` est l'inverse de la vue `@`.
pub mod conv {
    use creusot_std::prelude::*;

    #[logic]
    #[variant(x)]
    pub fn u64_of(x: Int) -> u64 {
        if x <= 0 { 0u64 } else { u64_of(x - 1) + 1u64 }
    }

    #[logic]
    #[variant(v@)]
    #[ensures(u64_of(v@) == v)]
    pub fn u64_of_view(v: u64) {
        pearlite! { if v@ > 0 { u64_of_view(v - 1u64) } else { () } }
    }
}
#[cfg(creusot)]
pub use conv::{u64_of, u64_of_view};

/// Longueur du message en bits, sur 64 bits.
#[logic(open)]
pub fn len_bits(msg: Seq<u8>) -> u64 {
    u64_of(msg.len() * 8)
}

/// Octet `i` du message bourré hors champ de longueur : message, puis 0x80, puis des zéros.
#[logic(open)]
pub fn pad_byte(msg: Seq<u8>, i: Int) -> u8 {
    if i < msg.len() { msg[i] } else if i == msg.len() { 0x80u8 } else { 0u8 }
}

/// Mot `j` du message bourré ; les deux derniers mots sont la longueur en bits (64 bits big-endian).
#[logic(open)]
pub fn pad_word(msg: Seq<u8>, j: Int) -> u32 {
    let last = 16 * nblocks(msg.len());
    if j == last - 2 {
        (len_bits(msg) >> 32u64) as u32
    } else if j == last - 1 {
        len_bits(msg) as u32
    } else {
        be_word(pad_byte(msg, 4 * j), pad_byte(msg, 4 * j + 1), pad_byte(msg, 4 * j + 2), pad_byte(msg, 4 * j + 3))
    }
}

/// Bloc `b` (16 mots) du message bourré.
#[logic(open)]
pub fn block(msg: Seq<u8>, b: Int) -> Seq<u32> {
    Seq::create(16, |j| pad_word(msg, 16 * b + j))
}

/// État après les `k` premiers blocs.
#[logic(open)]
#[variant(k)]
pub fn sha_state(msg: Seq<u8>, k: Int) -> St {
    pearlite! { if k <= 0 { st_of(IV@) } else { compress_l(sha_state(msg, k - 1), block(msg, k - 1)) } }
}

/// SHA-256(msg).
#[logic(open)]
pub fn sha256(msg: Seq<u8>) -> St {
    sha_state(msg, nblocks(msg.len()))
}

// ---------------- message du workload et checksum ----------------

#[logic(open)]
pub fn mix64_l(x: u64) -> u64 {
    let z = x + 0x9E3779B97F4A7C15u64;
    let z = (z ^ (z >> 30u64)) * 0xBF58476D1CE4E5B9u64;
    let z = (z ^ (z >> 27u64)) * 0x94D049BB133111EBu64;
    z ^ (z >> 31u64)
}

/// Octet numéro `q` (0..7, petit-boutien) de `x`.
#[logic(open)]
pub fn byte_le(x: u64, q: Int) -> u8 {
    if q == 0 {
        x as u8
    } else if q == 1 {
        (x >> 8u64) as u8
    } else if q == 2 {
        (x >> 16u64) as u8
    } else if q == 3 {
        (x >> 24u64) as u8
    } else if q == 4 {
        (x >> 32u64) as u8
    } else if q == 5 {
        (x >> 40u64) as u8
    } else if q == 6 {
        (x >> 48u64) as u8
    } else {
        (x >> 56u64) as u8
    }
}

/// Octet `i` du message de graine `base` : octet `i mod 8` (petit-boutien) de `mix64(base + i/8)`.
#[logic(open)]
pub fn msg_byte(base: u64, i: Int) -> u8 {
    byte_le(mix64_l(base + u64_of(i / 8)), i % 8)
}

/// Les `n` premiers octets du message de graine `base`.
#[logic(open)]
pub fn gen_msg(base: u64, n: Int) -> Seq<u8> {
    Seq::create(n, |i| msg_byte(base, i))
}

#[logic(open)]
pub fn ckfold_l(ck: u64, v: u64) -> u64 {
    (ck ^ v) * 0x100000001B3u64
}

/// Checksum du workload : repliement des 8 mots du condensé.
#[logic(open)]
pub fn ck_digest(ck: u64, h: St) -> u64 {
    let ck = ckfold_l(ck, h.0 as u64);
    let ck = ckfold_l(ck, h.1 as u64);
    let ck = ckfold_l(ck, h.2 as u64);
    let ck = ckfold_l(ck, h.3 as u64);
    let ck = ckfold_l(ck, h.4 as u64);
    let ck = ckfold_l(ck, h.5 as u64);
    let ck = ckfold_l(ck, h.6 as u64);
    ckfold_l(ck, h.7 as u64)
}

// =====================================================================================================
// Lemmes
// =====================================================================================================

#[logic(open)]
pub fn bswap_l(x: u32) -> u32 {
    ((x & 0xffu32) << 24u32) | ((x & 0xff00u32) << 8u32) | ((x >> 8u32) & 0xff00u32) | (x >> 24u32)
}

/// Un mot relu octet par octet en big-endian est lui-même.
#[logic]
#[bitwise_proof]
#[ensures(be_word((w >> 24u32) as u8, (w >> 16u32) as u8, (w >> 8u32) as u8, w as u8) == w)]
pub fn be_word_split(w: u32) {}

/// Les 4 octets petit-boutiens de poids faible (resp. fort) de `x`, lus en big-endian, donnent `bswap` de la moitié.
#[logic]
#[bitwise_proof]
#[ensures(be_word(x as u8, (x >> 8u64) as u8, (x >> 16u64) as u8, (x >> 24u64) as u8) == bswap_l(x as u32))]
#[ensures(be_word((x >> 32u64) as u8, (x >> 40u64) as u8, (x >> 48u64) as u8, (x >> 56u64) as u8)
    == bswap_l((x >> 32u64) as u32))]
pub fn le_bytes_bswap(x: u64) {}

/// Décaler de `s = 8q` puis tronquer, c'est prendre l'octet petit-boutien `q`.
#[logic]
#[bitwise_proof]
#[requires(s@ % 8 == 0 && s@ < 64)]
#[ensures((x >> s) as u8 == byte_le(x, s@ / 8))]
pub fn byte_le_shift(x: u64, s: u64) {
    pearlite! {
        if s@ == 0 { proof_assert!(s == 0u64) }
        else if s@ == 8 { proof_assert!(s == 8u64) }
        else if s@ == 16 { proof_assert!(s == 16u64) }
        else if s@ == 24 { proof_assert!(s == 24u64) }
        else if s@ == 32 { proof_assert!(s == 32u64) }
        else if s@ == 40 { proof_assert!(s == 40u64) }
        else if s@ == 48 { proof_assert!(s == 48u64) }
        else { proof_assert!(s == 56u64) }
    }
}

/// Mot `w` du message brut (sans bourrage), lu en big-endian.
#[logic(open)]
pub fn msg_word(base: u64, w: Int) -> u32 {
    be_word(msg_byte(base, 4 * w), msg_byte(base, 4 * w + 1), msg_byte(base, 4 * w + 2), msg_byte(base, 4 * w + 3))
}

/// Les deux mots big-endian issus de `mix64(base + w)` (huit octets 8w..8w+7 du message).
#[logic]
#[requires(w@ < 0x1000000000000000)]
#[ensures(msg_word(base, 2 * w@) == bswap_l(mix64_l(base + w) as u32))]
#[ensures(msg_word(base, 2 * w@ + 1) == bswap_l((mix64_l(base + w) >> 32u64) as u32))]
pub fn msg_word_mix(base: u64, w: u64) {
    pearlite! {
        let x = mix64_l(base + w);
        u64_of_view(w);
        le_bytes_bswap(x);
        proof_assert!(forall<r> 0 <= r && r < 8 ==> (8 * w@ + r) / 8 == w@ && (8 * w@ + r) % 8 == r);
        proof_assert!(forall<r> 0 <= r && r < 8 ==> mix64_l(base + u64_of((8 * w@ + r) / 8)) == x);
        proof_assert!(msg_byte(base, 8 * w@) == byte_le(x, 0));
        proof_assert!(msg_byte(base, 8 * w@ + 1) == byte_le(x, 1));
        proof_assert!(msg_byte(base, 8 * w@ + 2) == byte_le(x, 2));
        proof_assert!(msg_byte(base, 8 * w@ + 3) == byte_le(x, 3));
        proof_assert!(msg_byte(base, 8 * w@ + 4) == byte_le(x, 4));
        proof_assert!(msg_byte(base, 8 * w@ + 5) == byte_le(x, 5));
        proof_assert!(msg_byte(base, 8 * w@ + 6) == byte_le(x, 6));
        proof_assert!(msg_byte(base, 8 * w@ + 7) == byte_le(x, 7))
    }
}

/// Les mots entièrement contenus dans le message sont des mots du message brut.
#[logic]
#[requires(0 <= n)]
#[ensures(forall<j> 0 <= j && 4 * j + 4 <= n ==> pad_word(gen_msg(base, n), j) == msg_word(base, j))]
pub fn pad_word_in_msg(base: u64, n: Int) {}

// =====================================================================================================
// Code exécutable
// =====================================================================================================

#[ensures(result == mix64_l(x))]
#[check(terminates)]
#[inline(always)]
pub fn mix64(x: u64) -> u64 {
    let z = x.wrapping_add(0x9E3779B97F4A7C15);
    let z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    let z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

#[ensures(result == ckfold_l(ck, v))]
#[check(terminates)]
#[inline(always)]
pub fn ckfold(ck: u64, v: u64) -> u64 {
    (ck ^ v).wrapping_mul(0x100000001B3)
}

#[ensures(result == bswap_l(x))]
#[check(terminates)]
#[inline(always)]
fn bswap(x: u32) -> u32 {
    ((x & 0xff) << 24) | ((x & 0xff00) << 8) | ((x >> 8) & 0xff00) | (x >> 24)
}

/// Octet `i` du message de graine `base`.
#[ensures(result == msg_byte(base, i@))]
#[check(terminates)]
pub fn byte_at(base: u64, i: u64) -> u8 {
    let w = i / 8;
    let s = 8 * (i % 8);
    let x = mix64(base.wrapping_add(w));
    proof_assert! { u64_of_view(w); byte_le_shift(x, s); true };
    (x >> s) as u8
}

/// Bloc complet `b` du message : huit appels à mix64, chacun donnant deux mots big-endian.
#[requires(b@ < 0x1000000000000)]
#[ensures(forall<j> 0 <= j && j < 16 ==> (^m)@[j] == msg_word(base, 16 * b@ + j))]
#[check(terminates)]
#[inline(always)]
pub fn load_block(base: u64, b: u64, m: &mut [u32; 16]) {
    let mut k = 0usize;
    #[invariant(k@ <= 8)]
    #[invariant(forall<j> 0 <= j && j < 2 * k@ ==> m@[j] == msg_word(base, 16 * b@ + j))]
    #[variant(8 - k@)]
    while k < 8 {
        let w = b * 8 + k as u64;
        let x = mix64(base.wrapping_add(w));
        proof_assert! { msg_word_mix(base, w); true };
        m[2 * k] = bswap(x as u32);
        m[2 * k + 1] = bswap((x >> 32) as u32);
        k += 1;
    }
}

/// Mot big-endian des octets `buf[i..i+4]`.
#[requires(i@ + 4 <= 128)]
#[ensures(result == be_word(buf@[i@], buf@[i@ + 1], buf@[i@ + 2], buf@[i@ + 3]))]
#[check(terminates)]
#[inline(always)]
fn load_be32(buf: &[u8; 128], i: usize) -> u32 {
    ((buf[i] as u32) << 24) | ((buf[i + 1] as u32) << 16) | ((buf[i + 2] as u32) << 8) | (buf[i + 3] as u32)
}

/// Écrit `w` en big-endian dans `buf[i..i+4]`, sans toucher au reste.
#[requires(i@ + 4 <= 128)]
#[ensures(be_word((^buf)@[i@], (^buf)@[i@ + 1], (^buf)@[i@ + 2], (^buf)@[i@ + 3]) == w)]
#[ensures(forall<x> 0 <= x && x < 128 && (x < i@ || i@ + 4 <= x) ==> (^buf)@[x] == buf@[x])]
#[check(terminates)]
#[inline(always)]
fn store_be32(buf: &mut [u8; 128], i: usize, w: u32) {
    proof_assert! { be_word_split(w); true };
    buf[i] = (w >> 24) as u8;
    buf[i + 1] = (w >> 16) as u8;
    buf[i + 2] = (w >> 8) as u8;
    buf[i + 3] = w as u8;
}

/// Bourrage : écrit dans `tail` (nul au départ) les `n mod 64` derniers octets du message, 0x80 et la longueur en
/// bits ; renvoie le nombre d'octets à compresser (64 ou 128). Les mots big-endian de `tail` sont ceux du
/// message bourré à partir du bloc `n / 64`.
#[requires(n@ <= 1099511627776)]
#[requires(forall<x> 0 <= x && x < 128 ==> tail@[x] == 0u8)]
#[ensures(result@ == 64 || result@ == 128)]
#[ensures(n@ / 64 + result@ / 64 == nblocks(n@))]
#[ensures(forall<y> 0 <= y && 4 * y < result@ ==>
    be_word((^tail)@[4 * y], (^tail)@[4 * y + 1], (^tail)@[4 * y + 2], (^tail)@[4 * y + 3])
        == pad_word(gen_msg(base, n@), 16 * (n@ / 64) + y))]
#[check(terminates)]
#[inline(always)]
fn pad_tail(base: u64, n: usize, tail: &mut [u8; 128]) -> usize {
    let msg = snapshot!(gen_msg(base, n@));
    let t = n % 64;
    let start = (n - t) as u64;
    let mut j = 0usize;
    #[invariant(j@ <= t@)]
    #[invariant(forall<x> 0 <= x && x < j@ ==> tail@[x] == msg_byte(base, start@ + x))]
    #[invariant(forall<x> j@ <= x && x < 128 ==> tail@[x] == 0u8)]
    #[variant(t@ - j@)]
    while j < t {
        tail[j] = byte_at(base, start + j as u64);
        j += 1;
    }
    tail[t] = 0x80;
    let total = if t < 56 { 64usize } else { 128usize };
    let bits = (n as u64) * 8;
    store_be32(tail, total - 8, (bits >> 32) as u32);
    store_be32(tail, total - 4, bits as u32);
    proof_assert!(msg.len() == n@ && start@ + t@ == n@ && start@ == 64 * (n@ / 64));
    proof_assert!(nblocks(n@) == n@ / 64 + total@ / 64 && t@ + 9 <= total@);
    proof_assert!(forall<x> 0 <= x && x < t@ ==> tail@[x] == pad_byte(*msg, start@ + x));
    proof_assert!(tail@[t@] == pad_byte(*msg, start@ + t@));
    proof_assert!(forall<x> t@ < x && x < total@ - 8 ==> tail@[x] == pad_byte(*msg, start@ + x));
    proof_assert!(forall<y, r> 0 <= y && 0 <= r && r < 4 && 4 * y + r < total@ - 8
        ==> tail@[4 * y + r] == pad_byte(*msg, 4 * (16 * (n@ / 64) + y) + r));
    proof_assert! { u64_of_view(bits); len_bits(*msg) == bits };
    total
}

/// Condensé SHA-256 des `n` premiers octets du message de graine `base`.
#[requires(n@ <= 1099511627776)]
#[ensures(st_of(result@) == sha256(gen_msg(base, n@)))]
#[check(terminates)]
pub fn digest(base: u64, n: usize) -> [u32; 8] {
    let msg = snapshot!(gen_msg(base, n@));
    proof_assert! { pad_word_in_msg(base, n@); true };
    let mut h = IV;
    let blocks = (n / 64) as u64;
    let mut m = [0u32; 16];
    let mut b = 0u64;
    #[invariant(b@ <= blocks@ && h@.len() == 8)]
    #[invariant(st_of(h@) == sha_state(*msg, b@))]
    #[variant(blocks@ - b@)]
    while b < blocks {
        load_block(base, b, &mut m);
        proof_assert!(m@.ext_eq(block(*msg, b@)));
        compress(&mut h, &m);
        b += 1;
    }
    // Reste : n mod 64 octets, 0x80, zéros et longueur en bits (1 ou 2 blocs).
    let mut tail = [0u8; 128];
    let total = pad_tail(base, n, &mut tail);
    let mut off = 0usize;
    #[invariant(off@ % 64 == 0 && off@ <= total@ && h@.len() == 8)]
    #[invariant(st_of(h@) == sha_state(*msg, b@ + off@ / 64))]
    #[variant(total@ - off@)]
    while off < total {
        let mut i = 0usize;
        #[invariant(i@ <= 16)]
        #[invariant(forall<y> 0 <= y && y < i@ ==> m@[y] == pad_word(*msg, 16 * (b@ + off@ / 64) + y))]
        #[variant(16 - i@)]
        while i < 16 {
            proof_assert!(16 * (b@ + off@ / 64) + i@ == 16 * b@ + (off@ + 4 * i@) / 4 && (off@ + 4 * i@) % 4 == 0);
            m[i] = load_be32(&tail, off + 4 * i);
            i += 1;
        }
        proof_assert!(m@.ext_eq(block(*msg, b@ + off@ / 64)));
        compress(&mut h, &m);
        off += 64;
    }
    h
}

/// Checksum : repliement des mots H0..H7 du condensé.
#[requires(n@ <= 1099511627776)]
#[ensures(result == ck_digest(CK_INIT, sha256(gen_msg(SEED, n@))))]
#[check(terminates)]
pub fn run(n: usize, _threads: usize) -> u64 {
    let h = digest(SEED, n);
    let mut ck = CK_INIT;
    ck = ckfold(ck, h[0] as u64);
    ck = ckfold(ck, h[1] as u64);
    ck = ckfold(ck, h[2] as u64);
    ck = ckfold(ck, h[3] as u64);
    ck = ckfold(ck, h[4] as u64);
    ck = ckfold(ck, h[5] as u64);
    ck = ckfold(ck, h[6] as u64);
    ckfold(ck, h[7] as u64)
}
