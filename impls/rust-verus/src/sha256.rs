//! SHA-256 d'un message de n octets généré à la volée (spec/workloads.d/sha256.md). 0 <= n <= 2^40.
//!
//! **Prouvé (Verus)** : le condensé calculé est `sha256(gen_msg(SEED, n))`, où `sha256` est une fonction `spec`
//! écrite d'après FIPS 180-4 sur une séquence d'octets *quelconque* (bourrage §5.1.1, mots big-endian §5.2.1,
//! échéancier et 64 rondes §6.2.2, addition finale) et `gen_msg` le message du workload ; le checksum renvoyé par
//! `run` est le repliement `spec_ckfold` de ce condensé. Plus : absence de panique et de débordement, terminaison.
//!
//! Aucun `assume` / `admit` / `external_body`. Le noyau (`compress`, `digest`) est réutilisé par `parsha`.
//! Les égalités octets ↔ mots (big-endian, `bswap`) sont prouvées en mode `bit_vector`. Les rotations sont écrites
//! en décalages, comme dans la spécification ; LLVM les compile en `ror`.
use crate::common::*;
use vstd::prelude::*;
#[cfg(verus_only)]
use vstd::arithmetic::div_mod::lemma_fundamental_div_mod_converse;

verus! {

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

/// État de hachage (H0..H7) ou variables de travail (a..h).
pub type St = (u32, u32, u32, u32, u32, u32, u32, u32);

pub open spec fn st_of(s: Seq<u32>) -> St {
    (s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7])
}

pub open spec fn rotr_spec(x: u32, k: u32) -> u32 {
    (x >> k) | (x << (32 - k) as u32)
}

pub open spec fn bsig0_spec(x: u32) -> u32 {
    rotr_spec(x, 2) ^ rotr_spec(x, 13) ^ rotr_spec(x, 22)
}

pub open spec fn bsig1_spec(x: u32) -> u32 {
    rotr_spec(x, 6) ^ rotr_spec(x, 11) ^ rotr_spec(x, 25)
}

pub open spec fn ssig0_spec(x: u32) -> u32 {
    rotr_spec(x, 7) ^ rotr_spec(x, 18) ^ (x >> 3)
}

pub open spec fn ssig1_spec(x: u32) -> u32 {
    rotr_spec(x, 17) ^ rotr_spec(x, 19) ^ (x >> 10)
}

pub open spec fn ch_spec(e: u32, f: u32, g: u32) -> u32 {
    (e & f) ^ (!e & g)
}

pub open spec fn maj_spec(a: u32, b: u32, c: u32) -> u32 {
    (a & b) ^ (a & c) ^ (b & c)
}

/// Échéancier W_t du bloc `m` (16 mots).
pub open spec fn sched(m: Seq<u32>, t: int) -> u32
    decreases t,
{
    if t < 16 {
        m[t]
    } else {
        sched(m, t - 16).wrapping_add(ssig0_spec(sched(m, t - 15))).wrapping_add(sched(m, t - 7)).wrapping_add(
            ssig1_spec(sched(m, t - 2)),
        )
    }
}

/// Une ronde (FIPS 180-4 §6.2.2, étape 3).
pub open spec fn round(s: St, k: u32, w: u32) -> St {
    let t1 = s.7.wrapping_add(bsig1_spec(s.4)).wrapping_add(ch_spec(s.4, s.5, s.6)).wrapping_add(k).wrapping_add(w);
    let t2 = bsig0_spec(s.0).wrapping_add(maj_spec(s.0, s.1, s.2));
    (t1.wrapping_add(t2), s.0, s.1, s.2, s.3.wrapping_add(t1), s.4, s.5, s.6)
}

/// Variables de travail après les `t` premières rondes.
pub open spec fn rounds(h: St, m: Seq<u32>, t: int) -> St
    decreases t,
{
    if t <= 0 {
        h
    } else {
        round(rounds(h, m, t - 1), K@[t - 1], sched(m, t - 1))
    }
}

