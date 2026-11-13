//! Table de hachage à adressage ouvert, sondage linéaire, couples (clé, valeur) entrelacés dans un
//! seul tableau de 2*cap u64 (cap = plus petite puissance de 2 >= 2n). 1 <= n <= 2^26.
//!
//! Prouvé avec Creusot (correction fonctionnelle complète, terminaison comprise) :
//!   * la table réalise un dictionnaire logique `Model` (clé -> valeur optionnelle) : invariant
//!     `table_inv` (chaque clé présente est à sa place dans la chaîne de sondage partant de `home`,
//!     sans trou ; clés uniques ; valeurs = celles du modèle) ;
//!   * `insert` ajoute (somme modulo 2^64 si la clé existe), `lookup` renvoie la valeur du modèle, 0 si absente ;
//!   * après les insertions, le modèle est `model(n)` (fonction logique des seules clés `key_l(0, i)`),
//!     et `run` renvoie exactement le checksum logique des 2n recherches (`ck_lookups`) ;
//!   * terminaison des sondages *sans compteur d'essais à l'exécution* : la table reste à moitié vide
//!     (`occ`, nombre de cases occupées, < cap), donc une case vide arrête toujours la recherche ;
//!   * `slot` sans garde `if s < cap` : prouvé `(key*PHI) >> (64-lg) < 2^lg = cap`.
//! Aucun `trusted` ni `assume`.
use crate::common::CK_INIT;
use creusot_std::{logic::Mapping, prelude::{vec, *}};

pub const SEED: u64 = 0x2468ACE013579BDF;
pub const PHI: u64 = 0x9E3779B97F4A7C15;

// ---------------------------------------------------------------- fonctions logiques de base

/// splitmix64 dans la logique (arithmétique modulo 2^64).
#[logic(open)]
pub fn mix64_l(x: u64) -> u64 {
    let z = x + 0x9E3779B97F4A7C15u64;
    let z = (z ^ (z >> 30u64)) * 0xBF58476D1CE4E5B9u64;
    let z = (z ^ (z >> 27u64)) * 0x94D049BB133111EBu64;
    z ^ (z >> 31u64)
}

/// Clé numéro i de la série `off`.
#[logic(open)]
pub fn key_l(off: u64, i: u64) -> u64 {
    mix64_l(SEED + off + i) | 1u64
}

#[logic(open)]
pub fn fold(ck: u64, v: u64) -> u64 {
    (ck ^ v) * 0x100000001B3u64
}

/// Géométrie de la table : cap = 2^lg avec 1 <= lg <= 27.
#[logic(open)]
pub fn shape(cap: usize, lg: u32) -> bool {
    pearlite! { 1 <= lg@ && lg@ <= 27 && cap@ == lg@.pow2() }
}

/// Case initiale de sondage de `key`.
#[logic(open)]
pub fn home(key: u64, lg: u32) -> Int {
    pearlite! { ((key * PHI) >> (64u32 - lg))@ }
}

// ---------------------------------------------------------------- modèle : dictionnaire

/// Dictionnaire logique : clé -> valeur (None = absente).
pub type Model = Mapping<u64, Option<u64>>;

/// Valeur renvoyée par une recherche : la valeur associée, 0 si absente.
#[logic(open)]
pub fn get0(m: Model, k: u64) -> u64 {
    match m[k] {
        Some(v) => v,
        None => 0u64,
    }
}

/// Insertion : la valeur s'ajoute (mod 2^64) à celle déjà présente.
#[logic(open)]
pub fn add(m: Model, k: u64, v: u64) -> Model {
    match m[k] {
        Some(w) => m.set(k, Some(w + v)),
        None => m.set(k, Some(v)),
    }
}

/// Modèle après la phase 1 sur i clés : insérer key_l(0, j) avec la valeur j+1, j < i.
#[logic(open)]
#[variant(i)]
pub fn model(i: u64) -> Model {
    pearlite! {
        if i == 0u64 { Mapping::cst(None) } else { add(model(i - 1u64), key_l(0u64, i - 1u64), i) }
    }
}

/// Checksum de i recherches des clés key_l(off, j), j < i, dans m, en partant de ck0.
#[logic(open)]
#[variant(i)]
pub fn ck_lookups(m: Model, off: u64, i: u64, ck0: u64) -> u64 {
    pearlite! {
        if i == 0u64 { ck0 } else { fold(ck_lookups(m, off, i - 1u64, ck0), get0(m, key_l(off, i - 1u64))) }
    }
}

// ---------------------------------------------------------------- invariant de la table

/// Clé de la case s.
#[logic(open)]
pub fn key_at(t: Seq<u64>, s: Int) -> u64 {
    t[2 * s]
}

