module Bench.Sieve
(* Crible d'Ératosthène, 1 octet par entier. n <= 4_000_000_000.
   Prouvé : accès dans les bornes (garde `j <= n` explicite), pas de débordement dans `mark`.
   Non prouvé : j0 = i*i et i+1 utilisent *_mod (la valeur est exacte tant que n <= 4e9,
   mais l'obligation de non-débordement n'a pas été démontrée). *)

open FStar.HyperStack.ST
open LowStar.BufferOps
open Bench.Common

module B   = LowStar.Buffer
module HS  = FStar.HyperStack
module U8  = FStar.UInt8
module U32 = FStar.UInt32
module U64 = FStar.UInt64
module C   = FStar.Int.Cast

#set-options "--fuel 0 --ifuel 0 --z3rlimit 100"

val mark: f:B.buffer U8.t -> n:U32.t -> i:U32.t -> j:U32.t -> Stack unit
  (requires fun h -> B.live h f /\ B.length f == U32.v n + 1 /\ U32.v j <= U32.v n)
  (ensures fun h0 _ h1 -> B.modifies (B.loc_buffer f) h0 h1)
let rec mark f n i j =
  f.(j) <- 1uy;
  if U32.lte i (U32.sub n j) then mark f n i (U32.add j i)

val sieve_outer: f:B.buffer U8.t -> n:U32.t -> i:U32.t -> Stack unit
  (requires fun h -> B.live h f /\ B.length f == U32.v n + 1 /\ U32.v n <= 4000000000 /\ U32.v i >= 2 /\ U32.v i <= U32.v n + 1)
  (ensures fun h0 _ h1 -> B.modifies (B.loc_buffer f) h0 h1)
let rec sieve_outer f n i =
  if U32.lte i n && U32.lte i (U32.div n i) then begin
    (if U8.eq f.(i) 0uy then begin
      let j0 = U32.mul_mod i i in
      if U32.lte j0 n then mark f n i j0
    end);
    sieve_outer f n (U32.add i 1ul)
  end

val count: f:B.buffer U8.t -> n:U32.t -> i:U32.t -> ck:U64.t -> cnt:U64.t -> Stack U64.t
  (requires fun h -> B.live h f /\ B.length f == U32.v n + 1 /\ U32.v n <= 4000000000 /\ U32.v i >= 2 /\ U32.v i <= U32.v n + 1)
  (ensures fun h0 _ h1 -> h0 == h1)
let rec count f n i ck cnt =
  if U32.lte i n then begin
    if U8.eq f.(i) 0uy
    then count f n (U32.add i 1ul) (fold ck (C.uint32_to_uint64 i)) (U64.add_mod cnt 1uL)
    else count f n (U32.add i 1ul) ck cnt
  end else fold ck cnt

val run: n:U32.t -> ST U64.t
  (requires fun _ -> 2 <= U32.v n /\ U32.v n <= 4000000000)
  (ensures fun _ _ _ -> True)
let run n =
  let f = B.malloc HS.root 0uy (U32.add n 1ul) in
  sieve_outer f n 2ul;
  let ck = count f n 2ul ck_init 0uL in
  B.free f;
  ck