/// Fonction de compression : H + rondes(H, M).
pub open spec fn compress_spec(h: St, m: Seq<u32>) -> St {
    let r = rounds(h, m, 64);
    (
        h.0.wrapping_add(r.0),
        h.1.wrapping_add(r.1),
        h.2.wrapping_add(r.2),
        h.3.wrapping_add(r.3),
        h.4.wrapping_add(r.4),
        h.5.wrapping_add(r.5),
        h.6.wrapping_add(r.6),
        h.7.wrapping_add(r.7),
    )
}

/// Mot big-endian de 4 octets.
pub open spec fn be_word(b0: u8, b1: u8, b2: u8, b3: u8) -> u32 {
    ((b0 as u32) << 24) | ((b1 as u32) << 16) | ((b2 as u32) << 8) | (b3 as u32)
}

/// Mot big-endian numéro `y` d'un tampon d'octets.
pub open spec fn word_at(s: Seq<u8>, y: int) -> u32 {
    be_word(s[4 * y], s[4 * y + 1], s[4 * y + 2], s[4 * y + 3])
}

/// Nombre de blocs du message bourré (`len` octets + 0x80 + zéros + 8 octets de longueur).
pub open spec fn nblocks(len: int) -> int {
    len / 64 + if len % 64 < 56 { 1int } else { 2int }
}

/// Longueur du message en bits, sur 64 bits.
pub open spec fn len_bits(msg: Seq<u8>) -> u64 {
    (msg.len() * 8) as u64
}

/// Octet `i` du message bourré hors champ de longueur : message, puis 0x80, puis des zéros.
pub open spec fn pad_byte(msg: Seq<u8>, i: int) -> u8 {
    if i < msg.len() {
        msg[i]
    } else if i == msg.len() {
        0x80u8
    } else {
        0u8
    }
}

/// Mot `j` du message bourré ; les deux derniers mots sont la longueur en bits (64 bits big-endian).
pub open spec fn pad_word(msg: Seq<u8>, j: int) -> u32 {
    let last = 16 * nblocks(msg.len() as int);
    if j == last - 2 {
        (len_bits(msg) >> 32) as u32
    } else if j == last - 1 {
        len_bits(msg) as u32
    } else {
        be_word(pad_byte(msg, 4 * j), pad_byte(msg, 4 * j + 1), pad_byte(msg, 4 * j + 2), pad_byte(msg, 4 * j + 3))
    }
}

/// Bloc `b` (16 mots) du message bourré.
pub open spec fn block(msg: Seq<u8>, b: int) -> Seq<u32> {
    Seq::new(16, |j: int| pad_word(msg, 16 * b + j))
}

/// État après les `k` premiers blocs.
pub open spec fn sha_state(msg: Seq<u8>, k: int) -> St
    decreases k,
{
    if k <= 0 {
        st_of(IV@)
    } else {
        compress_spec(sha_state(msg, k - 1), block(msg, k - 1))
    }
}

/// SHA-256(msg).
pub open spec fn sha256(msg: Seq<u8>) -> St {
    sha_state(msg, nblocks(msg.len() as int))
}

// ---------------- message du workload et checksum ----------------

/// Octet `i` du message de graine `base` : octet `i mod 8` (petit-boutien) de `mix64(base + i/8)`.
pub open spec fn msg_byte(base: u64, i: int) -> u8 {
    (spec_mix64(base.wrapping_add((i / 8) as u64)) >> ((8 * (i % 8)) as u64)) as u8
}

/// Les `n` premiers octets du message de graine `base`.
pub open spec fn gen_msg(base: u64, n: int) -> Seq<u8> {
    Seq::new(n as nat, |i: int| msg_byte(base, i))
}

