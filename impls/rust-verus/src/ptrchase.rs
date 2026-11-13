//! ptrchase : arène par indices, BST non équilibré + recherches + chasse de pointeurs
//! (voir spec/workloads.d/ptrchase.md). 1 <= n <= 2^31 - 2.
//!
//! Prouvé (Verus, sans `assume`, `admit` ni `external_body`) :
//! * invariant d'arbre de recherche [`is_bst`] : chaque lien est `NIL` ou un nœud d'indice strictement plus grand
//!   (donc toute descente termine), et chaque nœud `t` a un chemin fantôme racine → `t` qui est exactement le
//!   chemin de recherche de sa clé (chaque clé est du bon côté de chacun de ses ancêtres) ;
//! * `build` : l'ensemble des clés de l'arbre est exactement `{ ins_key(i) | i < n }` ;
//! * `search` : trouvée ⇔ la clé est dans l'arbre (donc ⇔ elle a été insérée) ; les `n` recherches positives
//!   trouvent, les `n/2` négatives échouent ;
//! * `sattolo` produit une permutation de `0..n` (multiensembles) ; `chase` suit `next` (`chase_pos`, `chase_sum`) ;
//! * absence de panique et de débordement ; terminaison de tout le module.
//!
//! Pas de garde de preuve : ni `c < cnt`, ni borne de visites, ni `v < n` dans la chasse.
//! Non prouvé : le cycle unique de Sattolo (inutile à la sûreté).
use crate::common::*;
use vstd::prelude::*;

