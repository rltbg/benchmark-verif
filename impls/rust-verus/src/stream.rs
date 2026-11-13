//! stream : CRC-32 + tokeniseur JSON-lines sur un fichier lu par blocs de 1 Mio (spec/workloads.d/stream.md).
//!
//! **Prouvé (Verus)** :
//! * `chunk` : le registre CRC devient `crc_seq(crc, octets)`, CRC-32 **bit à bit** (8 décalages conditionnels par
//!   octet, polynôme réfléchi 0xEDB88320) ; le code utilise la table de 256 entrées, dont le contenu est prouvé
//!   (`crc_table`), et l'équivalence octet/table est un lemme `bit_vector` (`lemma_crc8_split`) ; l'état du
//!   tokeniseur devient `tok_seq(tok, octets)`, repliement de l'automate `tok_step` qui transcrit les règles 1–3 de
//!   la spécification ; `total` augmente de `len` (mod 2^64).
//! * `run` : après chaque lecture, l'état est celui de l'automate sur la concaténation `seen` des octets lus (lemmes
//!   `lemma_crc_seq_concat`, `lemma_tok_seq_concat`) ; le checksum renvoyé est `ck_stream(seen)` pour la suite
//!   `seen` des octets effectivement lus : CRC-32 du flux, compteurs de l'automate, longueur mod 2^64.
//!
//! **Non prouvé** : `Input::open` et `Input::read_block` (`#[verifier::external_body]`, type `Input` opaque) —
//! les E/S fichier ne sont pas modélisées ; seul contrat : `read_block() <= buf.len()`, longueur de `buf` conservée.
//! `run` porte `#[verifier::exec_allows_no_decreases_clause]` : la boucle de lecture s'arrête à la fin du fichier,
//! ce qui n'est pas prouvable sans modèle du fichier. Tout le reste termine (`decreases`).
use crate::common::*;
use vstd::prelude::*;
#[cfg(verus_only)]
use vstd::arithmetic::div_mod::lemma_add_mod_noop;

