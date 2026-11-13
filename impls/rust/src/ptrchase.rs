//! ptrchase : arène par indices, BST non équilibré + recherches + chasse de pointeurs
//! (voir spec/workloads.d/ptrchase.md). 1 <= n <= 2^31 - 2.
//!
//! Prouvé (Creusot, sans `trusted` ni `assume`) :
//! * invariant d'arbre de recherche [`is_bst`] : chaque lien est `NIL` ou un nœud d'indice strictement plus grand
//!   (donc la descente termine), et chaque nœud `t` a un chemin fantôme racine → `t` qui est exactement le chemin
//!   de recherche de sa clé (chaque clé est du bon côté de chacun de ses ancêtres) ;
//! * `build` : l'ensemble des clés de l'arbre est exactement `{ ins_key(i) | i < n }` ;
//! * `search` : trouvée ⇔ la clé est dans l'arbre (donc ⇔ elle a été insérée) ; les `n` recherches positives
//!   trouvent, les `n/2` négatives échouent ;
//! * `sattolo` produit une permutation de `0..n` ; `chase` suit `next` (`chase_pos`, `chase_sum`) ;
//! * absence de panique et de débordement ; terminaison de tous les noyaux.
//!
//! Les gardes de la version précédente (`c < cnt`, `v < cnt`, `v < n` dans la chasse) ont disparu : elles sont
//! remplacées par les invariants ci-dessus. Le cycle unique de Sattolo n'est pas prouvé (inutile à la sûreté).
use crate::common::*;
use creusot_std::prelude::*;

pub const NIL: u64 = 0xFFFF_FFFF;
pub const M: u64 = 0xFFFF_FFFF;
/// Mot de liens d'une feuille : NIL / NIL.
pub const LEAF: u64 = NIL | (NIL << 32);
pub const SEED1: u64 = 0xD00D000000000001;
pub const SEED2: u64 = 0xD00D000000000002;
pub const SEED3: u64 = 0xD00D000000000003;

// ---------------------------------------------------------------- modèle logique

/// splitmix64 dans la logique (arithmétique modulo 2^64 des `u64` de Pearlite).
#[logic(open)]
pub fn mix_l(x: u64) -> u64 {
    pearlite! {
        let z = x + 0x9E3779B97F4A7C15u64;
        let z = (z ^ (z >> 30u64)) * 0xBF58476D1CE4E5B9u64;
        let z = (z ^ (z >> 27u64)) * 0x94D049BB133111EBu64;
        z ^ (z >> 31u64)
    }
}

/// Clé insérée à l'étape `i` de la phase 1.
#[logic(open)]
pub fn ins_key(i: u64) -> u64 {
    pearlite! { mix_l(SEED1 + i) | 1u64 }
}

/// Clé du nœud `i`.
#[logic(open)]
pub fn key(a: Seq<u64>, i: Int) -> u64 {
    pearlite! { a[2 * i] }
}

/// Fils gauche (32 bits de poids faible du mot de liens).
#[logic(open)]
pub fn left(l: u64) -> u64 {
    pearlite! { l & M }
}

/// Fils droit (32 bits de poids fort du mot de liens).
#[logic(open)]
pub fn right(l: u64) -> u64 {
    pearlite! { l >> 32u64 }
}

/// Successeur de `i` sur le chemin de recherche de `k`.
#[logic(open)]
pub fn step(a: Seq<u64>, i: Int, k: u64) -> u64 {
    pearlite! { if k < key(a, i) { left(a[2 * i + 1]) } else { right(a[2 * i + 1]) } }
}

/// `p` est un préfixe du chemin de recherche de `k` : il part de la racine, ne passe que par des nœuds alloués,
/// aucun nœud avant le dernier ne porte `k`, et chaque nœud est le successeur du précédent.
#[logic(open)]
pub fn search_path(a: Seq<u64>, cnt: Int, k: u64, p: Seq<Int>) -> bool {
    pearlite! {
        p.len() >= 1 && p[0] == 0
        && (forall<j> 0 <= j && j < p.len() ==> 0 <= p[j] && p[j] < cnt)
        && (forall<j> 0 <= j && j < p.len() - 1 ==> key(a, p[j]) != k && p[j + 1] == step(a, p[j], k)@)
    }
}

/// Un lien du nœud `i` est `NIL` ou un nœud d'indice dans `(i, cnt)`.
#[logic(open)]
pub fn link_ok(c: u64, i: Int, cnt: Int) -> bool {
    pearlite! { c == NIL || (i < c@ && c@ < cnt) }
}

