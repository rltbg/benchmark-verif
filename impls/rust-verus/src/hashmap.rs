//! Table de hachage à adressage ouvert, sondage linéaire, couples (clé, valeur) entrelacés dans un
//! seul tableau de 2*cap u64 (cap = plus petite puissance de 2 >= 2n). 1 <= n <= 2^26.
//!
//! Prouvé avec Verus (correction fonctionnelle complète, terminaison comprise) :
//!   * la table réalise un dictionnaire logique `Map<u64, u64>` : invariant `table_inv` (toute clé
//!     présente est au bout d'un chemin de sondage sans trou depuis `home`, clés uniques, valeurs = celles
//!     du modèle) ;
//!   * `insert` ajoute (somme modulo 2^64 si la clé existe), `lookup` renvoie la valeur du modèle, 0 si absente ;
//!   * après les insertions le modèle est `model(n)` (fonction des seules clés `key_l(0, i)`), et `run`
//!     renvoie exactement le checksum logique des 2n recherches (`ck_lookups`) ;
//!   * terminaison des sondages *sans compteur d'essais à l'exécution* : la table reste à moitié vide
//!     (`occ` < cap), donc une case vide arrête toujours la recherche ;
//!   * `slot` sans garde `if s < cap` : prouvé `(key*PHI) >> (64-lg) < 2^lg = cap` (bit_vector).
//! Aucun `assume`, `admit` ni `external_body`.
use crate::common::*;
use vstd::prelude::*;

verus! {

pub const SEED: u64 = 0x2468ACE013579BDF;

pub const PHI: u64 = 0x9E3779B97F4A7C15;

// ---------------------------------------------------------------- fonctions de spécification

/// Clé numéro i de la série off.
pub open spec fn key_l(off: u64, i: u64) -> u64 {
    spec_mix64(SEED.wrapping_add(off).wrapping_add(i)) | 1u64
}

/// Géométrie : cap = 2^lg, 1 <= lg <= 27 (donc 2 <= cap <= 2^27).
pub open spec fn shape(cap: usize, lg: u32) -> bool {
    &&& 1 <= lg <= 27
    &&& cap as u64 == 1u64 << lg
    &&& 2 <= cap <= 134217728
}

/// Case initiale de sondage de key.
pub open spec fn home(key: u64, lg: u32) -> int {
    (key.wrapping_mul(PHI) >> (64 - lg) as u32) as int
}

/// Valeur renvoyée par une recherche dans le dictionnaire m : la valeur associée, 0 si absente.
pub open spec fn get0(m: Map<u64, u64>, k: u64) -> u64 {
    if m.contains_key(k) {
        m[k]
    } else {
        0
    }
}

/// Insertion : la valeur s'ajoute (mod 2^64) à celle déjà présente.
pub open spec fn add(m: Map<u64, u64>, k: u64, v: u64) -> Map<u64, u64> {
    if m.contains_key(k) {
        m.insert(k, m[k].wrapping_add(v))
    } else {
        m.insert(k, v)
    }
}

/// Dictionnaire après la phase 1 sur i clés : key_l(0, j) avec la valeur j+1, j < i.
pub open spec fn model(i: nat) -> Map<u64, u64>
    decreases i,
{
    if i == 0 {
        Map::empty()
    } else {
        add(model((i - 1) as nat), key_l(0, (i - 1) as u64), i as u64)
    }
}

/// Checksum de i recherches des clés key_l(off, j), j < i, dans m, en partant de ck0.
pub open spec fn ck_lookups(m: Map<u64, u64>, off: u64, i: nat, ck0: u64) -> u64
    decreases i,
{
    if i == 0 {
        ck0
    } else {
        spec_ckfold(ck_lookups(m, off, (i - 1) as nat, ck0), get0(m, key_l(off, (i - 1) as u64)))
    }
}

// ---------------------------------------------------------------- invariant de la table

/// Clé de la case s.
pub open spec fn key_at(t: Seq<u64>, s: int) -> u64 {
    t[2 * s]
}

/// Case atteinte après d pas de sondage depuis h.
pub open spec fn at(h: int, d: int, cap: int) -> int {
    if h + d < cap {
        h + d
    } else {
        h + d - cap
    }
}

/// Nombre de pas de sondage de h à s.
pub open spec fn dist(h: int, s: int, cap: int) -> int {
    if h <= s {
        s - h
    } else {
        s + cap - h
    }
}

/// Nombre de cases occupées parmi [0, c).
pub open spec fn occ(t: Seq<u64>, c: int) -> nat
    decreases c,
{
    if c <= 0 {
        0
    } else if key_at(t, c - 1) == 0 {
        occ(t, c - 1)
    } else {
        occ(t, c - 1) + 1
    }
}

/// La table t (cap cases) représente le dictionnaire m.
pub open spec fn table_inv(t: Seq<u64>, cap: int, lg: u32, m: Map<u64, u64>) -> bool {
    &&& t.len() == 2 * cap
    // (A) toute case occupée porte une clé du modèle avec sa valeur
    &&& forall|s: int|
        0 <= s < cap && #[trigger] key_at(t, s) != 0 ==> m.contains_key(key_at(t, s)) && m[key_at(
            t,
            s,
        )] == t[2 * s + 1]
    // (B) le chemin de sondage de home(clé) jusqu'à sa case est sans trou
    &&& forall|s: int, d: int|
        0 <= s < cap && key_at(t, s) != 0 && 0 <= d < dist(home(key_at(t, s), lg), s, cap)
            ==> #[trigger] key_at(t, at(home(key_at(t, s), lg), d, cap)) != 0
    // (C) clés uniques
    &&& forall|s1: int, s2: int|
        0 <= s1 < cap && 0 <= s2 < cap && #[trigger] key_at(t, s1) != 0 && key_at(t, s1)
            == #[trigger] key_at(t, s2) ==> s1 == s2
    // (D) toute clé du modèle est non nulle et présente
    &&& forall|k: u64|
        #[trigger] m.contains_key(k) ==> k != 0 && exists|s: int| 0 <= s < cap && key_at(t, s) == k
}

