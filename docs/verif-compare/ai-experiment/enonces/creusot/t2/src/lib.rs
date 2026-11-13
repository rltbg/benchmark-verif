// EXERCICE t2 (recherche dichotomique) — Creusot.
// Ajoute les annotations de preuve nécessaires (#[invariant], #[variant] si utile, proof_assert!, snapshot!, ghost!,
// fonctions logiques/lemmes auxiliaires) pour que `cargo creusot -- --lib < /dev/null` affiche « Proved ».
// INTERDIT : modifier les #[requires]/#[ensures] ou les fonctions logiques fournies, modifier le code exécutable,
// utiliser #[trusted], assume ou toute autre hypothèse non prouvée.
use creusot_std::prelude::*;

#[requires(forall<i: Int, j: Int> 0 <= i && i <= j && j < v@.len() ==> v@[i] <= v@[j])]
#[ensures(match result {
    Some(i) => i@ < v@.len() && v@[i@] == key,
    None => forall<i: Int> 0 <= i && i < v@.len() ==> v@[i] != key,
})]
pub fn search(v: &[u64], key: u64) -> Option<usize> {
    let mut lo: usize = 0;
    let mut hi: usize = v.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if v[mid] < key {
            lo = mid + 1;
        } else if v[mid] > key {
            hi = mid;
        } else {
            return Some(mid);
        }
    }
    None
}