/// Invariant d'arbre binaire de recherche sur les `cnt` premiers nœuds de l'arène, témoigné par `paths` :
/// `paths[t]` est le chemin de recherche de `key(t)` et il aboutit en `t`.
#[logic(open)]
pub fn is_bst(a: Seq<u64>, cnt: Int, paths: Seq<Seq<Int>>) -> bool {
    pearlite! {
        1 <= cnt && cnt < NIL@ && 2 * cnt <= a.len() && paths.len() == cnt
        && (forall<i> 0 <= i && i < cnt ==> link_ok(left(a[2 * i + 1]), i, cnt) && link_ok(right(a[2 * i + 1]), i, cnt))
        && (forall<t> 0 <= t && t < cnt ==> search_path(a, cnt, key(a, t), paths[t]) && paths[t][paths[t].len() - 1] == t)
    }
}

/// La clé `k` est portée par l'un des `cnt` premiers nœuds.
#[logic(open)]
pub fn contains(a: Seq<u64>, cnt: Int, k: u64) -> bool {
    pearlite! { exists<t> 0 <= t && t < cnt && key(a, t) == k }
}

/// Position après `h` sauts depuis 0.
#[logic(open)]
#[requires(h >= 0)]
#[variant(h)]
pub fn chase_pos(nx: Seq<u32>, h: Int) -> u32 {
    pearlite! { if h == 0 { 0u32 } else { nx[chase_pos(nx, h - 1)@] } }
}

/// Somme (modulo 2^64) des positions visitées pendant les `h` premiers sauts.
#[logic(open)]
#[requires(h >= 0)]
#[variant(h)]
pub fn chase_sum(nx: Seq<u32>, h: Int) -> u64 {
    pearlite! { if h == 0 { 0u64 } else { chase_sum(nx, h - 1) + chase_pos(nx, h) as u64 } }
}

/// `fold` dans la logique.
#[logic(open)]
pub fn ckfold_l(ck: u64, v: u64) -> u64 {
    pearlite! { (ck ^ v) * 0x100000001B3u64 }
}

/// Lemme : les deux liens d'une feuille sont `NIL`.
#[logic]
#[bitwise_proof]
#[ensures(left(LEAF) == NIL && right(LEAF) == NIL)]
pub fn leaf_links() {}

/// Lemme : insérer la feuille `cnt` (clé `nk`) sous `cur`, au bout du chemin de recherche `p` de `nk`,
/// préserve l'invariant d'arbre de recherche.
#[logic]
#[requires(is_bst(a0, cnt, paths) && cnt + 1 < NIL@ && 2 * (cnt + 1) <= a.len() && a.len() == a0.len())]
#[requires(search_path(a0, cnt, nk, p) && p[p.len() - 1] == cur)]
#[requires(key(a0, cur) != nk && step(a0, cur, nk) == NIL)]
#[requires(key(a, cnt) == nk && left(a[2 * cnt + 1]) == NIL && right(a[2 * cnt + 1]) == NIL)]
#[requires(nk < key(a0, cur) ==> left(a[2 * cur + 1])@ == cnt && right(a[2 * cur + 1]) == right(a0[2 * cur + 1]))]
#[requires(!(nk < key(a0, cur)) ==> right(a[2 * cur + 1])@ == cnt && left(a[2 * cur + 1]) == left(a0[2 * cur + 1]))]
#[requires(forall<t> 0 <= t && t < cnt ==> key(a, t) == key(a0, t))]
#[requires(forall<t> 0 <= t && t < cnt && t != cur ==> a[2 * t + 1] == a0[2 * t + 1])]
#[ensures(is_bst(a, cnt + 1, paths.snoc(p.snoc(cnt))))]
pub fn insert_preserves(a0: Seq<u64>, a: Seq<u64>, cnt: Int, cur: Int, nk: u64, paths: Seq<Seq<Int>>, p: Seq<Int>) {}

/// Lemme : prolonger un chemin de recherche d'un pas valide.
#[logic]
#[requires(search_path(a, cnt, k, p) && key(a, p[p.len() - 1]) != k)]
#[requires(c == step(a, p[p.len() - 1], k)@ && 0 <= c && c < cnt)]
#[ensures(search_path(a, cnt, k, p.snoc(c)))]
pub fn search_path_snoc(a: Seq<u64>, cnt: Int, k: u64, p: Seq<Int>, c: Int) {}