// ---------------------------------------------------------------- lemmes

proof fn lemma_at_dist(h: int, s: int, cap: int)
    requires
        0 <= h < cap,
        0 <= s < cap,
    ensures
        0 <= dist(h, s, cap) < cap,
        at(h, dist(h, s, cap), cap) == s,
{
}

proof fn lemma_dist_at(h: int, d: int, cap: int)
    requires
        0 <= h < cap,
        0 <= d < cap,
    ensures
        0 <= at(h, d, cap) < cap,
        dist(h, at(h, d, cap), cap) == d,
{
}

/// Moins de c cases occupées parmi c : il existe une case vide.
proof fn lemma_occ_has_empty(t: Seq<u64>, c: int)
    requires
        0 <= c,
        2 * c <= t.len(),
        occ(t, c) < c,
    ensures
        exists|e: int| 0 <= e < c && key_at(t, e) == 0,
    decreases c,
{
    if key_at(t, c - 1) != 0 {
        lemma_occ_has_empty(t, c - 1);
    }
}

proof fn lemma_occ_zero(t: Seq<u64>, c: int)
    requires
        0 <= c,
        forall|s: int| 0 <= s < c ==> #[trigger] key_at(t, s) == 0,
    ensures
        occ(t, c) == 0,
    decreases c,
{
    if c > 0 {
        lemma_occ_zero(t, c - 1);
    }
}

/// Les cases autres que s0 gardent leur clé et s0 était vide : au plus une case occupée de plus.
proof fn lemma_occ_fill(t0: Seq<u64>, t1: Seq<u64>, c: int, s0: int)
    requires
        0 <= c,
        key_at(t0, s0) == 0 || s0 < 0,
        forall|s: int| 0 <= s < c && s != s0 ==> #[trigger] key_at(t1, s) == key_at(t0, s),
    ensures
        occ(t1, c) <= occ(t0, c) + 1,
        s0 < 0 || c <= s0 ==> occ(t1, c) == occ(t0, c),
    decreases c,
{
    if c > 0 {
        lemma_occ_fill(t0, t1, c - 1, s0);
    }
}

