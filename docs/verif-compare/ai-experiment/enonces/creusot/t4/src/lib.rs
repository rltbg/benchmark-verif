// EXERCICE t4 (tri par insertion) — Creusot.
// Ajoute les annotations de preuve nécessaires (#[invariant], #[variant] si utile, proof_assert!, snapshot!, ghost!,
// fonctions logiques/lemmes auxiliaires) pour que `cargo creusot -- --lib < /dev/null` affiche « Proved ».
// INTERDIT : modifier les #[requires]/#[ensures] ou les fonctions logiques fournies, modifier le code exécutable,
// utiliser #[trusted], assume ou toute autre hypothèse non prouvée.
use creusot_std::prelude::*;

#[ensures((^v)@.len() == v@.len())]
#[ensures(forall<i: Int, j: Int> 0 <= i && i <= j && j < (^v)@.len() ==> (^v)@[i] <= (^v)@[j])]
#[ensures((^v)@.permutation_of(v@))]
pub fn insertion_sort(v: &mut [u32]) {
    let n = v.len();
    let mut i: usize = 1;
    while i < n {
        let mut j: usize = i;
        while j > 0 && v[j - 1] > v[j] {
            v.swap(j - 1, j);
            j -= 1;
        }
        i += 1;
    }
}
