module Bench.Common
(* Primitives communes aux cinq workloads (voir spec/WORKLOADS.md).
   Uniquement des entiers machine : le code d'exécution ne contient aucun `Prims.int`, ce qui est
   exigé par les backends C et F# de Custard (et évite Zarith côté OCaml). *)

module U8  = FStar.UInt8
module U32 = FStar.UInt32
module U64 = FStar.UInt64
module C   = FStar.Int.Cast

let ck_init : U64.t = 0xCBF29CE484222325uL

(* Finaliseur splitmix64, arithmétique modulo 2^64. *)
[@@"opaque_to_smt"]
let mix64 (x: U64.t) : U64.t =
  let z = U64.add_mod x 0x9E3779B97F4A7C15uL in
  let z = U64.mul_mod (U64.logxor z (U64.shift_right z 30ul)) 0xBF58476D1CE4E5B9uL in
  let z = U64.mul_mod (U64.logxor z (U64.shift_right z 27ul)) 0x94D049BB133111EBuL in
  U64.logxor z (U64.shift_right z 31ul)

[@@"opaque_to_smt"]
let ckfold (ck v: U64.t) : U64.t = U64.mul_mod (U64.logxor ck v) 0x100000001B3uL

(* Les 32 bits de poids fort. *)
[@@"opaque_to_smt"]
let hi32 (x: U64.t) : U32.t = C.uint64_to_uint32 (U64.shift_right x 32ul)

(* x & (2^20 - 1) <= 2^20 - 1 : indice de clé provablement dans la table de 2^20 clés. *)
let mask20 (x: U32.t) : Pure U32.t (requires True) (ensures fun r -> U32.v r <= 1048575) =
  FStar.UInt.logand_le #32 (U32.v x) 1048575;
  U32.logand x 1048575ul

(* Tout x < 65536 tient dans un size_t (C99 : size_t fait au moins 16 bits). *)
let fits_small (x: nat) : Lemma (requires x <= 65535) (ensures FStar.SizeT.fits x) =
  assert_norm (pow2 16 == 65536);
  FStar.SizeT.fits_at_least_16 x