/// Mise à jour de la valeur d'une clé présente.
proof fn lemma_inv_update(
    t0: Seq<u64>,
    t1: Seq<u64>,
    cap: int,
    lg: u32,
    m: Map<u64, u64>,
    s: int,
    key: u64,
    v: u64,
)
    requires
        table_inv(t0, cap, lg, m),
        0 <= s < cap,
        key_at(t0, s) == key,
        key != 0,
        t1 == t0.update(2 * s + 1, t0[2 * s + 1].wrapping_add(v)),
    ensures
        table_inv(t1, cap, lg, add(m, key, v)),
{
    let m1 = add(m, key, v);
    assert(m.contains_key(key) && m[key] == t0[2 * s + 1]);
    assert forall|p: int| 0 <= p < cap implies #[trigger] key_at(t1, p) == key_at(t0, p) by {}
    assert forall|p: int| 0 <= p < cap && #[trigger] key_at(t1, p) != 0 implies m1.contains_key(
        key_at(t1, p),
    ) && m1[key_at(t1, p)] == t1[2 * p + 1] by {
        if p != s {
            assert(key_at(t0, p) != key);
        }
    }
    assert forall|k: u64| #[trigger] m1.contains_key(k) implies k != 0 && exists|p: int|
        0 <= p < cap && key_at(t1, p) == k by {
        if k != key {
            let p = choose|p: int| 0 <= p < cap && key_at(t0, p) == k;
            assert(key_at(t1, p) == k);
        } else {
            assert(key_at(t1, s) == k);
        }
    }
}

/// Écriture d'une nouvelle clé dans la case vide s, au bout d'un chemin sans trou depuis home(key).
proof fn lemma_inv_insert(
    t0: Seq<u64>,
    t1: Seq<u64>,
    cap: int,
    lg: u32,
    m: Map<u64, u64>,
    s: int,
    key: u64,
    v: u64,
)
    requires
        table_inv(t0, cap, lg, m),
        0 <= s < cap,
        key_at(t0, s) == 0,
        key != 0,
        !m.contains_key(key),
        forall|d: int|
            0 <= d < dist(home(key, lg), s, cap) ==> #[trigger] key_at(
                t0,
                at(home(key, lg), d, cap),
            ) != 0,
        t1 == t0.update(2 * s, key).update(2 * s + 1, v),
    ensures
        table_inv(t1, cap, lg, add(m, key, v)),
{
    let m1 = add(m, key, v);
    assert forall|p: int| 0 <= p < cap && p != s implies #[trigger] key_at(t1, p) == key_at(t0, p)
        && t1[2 * p + 1] == t0[2 * p + 1] by {}
    assert(key_at(t1, s) == key);
    // (A)
    assert forall|p: int| 0 <= p < cap && #[trigger] key_at(t1, p) != 0 implies m1.contains_key(
        key_at(t1, p),
    ) && m1[key_at(t1, p)] == t1[2 * p + 1] by {
        if p != s {
            assert(key_at(t0, p) != key);
        }
    }
    // (B)
    assert forall|p: int, d: int|
        0 <= p < cap && key_at(t1, p) != 0 && 0 <= d < dist(home(key_at(t1, p), lg), p, cap)
            implies #[trigger] key_at(t1, at(home(key_at(t1, p), lg), d, cap)) != 0 by {
        let q = at(home(key_at(t1, p), lg), d, cap);
        if p != s {
            assert(key_at(t0, q) != 0);
        } else {
            assert(key_at(t0, q) != 0);
        }
        if q != s {
            assert(key_at(t1, q) == key_at(t0, q));
        }
    }
    // (C)
    assert forall|s1: int, s2: int|
        0 <= s1 < cap && 0 <= s2 < cap && #[trigger] key_at(t1, s1) != 0 && key_at(t1, s1)
            == #[trigger] key_at(t1, s2) implies s1 == s2 by {
        if s1 != s && s2 != s {
            assert(key_at(t0, s1) == key_at(t0, s2));
        } else if s1 != s {
            assert(key_at(t0, s1) == key);
        } else if s2 != s {
            assert(key_at(t0, s2) == key);
        }
    }
    // (D)
    assert forall|k: u64| #[trigger] m1.contains_key(k) implies k != 0 && exists|p: int|
        0 <= p < cap && key_at(t1, p) == k by {
        if k != key {
            let p = choose|p: int| 0 <= p < cap && key_at(t0, p) == k;
            assert(key_at(t1, p) == k);
        }
    }
}

// ---------------------------------------------------------------- code

#[inline(always)]
pub fn key_of(off: u64, i: u64) -> (r: u64)
    ensures
        r == key_l(off, i),
        r != 0,
{
    let x = mix64(SEED.wrapping_add(off).wrapping_add(i));
    assert((x | 1u64) != 0u64) by (bit_vector);
    x | 1
}

/// Position initiale : (key * PHI) >> (64 - lg), toujours < 2^lg (= cap).
#[inline(always)]
pub fn slot(key: u64, lg: u32) -> (s: usize)
    requires
        1 <= lg <= 27,
    ensures
        s as int == home(key, lg),
        (s as u64) < (1u64 << lg),
{
    let x = key.wrapping_mul(PHI);
    let sh = 64 - lg;
    let r = x >> sh;
    assert(r < (1u64 << lg) && r < 134217728u64) by (bit_vector)
        requires
            r == x >> sh,
            sh == 64 - lg,
            1 <= lg <= 27,
    ;
    r as usize
}