/// Checksum du workload : repliement des 8 mots du condensé.
pub open spec fn ck_digest(ck: u64, h: St) -> u64 {
    let ck = spec_ckfold(ck, h.0 as u64);
    let ck = spec_ckfold(ck, h.1 as u64);
    let ck = spec_ckfold(ck, h.2 as u64);
    let ck = spec_ckfold(ck, h.3 as u64);
    let ck = spec_ckfold(ck, h.4 as u64);
    let ck = spec_ckfold(ck, h.5 as u64);
    let ck = spec_ckfold(ck, h.6 as u64);
    spec_ckfold(ck, h.7 as u64)
}

// =====================================================================================================
// Lemmes
// =====================================================================================================

pub open spec fn bswap_spec(x: u32) -> u32 {
    ((x & 0xff) << 24) | ((x & 0xff00) << 8) | ((x >> 8) & 0xff00) | (x >> 24)
}

/// Un mot relu octet par octet en big-endian est lui-même.
pub proof fn lemma_be_word_split(w: u32)
    ensures
        be_word((w >> 24) as u8, (w >> 16) as u8, (w >> 8) as u8, w as u8) == w,
{
    assert(be_word((w >> 24) as u8, (w >> 16) as u8, (w >> 8) as u8, w as u8) == w) by (bit_vector);
}

/// Les octets petit-boutiens de `x`, lus par 4 en big-endian, donnent `bswap` de chaque moitié.
pub proof fn lemma_le_bytes_bswap(x: u64)
    ensures
        be_word(x as u8, (x >> 8) as u8, (x >> 16) as u8, (x >> 24) as u8) == bswap_spec(x as u32),
        be_word((x >> 32) as u8, (x >> 40) as u8, (x >> 48) as u8, (x >> 56) as u8) == bswap_spec((x >> 32) as u32),
{
    assert(be_word(x as u8, (x >> 8) as u8, (x >> 16) as u8, (x >> 24) as u8) == bswap_spec(x as u32))
        by (bit_vector);
    assert(be_word((x >> 32) as u8, (x >> 40) as u8, (x >> 48) as u8, (x >> 56) as u8) == bswap_spec(
        (x >> 32) as u32,
    )) by (bit_vector);
}

/// Mot `w` du message brut (sans bourrage), lu en big-endian.
pub open spec fn msg_word(base: u64, w: int) -> u32 {
    be_word(msg_byte(base, 4 * w), msg_byte(base, 4 * w + 1), msg_byte(base, 4 * w + 2), msg_byte(base, 4 * w + 3))
}

/// Les deux mots big-endian issus de `mix64(base + w)` (octets 8w..8w+7 du message).
pub proof fn lemma_msg_word_mix(base: u64, w: u64)
    requires
        w < 0x1000_0000_0000_0000,
    ensures
        msg_word(base, 2 * w) == bswap_spec(spec_mix64(base.wrapping_add(w)) as u32),
        msg_word(base, 2 * w + 1) == bswap_spec((spec_mix64(base.wrapping_add(w)) >> 32) as u32),
{
    let x = spec_mix64(base.wrapping_add(w));
    lemma_le_bytes_bswap(x);
    assert forall|r: int| 0 <= r < 8 implies #[trigger] msg_byte(base, 8 * w + r) == (x >> ((8 * r) as u64)) as u8 by {
        lemma_fundamental_div_mod_converse(8 * w + r, 8, w as int, r);
    }
    assert((x >> 0u64) == x) by (bit_vector);
    assert(msg_byte(base, 8 * w + 0) == x as u8);
    assert(msg_byte(base, 8 * w + 1) == (x >> 8) as u8);
    assert(msg_byte(base, 8 * w + 2) == (x >> 16) as u8);
    assert(msg_byte(base, 8 * w + 3) == (x >> 24) as u8);
    assert(msg_byte(base, 8 * w + 4) == (x >> 32) as u8);
    assert(msg_byte(base, 8 * w + 5) == (x >> 40) as u8);
    assert(msg_byte(base, 8 * w + 6) == (x >> 48) as u8);
    assert(msg_byte(base, 8 * w + 7) == (x >> 56) as u8);
    assert(msg_word(base, 2 * w) == be_word(x as u8, (x >> 8) as u8, (x >> 16) as u8, (x >> 24) as u8));
    assert(msg_word(base, 2 * w + 1) == be_word((x >> 32) as u8, (x >> 40) as u8, (x >> 48) as u8, (x >> 56) as u8));
}

