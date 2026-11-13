//! Débit « lignes/seconde » : génère n lignes ASCII « clé;valeur\n » dans un buffer, puis les relit
//! octet par octet et agrège somme/compte par clé (2^20 clés). 1 <= n <= 268_435_455.
//! Spécification : spec/WORKLOADS.md §4.
//!
//! Prouvé avec Verus (correction fonctionnelle) :
//!   * `generate` : `txt@ == text(n)`, concaténation des lignes `dec(clé) ';' dec(valeur) '\n'`
//!     avec `draw(i) = mix64(SEED + i)` (fonction `spec_mix64` de `common`) ;
//!   * `parse` : sur ce texte, `tab[2k]` = somme des valeurs des lignes de clé k, `tab[2k+1]` = leur nombre
//!     (`sum_v`, `cnt`) ; repose sur le lemme d'aller-retour `lemma_num_dec` : analyse ∘ écriture décimale = id ;
//!   * absence de panique et de débordement (additions de `parse` prouvées exactes : somme <= 999 999·n < 2^64),
//!     terminaison de toutes les fonctions (`decreases`).
//! Aucune garde d'exécution (pas de masque sur la clé lue, la lecture parcourt exactement le texte produit).
//! Aucun `assume`/`admit`/`external_body`.
use crate::common::*;
use vstd::prelude::*;

