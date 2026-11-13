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
    broadcast use vstd::seq::group_seq_lemmas, vstd::seq_lib::group_to_multiset_ensures;
    let n = v.len();
    let ghost old_v = v@;
    let mut i: usize = 1;
    while i < n
        invariant
            n == v@.len(),
            (i as int) <= (n as int) || i == 1,
            v@.to_multiset() == old_v.to_multiset(),
            forall|a: int, b: int| #![trigger v@[a], v@[b]] 0 <= a < b < (i as int) ==> v@[a] <= v@[b],
        decreases n - i,
    {
        let mut j: usize = i;
        while j > 0 && v[j - 1] > v[j]
            invariant
                n == v@.len(),
                0 <= (j as int) <= (i as int) < (n as int),
                v@.to_multiset() == old_v.to_multiset(),
                forall|a: int, b: int| #![trigger v@[a], v@[b]]
                    0 <= a < b <= (i as int) && a != (j as int) && b != (j as int)
                    ==> v@[a] <= v@[b],
                forall|k: int| #![trigger v@[k]]
                    (j as int) < k <= (i as int) && j > 0
                    ==> v@[(j as int) - 1] <= v@[k],
                forall|k: int| #![trigger v@[k]]
                    (j as int) < k <= (i as int)
                    ==> v@[j as int] <= v@[k],
            decreases j,
        {
            assert(forall|a: int| #![trigger v@[a]]
                0 <= a < (j as int) - 1 ==> v@[a] <= v@[(j as int) - 1]);
            let ghost pre = v@;
            let t = v[j - 1];
            let u = v[j];
            v[j - 1] = u;
            let ghost mid = v@;
            assert(mid == pre.update((j as int) - 1, u));
            proof {
                vstd::seq_lib::to_multiset_update(pre, (j as int) - 1, u);
            }
            assert(mid.to_multiset() == pre.to_multiset().insert(u).remove(pre[(j as int) - 1]));
            v[j] = t;
            assert(v@ == mid.update(j as int, t));
            assert(mid[j as int] == u);
            assert(t == pre[(j as int) - 1]);
            proof {
                vstd::seq_lib::to_multiset_update(mid, j as int, t);
            }
            assert(v@.to_multiset() == mid.to_multiset().insert(t).remove(mid[j as int]));
            assert(pre.contains(t));
            proof {
                vstd::seq_lib::to_multiset_contains(pre, t);
            }
            assert(pre.to_multiset().count(t) > 0);
            assert(pre.to_multiset().insert(u).remove(t).insert(t) =~= pre.to_multiset().insert(u));
            assert(v@.to_multiset() =~= pre.to_multiset());
            j -= 1;
        }
        assert(j == 0 || v@[(j as int) - 1] <= v@[j as int]);
        assert forall|a: int, b: int| 0 <= a < b <= (i as int) implies v@[a] <= v@[b] by {
            if a == j as int {
                assert(v@[a] <= v@[b]);
            } else if b == j as int {
                if a < (j as int) - 1 {
                    assert(v@[a] <= v@[(j as int) - 1]);
                }
            }
        }
        i += 1;
    }
}
fn main() {}
}
