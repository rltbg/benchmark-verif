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
    let mut i: usize = 0;
    let old = snapshot!((*v)@);
    #[invariant(i@ <= n@ / 2)]
    #[invariant((*v)@.len() == n@)]
    #[invariant(old.len() == n@)]
    #[invariant(forall<k: Int> 0 <= k && k < i@ ==> (*v)@[k] == old[n@ - 1 - k])]
    #[invariant(forall<k: Int> n@ - i@ <= k && k < n@ ==> (*v)@[k] == old[n@ - 1 - k])]
    #[invariant(forall<k: Int> i@ <= k && k < n@ - i@ ==> (*v)@[k] == old[k])]
    while i < n / 2 {
        v.swap(i, n - 1 - i);
        i += 1;
    }
}