/// Lemme : après l'étape `i` de la phase 1 (nouveau nœud ou doublon), les clés de l'arbre restent exactement
/// les clés insérées.
#[logic]
#[requires(forall<j: u64> j@ < i ==> contains(a0, cnt0, ins_key(j)))]
#[requires(forall<t> 0 <= t && t < cnt0 ==> exists<j: u64> j@ < i && key(a0, t) == ins_key(j))]
#[requires(forall<t> 0 <= t && t < cnt0 ==> key(a, t) == key(a0, t))]
#[requires(i == ii@ && (cnt == cnt0 || cnt == cnt0 + 1) && contains(a, cnt, ins_key(ii)))]
#[requires(cnt == cnt0 + 1 ==> key(a, cnt0) == ins_key(ii))]
#[ensures(forall<j: u64> j@ < i + 1 ==> contains(a, cnt, ins_key(j)))]
#[ensures(forall<t> 0 <= t && t < cnt ==> exists<j: u64> j@ < i + 1 && key(a, t) == ins_key(j))]
pub fn keys_step(a0: Seq<u64>, a: Seq<u64>, cnt0: Int, cnt: Int, i: Int, ii: u64) {}

/// Lemme : une clé paire n'est jamais de la forme `x | 1`.
#[logic]
#[bitwise_proof]
#[ensures(forall<x: u64> (x | 1u64) != (y & !1u64))]
pub fn odd_even(y: u64) {}

// ---------------------------------------------------------------- code

/// splitmix64 (même corps que `common::mix64`, avec sa spécification logique).
#[check(terminates)]
#[inline(always)]
#[ensures(result == mix_l(x))]
fn mix(x: u64) -> u64 {
    let z = x.wrapping_add(0x9E3779B97F4A7C15);
    let z = (z ^ (z >> 30u64)).wrapping_mul(0xBF58476D1CE4E5B9);
    let z = (z ^ (z >> 27u64)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31u64)
}

/// `fold(fold(CK_INIT, x), y)`, spécifié (la fonction `ckfold` de `common` n'a pas de contrat).
#[check(terminates)]
#[inline(always)]
#[ensures(result == ckfold_l(ckfold_l(CK_INIT, x), y))]
fn ck2(x: u64, y: u64) -> u64 {
    let c = (CK_INIT ^ x).wrapping_mul(0x100000001B3);
    (c ^ y).wrapping_mul(0x100000001B3)
}

/// Remplace le fils gauche (`is_left`) ou droit de `lnk` par `c`, l'autre est conservé.
#[check(terminates)]
#[inline(always)]
#[bitwise_proof]
#[requires(c < NIL)]
#[ensures(is_left ==> left(result) == c && right(result) == right(lnk))]
#[ensures(!is_left ==> right(result) == c && left(result) == left(lnk))]
fn set_link(lnk: u64, is_left: bool, c: u64) -> u64 {
    if is_left { (lnk & !M) | c } else { (lnk & M) | (c << 32u64) }
}

