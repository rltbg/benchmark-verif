//! Débit « lignes/seconde » : génère n lignes ASCII « clé;valeur\n » dans un buffer, puis les relit
//! octet par octet et agrège somme/compte par clé (2^20 clés). 1 <= n <= 268_435_455.
use crate::common::*;
use creusot_std::prelude::*;

const SEED: u64 = 0x1357924680ACEBDF;

/// Nombre de chiffres décimaux (au plus 7 ; au plus 6 si v < 10^6).
#[ensures(1 <= result@ && result@ <= 7)]
#[ensures(v@ < 1000000 ==> result@ <= 6)]
#[check(terminates)]
pub fn ndigits(v: u32) -> usize {
    if v < 10 {
        1
    } else if v < 100 {
        2
    } else if v < 1000 {
        3
    } else if v < 10000 {
        4
    } else if v < 100000 {
        5
    } else if v < 1000000 {
        6
    } else {
        7
    }
}

/// Écrit les l chiffres de v dans txt[off, off+l), de droite à gauche.
#[requires(1 <= l@ && off@ + l@ <= txt@.len())]
#[ensures((^txt)@.len() == txt@.len())]
#[check(terminates)]
pub fn write_dec(txt: &mut Vec<u8>, off: usize, v: u32, l: usize) {
    let mut c = l;
    let mut x = v;
    let orig = snapshot!(txt@);
    #[invariant(c@ <= l@ && txt@.len() == orig.len())]
    #[invariant(off@ + l@ <= txt@.len())]
    #[variant(c@)]
    while c > 0 {
        let d = x % 10;
        proof_assert!(d@ < 10);
        txt[off + c - 1] = 48 + d as u8;
        x /= 10;
        c -= 1;
    }
}

/// Génère les n lignes ; rend la longueur utile du texte.
#[requires(n@ <= 268435455 && txt@.len() == 15 * n@ + 20)]
#[ensures((^txt)@.len() == txt@.len() && result@ <= 15 * n@)]
#[check(terminates)]
pub fn generate(txt: &mut Vec<u8>, n: usize) -> usize {
    let mut i = 0usize;
    let mut off = 0usize;
    #[invariant(i@ <= n@ && off@ <= 15 * i@ && txt@.len() == 15 * n@ + 20)]
    #[variant(n@ - i@)]
    while i < n {
        let r = mix64(SEED.wrapping_add(i as u64));
        let key = ((r >> 44) as u32) & 0xFFFFF;
        let vl = ((r >> 8) & 0xFFFF_FFFF) as u32 % 1_000_000;
        let l1 = ndigits(key);
        write_dec(txt, off, key, l1);
        let off1 = off + l1;
        txt[off1] = b';';
        let off2 = off1 + 1;
        let l2 = ndigits(vl);
        write_dec(txt, off2, vl, l2);
        let off3 = off2 + l2;
        txt[off3] = b'\n';
        off = off3 + 1;
        i += 1;
    }
    off
}

/// tab[2k] = somme des valeurs, tab[2k+1] = nombre de lignes de la clé k.
/// Machine à états : un octet par itération (clé jusqu'à ';', puis valeur jusqu'à '\n').
#[requires(len@ <= txt@.len() && tab@.len() == 2097152)]
#[ensures((^tab)@.len() == 2097152)]
#[check(terminates)]
pub fn parse(txt: &Vec<u8>, tab: &mut Vec<u64>, len: usize) {
    let mut p = 0usize;
    let mut in_val = false;
    let mut key = 0u32;
    let mut vl = 0u32;
    #[invariant(p@ <= len@ && len@ <= txt@.len() && tab@.len() == 2097152)]
    #[variant(len@ - p@)]
    while p < len {
        let c = txt[p];
        let dgt = c.wrapping_sub(48) as u32;
        if in_val {
            if c == b'\n' {
                let k = (key % 1048576) as usize;
                tab[2 * k] = tab[2 * k].wrapping_add(vl as u64);
                tab[2 * k + 1] = tab[2 * k + 1].wrapping_add(1);
                in_val = false;
                key = 0;
                vl = 0;
            } else {
                vl = vl.wrapping_mul(10).wrapping_add(dgt);
            }
        } else if c == b';' {
            in_val = true;
        } else {
            key = key.wrapping_mul(10).wrapping_add(dgt);
        }
        p += 1;
    }
}

#[requires(tab@.len() == 2097152)]
#[check(terminates)]
pub fn checksum(tab: &Vec<u64>) -> u64 {
    let mut ck = CK_INIT;
    let mut k = 0usize;
    #[invariant(tab@.len() == 2097152 && k@ <= 1048576)]
    #[variant(1048576 - k@)]
    while k < 1048576 {
        ck = ckfold(ckfold(ck, tab[2 * k]), tab[2 * k + 1]);
        k += 1;
    }
    ck
}

#[requires(1 <= n@ && n@ <= 268435455)]
pub fn run(n: usize) -> u64 {
    let mut txt = std::vec![0u8; 15 * n + 20];
    let len = generate(&mut txt, n);
    let mut tab = std::vec![0u64; 2097152];
    parse(&txt, &mut tab, len);
    checksum(&tab)
}