/// Les mots entièrement contenus dans le message sont des mots du message brut.
pub proof fn lemma_pad_word_in_msg(base: u64, n: int)
    requires
        0 <= n,
    ensures
        forall|j: int| 0 <= j && 4 * j + 4 <= n ==> #[trigger] pad_word(gen_msg(base, n), j) == msg_word(base, j),
{
}

// =====================================================================================================
// Code exécutable
// =====================================================================================================

#[inline(always)]
fn rotr(x: u32, k: u32) -> (r: u32)
    requires
        0 < k < 32,
    ensures
        r == rotr_spec(x, k),
{
    (x >> k) | (x << (32 - k))
}

#[inline(always)]
fn bsig0(x: u32) -> (r: u32)
    ensures
        r == bsig0_spec(x),
{
    rotr(x, 2) ^ rotr(x, 13) ^ rotr(x, 22)
}

#[inline(always)]
fn bsig1(x: u32) -> (r: u32)
    ensures
        r == bsig1_spec(x),
{
    rotr(x, 6) ^ rotr(x, 11) ^ rotr(x, 25)
}

#[inline(always)]
fn ssig0(x: u32) -> (r: u32)
    ensures
        r == ssig0_spec(x),
{
    rotr(x, 7) ^ rotr(x, 18) ^ (x >> 3)
}

#[inline(always)]
fn ssig1(x: u32) -> (r: u32)
    ensures
        r == ssig1_spec(x),
{
    rotr(x, 17) ^ rotr(x, 19) ^ (x >> 10)
}

#[inline(always)]
fn bswap(x: u32) -> (r: u32)
    ensures
        r == bswap_spec(x),
{
    ((x & 0xff) << 24) | ((x & 0xff00) << 8) | ((x >> 8) & 0xff00) | (x >> 24)
}