/// Phase 1 : n insertions dans l'arène (2n u64) ; renvoie (nombre de nœuds, somme des chemins, témoin du BST).
#[requires(1 <= n@ && n@ <= 2147483646 && a@.len() == 2 * n@)]
#[ensures((^a)@.len() == a@.len() && 1 <= result.0@ && result.0@ <= n@)]
#[ensures(is_bst((^a)@, result.0@, *result.2))]
#[ensures(forall<i: u64> i@ < n@ ==> contains((^a)@, result.0@, ins_key(i)))]
#[ensures(forall<t> 0 <= t && t < result.0@ ==> exists<i: u64> i@ < n@ && key((^a)@, t) == ins_key(i))]
#[check(terminates)]
pub fn build(a: &mut Vec<u64>, n: usize) -> (usize, u64, Snapshot<Seq<Seq<Int>>>) {
    a[0] = mix(SEED1) | 1;
    a[1] = LEAF;
    let mut cnt = 1usize;
    let mut plen = 0u64;
    let mut paths = snapshot!(Seq::singleton(Seq::singleton(0)));
    let mut i = 1usize;
    snapshot!(leaf_links());
    proof_assert!(key(a@, 0) == ins_key(0u64));
    #[invariant(1 <= i@ && i@ <= n@ && 1 <= cnt@ && cnt@ <= i@ && a@.len() == 2 * n@)]
    #[invariant(is_bst(a@, cnt@, *paths))]
    #[invariant(forall<j: u64> j@ < i@ ==> contains(a@, cnt@, ins_key(j)))]
    #[invariant(forall<t> 0 <= t && t < cnt@ ==> exists<j: u64> j@ < i@ && key(a@, t) == ins_key(j))]
    #[variant(n@ - i@)]
    while i < n {
        let nk = mix(SEED1.wrapping_add(i as u64)) | 1;
        proof_assert!(nk == ins_key(i as u64));
        let mut cur = 0usize;
        let mut v = 0u64;
        // chemin de recherche de `nk` jusqu'à `cur` inclus
        let mut p = snapshot!(Seq::singleton(0));
        // `a`, `cnt` et `paths` ne changent qu'à la sortie de la descente
        let before = snapshot!((a@, cnt, *paths));
        #[invariant(a@ == before.0 && cnt == before.1 && *paths == before.2)]
        #[invariant(search_path(a@, cnt@, nk, *p) && p[p.len() - 1] == cur@)]
        #[invariant(v@ + 1 == p.len() && v@ <= cur@)]
        #[variant(cnt@ - cur@)]
        loop {
            v += 1;
            let kc = a[2 * cur];
            if nk == kc {
                snapshot!(keys_step(before.0, a@, cnt@, cnt@, i@, i as u64));
                break; // doublon : ignoré
            }
            let lnk = a[2 * cur + 1];
            let is_left = nk < kc;
            let c = if is_left { lnk & M } else { lnk >> 32 };
            if c != NIL {
                proof_assert!(link_ok(c, cur@, cnt@) && c@ != NIL@);
                proof_assert!(cur@ < c@ && c@ < cnt@);
                snapshot!(search_path_snoc(a@, cnt@, nk, *p, c@));
                cur = c as usize;
                p = snapshot!(p.snoc(c@));
                continue;
            }
            // feuille atteinte : nouveau nœud `cnt`, fils de `cur`
            a[2 * cnt] = nk;
            a[2 * cnt + 1] = LEAF;
            a[2 * cur + 1] = set_link(lnk, is_left, cnt as u64);
            snapshot!(leaf_links());
            proof_assert!(forall<t> 0 <= t && t < cnt@ ==> key(a@, t) == key(before.0, t));
            proof_assert!(forall<t> 0 <= t && t < cnt@ && t != cur@ ==> a@[2 * t + 1] == before.0[2 * t + 1]);
            snapshot!(insert_preserves(before.0, a@, cnt@, cur@, nk, *paths, *p));
            snapshot!(keys_step(before.0, a@, cnt@, cnt@ + 1, i@, i as u64));
            paths = snapshot!(paths.snoc(p.snoc(cnt@)));
            cnt += 1;
            break;
        }
        plen = plen.wrapping_add(v);
        i += 1;
    }
    (cnt, plen, paths)
}

/// Une recherche : (nombre de nœuds visités, trouvée).
#[requires(is_bst(a@, cnt@, *paths))]
#[ensures(result.1 == contains(a@, cnt@, k))]
#[ensures(1 <= result.0@ && result.0@ <= cnt@)]
#[check(terminates)]
pub fn search(a: &Vec<u64>, cnt: usize, k: u64, paths: Snapshot<Seq<Seq<Int>>>) -> (u64, bool) {
    let mut cur = 0usize;
    let mut v = 0u64;
    let mut p = snapshot!(Seq::singleton(0));
    #[invariant(search_path(a@, cnt@, k, *p) && p[p.len() - 1] == cur@)]
    #[invariant(v@ + 1 == p.len() && v@ <= cur@)]
    // `p` est un préfixe du chemin de tout nœud portant `k`
    #[invariant(forall<t> 0 <= t && t < cnt@ && key(a@, t) == k ==>
        p.len() <= paths[t].len() && forall<j> 0 <= j && j < p.len() ==> p[j] == paths[t][j])]
    #[variant(cnt@ - cur@)]
    loop {
        v += 1;
        let kc = a[2 * cur];
        if k == kc {
            return (v, true);
        }
        let lnk = a[2 * cur + 1];
        let c = if k < kc { lnk & M } else { lnk >> 32 };
        if c == NIL {
            return (v, false);
        }
        cur = c as usize;
        p = snapshot!(p.snoc(c@));
    }
}

