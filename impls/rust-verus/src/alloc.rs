//! Allocation en masse (spec/workloads.d/alloc.md) : n blocs de 8..256 octets, un bloc sur 16 conservé dans une
//! fenêtre circulaire de W = min(max(1, n/64), 2^25) blocs.
//!
//! Prouvé (Verus) :
//! * `fill` : `b[k] = (i + k) mod 251` ; `sum` : somme exacte du contenu relu (sans débordement) ;
//! * `run` : le résultat est `fold(ck_model(n), ⌈n/16⌉)`, où `ck_model` replie, bloc par bloc, la somme du contenu
//!   logique du bloc (`block_sum(i, size_s(i))`) — taille et contenu de la spécification ;
//! * fenêtre FIFO : après `kept` conservations, l'emplacement `j mod W` de l'anneau contient le bloc conservé
//!   n° `j` (bloc d'indice `16 j`, taille et contenu vérifiés) pour chacun des `W` derniers `j`, et les
//!   emplacements jamais utilisés sont vides ; le bloc remplacé est libéré (drop) à l'affectation ;
//! * absence de panique et de débordement ; terminaison de toutes les fonctions.
//!
//! `external_body` (non vérifié) : `barrier` et `barrier_mut`, la barrière d'optimisation `std::hint::black_box`,
//! sans spécification dans vstd ; leur contrat (« ne modifie pas le bloc ») est supposé.
use crate::common::*;
use vstd::prelude::*;

verus! {

pub const WMAX: usize = 33554432; // 2^25
pub const SEED: u64 = 0xA110C00000000001;

// ---------------------------------------------------------------- modèle logique

/// Taille (octets) du bloc `i`.
pub open spec fn size_s(i: int) -> int {
    8 + 8 * (spec_mix64(SEED.wrapping_add(i as u64)) % 32) as int
}

/// Contenu attendu du bloc `i` : `b[k] = (i + k) mod 251`.
pub open spec fn is_block(b: Seq<u8>, i: int) -> bool {
    forall|k: int| 0 <= k < b.len() ==> #[trigger] b[k] == (i + k) % 251
}

/// Somme des `k` premiers octets de `b`.
pub open spec fn seq_sum(b: Seq<u8>, k: int) -> int
    decreases k,
{
    if k <= 0 { 0 } else { seq_sum(b, k - 1) + b[k - 1] }
}

/// Somme logique des `k` premiers octets du bloc `i`, sans le construire.
pub open spec fn block_sum(i: int, k: int) -> int
    decreases k,
{
    if k <= 0 { 0 } else { block_sum(i, k - 1) + (i + k - 1) % 251 }
}

/// Checksum après les blocs `0 .. i`.
pub open spec fn ck_model(i: int) -> u64
    decreases i,
{
    if i <= 0 { CK_INIT } else { spec_ckfold(ck_model(i - 1), block_sum(i - 1, size_s(i - 1)) as u64) }
}

/// Emplacement de l'anneau du bloc conservé n° `j`.
pub open spec fn slot(ring: Seq<Option<Vec<u8>>>, w: int, j: int) -> Option<Vec<u8>> {
    ring[j % w]
}

/// Fenêtre FIFO : les `w` derniers blocs conservés (n° `j`, bloc `16 j`) occupent l'emplacement `j mod w`
/// avec leur taille et leur contenu ; les emplacements jamais utilisés sont vides.
pub open spec fn window(ring: Seq<Option<Vec<u8>>>, w: int, kept: int) -> bool {
    &&& ring.len() == w
    &&& forall|j: int| 0 <= j < kept && kept - w <= j ==> match #[trigger] slot(ring, w, j) {
        Some(b) => b@.len() == size_s(16 * j) && is_block(b@, 16 * j),
        None => false,
    }
    &&& forall|s: int| kept <= s < w ==> #[trigger] ring[s] is None
}

// ---------------------------------------------------------------- lemmes

/// La somme relue d'un bloc au contenu attendu est `block_sum`.
proof fn lemma_sum_block(b: Seq<u8>, i: int, k: int)
    requires
        is_block(b, i),
        0 <= k <= b.len(),
    ensures
        seq_sum(b, k) == block_sum(i, k),
    decreases k,
{
    if k > 0 {
        lemma_sum_block(b, i, k - 1);
        assert(b[k - 1] == (i + k - 1) % 251);
    }
}

/// Deux numéros distants de moins de `w` n'ont pas le même emplacement.
proof fn lemma_mod_distinct(a: int, b: int, w: int)
    requires
        0 <= a < b,
        b - a < w,
    ensures
        a % w != b % w,
{
    vstd::arithmetic::div_mod::lemma_mod_equivalence(b, a, w);
    vstd::arithmetic::div_mod::lemma_small_mod((b - a) as nat, w as nat);
}

