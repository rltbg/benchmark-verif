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

proof fn swap_multiset(s: Seq<u32>, a: int, b: int)
    requires
        0 <= a < b < s.len(),
    ensures
        s.update(a, s[b]).update(b, s[a]).to_multiset() == s.to_multiset(),
{
    broadcast use vstd::seq_lib::to_multiset_update, vstd::multiset::group_multiset_properties;
    vstd::seq_lib::to_multiset_contains(s, s[a]);
    assert(s.contains(s[a]));
    assert(s.to_multiset().count(s[a]) > 0);
    let s1 = s.update(a, s[b]);
    assert(s1[b] == s[b]);
    assert(s1.update(b, s[a]).to_multiset() == s1.to_multiset().insert(s[a]).remove(s1[b]));
    assert(s1.to_multiset() == s.to_multiset().insert(s[b]).remove(s[a]));
    assert(s.to_multiset().insert(s[b]).remove(s[a]).insert(s[a]).remove(s[b]) =~= s.to_multiset());
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
            1 <= i,
            i <= n || n == 0,
            v@.to_multiset() == old(v)@.to_multiset(),
            forall|a: int, b: int| 0 <= a <= b < i && b < n ==> v@[a] <= v@[b],
        decreases n - i,
    {
        let mut j: usize = i;
        while j > 0 && v[j - 1] > v[j]
            invariant
                n == v@.len(),
                i < n,
                j <= i,
                v@.to_multiset() == old(v)@.to_multiset(),
                forall|a: int, b: int| 0 <= a <= b <= i && a != j && b != j ==> v@[a] <= v@[b],
                forall|b: int| j < b <= i ==> v@[j as int] <= v@[b],
                forall|a: int, b: int| 0 <= a < j && j < b <= i ==> v@[a] <= v@[b],
            decreases j,
        {
            proof { swap_multiset(v@, (j - 1) as int, j as int); }
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
