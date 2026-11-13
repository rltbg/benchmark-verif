// EXERCICE t3 (inversion en place) — Creusot.
// Ajoute les annotations de preuve nécessaires (#[invariant], #[variant] si utile, proof_assert!, snapshot!, ghost!,
// fonctions logiques/lemmes auxiliaires) pour que `cargo creusot -- --lib < /dev/null` affiche « Proved ».
// INTERDIT : modifier les #[requires]/#[ensures] ou les fonctions logiques fournies, modifier le code exécutable,
// utiliser #[trusted], assume ou toute autre hypothèse non prouvée.
use creusot_std::prelude::*;

#[ensures((^v)@.len() == v@.len())]
#[ensures(forall<i: Int> 0 <= i && i < v@.len() ==> (^v)@[i] == v@[v@.len() - 1 - i])]
pub fn reverse(v: &mut [u32]) {
    let n = v.len();
    let old_v = snapshot!(v@);
    let mut i: usize = 0;
    #[invariant(i@ <= n@ / 2)]
    #[invariant(n@ == v@.len() && v@.len() == old_v.len())]
    #[invariant(forall<k: Int> 0 <= k && k < i@ ==> v@[k] == old_v[n@ - 1 - k] && v@[n@ - 1 - k] == old_v[k])]
    #[invariant(forall<k: Int> i@ <= k && k < n@ - i@ ==> v@[k] == old_v[k])]
    while i < n / 2 {
        v.swap(i, n - 1 - i);
        i += 1;
    }
}