/// Insère key (valeur i1, ajoutée si la clé est déjà là). La table doit garder une case vide après.
pub fn insert(t: &mut Vec<u64>, cap: usize, lg: u32, key: u64, i1: u64, Ghost(m): Ghost<Map<u64, u64>>)
    requires
        shape(cap, lg),
        table_inv(t@, cap as int, lg, m),
        occ(t@, cap as int) + 1 < cap,
        key != 0,
    ensures
        table_inv(final(t)@, cap as int, lg, add(m, key, i1)),
        occ(final(t)@, cap as int) <= occ(old(t)@, cap as int) + 1,
{
    let ghost t0 = t@;
    let ghost h = home(key, lg);
    let mut s = slot(key, lg);
    let ghost mut tries: int = 0;
    proof {
        lemma_occ_has_empty(t0, cap as int);
    }
    loop
        invariant
            t@ == t0,
            t0 == old(t)@,
            shape(cap, lg),
            table_inv(t0, cap as int, lg, m),
            occ(t0, cap as int) + 1 < cap,
            key != 0,
            h == home(key, lg),
            0 <= h < cap,
            0 <= tries < cap,
            s == at(h, tries, cap as int),
            forall|d: int|
                0 <= d < tries ==> #[trigger] key_at(t0, at(h, d, cap as int)) != 0 && key_at(
                    t0,
                    at(h, d, cap as int),
                ) != key,
            exists|e: int| 0 <= e < cap && key_at(t0, e) == 0,
        decreases cap - tries,
    {
        let k = t[2 * s];
        if k == 0 {
            proof {
                // key est absente : sinon sa case serait sur le chemin [h, s), déjà parcouru.
                if m.contains_key(key) {
                    let s2 = choose|s2: int| 0 <= s2 < cap && key_at(t0, s2) == key;
                    lemma_at_dist(h, s2, cap as int);
                    let d2 = dist(h, s2, cap as int);
                    if d2 > tries {
                        assert(key_at(t0, at(h, tries, cap as int)) != 0);
                    }
                }
                lemma_dist_at(h, tries, cap as int);
            }
            t[2 * s] = key;
            t[2 * s + 1] = i1;
            proof {
                lemma_occ_fill(t0, t@, cap as int, s as int);
                lemma_inv_insert(t0, t@, cap as int, lg, m, s as int, key, i1);
            }
            return;
        }
        if k == key {
            t[2 * s + 1] = t[2 * s + 1].wrapping_add(i1);
            proof {
                lemma_occ_fill(t0, t@, cap as int, -1);
                lemma_inv_update(t0, t@, cap as int, lg, m, s as int, key, i1);
            }
            return;
        }
        proof {
            // La case vide e est à une distance > tries : on ne fait pas le tour de la table.
            let e = choose|e: int| 0 <= e < cap && key_at(t0, e) == 0;
            lemma_at_dist(h, e, cap as int);
        }
        s += 1;
        if s == cap {
            s = 0;
        }
        proof {
            tries = tries + 1;
        }
    }
}

/// Recherche : la valeur associée à key dans le modèle, 0 si absente.
pub fn lookup(t: &Vec<u64>, cap: usize, lg: u32, key: u64, Ghost(m): Ghost<Map<u64, u64>>) -> (r: u64)
    requires
        shape(cap, lg),
        table_inv(t@, cap as int, lg, m),
        occ(t@, cap as int) < cap,
        key != 0,
    ensures
        r == get0(m, key),
{
    let ghost h = home(key, lg);
    let mut s = slot(key, lg);
    let ghost mut tries: int = 0;
    proof {
        lemma_occ_has_empty(t@, cap as int);
    }
    loop
        invariant
            shape(cap, lg),
            table_inv(t@, cap as int, lg, m),
            key != 0,
            h == home(key, lg),
            0 <= h < cap,
            0 <= tries < cap,
            s == at(h, tries, cap as int),
            forall|d: int|
                0 <= d < tries ==> #[trigger] key_at(t@, at(h, d, cap as int)) != 0 && key_at(
                    t@,
                    at(h, d, cap as int),
                ) != key,
            exists|e: int| 0 <= e < cap && key_at(t@, e) == 0,
        decreases cap - tries,
    {
        let k = t[2 * s];
        if k == 0 {
            proof {
                if m.contains_key(key) {
                    let s2 = choose|s2: int| 0 <= s2 < cap && key_at(t@, s2) == key;
                    lemma_at_dist(h, s2, cap as int);
                    let d2 = dist(h, s2, cap as int);
                    if d2 > tries {
                        assert(key_at(t@, at(h, tries, cap as int)) != 0);
                    }
                }
            }
            return 0;
        }
        if k == key {
            assert(key_at(t@, s as int) == key);
            return t[2 * s + 1];
        }
        proof {
            let e = choose|e: int| 0 <= e < cap && key_at(t@, e) == 0;
            lemma_at_dist(h, e, cap as int);
        }
        s += 1;
        if s == cap {
            s = 0;
        }
        proof {
            tries = tries + 1;
        }
    }
}

