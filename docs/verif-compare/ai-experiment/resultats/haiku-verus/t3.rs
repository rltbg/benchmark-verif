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
    broadcast use vstd::seq::group_seq_lemmas;
    let n = v.len();
    let ghost old_v = v@;
    let mut i: usize = 0;
    while i < n / 2
        invariant
            (n as int) == v@.len(),
            v@.len() == old_v.len(),
            i <= n / 2,
            forall|k: int| 0 <= k < (i as int) ==>
                #[trigger] v@[k] == old_v[(n as int) - 1 - k] && v@[(n as int) - 1 - k] == old_v[k],
            forall|k: int| (i as int) <= k < (n as int) - (i as int) ==> #[trigger] v@[k] == old_v[k],
        decreases n / 2 - i,
    {
        let t = v[i];
        v[i] = v[n - 1 - i];
        v[n - 1 - i] = t;
        assert(v@[i as int] == old_v[(n as int) - 1 - (i as int)]);
        assert(v@[(n as int) - 1 - (i as int)] == old_v[i as int]);
        i += 1;
    }
    assert forall|k: int| 0 <= k < v@.len() implies v@[k] == old_v[(n as int) - 1 - k] by {
        if k < (n / 2) as int {
            assert(v@[k] == old_v[(n as int) - 1 - k]);
        } else if k < (n as int) - ((n / 2) as int) {
            assert(v@[k] == old_v[k]);
            assert((n as int) - 1 - k == k);
        } else {
            let j = (n as int) - 1 - k;
            assert(0 <= j < (n / 2) as int);
            assert(i as int == (n / 2) as int);
            assert(j < i as int);
            assert(v@[j] == old_v[(n as int) - 1 - j]);
            assert(v@[(n as int) - 1 - j] == old_v[j]);
        }
    }
}
fn main() {}
}
