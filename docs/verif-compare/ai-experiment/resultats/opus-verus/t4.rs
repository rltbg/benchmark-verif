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
        invariant
            n == v@.len(),
            n == old(v)@.len(),
            1 <= i,
            n == 0 || i <= n,
            v@.to_multiset() == old(v)@.to_multiset(),
            forall|a: int, b: int| 0 <= a <= b < i && b < n ==> v@[a] <= v@[b],
        decreases n - i,
    {
        let mut j: usize = i;
        while j > 0 && v[j - 1] > v[j]
            invariant
                n == v@.len(),
                n == old(v)@.len(),
                1 <= i < n,
                j <= i,
                v@.to_multiset() == old(v)@.to_multiset(),
                forall|a: int, b: int| 0 <= a <= b <= i && a != j && b != j ==> v@[a] <= v@[b],
                forall|b: int| j < b <= i ==> v@[j as int] <= v@[b],
            decreases j,
        {
            let ghost s0 = v@;
            let t = v[j - 1];
            let u = v[j];
            v[j - 1] = u;
            v[j] = t;
            proof {
                broadcast use vstd::seq_lib::group_to_multiset_ensures;
                assert(v@ == s0.update(j - 1, u).update(j as int, t));
                assert(v@.to_multiset() =~= s0.to_multiset());
            }
            j -= 1;
        }
        assert forall|a: int, b: int| 0 <= a <= b <= i implies v@[a] <= v@[b] by {
            if a == j && b != j {
            } else if b == j && a != j {
                if j > 0 {
                    assert(v@[a] <= v@[j - 1]);
                }
            }
        }
        i += 1;
    }
}
fn main() {}
}