/// Conserver le bloc n° `kept` dans l'emplacement `kept mod w` fait avancer la fenêtre.
proof fn lemma_window_push(ring: Seq<Option<Vec<u8>>>, w: int, kept: int, b: Vec<u8>)
    requires
        1 <= w,
        0 <= kept,
        window(ring, w, kept),
        b@.len() == size_s(16 * kept),
        is_block(b@, 16 * kept),
    ensures
        window(ring.update(kept % w, Some(b)), w, kept + 1),
{
    let r2 = ring.update(kept % w, Some(b));
    assert forall|j: int| 0 <= j < kept + 1 && kept + 1 - w <= j implies match #[trigger] slot(r2, w, j) {
        Some(b) => b@.len() == size_s(16 * j) && is_block(b@, 16 * j),
        None => false,
    } by {
        if j < kept {
            lemma_mod_distinct(j, kept, w);
            assert(slot(r2, w, j) == slot(ring, w, j));
        }
    }
    assert forall|s: int| kept + 1 <= s < w implies #[trigger] r2[s] is None by {
        vstd::arithmetic::div_mod::lemma_small_mod(kept as nat, w as nat);
    }
}

// ---------------------------------------------------------------- code

/// Barrière d'optimisation (non vérifiée) : empêche le compilateur d'éliminer l'allocation du bloc.
#[verifier::external_body]
#[inline(always)]
fn barrier_mut(b: &mut Vec<u8>)
    ensures
        *final(b) == *old(b),
{
    std::hint::black_box(b);
}

/// Barrière d'optimisation (non vérifiée) avant la libération du bloc.
#[verifier::external_body]
#[inline(always)]
fn barrier(b: &Vec<u8>) {
    std::hint::black_box(b);
}

/// Remplit `b[k] = (i + k) mod 251`.
pub fn fill(b: &mut Vec<u8>, i: u64)
    ensures
        final(b)@.len() == old(b)@.len(),
        is_block(final(b)@, i as int),
{
    let mut v = (i % 251) as u8;
    let mut k = 0usize;
    let n = b.len();
    while k < n
        invariant
            b@.len() == n,
            k <= n,
            v == (i + k) % 251,
            forall|j: int| 0 <= j < k ==> #[trigger] b@[j] == (i + j) % 251,
        decreases n - k,
    {
        b[k] = v;
        v = if v == 250 { 0 } else { v + 1 };
        k += 1;
    }
}

/// Somme des octets du bloc (sans débordement : au plus 256 octets).
pub fn sum(b: &Vec<u8>) -> (r: u64)
    requires
        b@.len() <= 256,
    ensures
        r == seq_sum(b@, b@.len() as int),
{
    let mut s = 0u64;
    let mut k = 0usize;
    let n = b.len();
    while k < n
        invariant
            n == b@.len() <= 256,
            k <= n,
            s == seq_sum(b@, k as int),
            s <= 255 * k,
        decreases n - k,
    {
        s += b[k] as u64;
        k += 1;
    }
    s
}

pub fn run(n: usize, threads: usize) -> (r: u64)
    requires
        1 <= n <= 4000000000,
    ensures
        r == spec_ckfold(ck_model(n as int), ((n + 15) / 16) as u64),
{
    let _ = threads;
    let w = (n >> 6).clamp(1, WMAX);
    let mut ring: Vec<Option<Vec<u8>>> = vec![None; w];
    assert(window(ring@, w as int, 0));
    let mut ck = CK_INIT;
    let mut kept = 0usize;
    let mut i = 0usize;
    while i < n
        invariant
            1 <= w <= WMAX,
            i <= n <= 4000000000,
            kept == (i + 15) / 16,
            ck == ck_model(i as int),
            window(ring@, w as int, kept as int),
        decreases n - i,
    {
        let r = mix64(SEED.wrapping_add(i as u64));
        let s = 8 + 8 * ((r % 32) as usize);
        let mut b = vec![0u8; s];
        barrier_mut(&mut b);
        fill(&mut b, i as u64);
        let bs = sum(&b);
        proof {
            lemma_sum_block(b@, i as int, s as int);
        }
        ck = ckfold(ck, bs);
        if i % 16 == 0 {
            proof {
                lemma_window_push(ring@, w as int, kept as int, b);
            }
            ring[kept % w] = Some(b); // l'ancien occupant (bloc n° kept - w) est libéré ici
            kept += 1;
        } else {
            barrier(&b);
        }
        i += 1;
    }
    ckfold(ck, kept as u64)
}

} // verus!
