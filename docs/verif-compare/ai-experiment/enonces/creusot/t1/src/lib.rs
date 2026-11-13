// EXERCICE t1 (somme bornée) — Creusot.
// Ajoute les annotations de preuve nécessaires (#[invariant], #[variant] si utile, proof_assert!, snapshot!, ghost!,
// fonctions logiques/lemmes auxiliaires) pour que `cargo creusot -- --lib < /dev/null` affiche « Proved ».
// INTERDIT : modifier les #[requires]/#[ensures] ou les fonctions logiques fournies, modifier le code exécutable,
// utiliser #[trusted], assume ou toute autre hypothèse non prouvée.
use creusot_std::prelude::*;

#[logic(open)]
#[variant(n)]
#[requires(0 <= n && n <= s.len())]
pub fn sum_to(s: Seq<u32>, n: Int) -> Int {
    pearlite! { if n == 0 { 0 } else { sum_to(s, n - 1) + s[n - 1]@ } }
}

#[requires(v@.len() <= 1_000_000)]
#[requires(forall<i: Int> 0 <= i && i < v@.len() ==> v@[i]@ <= 1000)]
#[ensures(result@ == sum_to(v@, v@.len()))]
pub fn sum(v: &[u32]) -> u64 {
    let mut s: u64 = 0;
    let mut i: usize = 0;
    while i < v.len() {
        s += v[i] as u64;
        i += 1;
    }
    s
}

