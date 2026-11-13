//! Allocation en masse (spec/workloads.d/alloc.md) : n blocs de 8..256 octets, un bloc sur 16 conservé dans une
//! fenêtre circulaire de W = min(max(1, n/64), 2^25) blocs.
//!
//! Prouvé (Creusot) :
//! * `fill` : `b[k] = (i + k) mod 251` ; `sum` : somme (modulo 2^64) du contenu relu ;
//! * `run` : le résultat est `fold(fold_seq(sums, n), ⌈n/16⌉)` où `sums[i]` est la somme logique du contenu du
//!   bloc `i` (`block_sum(i, size_l(i))`) — taille et contenu sont ceux de la spécification ;
//! * fenêtre FIFO : après `kept` conservations, l'emplacement `j mod W` de l'anneau contient le bloc conservé
//!   n° `j` (bloc d'indice `16 j`, contenu vérifié) pour tout `j` des `W` derniers, et les emplacements jamais
//!   utilisés sont vides ; le bloc remplacé est libéré par `Drop` à l'affectation ;
//! * absence de panique et de débordement ; terminaison de `fill` et `sum`.
//!
//! Aucun `trusted` : la barrière `black_box` a un contrat dans `creusot-std` (`result == dummy`).
//! Non vérifié : la terminaison de `run` (elle alloue : `vec![x; n]` n'est pas `check(terminates)` dans
//! `creusot-std`) ; la libération effective des blocs est faite par `Drop`, que Creusot ne spécifie pas.
use crate::common::*;
use creusot_std::prelude::*;

pub const WMAX: usize = 33554432; // 2^25
pub const SEED: u64 = 0xA110C00000000001;

// ---------------------------------------------------------------- modèle logique

/// splitmix64 dans la logique (arithmétique modulo 2^64).
#[logic(open)]
pub fn mix_l(x: u64) -> u64 {
    pearlite! {
        let z = x + 0x9E3779B97F4A7C15u64;
        let z = (z ^ (z >> 30u64)) * 0xBF58476D1CE4E5B9u64;
        let z = (z ^ (z >> 27u64)) * 0x94D049BB133111EBu64;
        z ^ (z >> 31u64)
    }
}

/// `fold` dans la logique.
#[logic(open)]
pub fn ckfold_l(ck: u64, v: u64) -> u64 {
    pearlite! { (ck ^ v) * 0x100000001B3u64 }
}

/// Taille (octets) du bloc `i`.
#[logic(open)]
pub fn size_l(i: u64) -> Int {
    pearlite! { 8 + 8 * (mix_l(SEED + i)@ % 32) }
}

/// Contenu attendu du bloc `i` : `b[k] = (i + k) mod 251`.
#[logic(open)]
pub fn is_block(b: Seq<u8>, i: Int) -> bool {
    pearlite! { forall<k> 0 <= k && k < b.len() ==> b[k]@ == (i + k) % 251 }
}

/// Somme des `k` premiers octets de `b`.
#[logic(open)]
#[requires(0 <= k && k <= b.len())]
#[variant(k)]
pub fn seq_sum(b: Seq<u8>, k: Int) -> Int {
    pearlite! { if k == 0 { 0 } else { seq_sum(b, k - 1) + b[k - 1]@ } }
}

/// Somme logique des `k` premiers octets du bloc `i`, sans le construire.
#[logic(open)]
#[requires(0 <= k)]
#[variant(k)]
pub fn block_sum(i: Int, k: Int) -> Int {
    pearlite! { if k == 0 { 0 } else { block_sum(i, k - 1) + (i + k - 1) % 251 } }
}

/// Lemme : la somme relue d'un bloc au contenu attendu est `block_sum`.
#[logic]
#[requires(is_block(b, i) && 0 <= k && k <= b.len())]
#[ensures(seq_sum(b, k) == block_sum(i, k))]
#[variant(k)]
pub fn sum_block(b: Seq<u8>, i: Int, k: Int) {
    pearlite! { if k == 0 { () } else { sum_block(b, i, k - 1) } }
}

