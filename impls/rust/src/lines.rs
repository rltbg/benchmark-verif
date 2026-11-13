//! Débit « lignes/seconde » : génère n lignes ASCII « clé;valeur\n » dans un buffer, puis les relit
//! octet par octet et agrège somme/compte par clé (2^20 clés). 1 <= n <= 268_435_455.
//! Spécification : spec/WORKLOADS.md §4.
//!
//! Prouvé avec Creusot (correction fonctionnelle) :
//!   * `generate` : le buffer est exactement le texte logique `text(rs, n)`, concaténation des lignes
//!     `dec(clé) ';' dec(valeur) '\n'`, où `rs[j] = mix64(SEED + j)` (fonction logique `mix64_l`) ;
//!   * `parse` : sur ce texte, `tab[2k]` = somme des valeurs des lignes de clé k et `tab[2k+1]` = leur nombre
//!     (fonctions logiques `sum_v`, `cnt`) ; repose sur le lemme d'aller-retour `num_dec` :
//!     analyse décimale ∘ écriture décimale = identité ;
//!   * absence de panique et de débordement (les additions de `parse` sont prouvées exactes :
//!     somme <= 999 999·n < 2^64), terminaison de toutes les fonctions.
//! Aucune garde d'exécution : la clé lue est < 2^20 par la preuve (pas de masque), la boucle de lecture
//! parcourt exactement le texte produit (plus de longueur utile `len` à comparer). Aucun `trusted`/`assume`.
//! (Les variables fantômes `snapshot!` sont effacées par un `cargo build` normal, d'où le `allow` ci-dessous.)
#![allow(unused_variables, unused_assignments)]
use crate::common::{CK_INIT, ckfold};
use creusot_std::prelude::*;

pub const SEED: u64 = 0x1357924680ACEBDF;

// ───────────────────────────── Modèle logique ─────────────────────────────

/// splitmix64 dans la logique (arithmétique de vecteurs de bits modulo 2^64).
#[logic(open)]
pub fn mix64_l(x: u64) -> u64 {
    let z = x + 0x9E3779B97F4A7C15u64;
    let z = (z ^ (z >> 30u64)) * 0xBF58476D1CE4E5B9u64;
    let z = (z ^ (z >> 27u64)) * 0x94D049BB133111EBu64;
    z ^ (z >> 31u64)
}

/// Clé (20 bits de poids fort) et valeur tirées du mot aléatoire r.
#[logic(open)]
pub fn key_of(r: u64) -> Int {
    pearlite! { (r >> 44u64)@ }
}
#[logic(open)]
pub fn val_of(r: u64) -> Int {
    pearlite! { ((r >> 8u64) & 0xFFFF_FFFFu64)@ % 1000000 }
}

/// rs est la suite des n premiers tirages : rs[j] = mix64(SEED + j).
#[logic(open)]
pub fn draws(rs: Seq<u64>, n: Int) -> bool {
    pearlite! { rs.len() == n && forall<j: u64> j@ < n ==> rs[j@] == mix64_l(SEED + j) }
}

/// Écriture décimale de v >= 0 (codes ASCII, sans zéro de tête).
#[logic(open)]
#[requires(0 <= v)]
#[variant(v)]
pub fn dec(v: Int) -> Seq<Int> {
    if v < 10 { Seq::singleton(48 + v) } else { dec(v / 10).snoc(48 + v % 10) }
}

/// Valeur décimale des j premiers chiffres de s (analyse de gauche à droite, schéma de Horner).
#[logic(open)]
#[variant(j)]
pub fn num(s: Seq<Int>, j: Int) -> Int {
    if j <= 0 { 0 } else { num(s, j - 1) * 10 + (s[j - 1] - 48) }
}

/// Une ligne du texte : dec(k) ';' dec(v) '\n'.
#[logic(open)]
#[requires(0 <= k && 0 <= v)]
pub fn line(k: Int, v: Int) -> Seq<Int> {
    dec(k).snoc(59).concat(dec(v)).snoc(10)
}

#[logic(open)]
pub fn line_of(r: u64) -> Seq<Int> {
    line(key_of(r), val_of(r))
}

/// Texte des i premières lignes.
#[logic(open)]
#[variant(i)]
pub fn text(rs: Seq<u64>, i: Int) -> Seq<Int> {
    if i <= 0 { Seq::empty() } else { text(rs, i - 1).concat(line_of(rs[i - 1])) }
}