verus! {

pub const SEED: u64 = 0x1357924680ACEBDF;

// ───────────────────────────── Modèle ─────────────────────────────

/// i-ème tirage aléatoire (opaque : splitmix64 n'est déplié que dans `generate`).
#[verifier::opaque]
pub open spec fn draw(i: int) -> u64 {
    spec_mix64((SEED + i) as u64)
}

/// Clé (20 bits de poids fort) et valeur du tirage r.
pub open spec fn key_of(r: u64) -> nat {
    (r >> 44u64) as nat
}

pub open spec fn val_of(r: u64) -> nat {
    (((r >> 8u64) & 0xFFFF_FFFFu64) as nat) % 1000000
}

/// Écriture décimale (ASCII, sans zéro de tête).
pub open spec fn dec(v: nat) -> Seq<u8>
    decreases v,
{
    if v < 10 {
        seq![(48 + v) as u8]
    } else {
        dec(v / 10).push((48 + v % 10) as u8)
    }
}

/// Valeur décimale des j premiers chiffres de s (Horner, de gauche à droite).
pub open spec fn num(s: Seq<u8>, j: int) -> int
    decreases j,
{
    if j <= 0 {
        0
    } else {
        num(s, j - 1) * 10 + (s[j - 1] - 48)
    }
}

/// Une ligne : dec(k) ';' dec(v) '\n'.
pub open spec fn line(k: nat, v: nat) -> Seq<u8> {
    dec(k).push(59u8) + dec(v).push(10u8)
}

pub open spec fn line_of(r: u64) -> Seq<u8> {
    line(key_of(r), val_of(r))
}

/// Texte des i premières lignes.
pub open spec fn text(i: int) -> Seq<u8>
    decreases i,
{
    if i <= 0 {
        Seq::empty()
    } else {
        text(i - 1) + line_of(draw(i - 1))
    }
}

/// Somme des valeurs / nombre des lignes de clé k parmi les i premières.
pub open spec fn sum_v(i: int, k: int) -> nat
    decreases i,
{
    if i <= 0 {
        0
    } else {
        sum_v(i - 1, k) + if key_of(draw(i - 1)) == k { val_of(draw(i - 1)) } else { 0 }
    }
}

pub open spec fn cnt(i: int, k: int) -> nat
    decreases i,
{
    if i <= 0 {
        0
    } else {
        cnt(i - 1, k) + if key_of(draw(i - 1)) == k { 1nat } else { 0nat }
    }
}

/// La table t contient les agrégats des i premières lignes (opaque : dépliée seulement dans les lemmes).
#[verifier::opaque]
pub open spec fn agg_ok(t: Seq<u64>, i: int) -> bool {
    &&& t.len() == 2097152
    &&& forall|k: int| 0 <= k < 1048576 ==> t[2 * k] == #[trigger] sum_v(i, k)
    &&& forall|k: int| 0 <= k < 1048576 ==> t[2 * k + 1] == #[trigger] cnt(i, k)
}

// ───────────────────────────── Lemmes ─────────────────────────────

/// La clé tient sur 20 bits.
pub proof fn lemma_key_bound(r: u64)
    ensures
        key_of(r) < 1048576,
{
    assert((r >> 44u64) < 1048576u64) by (bit_vector);
}

/// dec(v) est formé de chiffres ; sa longueur est au plus k si v < 10^k (k = 1..7).
pub proof fn lemma_dec_shape(v: nat)
    ensures
        1 <= dec(v).len(),
        forall|p: int| 0 <= p < dec(v).len() ==> 48 <= #[trigger] dec(v)[p] <= 57,
        v < 10 ==> dec(v).len() <= 1,
        v < 100 ==> dec(v).len() <= 2,
        v < 1000 ==> dec(v).len() <= 3,
        v < 10000 ==> dec(v).len() <= 4,
        v < 100000 ==> dec(v).len() <= 5,
        v < 1000000 ==> dec(v).len() <= 6,
        v < 10000000 ==> dec(v).len() <= 7,
    decreases v,
{
    if v >= 10 {
        lemma_dec_shape(v / 10);
    }
}

/// num ne dépend que des j premiers éléments.
pub proof fn lemma_num_ext(s: Seq<u8>, t: Seq<u8>, j: int)
    requires
        forall|p: int| 0 <= p < j ==> s[p] == t[p],
    ensures
        num(s, j) == num(t, j),
    decreases j,
{
    if j > 0 {
        lemma_num_ext(s, t, j - 1);
    }
}

/// Lemme d'aller-retour : num(dec(v)) == v ; tout préfixe de dec(v) vaut entre 0 et v.
pub proof fn lemma_num_dec(v: nat)
    ensures
        num(dec(v), dec(v).len() as int) == v,
        forall|j: int| 0 <= j <= dec(v).len() ==> 0 <= #[trigger] num(dec(v), j) <= v,
    decreases v,
{
    lemma_dec_shape(v);
    if v >= 10 {
        lemma_num_dec(v / 10);
        let (s, t) = (dec(v), dec(v / 10));
        assert(s == t.push((48 + v % 10) as u8));
        assert forall|j: int| 0 <= j <= t.len() implies #[trigger] num(s, j) == num(t, j) by {
            lemma_num_ext(s, t, j);
        }
        assert(s[t.len() as int] == 48 + v % 10);
        assert(num(s, s.len() as int) == num(s, t.len() as int) * 10 + (s[t.len() as int] - 48));
        assert(num(s, s.len() as int) == v);
        assert forall|j: int| 0 <= j <= s.len() implies 0 <= #[trigger] num(s, j) <= v by {
            if j <= t.len() {
                assert(num(t, j) <= v / 10);
            }
        }
    } else {
        let s = dec(v);
        assert(num(s, 0) == 0);
        assert(num(s, 1) == num(s, 0) * 10 + (s[0] - 48));
    }
}

/// Découpage d'une ligne.
pub proof fn lemma_line(k: nat, v: nat)
    ensures
        line(k, v).len() == dec(k).len() + dec(v).len() + 2,
        forall|j: int| 0 <= j < dec(k).len() ==> #[trigger] line(k, v)[j] == dec(k)[j],
        line(k, v)[dec(k).len() as int] == 59,
        forall|j: int| 0 <= j < dec(v).len() ==> #[trigger] line(k, v)[dec(k).len() + 1 + j] == dec(v)[j],
        line(k, v)[(dec(k).len() + dec(v).len() + 1) as int] == 10,
{
}

/// text(i) est un préfixe de text(n) pour i <= n.
pub proof fn lemma_text_prefix(i: int, n: int)
    requires
        0 <= i <= n,
    ensures
        text(i).len() <= text(n).len(),
        forall|p: int| 0 <= p < text(i).len() ==> #[trigger] text(n)[p] == text(i)[p],
    decreases n,
{
    if i < n {
        lemma_text_prefix(i, n - 1);
    }
}

