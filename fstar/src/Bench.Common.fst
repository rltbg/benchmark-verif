module Bench.Common
(* Primitives communes aux cinq workloads (voir spec/WORKLOADS.md).
   Tout est `inline_for_extraction noextract` : le code généré (C ou ML) contient
   directement les opérations machine, sans appel de fonction ni module supplémentaire. *)

module U8  = FStar.UInt8
module U32 = FStar.UInt32
module U64 = FStar.UInt64
module C   = FStar.Int.Cast

inline_for_extraction noextract
let ck_init : U64.t = 0xCBF29CE484222325uL

(* Finaliseur splitmix64. Arithmétique modulo 2^64 : *_mod ne porte aucune obligation de preuve. *)
inline_for_extraction noextract
let mix64 (x: U64.t) : U64.t =
  let z = U64.add_mod x 0x9E3779B97F4A7C15uL in
  let z = U64.mul_mod (U64.logxor z (U64.shift_right z 30ul)) 0xBF58476D1CE4E5B9uL in
  let z = U64.mul_mod (U64.logxor z (U64.shift_right z 27ul)) 0x94D049BB133111EBuL in
  U64.logxor z (U64.shift_right z 31ul)

inline_for_extraction noextract
let fold (ck v: U64.t) : U64.t = U64.mul_mod (U64.logxor ck v) 0x100000001B3uL

(* Troncature u64 -> u32 avec sa valeur connue de l'SMT. *)
inline_for_extraction noextract
let to_u32 (x: U64.t) : Pure U32.t (requires True) (ensures fun r -> U32.v r == U64.v x % 4294967296) =
  assert_norm (pow2 32 == 4294967296);
  C.uint64_to_uint32 x

(* x & m <= m : rend un indice provablement dans [0, m] sans division. *)
inline_for_extraction noextract
let mask_idx (x m: U32.t) : Pure U32.t (requires True) (ensures fun r -> U32.v r <= U32.v m) =
  FStar.UInt.logand_le #32 (U32.v x) (U32.v m);
  U32.logand x m
