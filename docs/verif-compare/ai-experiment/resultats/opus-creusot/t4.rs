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
    let old_v = snapshot! { v };
    #[invariant(v@.len() == n@ && old_v@.len() == n@)]
    #[invariant(1 <= i@ && (i@ <= n@ || n@ == 0))]
    #[invariant(forall<a: Int, b: Int> 0 <= a && a <= b && b < i@ && b < n@ ==> v@[a] <= v@[b])]
    #[invariant(v@.permutation_of(old_v@))]
    while i < n {
        let mut j: usize = i;
        #[invariant(v@.len() == n@)]
        #[invariant(j@ <= i@ && i@ < n@)]
        #[invariant(forall<a: Int, b: Int> 0 <= a && a <= b && b < j@ ==> v@[a] <= v@[b])]
        #[invariant(forall<a: Int, b: Int> j@ <= a && a <= b && b <= i@ ==> v@[a] <= v@[b])]
        #[invariant(forall<a: Int, b: Int> 0 <= a && a < j@ && j@ < b && b <= i@ ==> v@[a] <= v@[b])]
        #[invariant(v@.permutation_of(old_v@))]
        while j > 0 && v[j - 1] > v[j] {
            v.swap(j - 1, j);
            j -= 1;
        }
        i += 1;
    }
}