/// Octet p du texte complet, situé dans la ligne i (à l'offset j) : chiffre de la clé, ';',
/// chiffre de la valeur ou '\n' selon j.
pub proof fn lemma_step(n: int, i: int, p: int)
    requires
        0 <= i < n,
        text(i).len() <= p < text(i + 1).len(),
    ensures
        ({
            let r = draw(i);
            let j = p - text(i).len();
            let kd = dec(key_of(r));
            let vd = dec(val_of(r));
            &&& key_of(r) < 1048576 && val_of(r) < 1000000
            &&& 1 <= kd.len() <= 7 && 1 <= vd.len() <= 6
            &&& text(i + 1).len() == text(i).len() + kd.len() + vd.len() + 2
            &&& text(i + 1).len() <= text(n).len()
            &&& (j < kd.len() ==> text(n)[p] == kd[j] && 48 <= text(n)[p] <= 57)
            &&& (j == kd.len() ==> text(n)[p] == 59)
            &&& (kd.len() < j <= kd.len() + vd.len() ==> text(n)[p] == vd[j - kd.len() - 1] && 48 <= text(n)[p] <= 57)
            &&& (j == kd.len() + vd.len() + 1 ==> text(n)[p] == 10)
        }),
{
    let r = draw(i);
    lemma_text_prefix(i + 1, n);
    lemma_key_bound(r);
    lemma_dec_shape(key_of(r));
    lemma_dec_shape(val_of(r));
    lemma_line(key_of(r), val_of(r));
    assert(text(i + 1) == text(i) + line_of(r));
    assert(text(n)[p] == line_of(r)[p - text(i).len()]);
}

/// Bornes des agrégats (absence de débordement dans `parse`).
pub proof fn lemma_agg_bound(i: int, k: int)
    requires
        0 <= i,
    ensures
        sum_v(i, k) <= 999999 * i,
        cnt(i, k) <= i,
    decreases i,
{
    if i > 0 {
        lemma_agg_bound(i - 1, k);
    }
}

/// Lecture d'une case de la table.
pub proof fn lemma_agg_get(t: Seq<u64>, i: int, k: int)
    requires
        agg_ok(t, i),
        0 <= k < 1048576,
    ensures
        t.len() == 2097152,
        t[2 * k] == sum_v(i, k),
        t[2 * k + 1] == cnt(i, k),
{
    reveal(agg_ok);
}

/// Fin de la ligne i (clé k = key_of(draw(i))) : mise à jour des deux cases de k.
pub proof fn lemma_agg_step(t: Seq<u64>, i: int, k: int)
    requires
        agg_ok(t, i),
        0 <= i,
        k == key_of(draw(i)),
        0 <= k < 1048576,
        t[2 * k] + val_of(draw(i)) <= u64::MAX,
        t[2 * k + 1] + 1 <= u64::MAX,
    ensures
        agg_ok(t.update(2 * k, (t[2 * k] + val_of(draw(i))) as u64).update(2 * k + 1, (t[2 * k + 1] + 1) as u64), i + 1),
{
    reveal(agg_ok);
    let t2 = t.update(2 * k, (t[2 * k] + val_of(draw(i))) as u64).update(2 * k + 1, (t[2 * k + 1] + 1) as u64);
    assert forall|kk: int| 0 <= kk < 1048576 implies t2[2 * kk] == #[trigger] sum_v(i + 1, kk) by {
        lemma_agg_get(t, i, kk);
        if kk != k {
            assert(2 * kk != 2 * k && 2 * kk != 2 * k + 1);
        }
    }
    assert forall|kk: int| 0 <= kk < 1048576 implies t2[2 * kk + 1] == #[trigger] cnt(i + 1, kk) by {
        lemma_agg_get(t, i, kk);
        if kk != k {
            assert(2 * kk + 1 != 2 * k && 2 * kk + 1 != 2 * k + 1);
        }
    }
}

// ───────────────────────────── Code exécutable ─────────────────────────────