/// Compression d'un bloc : `h ← compress(h, m)`.
#[inline(always)]
pub fn compress(h: &mut [u32; 8], m: &[u32; 16])
    ensures
        st_of(final(h)@) == compress_spec(st_of(old(h)@), m@),
{
    let mut w = [0u32; 64];
    let mut t = 0usize;
    while t < 16
        invariant
            t <= 16,
            forall|i: int| 0 <= i < t ==> w@[i] == sched(m@, i),
        decreases 16 - t,
    {
        w[t] = m[t];
        t += 1;
    }
    while t < 64
        invariant
            16 <= t <= 64,
            forall|i: int| 0 <= i < t ==> w@[i] == sched(m@, i),
        decreases 64 - t,
    {
        w[t] = w[t - 16].wrapping_add(ssig0(w[t - 15])).wrapping_add(w[t - 7]).wrapping_add(ssig1(w[t - 2]));
        t += 1;
    }
    let ghost h0 = st_of(h@);
    let mut a = h[0];
    let mut b = h[1];
    let mut c = h[2];
    let mut d = h[3];
    let mut e = h[4];
    let mut f = h[5];
    let mut g = h[6];
    let mut hh = h[7];
    t = 0;
    while t < 64
        invariant
            t <= 64,
            h0 == st_of(old(h)@),
            forall|i: int| 0 <= i < 64 ==> w@[i] == sched(m@, i),
            (a, b, c, d, e, f, g, hh) == rounds(h0, m@, t as int),
        decreases 64 - t,
    {
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

/// Octet `i` du message de graine `base`.
pub fn byte_at(base: u64, i: u64) -> (r: u8)
    ensures
        r == msg_byte(base, i as int),
{
    (mix64(base.wrapping_add(i / 8)) >> (8 * (i % 8))) as u8
}

/// Bloc complet `b` du message : huit appels à mix64, chacun donnant deux mots big-endian.
#[inline(always)]
pub fn load_block(base: u64, b: u64, m: &mut [u32; 16])
    requires
        b < 0x1_0000_0000_0000,
    ensures
        forall|j: int| 0 <= j < 16 ==> final(m)@[j] == msg_word(base, 16 * b + j),
{
    let mut k = 0usize;
    while k < 8
        invariant
            k <= 8,
            b < 0x1_0000_0000_0000,
            forall|j: int| 0 <= j < 2 * k ==> m@[j] == msg_word(base, 16 * b + j),
        decreases 8 - k,
    {
        let w = b * 8 + k as u64;
        let x = mix64(base.wrapping_add(w));
        proof {
            lemma_msg_word_mix(base, w);
        }
        m[2 * k] = bswap(x as u32);
        m[2 * k + 1] = bswap((x >> 32) as u32);
        k += 1;
    }
}

/// Mot big-endian des octets `buf[i..i+4]`.
#[inline(always)]
fn load_be32(buf: &[u8; 128], i: usize) -> (r: u32)
    requires
        i + 4 <= 128,
    ensures
        r == be_word(buf@[i as int], buf@[i + 1], buf@[i + 2], buf@[i + 3]),
{
    ((buf[i] as u32) << 24) | ((buf[i + 1] as u32) << 16) | ((buf[i + 2] as u32) << 8) | (buf[i + 3] as u32)
}

/// Écrit `w` en big-endian dans `buf[i..i+4]`, sans toucher au reste.
#[inline(always)]
fn store_be32(buf: &mut [u8; 128], i: usize, w: u32)
    requires
        i + 4 <= 128,
    ensures
        be_word(final(buf)@[i as int], final(buf)@[i + 1], final(buf)@[i + 2], final(buf)@[i + 3]) == w,
        forall|x: int| 0 <= x < 128 && (x < i || i + 4 <= x) ==> final(buf)@[x] == old(buf)@[x],
{
    proof {
        lemma_be_word_split(w);
    }
    buf[i] = (w >> 24) as u8;
    buf[i + 1] = (w >> 16) as u8;
    buf[i + 2] = (w >> 8) as u8;
    buf[i + 3] = w as u8;
}

/// Bourrage : écrit dans `tail` (nul au départ) les `n mod 64` derniers octets du message, 0x80 et la longueur en
/// bits ; renvoie le nombre d'octets à compresser (64 ou 128). Les mots big-endian de `tail` sont ceux du
/// message bourré à partir du bloc `n / 64`.
#[inline(always)]
fn pad_tail(base: u64, n: usize, tail: &mut [u8; 128]) -> (total: usize)
    requires
        n <= 0x100_0000_0000,
        forall|x: int| 0 <= x < 128 ==> old(tail)@[x] == 0u8,
    ensures
        total == 64 || total == 128,
        n / 64 + total / 64 == nblocks(n as int),
        forall|y: int|
            0 <= y && 4 * y < total ==> #[trigger] word_at(final(tail)@, y) == pad_word(
                gen_msg(base, n as int),
                16 * (n / 64) + y,
            ),
{
    let ghost msg = gen_msg(base, n as int);
    let t = n % 64;
    let start = (n - t) as u64;
    let mut j = 0usize;
    while j < t
        invariant
            j <= t,
            t == n % 64,
            start == n - t,
            n <= 0x100_0000_0000,
            forall|x: int| 0 <= x < j ==> tail@[x] == msg_byte(base, start + x),
            forall|x: int| j <= x < 128 ==> tail@[x] == 0u8,
        decreases t - j,
    {
        tail[j] = byte_at(base, start + j as u64);
        j += 1;
    }
    tail[t] = 0x80;
    let total = if t < 56 { 64usize } else { 128usize };
    let bits = (n as u64) * 8;
    store_be32(tail, total - 8, (bits >> 32) as u32);
    store_be32(tail, total - 4, bits as u32);
    assert(msg.len() == n && start + t == n && start == 64 * (n / 64));
    assert(nblocks(n as int) == n / 64 + total / 64 && t + 9 <= total);
    assert(len_bits(msg) == bits);
    assert forall|x: int| 0 <= x < total - 8 implies tail@[x] == pad_byte(msg, start + x) by {
        if x < t {
            assert(msg[start + x] == msg_byte(base, start + x));
        }
    }
    assert forall|y: int| 0 <= y && 4 * y < total implies #[trigger] word_at(tail@, y) == pad_word(
        msg,
        16 * (n / 64) + y,
    ) by {
        assert(4 * (16 * (n / 64) + y) == start + 4 * y);
    }
    total
}