/// Case atteinte après d pas de sondage depuis h (0 <= h, d < cap).
#[logic(open)]
pub fn at(h: Int, d: Int, cap: Int) -> Int {
    if h + d < cap { h + d } else { h + d - cap }
}

/// Nombre de pas de sondage de h à s.
#[logic(open)]
pub fn dist(h: Int, s: Int, cap: Int) -> Int {
    if h <= s { s - h } else { s + cap - h }
}

#[logic]
#[requires(0 <= h && h < cap && 0 <= s && s < cap)]
#[ensures(0 <= dist(h, s, cap) && dist(h, s, cap) < cap && at(h, dist(h, s, cap), cap) == s)]
pub fn at_dist(h: Int, s: Int, cap: Int) {}

#[logic]
#[requires(0 <= h && h < cap && 0 <= d && d < cap)]
#[ensures(0 <= at(h, d, cap) && at(h, d, cap) < cap && dist(h, at(h, d, cap), cap) == d)]
pub fn dist_at(h: Int, d: Int, cap: Int) {}

/// Nombre de cases occupées parmi [0, c).
#[logic(open)]
#[variant(c)]
pub fn occ(t: Seq<u64>, c: Int) -> Int {
    if c <= 0 { 0 } else if key_at(t, c - 1) == 0u64 { occ(t, c - 1) } else { occ(t, c - 1) + 1 }
}

/// La table t (cap cases) représente le dictionnaire m.
#[logic(open)]
pub fn table_inv(t: Seq<u64>, cap: Int, lg: u32, m: Model) -> bool {
    pearlite! {
        t.len() == 2 * cap
        // (A) toute case occupée porte une clé du modèle avec sa valeur
        && (forall<s: Int> 0 <= s && s < cap && key_at(t, s) != 0u64 ==> m[key_at(t, s)] == Some(t[2 * s + 1]))
        // (B) le chemin de sondage de home(clé) jusqu'à sa case est sans trou
        && (forall<s: Int, d: Int> 0 <= s && s < cap && key_at(t, s) != 0u64
                && 0 <= d && d < dist(home(key_at(t, s), lg), s, cap)
                ==> key_at(t, at(home(key_at(t, s), lg), d, cap)) != 0u64)
        // (C) clés uniques
        && (forall<s1: Int, s2: Int> 0 <= s1 && s1 < cap && 0 <= s2 && s2 < cap
                && key_at(t, s1) != 0u64 && key_at(t, s1) == key_at(t, s2) ==> s1 == s2)
        // (D) toute clé du modèle est non nulle et présente
        && (forall<k: u64> m[k] != None ==> k != 0u64 && exists<s: Int> 0 <= s && s < cap && key_at(t, s) == k)
    }
}

// ---------------------------------------------------------------- lemmes de bits (mode bitvecteur)

/// `x | 1` n'est jamais nul.
#[logic]
#[bitwise_proof]
#[ensures((x | 1u64) != 0u64)]
pub fn or1_nz(x: u64) {}

/// Les lg bits de poids fort de x forment un entier < 2^lg.
#[logic]
#[bitwise_proof]
#[requires(1 <= lg@ && lg@ <= 27)]
#[ensures((x >> (64u32 - lg))@ < lg@.pow2())]
pub fn shr_lt_pow2(x: u64, lg: u32) {}

/// 2^a < 2^b dès que 0 <= a < b.
#[logic]
#[variant(b - a)]
#[requires(0 <= a && a < b)]
#[ensures(a.pow2() < b.pow2())]
pub fn pow2_lt(a: Int, b: Int) {
    if a + 1 < b {
        pow2_lt(a + 1, b)
    }
}

// ---------------------------------------------------------------- lemmes de l'invariant

/// Mise à jour de la valeur d'une clé présente.
#[logic]
#[requires(table_inv(t0, cap, lg, m) && 0 <= s && s < cap && key_at(t0, s) == key && key != 0u64)]
#[requires(t1.len() == t0.len() && t1[2 * s + 1] == t0[2 * s + 1] + v)]
#[requires(forall<j: Int> 0 <= j && j < t0.len() && j != 2 * s + 1 ==> t1[j] == t0[j])]
#[ensures(table_inv(t1, cap, lg, add(m, key, v)))]
pub fn inv_update(t0: Seq<u64>, t1: Seq<u64>, cap: Int, lg: u32, m: Model, s: Int, key: u64, v: u64) {
    pearlite! {
        proof_assert!(m[key] == Some(t0[2 * s + 1]));
        proof_assert!(add(m, key, v) == m.set(key, Some(t0[2 * s + 1] + v)));
        proof_assert!(forall<p: Int> 0 <= p && p < cap ==> key_at(t1, p) == key_at(t0, p));
        proof_assert!(forall<p: Int> 0 <= p && p < cap && p != s && key_at(t0, p) != 0u64 ==>
            key_at(t0, p) != key && t1[2 * p + 1] == t0[2 * p + 1])
    }
}

