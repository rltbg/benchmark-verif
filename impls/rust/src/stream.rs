//! stream : CRC-32 + tokeniseur JSON-lines sur un fichier lu par blocs de 1 Mio (spec/workloads.d/stream.md).
//!
//! **Prouvé (Creusot)** :
//! * `chunk` : le registre CRC devient `crc_seq(crc, octets)`, où `crc_seq` est le CRC-32 **bit à bit** (8 décalages
//!   conditionnels par octet, polynôme réfléchi 0xEDB88320) — le code utilise la table de 256 entrées, dont le contenu
//!   est prouvé (`crc_table`) et l'équivalence octet/table est un lemme vecteurs de bits (`crc8_split`) ;
//!   l'état du tokeniseur devient `tok_seq(tok, octets)`, repliement de l'automate logique `tok_step` qui transcrit les
//!   règles 1–3 de la spécification ; `total` augmente de `len` (mod 2^64).
//! * `run` : à chaque lecture, l'état est celui de l'automate sur la concaténation `seen` des octets lus
//!   (lemmes de concaténation `crc_seq_concat`, `tok_seq_concat`) ; le checksum renvoyé est `ck_stream(seen)` pour la
//!   suite `seen` des octets effectivement lus — c'est-à-dire le CRC-32 du flux, les compteurs de l'automate et la
//!   longueur. Terminaison prouvée pour tout sauf la boucle de lecture (un fichier peut ne pas finir).
//!
//! **Non prouvé (`#[trusted]`)** : `open_input` (variable d'environnement, ouverture) et `read_block` (`File::read`) — les
//! E/S ne sont pas modélisées par Creusot ; leur seul contrat est `read_block() <= buf.len()` (longueur conservée).
use crate::common::CK_INIT;
use creusot_std::prelude::*;
use std::fs::File;

pub const BUF_LEN: usize = 1 << 20;

// =====================================================================================================
// Spécification
// =====================================================================================================

/// Un décalage du CRC réfléchi.
#[logic(open)]
pub fn crc_shift(x: u32) -> u32 {
    if x & 1u32 != 0u32 { (x >> 1u32) ^ 0xEDB88320u32 } else { x >> 1u32 }
}

/// `k` décalages.
#[logic(open)]
#[variant(k)]
pub fn crc_iter(x: u32, k: Int) -> u32 {
    if k <= 0 { x } else { crc_shift(crc_iter(x, k - 1)) }
}

/// Mise à jour du registre CRC-32 par un octet (définition bit à bit, sans table).
#[logic(open)]
pub fn crc_byte(crc: u32, c: u8) -> u32 {
    crc_iter(crc ^ (c as u32), 8)
}

/// Registre après la suite d'octets `s`.
#[logic(open)]
#[variant(s.len())]
pub fn crc_seq(crc: u32, s: Seq<u8>) -> u32 {
    pearlite! { if s.len() == 0 { crc } else { crc_byte(crc_seq(crc, s.subsequence(0, s.len() - 1)), s[s.len() - 1]) } }
}

/// CRC-32 (zlib) de `s`.
#[logic(open)]
pub fn crc32(s: Seq<u8>) -> u32 {
    crc_seq(0xFFFFFFFFu32, s) ^ 0xFFFFFFFFu32
}

/// État du tokeniseur. `mode` : 0 hors littéral, 1 chaîne, 2 après `\`, 3 nombre, 4 mot.
#[derive(creusot_std::prelude::Clone, Copy)]
pub struct Tok {
    pub mode: u8,
    pub depth: u64,
    pub maxd: u64,
    pub objs: u64,
    pub strs: u64,
    pub nums: u64,
    pub arrs: u64,
    pub lits: u64,
    pub lines: u64,
    pub idsum: u64,
    pub allsum: u64,
    pub acc: u64,
    pub isint: bool,
    pub gotid: bool,
}

#[logic(open)]
pub fn tok0() -> Tok {
    Tok {
        mode: 0u8, depth: 0u64, maxd: 0u64, objs: 0u64, strs: 0u64, nums: 0u64, arrs: 0u64, lits: 0u64,
        lines: 0u64, idsum: 0u64, allsum: 0u64, acc: 0u64, isint: false, gotid: false,
    }
}

#[logic(open)]
pub fn is_digit_l(c: u8) -> bool {
    48u8 <= c && c <= 57u8
}
#[logic(open)]
pub fn is_lower_l(c: u8) -> bool {
    97u8 <= c && c <= 122u8
}
/// `.` `e` `E` `+` `-`
#[logic(open)]
pub fn is_numpunct_l(c: u8) -> bool {
    c == 46u8 || c == 101u8 || c == 69u8 || c == 43u8 || c == 45u8
}