verus! {

pub const NIL: u64 = 0xFFFF_FFFF;
pub const M: u64 = 0xFFFF_FFFF;
/// Mot de liens d'une feuille : NIL / NIL.
pub const LEAF: u64 = 0xFFFF_FFFF_FFFF_FFFF;
pub const SEED1: u64 = 0xD00D000000000001;
pub const SEED2: u64 = 0xD00D000000000002;
pub const SEED3: u64 = 0xD00D000000000003;

// ---------------------------------------------------------------- modèle logique

/// Clé insérée à l'étape `i` de la phase 1.
pub open spec fn ins_key(i: u64) -> u64 {
    spec_mix64(SEED1.wrapping_add(i)) | 1
}

/// Clé du nœud `i`.
pub open spec fn key(a: Seq<u64>, i: int) -> u64 {
    a[2 * i]
}

/// Mot de liens du nœud `i`.
pub open spec fn links(a: Seq<u64>, i: int) -> u64 {
    a[2 * i + 1]
}

/// Fils gauche (32 bits de poids faible).
pub open spec fn left(l: u64) -> int {
    (l & M) as int
}

/// Fils droit (32 bits de poids fort).
pub open spec fn right(l: u64) -> int {
    (l >> 32u64) as int
}

/// Successeur de `i` sur le chemin de recherche de `k`.
pub open spec fn step(a: Seq<u64>, i: int, k: u64) -> int {
    if k < key(a, i) { left(links(a, i)) } else { right(links(a, i)) }
}

/// `p` est un préfixe du chemin de recherche de `k` : il part de la racine, ne passe que par des nœuds alloués,
/// aucun nœud avant le dernier ne porte `k`, et chaque nœud est le successeur du précédent.
pub open spec fn search_path(a: Seq<u64>, cnt: int, k: u64, p: Seq<int>) -> bool {
    &&& p.len() >= 1
    &&& p[0] == 0
    &&& forall|j: int| 0 <= j < p.len() ==> 0 <= #[trigger] p[j] < cnt
    &&& forall|j: int| 0 <= j < p.len() - 1 ==> key(a, #[trigger] p[j]) != k && p[j + 1] == step(a, p[j], k)
}

/// Un lien du nœud `i` est `NIL` ou un nœud d'indice dans `(i, cnt)`.
pub open spec fn link_ok(c: int, i: int, cnt: int) -> bool {
    c == NIL || (i < c && c < cnt)
}

/// Invariant d'arbre binaire de recherche sur les `cnt` premiers nœuds, témoigné par `paths` :
/// `paths[t]` est le chemin de recherche de `key(t)` et il aboutit en `t`.
pub open spec fn is_bst(a: Seq<u64>, cnt: int, paths: Seq<Seq<int>>) -> bool {
    &&& 1 <= cnt < NIL
    &&& 2 * cnt <= a.len()
    &&& paths.len() == cnt
    &&& forall|i: int| 0 <= i < cnt ==>
        link_ok(left(#[trigger] links(a, i)), i, cnt) && link_ok(right(links(a, i)), i, cnt)
    &&& forall|t: int| 0 <= t < cnt ==>
        search_path(a, cnt, key(a, t), #[trigger] paths[t]) && paths[t].last() == t
}

/// `k` est l'une des `n` premières clés insérées.
pub open spec fn inserted(k: u64, n: int) -> bool {
    exists|i: u64| i < n && #[trigger] ins_key(i) == k
}

/// La clé `k` est portée par l'un des `cnt` premiers nœuds.
pub open spec fn contains(a: Seq<u64>, cnt: int, k: u64) -> bool {
    exists|t: int| 0 <= t < cnt && #[trigger] key(a, t) == k
}

/// Position après `h` sauts depuis 0.
pub open spec fn chase_pos(nx: Seq<u32>, h: nat) -> u32
    decreases h,
{
    if h == 0 { 0 } else { nx[chase_pos(nx, (h - 1) as nat) as int] }
}

/// Somme (modulo 2^64) des positions visitées pendant les `h` premiers sauts.
pub open spec fn chase_sum(nx: Seq<u32>, h: nat) -> u64
    decreases h,
{
    if h == 0 { 0 } else { chase_sum(nx, (h - 1) as nat).wrapping_add(chase_pos(nx, h) as u64) }
}

// ---------------------------------------------------------------- lemmes binaires

proof fn lemma_links(lnk: u64, c: u64)
    by (bit_vector)
    requires
        c < 0x1_0000_0000,
    ensures
        ((lnk & !M) | c) & M == c,
        ((lnk & !M) | c) >> 32u64 == lnk >> 32u64,
        ((lnk & M) | (c << 32u64)) >> 32u64 == c,
        ((lnk & M) | (c << 32u64)) & M == lnk & M,
        LEAF & M == NIL,
        LEAF >> 32u64 == NIL,
{
}

/// Une clé paire n'est jamais une clé insérée (toutes impaires).
proof fn lemma_odd_even(x: u64, y: u64)
    by (bit_vector)
    ensures
        (x | 1) != (y & !1u64),
{
}

// ---------------------------------------------------------------- code

/// Phase 1 : n insertions dans l'arène (2n u64) ; renvoie (nombre de nœuds, somme des chemins, témoin du BST).
pub fn build(a: &mut Vec<u64>, n: usize) -> (r: (usize, u64, Ghost<Seq<Seq<int>>>))
    requires
        1 <= n <= 2147483646,
        old(a)@.len() == 2 * n,
    ensures
        final(a)@.len() == 2 * n,
        1 <= r.0 <= n,
        is_bst(final(a)@, r.0 as int, r.2@),
        forall|i: u64| i < n ==> #[trigger] contains(final(a)@, r.0 as int, ins_key(i)),
        forall|t: int| 0 <= t < r.0 ==> inserted(#[trigger] key(final(a)@, t), n as int),
{
    a[0] = mix64(SEED1) | 1;
    a[1] = LEAF;
    proof {
        lemma_links(0, 0);
        assert(SEED1.wrapping_add(0) == SEED1);
        assert(key(a@, 0) == ins_key(0));
    }
    let mut cnt = 1usize;
    let mut plen = 0u64;
    let ghost mut paths: Seq<Seq<int>> = seq![seq![0int]];
    let mut i = 1usize;
    while i < n
        invariant
            1 <= i <= n <= 2147483646,
            1 <= cnt <= i,
            a@.len() == 2 * n,
            is_bst(a@, cnt as int, paths),
            forall|j: u64| j < i ==> #[trigger] contains(a@, cnt as int, ins_key(j)),
            forall|t: int| 0 <= t < cnt ==> inserted(#[trigger] key(a@, t), i as int),
        decreases n - i,
    {
        let nk = mix64(SEED1.wrapping_add(i as u64)) | 1;
        assert(nk == ins_key(i as u64));
        let mut cur = 0usize;
        let mut v = 0u64;
        let mut dup = false;
        let mut lnk = 0u64;
        let mut is_left = false;
        // chemin de recherche de `nk` jusqu'à `cur` inclus
        let ghost mut p: Seq<int> = seq![0int];
        loop
            invariant_except_break
                !dup,
                v + 1 == p.len(),
                v <= cur,
            invariant
                // faits de la boucle externe (inchangés pendant la descente)
                1 <= cnt <= i < n <= 2147483646,
                a@.len() == 2 * n,
                nk == ins_key(i as u64),
                is_bst(a@, cnt as int, paths),
                forall|j: u64| j < i ==> #[trigger] contains(a@, cnt as int, ins_key(j)),
                forall|t: int| 0 <= t < cnt ==> inserted(#[trigger] key(a@, t), i as int),
                // descente
                search_path(a@, cnt as int, nk, p),
                p.last() == cur,
                cur < cnt,
            ensures
                v == p.len(),
                v <= cur + 1,
                dup ==> key(a@, cur as int) == nk,
                !dup ==> key(a@, cur as int) != nk && lnk == links(a@, cur as int)
                    && is_left == (nk < key(a@, cur as int)) && step(a@, cur as int, nk) == NIL,
            decreases cnt - cur,
        {
            v += 1;
            let kc = a[2 * cur];
            assert(kc == key(a@, cur as int));
            if nk == kc {
                dup = true;
                break;
            }
            lnk = a[2 * cur + 1];
            is_left = nk < kc;
            let c = if is_left { lnk & M } else { lnk >> 32 };
            if c == NIL {
                break;
            }
            proof {
                assert(link_ok(left(links(a@, cur as int)), cur as int, cnt as int));
                lemma_search_path_push(a@, cnt as int, nk, p, c as int);
                p = p.push(c as int);
            }
            cur = c as usize;
        }
        let ghost a0 = a@;
        let ghost cnt0 = cnt as int;
        if !dup {
            // feuille atteinte : nouveau nœud `cnt`, fils de `cur`
            a[2 * cnt] = nk;
            a[2 * cnt + 1] = LEAF;
            a[2 * cur + 1] = if is_left { (lnk & !M) | (cnt as u64) } else { (lnk & M) | ((cnt as u64) << 32) };
            proof {
                lemma_links(lnk, cnt as u64);
                assert(key(a@, cnt as int) == nk && links(a@, cnt as int) == LEAF);
                assert(links(a@, cur as int) == a@[2 * cur + 1]);
                assert forall|t: int| 0 <= t < cnt implies #[trigger] key(a@, t) == key(a0, t) by {}
                assert forall|t: int| 0 <= t < cnt && t != cur implies #[trigger] links(a@, t) == links(a0, t) by {}
                lemma_insert(a0, a@, cnt as int, cur as int, nk, paths, p);
                paths = paths.push(p.push(cnt as int));
            }
            cnt += 1;
        }
        proof {
            if dup {
                assert(key(a@, cur as int) == ins_key(i as u64));
            } else {
                assert(key(a@, cnt0) == ins_key(i as u64));
            }
            lemma_keys_complete(a0, a@, cnt0, cnt as int, i as u64);
            lemma_keys_sound(a0, a@, cnt0, cnt as int, i as u64);
        }
        plen = plen.wrapping_add(v);
        i += 1;
    }
    (cnt, plen, Ghost(paths))
}

/// Après l'étape `i` de la phase 1 (nouveau nœud ou doublon), toutes les clés insérées sont dans l'arbre.
proof fn lemma_keys_complete(a0: Seq<u64>, a: Seq<u64>, cnt0: int, cnt: int, i: u64)
    requires
        forall|j: u64| j < i ==> #[trigger] contains(a0, cnt0, ins_key(j)),
        forall|t: int| 0 <= t < cnt0 ==> #[trigger] key(a, t) == key(a0, t),
        cnt0 <= cnt,
        contains(a, cnt, ins_key(i)),
    ensures
        forall|j: u64| j < i + 1 ==> #[trigger] contains(a, cnt, ins_key(j)),
{
    assert forall|j: u64| j < i + 1 implies #[trigger] contains(a, cnt, ins_key(j)) by {
        if j < i {
            assert(contains(a0, cnt0, ins_key(j)));
            let t = choose|t: int| 0 <= t < cnt0 && #[trigger] key(a0, t) == ins_key(j);
            assert(key(a, t) == key(a0, t));
        }
    }
}

/// Après l'étape `i` de la phase 1, toute clé de l'arbre est une clé insérée.
proof fn lemma_keys_sound(a0: Seq<u64>, a: Seq<u64>, cnt0: int, cnt: int, i: u64)
    requires
        forall|t: int| 0 <= t < cnt0 ==> inserted(#[trigger] key(a0, t), i as int),
        forall|t: int| 0 <= t < cnt0 ==> #[trigger] key(a, t) == key(a0, t),
        cnt == cnt0 || cnt == cnt0 + 1,
        cnt == cnt0 + 1 ==> key(a, cnt0) == ins_key(i),
    ensures
        forall|t: int| 0 <= t < cnt ==> inserted(#[trigger] key(a, t), i + 1),
{
    assert forall|t: int| 0 <= t < cnt implies inserted(#[trigger] key(a, t), i + 1) by {
        if t < cnt0 {
            assert(key(a, t) == key(a0, t));
            assert(inserted(key(a0, t), i as int));
            let j = choose|j: u64| j < i && #[trigger] ins_key(j) == key(a0, t);
            assert(ins_key(j) == key(a, t));
        } else {
            assert(ins_key(i) == key(a, t));
        }
    }
}

/// Prolonger un chemin de recherche d'un pas valide.
proof fn lemma_search_path_push(a: Seq<u64>, cnt: int, k: u64, p: Seq<int>, c: int)
    requires
        search_path(a, cnt, k, p),
        key(a, p.last()) != k,
        c == step(a, p.last(), k),
        0 <= c < cnt,
    ensures
        search_path(a, cnt, k, p.push(c)),
{
    let q = p.push(c);
    assert forall|j: int| 0 <= j < q.len() implies 0 <= #[trigger] q[j] < cnt by {
        if j < p.len() {
            assert(q[j] == p[j]);
        }
    }
    assert forall|j: int| 0 <= j < q.len() - 1 implies key(a, #[trigger] q[j]) != k && q[j + 1] == step(a, q[j], k) by {
        assert(q[j] == p[j]);
        if j < p.len() - 1 {
            assert(q[j + 1] == p[j + 1]);
        }
    }
}

/// L'insertion d'une feuille `cnt` (clé `nk`) sous `cur`, au bout du chemin de recherche `p`, préserve l'invariant.
proof fn lemma_insert(a0: Seq<u64>, a: Seq<u64>, cnt: int, cur: int, nk: u64, paths: Seq<Seq<int>>, p: Seq<int>)
    requires
        is_bst(a0, cnt, paths),
        cnt + 1 < NIL,
        2 * (cnt + 1) <= a.len(),
        a.len() == a0.len(),
        search_path(a0, cnt, nk, p),
        p.last() == cur,
        key(a0, cur) != nk,
        step(a0, cur, nk) == NIL,
        key(a, cnt) == nk,
        left(links(a, cnt)) == NIL && right(links(a, cnt)) == NIL,
        nk < key(a0, cur) ==> left(links(a, cur)) == cnt && right(links(a, cur)) == right(links(a0, cur)),
        !(nk < key(a0, cur)) ==> right(links(a, cur)) == cnt && left(links(a, cur)) == left(links(a0, cur)),
        forall|i: int| 0 <= i < cnt ==> #[trigger] key(a, i) == key(a0, i),
        forall|i: int| 0 <= i < cnt && i != cur ==> #[trigger] links(a, i) == links(a0, i),
    ensures
        is_bst(a, cnt + 1, paths.push(p.push(cnt))),
{
    let np = paths.push(p.push(cnt));
    // les liens restent bien formés
    assert forall|i: int| 0 <= i < cnt + 1 implies
        link_ok(left(#[trigger] links(a, i)), i, cnt + 1) && link_ok(right(links(a, i)), i, cnt + 1) by {
        if i < cnt && i != cur {
            assert(links(a, i) == links(a0, i));
        }
    }
    // hors de `cur` (côté inséré), les pas de recherche sont inchangés
    assert forall|i: int, k: u64| 0 <= i < cnt && step(a0, i, k) != NIL implies #[trigger] step(a, i, k) == step(a0, i, k) by {
        if i != cur {
            assert(links(a, i) == links(a0, i));
        }
    }
    // les anciens chemins restent des chemins de recherche
    assert forall|t: int| 0 <= t < cnt + 1 implies
        search_path(a, cnt + 1, key(a, t), #[trigger] np[t]) && np[t].last() == t by {
        if t < cnt {
            let q = paths[t];
            assert(np[t] == q);
            assert forall|j: int| 0 <= j < q.len() - 1 implies
                key(a, #[trigger] q[j]) != key(a, t) && q[j + 1] == step(a, q[j], key(a, t)) by {
                assert(0 <= q[j] < cnt && 0 <= q[j + 1] < cnt);
                assert(step(a0, q[j], key(a0, t)) == q[j + 1]);
            }
        } else {
            let q = p.push(cnt);
            assert(np[t] == q);
            assert forall|j: int| 0 <= j < q.len() - 1 implies
                key(a, #[trigger] q[j]) != nk && q[j + 1] == step(a, q[j], nk) by {
                if j < p.len() - 1 {
                    assert(q[j] == p[j] && q[j + 1] == p[j + 1]);
                    assert(step(a0, p[j], nk) == p[j + 1]);
                } else {
                    assert(q[j] == cur);
                }
            }
            assert forall|j: int| 0 <= j < q.len() implies 0 <= #[trigger] q[j] < cnt + 1 by {
                if j < p.len() {
                    assert(q[j] == p[j]);
                }
            }
        }
    }
}

/// Une recherche : (nombre de nœuds visités, trouvée).
pub fn search(a: &Vec<u64>, cnt: usize, k: u64, Ghost(paths): Ghost<Seq<Seq<int>>>) -> (r: (u64, bool))
    requires
        cnt <= 2147483646,
        is_bst(a@, cnt as int, paths),
    ensures
        r.1 == contains(a@, cnt as int, k),
        1 <= r.0 <= cnt,
{
    let mut cur = 0usize;
    let mut v = 0u64;
    let ghost mut p: Seq<int> = seq![0int];
    loop
        invariant
            cnt <= 2147483646,
            is_bst(a@, cnt as int, paths),
            search_path(a@, cnt as int, k, p),
            p.last() == cur,
            cur < cnt,
            v + 1 == p.len(),
            v <= cur,
            // `p` est un préfixe du chemin de tout nœud portant `k`
            forall|t: int| 0 <= t < cnt && #[trigger] key(a@, t) == k ==>
                p.len() <= paths[t].len() && forall|j: int| 0 <= j < p.len() ==> p[j] == #[trigger] paths[t][j],
        decreases cnt - cur,
    {
        v += 1;
        let kc = a[2 * cur];
        if k == kc {
            assert(key(a@, cur as int) == k);
            return (v, true);
        }
        let lnk = a[2 * cur + 1];
        let c = if k < kc { lnk & M } else { lnk >> 32 };
        assert(c as int == step(a@, cur as int, k));
        proof {
            // tout nœud portant `k` a un chemin qui prolonge strictement `p`, par `c`
            assert forall|t: int| 0 <= t < cnt && #[trigger] key(a@, t) == k implies
                p.len() < paths[t].len() && paths[t][p.len() as int] == c by {
                let q = paths[t];
                assert(q[p.len() - 1] == p[p.len() - 1]);
                if p.len() == q.len() {
                    assert(q.last() == t);
                }
                assert(key(a@, q[p.len() - 1]) != k);
            }
        }
        if c == NIL {
            return (v, false);
        }
        proof {
            let p0 = p;
            p = p.push(c as int);
            assert forall|t: int| 0 <= t < cnt && #[trigger] key(a@, t) == k implies
                p.len() <= paths[t].len() && forall|j: int| 0 <= j < p.len() ==> p[j] == #[trigger] paths[t][j] by {
                assert forall|j: int| 0 <= j < p.len() implies p[j] == #[trigger] paths[t][j] by {
                    if j < p0.len() {
                        assert(p[j] == p0[j]);
                    }
                }
            }
            assert forall|j: int| 0 <= j < p.len() - 1 implies
                key(a@, #[trigger] p[j]) != k && p[j + 1] == step(a@, p[j], k) by {
                if j < p0.len() - 1 {
                    assert(p[j] == p0[j] && p[j + 1] == p0[j + 1]);
                }
            }
        }
        cur = c as usize;
    }
}

/// Phase 2 : n succès (clés insérées) puis n/2 échecs (clés paires) ; renvoie r2.
pub fn search_all(a: &Vec<u64>, cnt: usize, n: usize, Ghost(paths): Ghost<Seq<Seq<int>>>) -> u64
    requires
        cnt <= n <= 2147483646,
        is_bst(a@, cnt as int, paths),
        forall|i: u64| i < n ==> #[trigger] contains(a@, cnt as int, ins_key(i)),
        forall|t: int| 0 <= t < cnt ==> inserted(#[trigger] key(a@, t), n as int),
{
    let mut ck = CK_INIT;
    let mut k = 0usize;
    while k < n
        invariant
            k <= n <= 2147483646,
            cnt <= n,
            is_bst(a@, cnt as int, paths),
            forall|i: u64| i < n ==> #[trigger] contains(a@, cnt as int, ins_key(i)),
        decreases n - k,
    {
        let j = (3 * (k as u64)) % (n as u64);
        let (v, found) = search(a, cnt, mix64(SEED1.wrapping_add(j)) | 1, Ghost(paths));
        assert(contains(a@, cnt as int, ins_key(j)));
        assert(found); // toute clé insérée est trouvée
        ck = ckfold(ck, (v << 1) | (found as u64));
        k += 1;
    }
    let half = n / 2;
    let mut m = 0usize;
    while m < half
        invariant
            m <= half <= n <= 2147483646,
            cnt <= n,
            is_bst(a@, cnt as int, paths),
            forall|t: int| 0 <= t < cnt ==> inserted(#[trigger] key(a@, t), n as int),
        decreases half - m,
    {
        let fk = mix64(SEED3.wrapping_add(m as u64)) & !1u64;
        let (v, found) = search(a, cnt, fk, Ghost(paths));
        proof {
            // clé paire, clés insérées impaires
            if found {
                let t = choose|t: int| 0 <= t < cnt && #[trigger] key(a@, t) == fk;
                assert(inserted(key(a@, t), n as int));
                let i = choose|i: u64| i < n && #[trigger] ins_key(i) == key(a@, t);
                lemma_odd_even(spec_mix64(SEED1.wrapping_add(i)), spec_mix64(SEED3.wrapping_add(m as u64)));
            }
        }
        assert(!found);
        ck = ckfold(ck, (v << 1) | (found as u64));
        m += 1;
    }
    ck
}

/// Phase 3a : permutation de Sattolo de [0, n) (valeurs < n, permutation de l'identité).
pub fn sattolo(next: &mut Vec<u32>, n: usize)
    requires
        1 <= n <= 2147483646,
        old(next)@.len() == n,
    ensures
        final(next)@.len() == n,
        forall|i: int| 0 <= i < n ==> #[trigger] final(next)@[i] < n,
        final(next)@.to_multiset() == Seq::new(n as nat, |i: int| i as u32).to_multiset(),
{
    let mut i = 0usize;
    while i < n
        invariant
            i <= n <= 2147483646,
            next@.len() == n,
            forall|k: int| 0 <= k < i ==> #[trigger] next@[k] == k,
        decreases n - i,
    {
        next[i] = i as u32;
        i += 1;
    }
    assert(next@ =~= Seq::new(n as nat, |i: int| i as u32));
    let ghost id = next@;
    let mut i = n - 1;
    while i >= 1
        invariant
            i < n <= 2147483646,
            next@.len() == n,
            forall|k: int| 0 <= k < n ==> #[trigger] next@[k] < n,
            next@.to_multiset() == id.to_multiset(),
        decreases i,
    {
        let j = (mix64(SEED2.wrapping_add(i as u64)) % (i as u64)) as usize;
        let ghost s0 = next@;
        next.swap(i, j);
        proof {
            broadcast use vstd::seq_lib::to_multiset_update, vstd::seq_lib::to_multiset_contains,
                vstd::multiset::group_multiset_axioms;
            assert(s0.contains(s0[i as int]) && s0.contains(s0[j as int]));
            let s1 = s0.update(i as int, s0[j as int]);
            assert(s1.to_multiset() =~= s0.to_multiset().insert(s0[j as int]).remove(s0[i as int]));
            assert(next@ == s1.update(j as int, s0[i as int]));
            assert(next@.to_multiset() =~= s1.to_multiset().insert(s0[i as int]).remove(s1[j as int]));
            assert(next@.to_multiset() =~= s0.to_multiset());
        }
        i -= 1;
    }
}

/// Phase 3b : 4n sauts en suivant `next` ; renvoie r3.
pub fn chase(next: &Vec<u32>, n: usize) -> (r: u64)
    requires
        1 <= n <= 2147483646,
        next@.len() == n,
        forall|i: int| 0 <= i < n ==> #[trigger] next@[i] < n,
    ensures
        r == spec_ckfold(spec_ckfold(CK_INIT, chase_pos(next@, 4 * n as nat) as u64), chase_sum(next@, 4 * n as nat)),
{
    let total = 4 * (n as u64);
    let mut cur = 0usize;
    let mut sum = 0u64;
    let mut h = 0u64;
    while h < total
        invariant
            total == 4 * n,
            h <= total,
            cur < n,
            next@.len() == n,
            forall|i: int| 0 <= i < n ==> #[trigger] next@[i] < n,
            cur == chase_pos(next@, h as nat),
            sum == chase_sum(next@, h as nat),
        decreases total - h,
    {
        cur = next[cur] as usize;
        sum = sum.wrapping_add(cur as u64);
        h += 1;
    }
    ckfold(ckfold(CK_INIT, cur as u64), sum)
}

pub fn run(n: usize, threads: usize) -> u64
    requires
        1 <= n <= 2147483646,
{
    let _ = threads;
    let mut a = vec![0u64; 2 * n];
    let mut next = vec![0u32; n];
    let (cnt, plen, paths) = build(&mut a, n);
    let r1 = ckfold(ckfold(CK_INIT, cnt as u64), plen);
    let r2 = search_all(&a, cnt, n, paths);
    sattolo(&mut next, n);
    let r3 = chase(&next, n);
    ckfold(ckfold(ckfold(CK_INIT, r1), r2), r3)
}

} // verus!
