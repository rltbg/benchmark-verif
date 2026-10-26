//! Primitives communes (splitmix64 et chaînage du checksum), arithmétique modulo 2^64.


use creusot_std::prelude::*;

pub const CK_INIT: u64 = 0xCBF29CE484222325;

#[check(terminates)]
#[inline(always)]
pub fn mix64(x: u64) -> u64 {
    let z = x.wrapping_add(0x9E3779B97F4A7C15);
    let z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    let z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

#[check(terminates)]
#[inline(always)]
pub fn ckfold(ck: u64, v: u64) -> u64 {
    (ck ^ v).wrapping_mul(0x100000001B3)
}

/// Les 32 bits de poids fort.
#[check(terminates)]
#[inline(always)]
pub fn hi32(x: u64) -> u32 {
    (x >> 32) as u32
}