/// Règle 1 : fin de nombre ou de mot.
#[logic(open)]
pub fn tok_end(t: Tok, c: u8) -> Tok {
    if t.mode == 3u8 && !(is_digit_l(c) || is_numpunct_l(c)) {
        Tok {
            nums: t.nums + 1u64,
            allsum: if t.isint { t.allsum + t.acc } else { t.allsum },
            idsum: if t.isint && t.depth == 1u64 && !t.gotid { t.idsum + t.acc } else { t.idsum },
            gotid: t.gotid || t.depth == 1u64,
            mode: 0u8,
            ..t
        }
    } else if t.mode == 4u8 && !is_lower_l(c) {
        Tok { lits: t.lits + 1u64, mode: 0u8, ..t }
    } else {
        t
    }
}

/// Ouverture d'un objet ou d'un tableau : profondeur + 1, maximum mis à jour.
#[logic(open)]
pub fn tok_open(t: Tok) -> Tok {
    let d = t.depth + 1u64;
    Tok { depth: d, maxd: if d > t.maxd { d } else { t.maxd }, ..t }
}

/// Règles 2 et 3 : action selon le mode (le mode 0 est la branche finale).
#[logic(open)]
pub fn tok_act(t: Tok, c: u8) -> Tok {
    if t.mode == 2u8 {
        Tok { mode: 1u8, ..t }
    } else if t.mode == 1u8 {
        Tok { mode: if c == 92u8 { 2u8 } else if c == 34u8 { 0u8 } else { 1u8 }, ..t }
    } else if t.mode == 3u8 {
        if is_digit_l(c) { Tok { acc: t.acc * 10u64 + (c - 48u8) as u64, ..t } } else { Tok { isint: false, ..t } }
    } else if t.mode == 4u8 {
        t
    } else if c == 34u8 {
        Tok { strs: t.strs + 1u64, mode: 1u8, ..t }
    } else if c == 123u8 {
        tok_open(Tok { objs: t.objs + 1u64, ..t })
    } else if c == 91u8 {
        tok_open(Tok { arrs: t.arrs + 1u64, ..t })
    } else if c == 125u8 || c == 93u8 {
        Tok { depth: if t.depth > 0u64 { t.depth - 1u64 } else { t.depth }, ..t }
    } else if c == 10u8 {
        Tok { lines: t.lines + 1u64, gotid: false, ..t }
    } else if is_digit_l(c) {
        Tok { mode: 3u8, acc: (c - 48u8) as u64, isint: true, ..t }
    } else if c == 45u8 {
        Tok { mode: 3u8, acc: 0u64, isint: true, ..t }
    } else if is_lower_l(c) {
        Tok { mode: 4u8, ..t }
    } else {
        t
    }
}

/// Une transition de l'automate.
#[logic(open)]
pub fn tok_step(t: Tok, c: u8) -> Tok {
    tok_act(tok_end(t, c), c)
}

/// Automate après la suite d'octets `s`.
#[logic(open)]
#[variant(s.len())]
pub fn tok_seq(t: Tok, s: Seq<u8>) -> Tok {
    pearlite! { if s.len() == 0 { t } else { tok_step(tok_seq(t, s.subsequence(0, s.len() - 1)), s[s.len() - 1]) } }
}

#[logic(open)]
pub fn ckfold_l(ck: u64, v: u64) -> u64 {
    (ck ^ v) * 0x100000001B3u64
}

/// Checksum du workload pour un CRC-32 `crc`, un état final `t` et un nombre d'octets `total`.
#[logic(open)]
pub fn ck_of(crc: u32, t: Tok, total: u64) -> u64 {
    let ck = ckfold_l(CK_INIT, crc as u64);
    let ck = ckfold_l(ck, t.objs);
    let ck = ckfold_l(ck, t.strs);
    let ck = ckfold_l(ck, t.nums);
    let ck = ckfold_l(ck, t.arrs);
    let ck = ckfold_l(ck, t.lits);
    let ck = ckfold_l(ck, t.maxd);
    let ck = ckfold_l(ck, t.lines);
    let ck = ckfold_l(ck, t.idsum);
    let ck = ckfold_l(ck, t.allsum);
    ckfold_l(ck, total)
}