verus! {

pub const BUF_LEN: usize = 1 << 20;

// =====================================================================================================
// Spécification
// =====================================================================================================

/// Un décalage du CRC réfléchi.
pub open spec fn crc_shift(x: u32) -> u32 {
    if x & 1 != 0 {
        (x >> 1) ^ 0xEDB88320u32
    } else {
        x >> 1
    }
}

/// `k` décalages.
pub open spec fn crc_iter(x: u32, k: int) -> u32
    decreases k,
{
    if k <= 0 {
        x
    } else {
        crc_shift(crc_iter(x, k - 1))
    }
}

/// Mise à jour du registre CRC-32 par un octet (définition bit à bit, sans table).
pub open spec fn crc_byte(crc: u32, c: u8) -> u32 {
    crc_iter(crc ^ (c as u32), 8)
}

/// Registre après la suite d'octets `s`.
pub open spec fn crc_seq(crc: u32, s: Seq<u8>) -> u32
    decreases s.len(),
{
    if s.len() == 0 {
        crc
    } else {
        crc_byte(crc_seq(crc, s.drop_last()), s.last())
    }
}

/// CRC-32 (zlib) de `s`.
pub open spec fn crc32(s: Seq<u8>) -> u32 {
    crc_seq(0xFFFF_FFFFu32, s) ^ 0xFFFF_FFFFu32
}

/// État du tokeniseur. `mode` : 0 hors littéral, 1 chaîne, 2 après `\`, 3 nombre, 4 mot.
#[derive(Clone, Copy)]
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

pub open spec fn tok0() -> Tok {
    Tok {
        mode: 0,
        depth: 0,
        maxd: 0,
        objs: 0,
        strs: 0,
        nums: 0,
        arrs: 0,
        lits: 0,
        lines: 0,
        idsum: 0,
        allsum: 0,
        acc: 0,
        isint: false,
        gotid: false,
    }
}

pub open spec fn is_digit_spec(c: u8) -> bool {
    48 <= c <= 57
}

pub open spec fn is_lower_spec(c: u8) -> bool {
    97 <= c <= 122
}

/// `.` `e` `E` `+` `-`
pub open spec fn is_numpunct_spec(c: u8) -> bool {
    c == 46 || c == 101 || c == 69 || c == 43 || c == 45
}

/// Règle 1 : fin de nombre ou de mot.
pub open spec fn tok_end(t: Tok, c: u8) -> Tok {
    if t.mode == 3 && !(is_digit_spec(c) || is_numpunct_spec(c)) {
        Tok {
            nums: t.nums.wrapping_add(1),
            allsum: if t.isint { t.allsum.wrapping_add(t.acc) } else { t.allsum },
            idsum: if t.isint && t.depth == 1 && !t.gotid { t.idsum.wrapping_add(t.acc) } else { t.idsum },
            gotid: t.gotid || t.depth == 1,
            mode: 0,
            ..t
        }
    } else if t.mode == 4 && !is_lower_spec(c) {
        Tok { lits: t.lits.wrapping_add(1), mode: 0, ..t }
    } else {
        t
    }
}

/// Ouverture d'un objet ou d'un tableau : profondeur + 1, maximum mis à jour.
pub open spec fn tok_open(t: Tok) -> Tok {
    let d = t.depth.wrapping_add(1);
    Tok { depth: d, maxd: if d > t.maxd { d } else { t.maxd }, ..t }
}

/// Règles 2 et 3 : action selon le mode (le mode 0 est la branche finale).
pub open spec fn tok_act(t: Tok, c: u8) -> Tok {
    if t.mode == 2 {
        Tok { mode: 1, ..t }
    } else if t.mode == 1 {
        Tok {
            mode: if c == 92 {
                2
            } else if c == 34 {
                0
            } else {
                1
            },
            ..t
        }
    } else if t.mode == 3 {
        if is_digit_spec(c) {
            Tok { acc: t.acc.wrapping_mul(10).wrapping_add((c - 48) as u64), ..t }
        } else {
            Tok { isint: false, ..t }
        }
    } else if t.mode == 4 {
        t
    } else if c == 34 {
        Tok { strs: t.strs.wrapping_add(1), mode: 1, ..t }
    } else if c == 123 {
        tok_open(Tok { objs: t.objs.wrapping_add(1), ..t })
    } else if c == 91 {
        tok_open(Tok { arrs: t.arrs.wrapping_add(1), ..t })
    } else if c == 125 || c == 93 {
        Tok { depth: if t.depth > 0 { (t.depth - 1) as u64 } else { t.depth }, ..t }
    } else if c == 10 {
        Tok { lines: t.lines.wrapping_add(1), gotid: false, ..t }
    } else if is_digit_spec(c) {
        Tok { mode: 3, acc: (c - 48) as u64, isint: true, ..t }
    } else if c == 45 {
        Tok { mode: 3, acc: 0, isint: true, ..t }
    } else if is_lower_spec(c) {
        Tok { mode: 4, ..t }
    } else {
        t
    }
}

/// Une transition de l'automate.
pub open spec fn tok_step(t: Tok, c: u8) -> Tok {
    tok_act(tok_end(t, c), c)
}

/// Automate après la suite d'octets `s`.
pub open spec fn tok_seq(t: Tok, s: Seq<u8>) -> Tok
    decreases s.len(),
{
    if s.len() == 0 {
        t
    } else {
        tok_step(tok_seq(t, s.drop_last()), s.last())
    }
}

/// Checksum du workload pour un CRC-32 `crc`, un état final `t` et un nombre d'octets `total`.
pub open spec fn ck_of(crc: u32, t: Tok, total: u64) -> u64 {
    let ck = spec_ckfold(CK_INIT, crc as u64);
    let ck = spec_ckfold(ck, t.objs);
    let ck = spec_ckfold(ck, t.strs);
    let ck = spec_ckfold(ck, t.nums);
    let ck = spec_ckfold(ck, t.arrs);
    let ck = spec_ckfold(ck, t.lits);
    let ck = spec_ckfold(ck, t.maxd);
    let ck = spec_ckfold(ck, t.lines);
    let ck = spec_ckfold(ck, t.idsum);
    let ck = spec_ckfold(ck, t.allsum);
    spec_ckfold(ck, total)
}

/// Le checksum attendu pour le flux d'octets `s` : CRC-32, compteurs de l'automate, longueur mod 2^64.
pub open spec fn ck_stream(s: Seq<u8>) -> u64 {
    ck_of(crc32(s), tok_seq(tok0(), s), (s.len() % 0x1_0000_0000_0000_0000) as u64)
}

// =====================================================================================================
// Lemmes
// =====================================================================================================

/// Huit décalages écrits à plat (pour le solveur `bit_vector`, qui ne déplie pas la récursion).
pub open spec fn crc8n(x: u32) -> u32 {
    crc_shift(crc_shift(crc_shift(crc_shift(crc_shift(crc_shift(crc_shift(crc_shift(x))))))))
}

pub proof fn lemma_crc_iter8(x: u32)
    ensures
        crc_iter(x, 8) == crc8n(x),
{
    reveal_with_fuel(crc_iter, 9);
}

/// Linéarité du CRC sur l'octet de poids faible : c'est ce qui justifie la table.
pub proof fn lemma_crc8_split(x: u32, c: u8)
    ensures
        crc8n(x) == crc8n(x & 0xff) ^ (x >> 8),
        (x ^ (c as u32)) >> 8 == x >> 8,
        x & 0xff < 256,
{
    assert(crc8n(x) == crc8n(x & 0xff) ^ (x >> 8)) by (bit_vector);
    assert((x ^ (c as u32)) >> 8 == x >> 8) by (bit_vector);
    assert(x & 0xff < 256) by (bit_vector);
}

pub proof fn lemma_crc_seq_concat(crc: u32, a: Seq<u8>, b: Seq<u8>)
    ensures
        crc_seq(crc, a + b) == crc_seq(crc_seq(crc, a), b),
    decreases b.len(),
{
    if b.len() == 0 {
        assert(a + b =~= a);
    } else {
        lemma_crc_seq_concat(crc, a, b.drop_last());
        assert((a + b).drop_last() =~= a + b.drop_last());
    }
}

pub proof fn lemma_tok_seq_concat(t: Tok, a: Seq<u8>, b: Seq<u8>)
    ensures
        tok_seq(t, a + b) == tok_seq(tok_seq(t, a), b),
    decreases b.len(),
{
    if b.len() == 0 {
        assert(a + b =~= a);
    } else {
        lemma_tok_seq_concat(t, a, b.drop_last());
        assert((a + b).drop_last() =~= a + b.drop_last());
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

#[inline(always)]
fn is_digit(c: u8) -> (r: bool)
    ensures
        r == is_digit_spec(c),
{
    c >= 48 && c <= 57
}

#[inline(always)]
fn is_lower(c: u8) -> (r: bool)
    ensures
        r == is_lower_spec(c),
{
    c >= 97 && c <= 122
}

#[inline(always)]
fn is_numpunct(c: u8) -> (r: bool)
    ensures
        r == is_numpunct_spec(c),
{
    c == 46 || c == 101 || c == 69 || c == 43 || c == 45
}

/// Table du CRC-32 : `tab[i]` = huit décalages de `i`.
pub fn crc_table() -> (t: [u32; 256])
    ensures
        forall|i: int| 0 <= i < 256 ==> t@[i] == crc_iter(i as u32, 8),
{
    let mut t = [0u32; 256];
    let mut i = 0usize;
    while i < 256
        invariant
            i <= 256,
            forall|j: int| 0 <= j < i ==> t@[j] == crc_iter(j as u32, 8),
        decreases 256 - i,
    {
        let mut x = i as u32;
        let mut k = 0usize;
        while k < 8
            invariant
                k <= 8,
                i < 256,
                x == crc_iter(i as u32, k as int),
            decreases 8 - k,
        {
            x = if x & 1 != 0 { (x >> 1) ^ 0xEDB88320 } else { x >> 1 };
            k += 1;
        }
        t[i] = x;
        i += 1;
    }
    t
}

/// Une transition de l'automate (règles 1 à 3).
#[inline(always)]
fn tok_step_exec(t: &mut Tok, c: u8)
    ensures
        *final(t) == tok_step(*old(t), c),
{
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
pub fn chunk(buf: &Vec<u8>, len: usize, st: &mut State, tab: &[u32; 256])
    requires
        len <= buf.len(),
        forall|i: int| 0 <= i < 256 ==> tab@[i] == crc_iter(i as u32, 8),
    ensures
        final(st).crc == crc_seq(old(st).crc, buf@.subrange(0, len as int)),
        final(st).tok == tok_seq(old(st).tok, buf@.subrange(0, len as int)),
        final(st).total == old(st).total.wrapping_add(len as u64),
{
    let mut crc = st.crc;
    let mut tok = st.tok;
    let mut p = 0usize;
    while p < len
        invariant
            p <= len <= buf.len(),
            forall|i: int| 0 <= i < 256 ==> tab@[i] == crc_iter(i as u32, 8),
            crc == crc_seq(old(st).crc, buf@.subrange(0, p as int)),
            tok == tok_seq(old(st).tok, buf@.subrange(0, p as int)),
        decreases len - p,
    {
        let c = buf[p];
        proof {
            assert(buf@.subrange(0, p + 1).drop_last() =~= buf@.subrange(0, p as int));
            assert(buf@.subrange(0, p + 1).last() == c);
            lemma_crc_iter8(crc ^ (c as u32));
            lemma_crc_iter8((crc ^ (c as u32)) & 0xff);
            lemma_crc8_split(crc ^ (c as u32), c);
            lemma_crc8_split(crc, c);
        }
        let idx = ((crc ^ c as u32) & 0xFF) as usize;
        assert(tab@[idx as int] == crc_iter((crc ^ (c as u32)) & 0xff, 8));
        let ghost old_crc = crc;
        crc = tab[idx] ^ (crc >> 8);
        assert(crc == crc_byte(old_crc, c));
        tok_step_exec(&mut tok, c);
        p += 1;
    }
    st.crc = crc;
    st.tok = tok;
    st.total = st.total.wrapping_add(len as u64);
}

/// Checksum de l'état final.
pub fn finish(st: &State) -> (ck: u64)
    ensures
        ck == ck_of(st.crc ^ 0xFFFF_FFFFu32, st.tok, st.total),
{
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

/// Fichier d'entrée (type opaque pour Verus).
#[verifier::external_body]
pub struct Input {
    f: std::fs::File,
}

impl Input {
    /// Ouvre `$BENCH_DATA_DIR/stream-<n>.json` (défaut `/var/tmp/bench-data`) ; quitte le processus en cas d'erreur.
    #[verifier::external_body]
    pub fn open(n_mio: usize) -> Input {
        let dir = std::env::var("BENCH_DATA_DIR").unwrap_or_else(|_| "/var/tmp/bench-data".to_string());
        let path = format!("{dir}/stream-{n_mio}.json");
        let f = std::fs::File::open(&path).unwrap_or_else(|e| {
            eprintln!("{path}: {e}");
            std::process::exit(3)
        });
        Input { f }
    }

    /// Lit au plus `buf.len()` octets (0 = fin de fichier) ; quitte le processus en cas d'erreur.
    #[verifier::external_body]
    pub fn read_block(&mut self, buf: &mut Vec<u8>) -> (r: usize)
        ensures
            r <= old(buf).len(),
            final(buf).len() == old(buf).len(),
    {
        use std::io::Read;
        self.f.read(buf).unwrap_or_else(|e| {
            eprintln!("read: {e}");
            std::process::exit(3)
        })
    }
}

/// Le checksum est celui (`ck_stream`) de la suite des octets lus : seule la lecture n'est pas vérifiée.
#[verifier::exec_allows_no_decreases_clause]
pub fn run(n_mio: usize, _threads: usize) -> (ck: u64)
    ensures
        exists|s: Seq<u8>| ck == ck_stream(s),
{
    let mut f = Input::open(n_mio);
    let tab = crc_table();
    let mut st = State {
        crc: 0xFFFF_FFFF,
        tok: Tok {
            mode: 0,
            depth: 0,
            maxd: 0,
            objs: 0,
            strs: 0,
            nums: 0,
            arrs: 0,
            lits: 0,
            lines: 0,
            idsum: 0,
            allsum: 0,
            acc: 0,
            isint: false,
            gotid: false,
        },
        total: 0,
    };
    let mut buf = vec![0u8; BUF_LEN];
    let ghost mut seen: Seq<u8> = Seq::empty();
    loop
        invariant
            buf.len() == BUF_LEN,
            forall|i: int| 0 <= i < 256 ==> tab@[i] == crc_iter(i as u32, 8),
            st.crc == crc_seq(0xFFFF_FFFFu32, seen),
            st.tok == tok_seq(tok0(), seen),
            st.total == (seen.len() % 0x1_0000_0000_0000_0000) as u64,
    {
        let got = f.read_block(&mut buf);
        if got == 0 {
            break;
        }
        let ghost bytes = buf@.subrange(0, got as int);
        proof {
            lemma_crc_seq_concat(0xFFFF_FFFFu32, seen, bytes);
            lemma_tok_seq_concat(tok0(), seen, bytes);
            lemma_add_mod_noop(seen.len() as int, got as int, 0x1_0000_0000_0000_0000);
        }
        chunk(&buf, got, &mut st, &tab);
        proof {
            seen = seen + bytes;
        }
    }
    let ck = finish(&st);
    assert(ck == ck_stream(seen));
    ck
}

} // verus!