/// Somme des valeurs / nombre des lignes de clé k parmi les i premières.
#[logic(open)]
#[variant(i)]
pub fn sum_v(rs: Seq<u64>, i: Int, k: Int) -> Int {
    if i <= 0 { 0 } else { sum_v(rs, i - 1, k) + if key_of(rs[i - 1]) == k { val_of(rs[i - 1]) } else { 0 } }
}
#[logic(open)]
#[variant(i)]
pub fn cnt(rs: Seq<u64>, i: Int, k: Int) -> Int {
    if i <= 0 { 0 } else { cnt(rs, i - 1, k) + if key_of(rs[i - 1]) == k { 1 } else { 0 } }
}

/// Le buffer d'octets t a pour modèle la suite de codes m.
#[logic(open)]
pub fn bytes_are(t: Seq<u8>, m: Seq<Int>) -> bool {
    pearlite! { t.len() == m.len() && forall<p: Int> 0 <= p && p < m.len() ==> t[p]@ == m[p] }
}

// ───────────────────────────── Lemmes ─────────────────────────────

/// La clé tient sur 20 bits ; la valeur est < 10^6.
#[logic]
#[bitwise_proof]
#[ensures(0 <= key_of(r) && key_of(r) < 1048576)]
pub fn key_bound(r: u64) {}

/// dec(v) est formé de chiffres ASCII ; sa longueur est 1..=6 si v < 10^6, 1..=7 si v < 10^7.
#[logic]
#[requires(0 <= v)]
#[variant(v)]
#[ensures(1 <= dec(v).len())]
#[ensures(forall<p: Int> 0 <= p && p < dec(v).len() ==> 48 <= dec(v)[p] && dec(v)[p] <= 57)]
#[ensures(v < 1000000 ==> dec(v).len() <= 6)]
#[ensures(v < 10000000 ==> dec(v).len() <= 7)]
pub fn dec_shape(v: Int) {
    if v >= 10 {
        dec_shape(v / 10)
    }
}

/// num ne dépend que des j premiers éléments.
#[logic]
#[requires(forall<p: Int> 0 <= p && p < j ==> s[p] == t[p])]
#[variant(j)]
#[ensures(num(s, j) == num(t, j))]
pub fn num_ext(s: Seq<Int>, t: Seq<Int>, j: Int) {
    if j > 0 {
        num_ext(s, t, j - 1)
    }
}

/// Lemme d'aller-retour : l'analyse décimale de l'écriture décimale de v rend v ;
/// de plus tout préfixe de l'écriture vaut entre 0 et v (absence de débordement pendant l'analyse).
#[logic]
#[requires(0 <= v)]
#[variant(v)]
#[ensures(num(dec(v), dec(v).len()) == v)]
#[ensures(forall<j: Int> 0 <= j && j <= dec(v).len() ==> 0 <= num(dec(v), j) && num(dec(v), j) <= v)]
pub fn num_dec(v: Int) {
    dec_shape(v);
    if v >= 10 {
        num_dec(v / 10);
        dec_shape(v / 10);
        num_ext_all(dec(v), dec(v / 10), dec(v / 10).len());
    }
}

/// num_ext pour tous les j <= m.
#[logic]
#[requires(forall<p: Int> 0 <= p && p < m ==> s[p] == t[p])]
#[variant(m)]
#[ensures(forall<j: Int> j <= m ==> num(s, j) == num(t, j))]
pub fn num_ext_all(s: Seq<Int>, t: Seq<Int>, m: Int) {
    if m > 0 {
        num_ext(s, t, m);
        num_ext_all(s, t, m - 1)
    }
}

/// Découpage d'une ligne.
#[logic]
#[requires(0 <= k && 0 <= v)]
#[ensures(line(k, v).len() == dec(k).len() + dec(v).len() + 2)]
#[ensures(forall<j: Int> 0 <= j && j < dec(k).len() ==> line(k, v)[j] == dec(k)[j])]
#[ensures(line(k, v)[dec(k).len()] == 59)]
#[ensures(forall<j: Int> 0 <= j && j < dec(v).len() ==> line(k, v)[dec(k).len() + 1 + j] == dec(v)[j])]
#[ensures(line(k, v)[dec(k).len() + dec(v).len() + 1] == 10)]
pub fn line_at(k: Int, v: Int) {}