/// `x mod 2^64` comme entier machine (Creusot n'a pas de conversion `Int -> u64` en logique) : définition
/// récursive, corps fermé hors du sous-module pour ne pas faire boucler l'instanciation des solveurs.
pub mod conv {
    use creusot_std::prelude::*;

    #[logic]
    #[variant(x)]
    pub fn u64_of(x: Int) -> u64 {
        if x <= 0 { 0u64 } else { u64_of(x - 1) + 1u64 }
    }

    #[logic]
    #[ensures(u64_of(0) == 0u64)]
    pub fn u64_of_zero() {}

    #[logic]
    #[variant(v@)]
    #[requires(x >= 0)]
    #[ensures(u64_of(x + v@) == u64_of(x) + v)]
    pub fn u64_of_add(x: Int, v: u64) {
        pearlite! { if v@ > 0 { u64_of_add(x, v - 1u64) } else { () } }
    }
}
#[cfg(creusot)]
use conv::{u64_of, u64_of_add, u64_of_zero};

/// Le checksum attendu pour le flux d'octets `s` : CRC-32, compteurs de l'automate, longueur mod 2^64.
#[logic(open)]
pub fn ck_stream(s: Seq<u8>) -> u64 {
    pearlite! { ck_of(crc32(s), tok_seq(tok0(), s), u64_of(s.len())) }
}

// =====================================================================================================
// Lemmes
// =====================================================================================================

/// Huit décalages écrits à plat (pour le mode vecteurs de bits).
#[logic(open)]
pub fn crc8n(x: u32) -> u32 {
    crc_shift(crc_shift(crc_shift(crc_shift(crc_shift(crc_shift(crc_shift(crc_shift(x))))))))
}

#[logic]
#[ensures(crc_iter(x, 8) == crc8n(x))]
pub fn crc_iter8(x: u32) {}

/// Linéarité du CRC sur l'octet de poids faible : c'est ce qui justifie la table.
#[logic]
#[bitwise_proof]
#[ensures(crc8n(x) == crc8n(x & 0xffu32) ^ (x >> 8u32))]
#[ensures((x ^ (c as u32)) >> 8u32 == x >> 8u32)]
#[ensures((x & 0xffu32)@ < 256)]
pub fn crc8_split(x: u32, c: u8) {}

#[logic]
#[variant(b.len())]
#[ensures(crc_seq(crc, a.concat(b)) == crc_seq(crc_seq(crc, a), b))]
pub fn crc_seq_concat(crc: u32, a: Seq<u8>, b: Seq<u8>) {
    pearlite! {
        if b.len() > 0 {
            let b1 = b.subsequence(0, b.len() - 1);
            crc_seq_concat(crc, a, b1);
            proof_assert!(a.concat(b).subsequence(0, a.len() + b.len() - 1).ext_eq(a.concat(b1)));
            proof_assert!(a.concat(b)[a.len() + b.len() - 1] == b[b.len() - 1])
        } else {
            proof_assert!(a.concat(b).ext_eq(a))
        }
    }
}

#[logic]
#[variant(b.len())]
#[ensures(tok_seq(t, a.concat(b)) == tok_seq(tok_seq(t, a), b))]
pub fn tok_seq_concat(t: Tok, a: Seq<u8>, b: Seq<u8>) {
    pearlite! {
        if b.len() > 0 {
            let b1 = b.subsequence(0, b.len() - 1);
            tok_seq_concat(t, a, b1);
            proof_assert!(a.concat(b).subsequence(0, a.len() + b.len() - 1).ext_eq(a.concat(b1)));
            proof_assert!(a.concat(b)[a.len() + b.len() - 1] == b[b.len() - 1])
        } else {
            proof_assert!(a.concat(b).ext_eq(a))
        }
    }
}

// =====================================================================================================
// Code exécutable
// =====================================================================================================

/// État complet : registre CRC, automate, octets lus.
pub struct State {
    pub crc: u32,
    pub tok: Tok,
    pub total: u64,
}

#[ensures(result == ckfold_l(ck, v))]
#[check(terminates)]
#[inline(always)]
fn ckfold(ck: u64, v: u64) -> u64 {
    (ck ^ v).wrapping_mul(0x100000001B3)
}

#[ensures(result == is_digit_l(c))]
#[check(terminates)]
#[inline(always)]
fn is_digit(c: u8) -> bool {
    c >= 48 && c <= 57
}

#[ensures(result == is_lower_l(c))]
#[check(terminates)]
#[inline(always)]
fn is_lower(c: u8) -> bool {
    c >= 97 && c <= 122
}