/// Repli du checksum sur les `k` premières sommes de blocs.
#[logic(open)]
#[requires(0 <= k && k <= sums.len())]
#[variant(k)]
pub fn fold_seq(sums: Seq<u64>, k: Int) -> u64 {
    pearlite! { if k == 0 { CK_INIT } else { ckfold_l(fold_seq(sums, k - 1), sums[k - 1]) } }
}

/// Lemme : ajouter une somme en fin de suite ne change pas le repli des `k` premières.
#[logic]
#[requires(0 <= k && k <= s.len())]
#[ensures(fold_seq(s.snoc(x), k) == fold_seq(s, k))]
#[variant(k)]
pub fn fold_snoc(s: Seq<u64>, x: u64, k: Int) {
    pearlite! { if k == 0 { () } else { fold_snoc(s, x, k - 1) } }
}

/// `sums[i]` est la somme du contenu du bloc `i`, pour tout `i < n`. (Les `u64` de Pearlite n'ont ni `%` ni
/// conversion depuis `Int` : le modèle passe donc par cette suite, unique, plutôt que par une fonction `ck_model(n)`.)
#[logic(open)]
pub fn block_sums(sums: Seq<u64>, n: Int) -> bool {
    pearlite! { sums.len() == n && forall<i: u64> i@ < n ==> sums[i@]@ == block_sum(i@, size_l(i)) }
}

/// Fenêtre FIFO : chaque bloc conservé `i` (multiple de 16, n° `i/16`) parmi les `w` derniers occupe
/// l'emplacement `(i/16) mod w` avec sa taille et son contenu ; les emplacements jamais utilisés sont vides.
#[logic(open)]
pub fn window(ring: Seq<Option<Vec<u8>>>, w: Int, kept: Int) -> bool {
    pearlite! {
        ring.len() == w
        && (forall<i: u64> i@ % 16 == 0 && kept - w <= i@ / 16 && i@ / 16 < kept ==>
            match ring[(i@ / 16) % w] { Some(b) => b@.len() == size_l(i) && is_block(b@, i@), None => false })
        && (forall<s> kept <= s && s < w ==> ring[s] == None)
    }
}

/// Lemme : deux numéros distants de moins de `w` n'ont pas le même emplacement.
#[logic]
#[requires(1 <= w)]
#[ensures(forall<a, b> 0 <= a && a < b && b - a < w ==> a % w != b % w)]
pub fn mod_distinct(w: Int) {
    mul_mono(w)
}

/// Lemme (non linéaire) : les multiples de `w` sont espacés d'au moins `w`.
#[logic]
#[requires(1 <= w)]
#[ensures(forall<qa, qb> qa < qb ==> w * qb - w * qa >= w)]
#[ensures(forall<qa, qb> qb <= qa ==> w * qb - w * qa <= 0)]
pub fn mul_mono(w: Int) {}

/// Lemme : conserver le bloc `i = 16·kept` dans l'emplacement `kept mod w` fait avancer la fenêtre.
#[logic]
#[requires(1 <= w && 0 <= kept && window(ring, w, kept))]
#[requires(i@ == 16 * kept && b@.len() == size_l(i) && is_block(b@, i@))]
#[ensures(window(ring.set(kept % w, Some(b)), w, kept + 1))]
pub fn window_push(ring: Seq<Option<Vec<u8>>>, w: Int, kept: Int, i: u64, b: Vec<u8>) {
    mod_distinct(w)
}

// ---------------------------------------------------------------- code

/// splitmix64 (même corps que `common::mix64`, avec sa spécification logique).
#[check(terminates)]
#[inline(always)]
#[ensures(result == mix_l(x))]
fn mix(x: u64) -> u64 {
    let z = x.wrapping_add(0x9E3779B97F4A7C15);
    let z = (z ^ (z >> 30u64)).wrapping_mul(0xBF58476D1CE4E5B9);
    let z = (z ^ (z >> 27u64)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31u64)
}

