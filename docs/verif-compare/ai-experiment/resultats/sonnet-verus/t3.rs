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
        invariant
            n == v@.len(),
            n == old(v)@.len(),
            0 <= i <= n / 2,
            forall|k: int| 0 <= k < i ==> v@[k] == old(v)@[n - 1 - k],
            forall|k: int| n - i <= k < n ==> v@[k] == old(v)@[n - 1 - k],
            forall|k: int| i <= k < n - i ==> v@[k] == old(v)@[k],
        decreases n / 2 - i,
    {
        let t = v[i];
        v[i] = v[n - 1 - i];
        v[n - 1 - i] = t;
        i += 1;
    }
}
fn main() {}
}