/// text(rs, i) est un préfixe de text(rs, n) pour i <= n.
#[logic]
#[requires(0 <= i && i <= n)]
#[variant(n)]
#[ensures(text(rs, i).len() <= text(rs, n).len())]
#[ensures(forall<p: Int> 0 <= p && p < text(rs, i).len() ==> text(rs, n)[p] == text(rs, i)[p])]
pub fn text_prefix(rs: Seq<u64>, i: Int, n: Int) {
    if i < n {
        text_prefix(rs, i, n - 1)
    }
}

/// text(rs, i) ne dépend que des i premiers tirages.
#[logic]
#[requires(0 <= i && i <= rs.len())]
#[variant(i)]
#[ensures(text(rs.snoc(r), i) == text(rs, i))]
pub fn text_snoc(rs: Seq<u64>, r: u64, i: Int) {
    if i > 0 {
        text_snoc(rs, r, i - 1)
    }
}

/// Bornes des agrégats (absence de débordement dans `parse`).
#[logic]
#[requires(0 <= i)]
#[variant(i)]
#[ensures(0 <= sum_v(rs, i, k) && sum_v(rs, i, k) <= 999999 * i)]
#[ensures(0 <= cnt(rs, i, k) && cnt(rs, i, k) <= i)]
pub fn agg_bound(rs: Seq<u64>, i: Int, k: Int) {
    if i > 0 {
        agg_bound(rs, i - 1, k)
    }
}

// ───────────────────────────── Code exécutable ─────────────────────────────