#[ensures(result == is_numpunct_l(c))]
#[check(terminates)]
#[inline(always)]
fn is_numpunct(c: u8) -> bool {
    c == 46 || c == 101 || c == 69 || c == 43 || c == 45
}

/// Table du CRC-32 : `tab[i]` = huit décalages de `i`.
#[ensures(forall<i: usize> i@ < 256 ==> result@[i@] == crc_iter(i as u32, 8))]
#[check(terminates)]
pub fn crc_table() -> [u32; 256] {
    let mut t = [0u32; 256];
    let mut i = 0usize;
    #[invariant(i@ <= 256)]
    #[invariant(forall<j: usize> j@ < i@ ==> t@[j@] == crc_iter(j as u32, 8))]
    #[variant(256 - i@)]
    while i < 256 {
        let mut x = i as u32;
        let mut k = 0usize;
        #[invariant(k@ <= 8 && x == crc_iter(i as u32, k@))]
        #[variant(8 - k@)]
        while k < 8 {
            x = if x & 1 != 0 { (x >> 1) ^ 0xEDB88320 } else { x >> 1 };
            k += 1;
        }
        t[i] = x;
        i += 1;
    }
    t
}

/// Une transition de l'automate (règles 1 à 3).
#[ensures(^t == tok_step(*t, c))]
#[check(terminates)]
#[inline(always)]
fn tok_step_exec(t: &mut Tok, c: u8) {
    let dg = is_digit(c);
    let lo = is_lower(c);
    if t.mode == 3 && !(dg || is_numpunct(c)) {
        t.nums = t.nums.wrapping_add(1);
        if t.isint {
            t.allsum = t.allsum.wrapping_add(t.acc);
            if t.depth == 1 && !t.gotid {
                t.idsum = t.idsum.wrapping_add(t.acc);
            }
        }
        if t.depth == 1 {
            t.gotid = true;
        }
        t.mode = 0;
    } else if t.mode == 4 && !lo {
        t.lits = t.lits.wrapping_add(1);
        t.mode = 0;
    }
    if t.mode == 2 {
        t.mode = 1;
    } else if t.mode == 1 {
        t.mode = if c == 92 { 2 } else if c == 34 { 0 } else { 1 };
    } else if t.mode == 3 {
        if dg {
            t.acc = t.acc.wrapping_mul(10).wrapping_add((c - 48) as u64);
        } else {
            t.isint = false;
        }
    } else if t.mode == 4 {
    } else if c == 34 {
        t.strs = t.strs.wrapping_add(1);
        t.mode = 1;
    } else if c == 123 {
        t.objs = t.objs.wrapping_add(1);
        t.depth = t.depth.wrapping_add(1);
        if t.depth > t.maxd {
            t.maxd = t.depth;
        }
    } else if c == 91 {
        t.arrs = t.arrs.wrapping_add(1);
        t.depth = t.depth.wrapping_add(1);
        if t.depth > t.maxd {
            t.maxd = t.depth;
        }
    } else if c == 125 || c == 93 {
        if t.depth > 0 {
            t.depth -= 1;
        }
    } else if c == 10 {
        t.lines = t.lines.wrapping_add(1);
        t.gotid = false;
    } else if dg {
        t.mode = 3;
        t.acc = (c - 48) as u64;
        t.isint = true;
    } else if c == 45 {
        t.mode = 3;
        t.acc = 0;
        t.isint = true;
    } else if lo {
        t.mode = 4;
    }
}

/// Traite les `len` premiers octets de `buf` ; l'état traverse les frontières de blocs.
#[requires(len@ <= buf@.len())]
#[requires(forall<i: usize> i@ < 256 ==> tab@[i@] == crc_iter(i as u32, 8))]
#[ensures((^st).crc == crc_seq(st.crc, buf@.subsequence(0, len@)))]
#[ensures((^st).tok == tok_seq(st.tok, buf@.subsequence(0, len@)))]
#[ensures((^st).total == st.total + len as u64)]
#[check(terminates)]
pub fn chunk(buf: &Vec<u8>, len: usize, st: &mut State, tab: &[u32; 256]) {
    let mut crc = st.crc;
    let mut tok = st.tok;
    let mut p = 0usize;
    #[invariant(p@ <= len@)]
    #[invariant(crc == crc_seq(st.crc, buf@.subsequence(0, p@)))]
    #[invariant(tok == tok_seq(st.tok, buf@.subsequence(0, p@)))]
    #[variant(len@ - p@)]
    while p < len {
        let c = buf[p];
        proof_assert!(buf@.subsequence(0, p@ + 1).subsequence(0, p@).ext_eq(buf@.subsequence(0, p@)));
        proof_assert! { crc_iter8(crc ^ (c as u32)); crc_iter8((crc ^ (c as u32)) & 0xffu32); crc8_split(crc ^ (c as u32), c); true };
        proof_assert!(((crc ^ (c as u32)) & 0xffu32)@ < 256);
        crc = tab[((crc ^ c as u32) & 0xFF) as usize] ^ (crc >> 8);
        tok_step_exec(&mut tok, c);
        p += 1;
    }
    st.crc = crc;
    st.tok = tok;
    st.total = st.total.wrapping_add(len as u64);
}