/// Phase 1 : insère key_l(0, i) avec la valeur i+1 pour i < n.
pub fn insert_all(t: &mut Vec<u64>, cap: usize, lg: u32, n: usize)
    requires
        shape(cap, lg),
        t@.len() == 2 * cap,
        1 <= n,
        2 * n <= cap,
        forall|j: int| 0 <= j < t@.len() ==> t@[j] == 0,
    ensures
        table_inv(final(t)@, cap as int, lg, model(n as nat)),
        occ(final(t)@, cap as int) <= n,
{
    proof {
        lemma_occ_zero(t@, cap as int);
    }
    let mut i: usize = 0;
    while i < n
        invariant
            shape(cap, lg),
            2 * n <= cap,
            i <= n,
            table_inv(t@, cap as int, lg, model(i as nat)),
            occ(t@, cap as int) <= i,
        decreases n - i,
    {
        insert(t, cap, lg, key_of(0, i as u64), (i as u64).wrapping_add(1), Ghost(model(i as nat)));
        i += 1;
    }
}

/// Phases 2 et 3 : replie dans ck0 les recherches de key_l(off, i), i < n.
pub fn lookup_all(
    t: &Vec<u64>,
    cap: usize,
    lg: u32,
    n: usize,
    off: u64,
    ck0: u64,
    Ghost(m): Ghost<Map<u64, u64>>,
) -> (r: u64)
    requires
        shape(cap, lg),
        table_inv(t@, cap as int, lg, m),
        occ(t@, cap as int) < cap,
        n <= 67108864,
    ensures
        r == ck_lookups(m, off, n as nat, ck0),
{
    let mut ck = ck0;
    let mut i: usize = 0;
    while i < n
        invariant
            shape(cap, lg),
            table_inv(t@, cap as int, lg, m),
            occ(t@, cap as int) < cap,
            i <= n <= 67108864,
            ck == ck_lookups(m, off, i as nat, ck0),
        decreases n - i,
    {
        ck = ckfold(ck, lookup(t, cap, lg, key_of(off, i as u64), Ghost(m)));
        i += 1;
    }
    ck
}

pub fn run(n: usize, _threads: usize) -> (r: u64)
    requires
        1 <= n <= 67108864,
    ensures
        r == ck_lookups(
            model(n as nat),
            n as u64,
            n as nat,
            ck_lookups(model(n as nat), 0, n as nat, CK_INIT),
        ),
{
    let n2 = n + n;
    let mut cap: usize = 2;
    let mut lg: u32 = 1;
    assert(2u64 == 1u64 << 1u32) by (bit_vector);
    assert(cap as u64 == 2u64);
    // cap = plus petite puissance de 2 >= 2n ; n <= 2^26 donne lg <= 27 sans garde.
    while cap < n2
        invariant
            shape(cap, lg),
            2 <= cap,
            n2 == n + n,
            n <= 67108864,
            cap < 2 * n2,
        decreases 27 - lg,
    {
        let ghost c = cap as u64;
        assert(lg < 27 && c + c == 1u64 << (lg + 1) as u32 && c + c <= 134217728u64) by (bit_vector)
            requires
                c == 1u64 << lg,
                c < 134217728u64,
                1 <= lg <= 27,
        ;
        cap += cap;
        lg += 1;
    }
    let mut t = vec![0u64; 2 * cap];
    insert_all(&mut t, cap, lg, n);
    let ck1 = lookup_all(&t, cap, lg, n, 0, CK_INIT, Ghost(model(n as nat)));
    lookup_all(&t, cap, lg, n, n as u64, ck1, Ghost(model(n as nat)))
}

} // verus!