/// `fold`, spécifié (la fonction `ckfold` de `common` n'a pas de contrat).
#[check(terminates)]
#[inline(always)]
#[ensures(result == ckfold_l(ck, v))]
fn fold(ck: u64, v: u64) -> u64 {
    (ck ^ v).wrapping_mul(0x100000001B3)
}

/// Remplit `b[k] = (i + k) mod 251`.
#[ensures((^b)@.len() == b@.len() && is_block((^b)@, i@))]
#[check(terminates)]
pub fn fill(b: &mut Vec<u8>, i: u64) {
    let mut v = (i % 251) as u8;
    let mut k = 0usize;
    let n = b.len();
    #[invariant(b@.len() == n@ && k@ <= n@)]
    #[invariant(v@ == (i@ + k@) % 251)]
    #[invariant(forall<j> 0 <= j && j < k@ ==> b[j]@ == (i@ + j) % 251)]
    #[variant(n@ - k@)]
    while k < n {
        b[k] = v;
        v = if v == 250 { 0 } else { v + 1 };
        k += 1;
    }
}

/// Somme des octets du bloc (sans débordement : au plus 256 octets).
#[requires(b@.len() <= 256)]
#[ensures(result@ == seq_sum(b@, b@.len()))]
#[check(terminates)]
pub fn sum(b: &Vec<u8>) -> u64 {
    let mut s = 0u64;
    let mut k = 0usize;
    let n = b.len();
    #[invariant(k@ <= n@ && n@ == b@.len() && s@ == seq_sum(b@, k@) && s@ <= 255 * k@)]
    #[variant(n@ - k@)]
    while k < n {
        s += b[k] as u64;
        k += 1;
    }
    s
}

#[requires(1 <= n@ && n@ <= 4000000000)]
#[ensures(exists<sums: Seq<u64>, kept: u64> block_sums(sums, n@) && kept@ == (n@ + 15) / 16
    && result == ckfold_l(fold_seq(sums, n@), kept))]
pub fn run(n: usize) -> u64 {
    let w = (n >> 6).clamp(1, WMAX);
    let mut ring: Vec<Option<Vec<u8>>> = std::vec![None; w];
    let mut ck = CK_INIT;
    let mut kept = 0usize;
    let mut i = 0usize;
    let mut sums = snapshot!(Seq::empty());
    proof_assert!(forall<s> 0 <= s && s < w@ ==> ring[s] == None);
    proof_assert!(window(ring@, w@, 0));
    #[invariant(1 <= w@ && w@ <= WMAX@ && i@ <= n@)]
    #[invariant(kept@ == (i@ + 15) / 16)]
    #[invariant(block_sums(*sums, i@) && ck == fold_seq(*sums, i@))]
    #[invariant(window(ring@, w@, kept@))]
    while i < n {
        let r = mix(SEED.wrapping_add(i as u64));
        let s = 8 + 8 * ((r % 32) as usize);
        let mut b = std::vec![0u8; s];
        std::hint::black_box(&mut b);
        fill(&mut b, i as u64);
        let bs = sum(&b);
        snapshot!(sum_block(b@, i@, s@));
        ck = fold(ck, bs);
        proof_assert!(s@ == size_l(i as u64) && bs@ == block_sum(i@, size_l(i as u64)));
        proof_assert!(forall<j: u64> j@ == i@ ==> j == i as u64);
        snapshot!(fold_snoc(*sums, bs, i@));
        sums = snapshot!(sums.snoc(bs));
        if i % 16 == 0 {
            snapshot!(window_push(ring@, w@, kept@, i as u64, b));
            ring[kept % w] = Some(b); // l'ancien occupant (bloc n° kept - w) est libéré ici
            kept += 1;
        } else {
            std::hint::black_box(&b);
        }
        i += 1;
    }
    proof_assert!(i@ == n@ && block_sums(*sums, n@) && ck == fold_seq(*sums, n@));
    proof_assert!((kept as u64)@ == (n@ + 15) / 16);
    fold(ck, kept as u64)
}
