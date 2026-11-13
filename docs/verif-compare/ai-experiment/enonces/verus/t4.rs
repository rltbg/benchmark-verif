// EXERCICE t4 (tri par insertion) — Verus.
// Ajoute les annotations de preuve nécessaires (invariant, decreases, assert, blocs proof, lemmes `proof fn`,
// `broadcast use`) pour que `verus t4.rs` affiche « 0 errors ».
// INTERDIT : modifier les requires/ensures ou les fonctions `spec` fournies, modifier le code exécutable,
// utiliser assume, admit, #[verifier::external_body] ou toute autre hypothèse non prouvée.
use vstd::prelude::*;
verus! {

pub open spec fn sorted(s: Seq<u32>) -> bool {
    forall|i: int, j: int| 0 <= i <= j < s.len() ==> s[i] <= s[j]
}

fn insertion_sort(v: &mut Vec<u32>)
    ensures
        final(v)@.len() == old(v)@.len(),
        sorted(final(v)@),
        final(v)@.to_multiset() == old(v)@.to_multiset(),
{
    let n = v.len();
    let mut i: usize = 1;
    while i < n
    {
        let mut j: usize = i;
        while j > 0 && v[j - 1] > v[j]
        {
            let t = v[j - 1];
            let u = v[j];
            v[j - 1] = u;
            v[j] = t;
            j -= 1;
        }
        i += 1;
    }
}
fn main() {}
}