/// Écriture d'une nouvelle clé dans la case vide s, au bout d'un chemin sans trou depuis home(key).
#[logic]
#[requires(table_inv(t0, cap, lg, m) && 0 <= s && s < cap && key_at(t0, s) == 0u64 && key != 0u64)]
#[requires(m[key] == None)]
#[requires(forall<d: Int> 0 <= d && d < dist(home(key, lg), s, cap) ==>
    key_at(t0, at(home(key, lg), d, cap)) != 0u64)]
#[requires(t1.len() == t0.len() && t1[2 * s] == key && t1[2 * s + 1] == v)]
#[requires(forall<j: Int> 0 <= j && j < t0.len() && j != 2 * s && j != 2 * s + 1 ==> t1[j] == t0[j])]
#[ensures(table_inv(t1, cap, lg, add(m, key, v)))]
pub fn inv_insert(t0: Seq<u64>, t1: Seq<u64>, cap: Int, lg: u32, m: Model, s: Int, key: u64, v: u64) {
    pearlite! {
        proof_assert!(add(m, key, v) == m.set(key, Some(v)));
        proof_assert!(forall<p: Int> 0 <= p && p < cap && p != s ==>
            key_at(t1, p) == key_at(t0, p) && t1[2 * p + 1] == t0[2 * p + 1]);
        proof_assert!(forall<p: Int> 0 <= p && p < cap && key_at(t0, p) != 0u64 ==> key_at(t1, p) != 0u64);
        proof_assert!(forall<p: Int> 0 <= p && p < cap && key_at(t0, p) != 0u64 ==> key_at(t0, p) != key)
    }
}

// ---------------------------------------------------------------- lemmes sur occ

#[logic]
#[variant(c)]
#[requires(0 <= c && 2 * c <= t.len() && occ(t, c) < c)]
#[ensures(exists<e: Int> 0 <= e && e < c && key_at(t, e) == 0u64)]
pub fn occ_has_empty(t: Seq<u64>, c: Int) {
    if c > 0 && key_at(t, c - 1) != 0u64 {
        occ_has_empty(t, c - 1)
    }
}

#[logic]
#[variant(c)]
#[requires(0 <= c)]
#[requires(forall<s: Int> 0 <= s && s < c ==> key_at(t, s) == 0u64)]
#[ensures(occ(t, c) == 0)]
pub fn occ_zero(t: Seq<u64>, c: Int) {
    if c > 0 {
        occ_zero(t, c - 1)
    }
}

/// Les cases autres que s0 gardent leur clé, et s0 était vide : au plus une case occupée de plus.
#[logic]
#[variant(c)]
#[requires(0 <= c && key_at(t0, s0) == 0u64)]
#[requires(forall<s: Int> 0 <= s && s < c && s != s0 ==> key_at(t1, s) == key_at(t0, s))]
#[ensures(occ(t1, c) <= occ(t0, c) + 1)]
#[ensures(s0 < 0 || c <= s0 ==> occ(t1, c) == occ(t0, c))]
pub fn occ_fill(t0: Seq<u64>, t1: Seq<u64>, c: Int, s0: Int) {
    if c > 0 {
        occ_fill(t0, t1, c - 1, s0)
    }
}

// ---------------------------------------------------------------- code