/// Condensé SHA-256 des `n` premiers octets du message de graine `base`.
pub fn digest(base: u64, n: usize) -> (h: [u32; 8])
    requires
        n <= 0x100_0000_0000,
    ensures
        st_of(h@) == sha256(gen_msg(base, n as int)),
{
    let ghost msg = gen_msg(base, n as int);
    proof {
        lemma_pad_word_in_msg(base, n as int);
    }
    let mut h = IV;
    let blocks = (n / 64) as u64;
    let mut m = [0u32; 16];
    let mut b = 0u64;
    while b < blocks
        invariant
            b <= blocks,
            blocks == n / 64,
            n <= 0x100_0000_0000,
            msg == gen_msg(base, n as int),
            forall|j: int| 0 <= j && 4 * j + 4 <= n ==> #[trigger] pad_word(msg, j) == msg_word(base, j),
            st_of(h@) == sha_state(msg, b as int),
        decreases blocks - b,
    {
        load_block(base, b, &mut m);
        assert(m@ =~= block(msg, b as int));
        compress(&mut h, &m);
        b += 1;
    }
    // Reste : n mod 64 octets, 0x80, zéros et longueur en bits (1 ou 2 blocs).
    let mut tail = [0u8; 128];
    let total = pad_tail(base, n, &mut tail);
    let mut off = 0usize;
    while off < total
        invariant
            off % 64 == 0,
            off <= total,
            total == 64 || total == 128,
            b == n / 64,
            msg == gen_msg(base, n as int),
            forall|y: int| 0 <= y && 4 * y < total ==> #[trigger] word_at(tail@, y) == pad_word(msg, 16 * b + y),
            st_of(h@) == sha_state(msg, b + off / 64),
        decreases total - off,
    {
        let mut i = 0usize;
        while i < 16
            invariant
                i <= 16,
                off % 64 == 0,
                off < total,
                total == 64 || total == 128,
                forall|y: int| 0 <= y && 4 * y < total ==> #[trigger] word_at(tail@, y) == pad_word(msg, 16 * b + y),
                forall|y: int| 0 <= y < i ==> m@[y] == pad_word(msg, 16 * (b + off / 64) + y),
            decreases 16 - i,
        {
            assert(16 * (b + off / 64) + i == 16 * b + (off + 4 * i) / 4);
            assert(word_at(tail@, (off + 4 * i) / 4) == pad_word(msg, 16 * b + (off + 4 * i) / 4));
            m[i] = load_be32(&tail, off + 4 * i);
            i += 1;
        }
        assert(m@ =~= block(msg, b + off / 64));
        compress(&mut h, &m);
        off += 64;
    }
    h
}

/// Checksum : repliement des mots H0..H7 du condensé.
pub fn run(n: usize, _threads: usize) -> (ck: u64)
    requires
        n <= 0x100_0000_0000,
    ensures
        ck == ck_digest(CK_INIT, sha256(gen_msg(SEED as u64, n as int))),
{
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

} // verus!
