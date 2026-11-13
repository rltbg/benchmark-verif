//! Primitives communes (splitmix64 et chaînage du checksum), arithmétique modulo 2^64.
use vstd::prelude::*;

verus! {

pub const CK_INIT: u64 = 0xCBF29CE484222325;

pub open spec fn spec_mix64(x: u64) -> u64 {
    let z = x.wrapping_add(0x9E3779B97F4A7C15);
    let z = (z ^ (z >> 30u64)).wrapping_mul(0xBF58476D1CE4E5B9);
    let z = (z ^ (z >> 27u64)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31u64)
}

#[inline(always)]
pub fn mix64(x: u64) -> (r: u64)
    ensures r == spec_mix64(x),
{
    let z = x.wrapping_add(0x9E3779B97F4A7C15);
    let z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    let z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

pub open spec fn spec_ckfold(ck: u64, v: u64) -> u64 {
    (ck ^ v).wrapping_mul(0x100000001B3)
}

#[inline(always)]
pub fn ckfold(ck: u64, v: u64) -> (r: u64)
    ensures r == spec_ckfold(ck, v),
{
    (ck ^ v).wrapping_mul(0x100000001B3)
}

/// Les 32 bits de poids fort.
#[inline(always)]
pub fn hi32(x: u64) -> (r: u32)
    ensures r == (x >> 32u64) as u32,
{
    (x >> 32) as u32
}

} // verus!
