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
    let old = snapshot!((*v)@);
    #[invariant(i@ >= 1)]
    #[invariant(i@ <= n@ || i@ == 1)]
    #[invariant((*v)@.len() == n@)]
    #[invariant(old.len() == n@)]
    #[invariant((*v)@.permutation_of(*old))]
    #[invariant(forall<a: Int, b: Int> 0 <= a && a <= b && b < i@ ==> (*v)@[a] <= (*v)@[b])]
    while i < n {
        let mut j: usize = i;
        let x = snapshot!((*v)@[i@]);
        #[invariant(j@ <= i@)]
        #[invariant((*v)@[j@] == *x)]
        #[invariant((*v)@.len() == n@)]
        #[invariant((*v)@.permutation_of(*old))]
        #[invariant(forall<a: Int, b: Int> 0 <= a && a <= b && b <= i@ && a != j@ && b != j@ ==> (*v)@[a] <= (*v)@[b])]
        #[invariant(forall<k: Int> j@ < k && k <= i@ ==> (*v)@[k] > *x)]
        while j > 0 && v[j - 1] > v[j] {
            v.swap(j - 1, j);
            j -= 1;
        }
        i += 1;
    }
}