#[ensures(result == mix64_l(x))]
#[check(terminates)]
#[inline(always)]
fn mix64(x: u64) -> u64 {
    let z = x.wrapping_add(0x9E3779B97F4A7C15);
    let z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    let z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

#[ensures(result == fold(ck, v))]
#[check(terminates)]
#[inline(always)]
fn ckfold(ck: u64, v: u64) -> u64 {
    (ck ^ v).wrapping_mul(0x100000001B3)
}

#[ensures(result == key_l(off, i) && result != 0u64)]
#[check(terminates)]
#[inline(always)]
pub fn key_of(off: u64, i: u64) -> u64 {
    let r = mix64(SEED.wrapping_add(off).wrapping_add(i)) | 1;
    proof_assert!({ or1_nz(mix64_l(SEED + off + i)); true });
    r
}

/// Position initiale : (key * PHI) >> (64 - lg), toujours < 2^lg (= cap).
#[requires(1 <= lg@ && lg@ <= 27)]
#[ensures(result@ == home(key, lg) && result@ < lg@.pow2())]
#[check(terminates)]
#[inline(always)]
pub fn slot(key: u64, lg: u32) -> usize {
    let sh = 64 - lg;
    proof_assert!(sh == 64u32 - lg);
    proof_assert!({ shr_lt_pow2(key * PHI, lg); true });
    (key.wrapping_mul(PHI) >> sh) as usize
}

/// Insère `key` (valeur i1, ajoutée si la clé est déjà là). La table doit garder une case vide après.
#[requires(shape(cap, lg) && table_inv(t@, cap@, lg, *m))]
#[requires(occ(t@, cap@) + 1 < cap@ && key != 0u64)]
#[ensures(table_inv((^t)@, cap@, lg, add(*m, key, i1)))]
#[ensures(occ((^t)@, cap@) <= occ(t@, cap@) + 1)]
#[check(terminates)]
pub fn insert(t: &mut Vec<u64>, cap: usize, lg: u32, key: u64, i1: u64, m: Snapshot<Model>) {
    let t0 = snapshot!(t@);
    let h = snapshot!(home(key, lg));
    let mut s = slot(key, lg);
    proof_assert!(s@ == *h && *h < cap@);
    let mut tries = snapshot!(0);
    proof_assert!({ occ_has_empty(*t0, cap@); true });
    #[invariant(t@ == *t0)]
    #[invariant(0 <= *tries && *tries < cap@ && s@ == at(*h, *tries, cap@))]
    #[invariant(forall<d: Int> 0 <= d && d < *tries ==>
        key_at(t@, at(*h, d, cap@)) != 0u64 && key_at(t@, at(*h, d, cap@)) != key)]
    #[variant(cap@ - *tries)]
    loop {
        let k = t[2 * s];
        if k == 0 {
            // key est absente : sinon sa case serait sur le chemin [h, s), déjà parcouru.
            proof_assert!(forall<s2: Int> 0 <= s2 && s2 < cap@ && key_at(*t0, s2) == key ==>
                { at_dist(*h, s2, cap@); false });
            proof_assert!((*m)[key] == None);
            proof_assert!({ dist_at(*h, *tries, cap@); dist(*h, s@, cap@) == *tries });
            t[2 * s] = key;
            t[2 * s + 1] = i1;
            proof_assert!({ occ_fill(*t0, t@, cap@, s@); true });
            proof_assert!(t[2 * s@] == key && t[2 * s@ + 1] == i1 && t@.len() == (*t0).len());
            proof_assert!(forall<j: Int> 0 <= j && j < t@.len() && j != 2 * s@ && j != 2 * s@ + 1 ==> t[j] == (*t0)[j]);
            proof_assert!(forall<d: Int> 0 <= d && d < dist(home(key, lg), s@, cap@) ==>
                key_at(*t0, at(home(key, lg), d, cap@)) != 0u64);
            proof_assert!({ inv_insert(*t0, t@, cap@, lg, *m, s@, key, i1); table_inv(t@, cap@, lg, add(*m, key, i1)) });
            return;
        }
        if k == key {
            t[2 * s + 1] = t[2 * s + 1].wrapping_add(i1);
            proof_assert!({ occ_fill(*t0, t@, cap@, -1); true });
            proof_assert!(t[2 * s@ + 1] == (*t0)[2 * s@ + 1] + i1);
            proof_assert!(forall<j: Int> 0 <= j && j < t@.len() && j != 2 * s@ + 1 ==> t[j] == (*t0)[j]);
            proof_assert!({ inv_update(*t0, t@, cap@, lg, *m, s@, key, i1); table_inv(t@, cap@, lg, add(*m, key, i1)) });
            return;
        }
        // La case vide e est à une distance > tries : on ne fait pas le tour de la table.
        proof_assert!(forall<e: Int> 0 <= e && e < cap@ && key_at(t@, e) == 0u64 ==>
            { at_dist(*h, e, cap@); *tries < dist(*h, e, cap@) });
        proof_assert!(*tries + 1 < cap@);
        s += 1;
        if s == cap {
            s = 0;
        }
        tries = snapshot!(*tries + 1);
    }
}