/// Checksum de l'état final.
#[ensures(result == ck_of(st.crc ^ 0xFFFFFFFFu32, st.tok, st.total))]
#[check(terminates)]
pub fn finish(st: &State) -> u64 {
    let t = &st.tok;
    let mut ck = CK_INIT;
    ck = ckfold(ck, (st.crc ^ 0xFFFF_FFFF) as u64);
    ck = ckfold(ck, t.objs);
    ck = ckfold(ck, t.strs);
    ck = ckfold(ck, t.nums);
    ck = ckfold(ck, t.arrs);
    ck = ckfold(ck, t.lits);
    ck = ckfold(ck, t.maxd);
    ck = ckfold(ck, t.lines);
    ck = ckfold(ck, t.idsum);
    ck = ckfold(ck, t.allsum);
    ckfold(ck, st.total)
}

/// Ouvre `$BENCH_DATA_DIR/stream-<n>.json` (défaut `/var/tmp/bench-data`) ; quitte le processus en cas d'erreur.
#[trusted]
fn open_input(n_mio: usize) -> File {
    let dir = std::env::var("BENCH_DATA_DIR").unwrap_or_else(|_| "/var/tmp/bench-data".to_string());
    let path = format!("{dir}/stream-{n_mio}.json");
    File::open(&path).unwrap_or_else(|e| {
        eprintln!("{path}: {e}");
        std::process::exit(3)
    })
}

/// Lit au plus `buf.len()` octets (0 = fin de fichier) ; quitte le processus en cas d'erreur.
#[trusted]
#[ensures(result@ <= buf@.len() && (^buf)@.len() == buf@.len())]
fn read_block(f: &mut File, buf: &mut Vec<u8>) -> usize {
    use std::io::Read;
    f.read(buf).unwrap_or_else(|e| {
        eprintln!("read: {e}");
        std::process::exit(3)
    })
}

/// Le checksum est celui (`ck_stream`) de la suite des octets lus : seule la lecture n'est pas vérifiée.
#[ensures(exists<s: Seq<u8>> result == ck_stream(s))]
pub fn run(n_mio: usize, _threads: usize) -> u64 {
    let mut f = open_input(n_mio);
    let tab = crc_table();
    let mut st = State { crc: 0xFFFF_FFFF, tok: Tok {
        mode: 0, depth: 0, maxd: 0, objs: 0, strs: 0, nums: 0, arrs: 0, lits: 0,
        lines: 0, idsum: 0, allsum: 0, acc: 0, isint: false, gotid: false,
    }, total: 0 };
    let mut buf = creusot_std::std::vec::vec![0u8; BUF_LEN];
    let mut seen = snapshot!(Seq::empty());
    proof_assert! { u64_of_zero(); true };
    #[invariant(buf@.len() == BUF_LEN@)]
    #[invariant(st.crc == crc_seq(0xFFFFFFFFu32, *seen) && st.tok == tok_seq(tok0(), *seen))]
    #[invariant(st.total == u64_of(seen.len()))]
    loop {
        let got = read_block(&mut f, &mut buf);
        if got == 0 {
            break;
        }
        let chunk_bytes = snapshot!(buf@.subsequence(0, got@));
        proof_assert! { crc_seq_concat(0xFFFFFFFFu32, *seen, *chunk_bytes); tok_seq_concat(tok0(), *seen, *chunk_bytes); true };
        proof_assert! { u64_of_add(seen.len(), got as u64); true };
        chunk(&buf, got, &mut st, &tab);
        seen = snapshot!(seen.concat(*chunk_bytes));
    }
    proof_assert!(st.crc ^ 0xFFFFFFFFu32 == crc32(*seen));
    finish(&st)
}