/// Phase 2 : n succès (clés insérées) puis n/2 échecs (clés paires) ; renvoie r2.
#[requires(n@ <= 2147483646 && is_bst(a@, cnt@, *paths))]
#[requires(forall<i: u64> i@ < n@ ==> contains(a@, cnt@, ins_key(i)))]
#[requires(forall<t> 0 <= t && t < cnt@ ==> exists<i: u64> i@ < n@ && key(a@, t) == ins_key(i))]
#[check(terminates)]
pub fn search_all(a: &Vec<u64>, cnt: usize, n: usize, paths: Snapshot<Seq<Seq<Int>>>) -> u64 {
    let mut ck = CK_INIT;
    let mut k = 0usize;
    #[invariant(k@ <= n@)]
    #[variant(n@ - k@)]
    while k < n {
        let j = (3 * (k as u64)) % (n as u64);
        let (v, found) = search(a, cnt, mix(SEED1.wrapping_add(j)) | 1, paths);
        proof_assert!(found); // toute clé insérée est trouvée
        ck = ckfold(ck, (v << 1) | (found as u64));
        k += 1;
    }
    let half = n / 2;
    let mut m = 0usize;
    #[invariant(m@ <= half@)]
    #[variant(half@ - m@)]
    while m < half {
        let fk = mix(SEED3.wrapping_add(m as u64)) & !1u64;
        let (v, found) = search(a, cnt, fk, paths);
        snapshot!(odd_even(mix_l(SEED3 + m as u64)));
        proof_assert!(forall<i: u64> ins_key(i) != fk); // clé paire, clés insérées impaires
        proof_assert!(!found);
        ck = ckfold(ck, (v << 1) | (found as u64));
        m += 1;
    }
    ck
}

/// Phase 3a : permutation de Sattolo de [0, n) (valeurs < n, permutation de l'identité).
#[requires(1 <= n@ && n@ <= 2147483646 && next@.len() == n@)]
#[ensures((^next)@.len() == n@)]
#[ensures(forall<i> 0 <= i && i < n@ ==> (^next)[i]@ < n@)]
#[ensures(exists<id: Seq<u32>> id.len() == n@ && (forall<k> 0 <= k && k < n@ ==> id[k]@ == k) && (^next)@.permutation_of(id))]
#[check(terminates)]
pub fn sattolo(next: &mut Vec<u32>, n: usize) {
    let mut i = 0usize;
    #[invariant(i@ <= n@ && next@.len() == n@)]
    #[invariant(forall<k> 0 <= k && k < i@ ==> next[k]@ == k)]
    #[variant(n@ - i@)]
    while i < n {
        next[i] = i as u32;
        i += 1;
    }
    let id = snapshot!(next@);
    let mut i = n - 1;
    #[invariant(i@ < n@ && next@.len() == n@)]
    #[invariant(forall<k> 0 <= k && k < n@ ==> next[k]@ < n@)]
    #[invariant(next@.permutation_of(*id))]
    #[variant(i@)]
    while i >= 1 {
        let j = (mix(SEED2.wrapping_add(i as u64)) % (i as u64)) as usize;
        next.swap(i, j);
        i -= 1;
    }
}

/// Phase 3b : 4n sauts en suivant `next` ; renvoie r3.
#[requires(1 <= n@ && n@ <= 2147483646 && next@.len() == n@)]
#[requires(forall<i> 0 <= i && i < n@ ==> next[i]@ < n@)]
#[ensures(result == ckfold_l(ckfold_l(CK_INIT, chase_pos(next@, 4 * n@) as u64), chase_sum(next@, 4 * n@)))]
#[check(terminates)]
pub fn chase(next: &Vec<u32>, n: usize) -> u64 {
    let total = 4 * (n as u64);
    let mut cur = 0usize;
    let mut sum = 0u64;
    let mut h = 0u64;
    #[invariant(h@ <= total@ && cur@ < n@ && cur@ == chase_pos(next@, h@)@ && sum == chase_sum(next@, h@))]
    #[variant(total@ - h@)]
    while h < total {
        cur = next[cur] as usize;
        sum = sum.wrapping_add(cur as u64);
        h += 1;
    }
    ck2(cur as u64, sum)
}

#[requires(1 <= n@ && n@ <= 2147483646)]
pub fn run(n: usize, threads: usize) -> u64 {
    let _ = threads;
    let mut a = std::vec![0u64; 2 * n];
    let mut next = std::vec![0u32; n];
    let (cnt, plen, paths) = build(&mut a, n);
    let r1 = ckfold(ckfold(CK_INIT, cnt as u64), plen);
    let r2 = search_all(&a, cnt, n, paths);
    sattolo(&mut next, n);
    let r3 = chase(&next, n);
    ckfold(ckfold(ckfold(CK_INIT, r1), r2), r3)
}