/// Recherche : la valeur associée à `key` dans le modèle, 0 si absente.
#[requires(shape(cap, lg) && table_inv(t@, cap@, lg, *m) && occ(t@, cap@) < cap@ && key != 0u64)]
#[ensures(result == get0(*m, key))]
#[check(terminates)]
pub fn lookup(t: &Vec<u64>, cap: usize, lg: u32, key: u64, m: Snapshot<Model>) -> u64 {
    let h = snapshot!(home(key, lg));
    let mut s = slot(key, lg);
    proof_assert!(s@ == *h && *h < cap@);
    let mut tries = snapshot!(0);
    proof_assert!({ occ_has_empty(t@, cap@); true });
    #[invariant(0 <= *tries && *tries < cap@ && s@ == at(*h, *tries, cap@))]
    #[invariant(forall<d: Int> 0 <= d && d < *tries ==>
        key_at(t@, at(*h, d, cap@)) != 0u64 && key_at(t@, at(*h, d, cap@)) != key)]
    #[variant(cap@ - *tries)]
    loop {
        let k = t[2 * s];
        if k == 0 {
            proof_assert!(forall<s2: Int> 0 <= s2 && s2 < cap@ && key_at(t@, s2) == key ==>
                { at_dist(*h, s2, cap@); false });
            return 0;
        }
        if k == key {
            return t[2 * s + 1];
        }
        proof_assert!(forall<e: Int> 0 <= e && e < cap@ && key_at(t@, e) == 0u64 ==>
            { at_dist(*h, e, cap@); *tries < dist(*h, e, cap@) });
        s += 1;
        if s == cap {
            s = 0;
        }
        tries = snapshot!(*tries + 1);
    }
}

/// Phase 1 : insère key_l(0, i) avec la valeur i+1 pour i < n.
#[requires(shape(cap, lg) && t@.len() == 2 * cap@ && 1 <= n@ && 2 * n@ <= cap@)]
#[requires(forall<j: Int> 0 <= j && j < t@.len() ==> t[j] == 0u64)]
#[ensures(table_inv((^t)@, cap@, lg, model(n as u64)) && occ((^t)@, cap@) <= n@)]
#[check(terminates)]
pub fn insert_all(t: &mut Vec<u64>, cap: usize, lg: u32, n: usize) {
    proof_assert!({ occ_zero(t@, cap@); true });
    proof_assert!(model(0u64) == Mapping::cst(None));
    let mut i = 0usize;
    #[invariant(i@ <= n@ && table_inv(t@, cap@, lg, model(i as u64)) && occ(t@, cap@) <= i@)]
    #[variant(n@ - i@)]
    while i < n {
        let m = snapshot!(model(i as u64));
        proof_assert!((i + 1usize) as u64 - 1u64 == i as u64);
        insert(t, cap, lg, key_of(0, i as u64), (i as u64).wrapping_add(1), m);
        i += 1;
    }
}

/// Phases 2 et 3 : replie dans ck0 les recherches de key_l(off, i), i < n.
#[requires(shape(cap, lg) && table_inv(t@, cap@, lg, *m) && occ(t@, cap@) < cap@)]
#[requires(n@ <= 67108864)]
#[ensures(result == ck_lookups(*m, off, n as u64, ck0))]
#[check(terminates)]
pub fn lookup_all(t: &Vec<u64>, cap: usize, lg: u32, n: usize, off: u64, ck0: u64, m: Snapshot<Model>) -> u64 {
    let mut ck = ck0;
    let mut i = 0usize;
    #[invariant(i@ <= n@ && ck == ck_lookups(*m, off, i as u64, ck0))]
    #[variant(n@ - i@)]
    while i < n {
        proof_assert!((i + 1usize) as u64 - 1u64 == i as u64);
        ck = ckfold(ck, lookup(t, cap, lg, key_of(off, i as u64), m));
        i += 1;
    }
    ck
}

#[requires(1 <= n@ && n@ <= 67108864)]
#[ensures(result == ck_lookups(model(n as u64), n as u64, n as u64,
                               ck_lookups(model(n as u64), 0u64, n as u64, CK_INIT)))]
// Pas de `#[check(terminates)]` : `std::vec::from_elem` n'est pas spécifié `terminates` dans creusot-std.
pub fn run(n: usize) -> u64 {
    let n2 = n + n;
    let mut cap = 2usize;
    let mut lg = 1u32;
    // cap = plus petite puissance de 2 >= 2n ; n <= 2^26 donne lg <= 27 sans garde.
    #[invariant(shape(cap, lg) && cap@ < 2 * n2@)]
    #[variant(n2@ - cap@)]
    while cap < n2 {
        proof_assert!(lg@ < 27 || { pow2_lt(26, lg@); false });
        cap += cap;
        lg += 1;
    }
    let mut t = vec![0u64; 2 * cap];
    insert_all(&mut t, cap, lg, n);
    let m = snapshot!(model(n as u64));
    let ck1 = lookup_all(&t, cap, lg, n, 0, CK_INIT, m);
    lookup_all(&t, cap, lg, n, n as u64, ck1, m)
}
