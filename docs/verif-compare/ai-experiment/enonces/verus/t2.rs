// EXERCICE t2 (recherche dichotomique) — Verus.
// Ajoute les annotations de preuve nécessaires (invariant, decreases, assert, blocs proof, lemmes `proof fn`,
// `broadcast use`) pour que `verus t2.rs` affiche « 0 errors ».
// INTERDIT : modifier les requires/ensures ou les fonctions `spec` fournies, modifier le code exécutable,
// utiliser assume, admit, #[verifier::external_body] ou toute autre hypothèse non prouvée.
use vstd::prelude::*;
verus! {
fn search(v: &[u64], key: u64) -> (r: Option<usize>)
    requires forall|i: int, j: int| 0 <= i <= j < v@.len() ==> v@[i] <= v@[j],
    ensures
        match r {
            Some(i) => i < v@.len() && v@[i as int] == key,
            None => forall|i: int| 0 <= i < v@.len() ==> v@[i] != key,
        },
{
    let mut lo: usize = 0;
    let mut hi: usize = v.len();
    while lo < hi
    {
        let mid = lo + (hi - lo) / 2;
        if v[mid] < key {
            lo = mid + 1;
        } else if v[mid] > key {
            hi = mid;
        } else {
            return Some(mid);
        }
    }
    None
}
fn main() {}
}