/// Ajoute l'écriture décimale de v à out (chiffres calculés de droite à gauche dans un tampon local).
pub fn put_dec(out: &mut Vec<u8>, v: u32)
    requires
        v < 10000000,
    ensures
        final(out)@ == old(out)@ + dec(v as nat),
{
    let mut tmp = [0u8; 10];
    let mut l: usize = 0;
    let mut x = v;
    proof {
        lemma_dec_shape(v as nat);
    }
    // tmp[0..l) contient, à l'envers, la fin de dec(v) ; dec(x) en est le début.
    loop
        invariant_except_break
            0 < x || l == 0,
            l + dec(x as nat).len() == dec(v as nat).len(),
            forall|p: int| 0 <= p < dec(x as nat).len() ==> #[trigger] dec(x as nat)[p] == dec(v as nat)[p],
        invariant
            dec(v as nat).len() <= 7,
            forall|p: int| 0 <= p < l ==> tmp@[p] == #[trigger] dec(v as nat)[dec(v as nat).len() - 1 - p],
        ensures
            l == dec(v as nat).len(),
        decreases x,
    {
        let ghost x0 = x as nat;
        proof {
            lemma_dec_shape(x0);
            lemma_dec_shape(x0 / 10);
        }
        let ghost dv = dec(v as nat);
        let ghost dx = dec(x0);
        assert(dx[dx.len() - 1] == (48 + x0 % 10) as u8);  // dernier chiffre de x
        assert(dx[dx.len() - 1] == dv[dv.len() - 1 - l]);
        tmp[l] = b'0' + (x % 10) as u8;
        l += 1;
        x /= 10;
        proof {
            if x0 >= 10 {
                assert(dx == dec(x as nat).push((48 + x0 % 10) as u8));
                assert forall|p: int| 0 <= p < dec(x as nat).len() implies #[trigger] dec(x as nat)[p] == dv[p] by {
                    assert(dec(x as nat)[p] == dx[p]);
                }
            }
            assert forall|p: int| 0 <= p < l implies tmp@[p] == #[trigger] dv[dv.len() - 1 - p] by {
                if p < l - 1 {
                    assert(tmp@[p] == dv[dv.len() - 1 - p]);
                }
            }
        }
        if x == 0 {
            break;
        }
    }
    let ghost orig = out@;
    let ghost d = dec(v as nat);
    let mut j = l;
    while j > 0
        invariant
            j <= l,
            l == d.len() <= 7,
            d == dec(v as nat),
            forall|p: int| 0 <= p < l ==> tmp@[p] == #[trigger] d[d.len() - 1 - p],
            out@ =~= orig + d.take(l - j),
        decreases j,
    {
        j -= 1;
        assert(tmp@[j as int] == d[d.len() - 1 - j]);
        out.push(tmp[j]);
        assert(d.take(l - j) =~= d.take(l - j - 1).push(d[l - j - 1]));
    }
    assert(d.take(l as int) =~= d);
}

/// Génère les n lignes.
pub fn generate(n: usize) -> (txt: Vec<u8>)
    requires
        n <= 268435455,
    ensures
        txt@ == text(n as int),
{
    let mut txt: Vec<u8> = Vec::with_capacity(n * 15 + 20);
    let mut i: usize = 0;
    while i < n
        invariant
            i <= n <= 268435455,
            txt@ == text(i as int),
        decreases n - i,
    {
        let r = mix64(SEED.wrapping_add(i as u64));
        let key = (r >> 44) as u32;
        let val = (((r >> 8) & 0xFFFF_FFFF) % 1_000_000) as u32;
        proof {
            lemma_key_bound(r);
            reveal(draw);
            assert(r == draw(i as int));
        }
        put_dec(&mut txt, key);
        txt.push(b';');
        put_dec(&mut txt, val);
        txt.push(b'\n');
        assert(txt@ =~= text(i as int) + line_of(r));
        i += 1;
    }
    txt
}

