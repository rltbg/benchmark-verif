// EXERCICE t3 (inversion en place) — Verus.
// Ajoute les annotations de preuve nécessaires (invariant, decreases, assert, blocs proof, lemmes `proof fn`,
// `broadcast use`) pour que `verus t3.rs` affiche « 0 errors ».
// INTERDIT : modifier les requires/ensures ou les fonctions `spec` fournies, modifier le code exécutable,
// utiliser assume, admit, #[verifier::external_body] ou toute autre hypothèse non prouvée.
use vstd::prelude::*;
verus! {
fn reverse(v: &mut Vec<u32>)
    ensures
        final(v)@.len() == old(v)@.len(),
        forall|i: int| 0 <= i < final(v)@.len() ==> final(v)@[i] == old(v)@[final(v)@.len() - 1 - i],
{
    let n = v.len();
    let mut i: usize = 0;
    while i < n / 2
    {
        let t = v[i];
        v[i] = v[n - 1 - i];
        v[n - 1 - i] = t;
        i += 1;
    }
}
fn main() {}
}
