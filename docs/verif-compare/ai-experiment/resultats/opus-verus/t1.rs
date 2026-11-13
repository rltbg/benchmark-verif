// EXERCICE t1 (somme bornée) — Verus.
// Ajoute les annotations de preuve nécessaires (invariant, decreases, assert, blocs proof, lemmes `proof fn`,
// `broadcast use`) pour que `verus t1.rs` affiche « 0 errors ».
// INTERDIT : modifier les requires/ensures ou les fonctions `spec` fournies, modifier le code exécutable,
// utiliser assume, admit, #[verifier::external_body] ou toute autre hypothèse non prouvée.
use vstd::prelude::*;
verus! {
pub open spec fn sum_to(s: Seq<u32>, n: int) -> int
    decreases n,
{
    if n <= 0 { 0 } else { sum_to(s, n - 1) + s[n - 1] }
}
fn sum(v: &[u32]) -> (r: u64)
    requires
        v@.len() <= 1_000_000,
        forall|i: int| 0 <= i < v@.len() ==> v@[i] <= 1000,
    ensures r == sum_to(v@, v@.len() as int),
{
    let mut s: u64 = 0;
    let mut i: usize = 0;
    while i < v.len()
        invariant
            i <= v@.len(),
            v@.len() <= 1_000_000,
            forall|k: int| 0 <= k < v@.len() ==> v@[k] <= 1000,
            s == sum_to(v@, i as int),
            s <= 1000 * i,
        decreases v@.len() - i,
    {
        assert(sum_to(v@, i as int + 1) == sum_to(v@, i as int) + v@[i as int]);
        s += v[i] as u64;
        i += 1;
    }
    s
}
fn main() {}
}