/// Lit le texte octet par octet (machine à états : chiffres de la clé jusqu'à ';', puis chiffres de la valeur
/// jusqu'à '\n') et agrège : tab[2k] = somme des valeurs, tab[2k+1] = nombre de lignes de la clé k.
pub fn parse(txt: &Vec<u8>, tab: &mut Vec<u64>, n: Ghost<int>)
    requires
        0 <= n@ <= 268435455,
        txt@ == text(n@),
        old(tab)@.len() == 2097152,
        forall|p: int| 0 <= p < 2097152 ==> old(tab)@[p] == 0,
    ensures
        final(tab)@.len() == 2097152,
        forall|k: int| 0 <= k < 1048576 ==> final(tab)@[2 * k] == #[trigger] sum_v(n@, k),
        forall|k: int| 0 <= k < 1048576 ==> final(tab)@[2 * k + 1] == #[trigger] cnt(n@, k),
{
    let ghost mut i: int = 0;  // ligne courante
    let mut p: usize = 0;
    let mut in_val = false;
    let mut key: u32 = 0;
    let mut vl: u32 = 0;
    proof {
        reveal(agg_ok);
        assert forall|k: int| 0 <= k < 1048576 implies tab@[2 * k] == #[trigger] sum_v(0, k) by {
            assert(tab@[2 * k] == 0);
        }
        assert forall|k: int| 0 <= k < 1048576 implies tab@[2 * k + 1] == #[trigger] cnt(0, k) by {
            assert(tab@[2 * k + 1] == 0);
        }
    }
    while p < txt.len()
        invariant
            0 <= i <= n@ <= 268435455,
            txt@ == text(n@),
            text(i).len() <= p <= txt@.len(),
            i == n@ ==> p == text(i).len() && !in_val,
            i < n@ ==> p < text(i + 1).len(),
            i < n@ && !in_val ==> p - text(i).len() <= dec(key_of(draw(i))).len()
                && key == num(dec(key_of(draw(i))), p - text(i).len()) && vl == 0,
            i < n@ && in_val ==> dec(key_of(draw(i))).len() < p - text(i).len()
                <= dec(key_of(draw(i))).len() + 1 + dec(val_of(draw(i))).len()
                && key == key_of(draw(i))
                && vl == num(dec(val_of(draw(i))), p - text(i).len() - dec(key_of(draw(i))).len() - 1),
            agg_ok(tab@, i),
        decreases txt@.len() - p,
    {
        let c = txt[p];
        // Position courante : octet j de la ligne i, découpée en kd ';' vd '\n'.
        let ghost r = draw(i);
        let ghost j = p - text(i).len();
        let ghost kd = dec(key_of(r));
        let ghost vd = dec(val_of(r));
        proof {
            lemma_step(n@, i, p as int);
            lemma_num_dec(key_of(r));
            lemma_num_dec(val_of(r));
        }
        if in_val {
            if c == b'\n' {
                let ghost tab0 = tab@;
                proof {
                    lemma_agg_bound(i, key as int);
                    lemma_agg_get(tab0, i, key as int);
                }
                let k = key as usize;
                tab[2 * k] += vl as u64;
                tab[2 * k + 1] += 1;
                proof {
                    lemma_agg_step(tab0, i, k as int);
                    assert(tab@ == tab0.update(2 * k, (tab0[2 * k] + vl) as u64).update(2 * k + 1, (tab0[2 * k + 1] + 1) as u64));
                    i = i + 1;
                    if i < n@ {
                        lemma_step(n@, i, text(i).len() as int);
                    }
                }
                in_val = false;
                key = 0;
                vl = 0;
            } else {
                vl = vl * 10 + (c - b'0') as u32;
            }
        } else if c == b';' {
            in_val = true;
        } else {
            key = key * 10 + (c - b'0') as u32;
        }
        p += 1;
    }
    proof {
        if i < n@ {
            lemma_text_prefix(i + 1, n@);
        }
        assert(i == n@);
        assert forall|k: int| 0 <= k < 1048576 implies tab@[2 * k] == #[trigger] sum_v(n@, k) by {
            lemma_agg_get(tab@, n@, k);
        }
        assert forall|k: int| 0 <= k < 1048576 implies tab@[2 * k + 1] == #[trigger] cnt(n@, k) by {
            lemma_agg_get(tab@, n@, k);
        }
    }
}

/// ck = fold(fold(ck, tab[2k]), tab[2k+1]) pour k croissant.
pub fn checksum(tab: &Vec<u64>) -> u64
    requires
        tab@.len() == 2097152,
{
    let mut ck = CK_INIT;
    let mut k: usize = 0;
    while k < 1048576
        invariant
            tab@.len() == 2097152,
            k <= 1048576,
        decreases 1048576 - k,
    {
        ck = ckfold(ckfold(ck, tab[2 * k]), tab[2 * k + 1]);
        k += 1;
    }
    ck
}

pub fn run(n: usize, _threads: usize) -> u64
    requires
        1 <= n <= 268435455,
{
    let txt = generate(n);
    let mut tab = vec![0u64; 2097152];
    parse(&txt, &mut tab, Ghost(n as int));
    checksum(&tab)
}

} // verus!