/// splitmix64 (copie spécifiée de `common::mix64`, même code machine).
#[ensures(result == mix64_l(x))]
#[check(terminates)]
#[inline(always)]
fn mix64(x: u64) -> u64 {
    let z = x.wrapping_add(0x9E3779B97F4A7C15);
    let z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    let z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

/// Ajoute l'écriture décimale de v à out (chiffres calculés de droite à gauche dans un tampon local).
#[requires(v@ < 10000000)]
#[ensures((^out)@.len() == out@.len() + dec(v@).len())]
#[ensures(forall<p: Int> 0 <= p && p < out@.len() ==> (^out)@[p] == out@[p])]
#[ensures(forall<p: Int> 0 <= p && p < dec(v@).len() ==> (^out)@[out@.len() + p]@ == dec(v@)[p])]
#[check(terminates)]
pub fn put_dec(out: &mut Vec<u8>, v: u32) {
    let mut tmp = [0u8; 10];
    let mut l = 0usize;
    let mut x = v;
    proof_assert! { dec_shape(v@); true };
    // tmp[0..l) contient, à l'envers, la fin de dec(v) ; dec(x) en est le début.
    #[invariant(0 < x@ || l@ == 0)]
    #[invariant(l@ + dec(x@).len() == dec(v@).len() && dec(v@).len() <= 7)]
    #[invariant(forall<p: Int> 0 <= p && p < l@ ==> tmp@[p]@ == dec(v@)[dec(v@).len() - 1 - p])]
    #[invariant(forall<p: Int> 0 <= p && p < dec(x@).len() ==> dec(x@)[p] == dec(v@)[p])]
    #[variant(x@)]
    loop {
        proof_assert! { dec_shape(x@); dec_shape(x@ / 10); true };
        tmp[l] = b'0' + (x % 10) as u8;
        l += 1;
        x /= 10;
        if x == 0 {
            break;
        }
    }
    let orig = snapshot!(out@);
    let mut j = l;
    #[invariant(j@ <= l@ && l@ == dec(v@).len() && out@.len() == orig.len() + (l@ - j@))]
    #[invariant(forall<p: Int> 0 <= p && p < orig.len() ==> out@[p] == orig[p])]
    #[invariant(forall<p: Int> 0 <= p && p < l@ - j@ ==> out@[orig.len() + p]@ == dec(v@)[p])]
    #[variant(j@)]
    while j > 0 {
        j -= 1;
        out.push(tmp[j]);
    }
}

/// Génère les n lignes ; rend le texte et (fantôme) la suite des tirages.
#[requires(n@ <= 268435455)]
#[ensures(draws(*result.1, n@))]
#[ensures(bytes_are(result.0@, text(*result.1, n@)))]
#[check(terminates)]
pub fn generate(n: usize) -> (Vec<u8>, Snapshot<Seq<u64>>) {
    let mut txt: Vec<u8> = Vec::with_capacity(n * 15 + 20);
    let mut rs: Snapshot<Seq<u64>> = snapshot!(Seq::empty());
    let mut i = 0usize;
    #[invariant(i@ <= n@ && draws(*rs, i@))]
    #[invariant(bytes_are(txt@, text(*rs, i@)))]
    #[variant(n@ - i@)]
    while i < n {
        let r = mix64(SEED.wrapping_add(i as u64));
        let key = (r >> 44) as u32;
        let val = (((r >> 8) & 0xFFFF_FFFF) % 1_000_000) as u32;
        proof_assert! { key_bound(r); key@ == key_of(r) && val@ == val_of(r) };
        let t0 = snapshot!(txt@);
        put_dec(&mut txt, key);
        txt.push(b';');
        let t1 = snapshot!(txt@);
        put_dec(&mut txt, val);
        txt.push(b'\n');
        let rs0 = rs;
        rs = snapshot!(rs0.snoc(r));
        // Le texte a grandi exactement d'une ligne : on le vérifie morceau par morceau.
        let kd = snapshot!(dec(key@));
        let vd = snapshot!(dec(val@));
        let l = snapshot!(line_of(r));
        proof_assert! { text_snoc(*rs0, r, i@); line_at(key@, val@); true };
        proof_assert!(t1.len() == t0.len() + kd.len() + 1 && txt@.len() == t0.len() + l.len());
        proof_assert!(forall<q: Int> 0 <= q && q < t0.len() ==> txt@[q] == t0[q]);
        proof_assert!(forall<q: Int> 0 <= q && q < kd.len() ==> txt@[t0.len() + q]@ == l[q]);
        proof_assert!(txt@[t0.len() + kd.len()]@ == l[kd.len()]);
        proof_assert!(forall<q: Int> 0 <= q && q < vd.len() ==> txt@[t0.len() + kd.len() + 1 + q]@ == l[kd.len() + 1 + q]);
        proof_assert!(forall<q: Int> kd.len() < q && q <= kd.len() + vd.len() ==> {
            let q2 = q - kd.len() - 1; txt@[t0.len() + kd.len() + 1 + q2]@ == l[kd.len() + 1 + q2] });
        proof_assert!(forall<q: Int> kd.len() < q && q <= kd.len() + vd.len() ==> txt@[t0.len() + q]@ == l[q]);
        proof_assert!(txt@[t0.len() + kd.len() + vd.len() + 1]@ == l[kd.len() + vd.len() + 1]);
        proof_assert!(forall<q: Int> 0 <= q && q < l.len() ==> txt@[t0.len() + q]@ == l[q]);
        proof_assert!(text(*rs, i@ + 1) == text(*rs0, i@).concat(*l));
        i += 1;
    }
    (txt, rs)
}

/// Lit le texte octet par octet (machine à états : chiffres de la clé jusqu'à ';', puis chiffres de la valeur
/// jusqu'à '\n') et agrège : tab[2k] = somme des valeurs, tab[2k+1] = nombre de lignes de la clé k.
#[requires(rs.len() <= 268435455)]
#[requires(bytes_are(txt@, text(*rs, rs.len())))]
#[requires(tab@.len() == 2097152 && forall<p: Int> 0 <= p && p < 2097152 ==> tab@[p]@ == 0)]
#[ensures((^tab)@.len() == 2097152)]
#[ensures(forall<k: Int> 0 <= k && k < 1048576 ==>
    (^tab)@[2 * k]@ == sum_v(*rs, rs.len(), k) && (^tab)@[2 * k + 1]@ == cnt(*rs, rs.len(), k))]
#[check(terminates)]
pub fn parse(txt: &Vec<u8>, tab: &mut Vec<u64>, rs: Snapshot<Seq<u64>>) {
    let n = snapshot!(rs.len());
    let mut i = snapshot!(0); // ligne courante (fantôme)
    let mut p = 0usize;
    let mut in_val = false;
    let mut key = 0u32;
    let mut vl = 0u32;
    #[invariant(0 <= *i && *i <= *n && tab@.len() == 2097152)]
    #[invariant(text(*rs, *i).len() <= p@ && p@ <= txt@.len())]
    #[invariant(*i == *n ==> p@ == text(*rs, *i).len() && !in_val)]
    #[invariant(*i < *n ==> p@ < text(*rs, *i + 1).len())]
    #[invariant(*i < *n ==> !in_val ==>
        p@ - text(*rs, *i).len() <= dec(key_of(rs[*i])).len()
        && key@ == num(dec(key_of(rs[*i])), p@ - text(*rs, *i).len()) && vl@ == 0)]
    #[invariant(*i < *n ==> in_val ==>
        dec(key_of(rs[*i])).len() < p@ - text(*rs, *i).len()
        && p@ - text(*rs, *i).len() <= dec(key_of(rs[*i])).len() + 1 + dec(val_of(rs[*i])).len()
        && key@ == key_of(rs[*i])
        && vl@ == num(dec(val_of(rs[*i])), p@ - text(*rs, *i).len() - dec(key_of(rs[*i])).len() - 1))]
    #[invariant(forall<k: Int> 0 <= k && k < 1048576 ==>
        tab@[2 * k]@ == sum_v(*rs, *i, k) && tab@[2 * k + 1]@ == cnt(*rs, *i, k))]
    #[variant(txt@.len() - p@)]
    while p < txt.len() {
        let c = txt[p];
        // Position courante : octet j de la ligne i, de découpage kd ';' vd '\n'.
        let r = snapshot!(rs[*i]);
        let j = snapshot!(p@ - text(*rs, *i).len());
        let kd = snapshot!(dec(key_of(*r)));
        let vd = snapshot!(dec(val_of(*r)));
        proof_assert! {
            text_prefix(*rs, *i + 1, *n); key_bound(*r); dec_shape(key_of(*r)); dec_shape(val_of(*r));
            num_dec(key_of(*r)); num_dec(val_of(*r)); line_at(key_of(*r), val_of(*r)); true
        };
        proof_assert!(text(*rs, *i + 1).len() == text(*rs, *i).len() + kd.len() + vd.len() + 2);
        proof_assert!(c@ == line_of(*r)[*j]);
        proof_assert!(*j < kd.len() ==> c@ == kd[*j] && 48 <= c@ && c@ <= 57);
        proof_assert!(*j == kd.len() ==> c@ == 59);
        proof_assert!(kd.len() < *j && *j <= kd.len() + vd.len() ==> c@ == vd[*j - kd.len() - 1] && 48 <= c@ && c@ <= 57);
        proof_assert!(*j == kd.len() + vd.len() + 1 ==> c@ == 10);
        if in_val {
            if c == b'\n' {
                proof_assert!(*j == kd.len() + vd.len() + 1 && vl@ == val_of(*r) && key@ == key_of(*r));
                proof_assert! { agg_bound(*rs, *i, key@); true };
                let k = key as usize;
                proof_assert!(tab@[2 * k@]@ == sum_v(*rs, *i, k@) && tab@[2 * k@ + 1]@ == cnt(*rs, *i, k@));
                tab[2 * k] += vl as u64;
                tab[2 * k + 1] += 1;
                i = snapshot!(*i + 1);
                proof_assert!(p@ + 1 == text(*rs, *i).len());
                proof_assert!(text(*rs, *i + 1) == text(*rs, *i).concat(line_of(rs[*i])));
                proof_assert! { line_at(key_of(rs[*i]), val_of(rs[*i])); line_of(rs[*i]).len() >= 2 };
                in_val = false;
                key = 0;
                vl = 0;
            } else {
                vl = vl * 10 + (c - b'0') as u32;
            }
        } else if c == b';' {
            in_val = true;
        } else {
            key = key * 10 + (c - b'0') as u32;
        }
        p += 1;
    }
    // Sortie : tout le texte est lu, donc toutes les lignes.
    proof_assert! { if *i < *n { text_prefix(*rs, *i + 1, *n) } else { () }; *i == *n };
}

/// ck = fold(fold(ck, tab[2k]), tab[2k+1]) pour k croissant.
#[requires(tab@.len() == 2097152)]
#[check(terminates)]
pub fn checksum(tab: &Vec<u64>) -> u64 {
    let mut ck = CK_INIT;
    let mut k = 0usize;
    #[invariant(tab@.len() == 2097152 && k@ <= 1048576)]
    #[variant(1048576 - k@)]
    while k < 1048576 {
        ck = ckfold(ckfold(ck, tab[2 * k]), tab[2 * k + 1]);
        k += 1;
    }
    ck
}

#[requires(1 <= n@ && n@ <= 268435455)]
pub fn run(n: usize) -> u64 {
    let (txt, rs) = generate(n);
    let mut tab = std::vec![0u64; 2097152];
    parse(&txt, &mut tab, rs);
    checksum(&tab)
}
