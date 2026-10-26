module Bench.Matmul
(* C = A*B, matrices n x n de u32 (arithmétique modulo 2^32), ordre i-k-j.
   Un seul buffer de 3n^2 : A = [0,nn), B = [nn,2nn), C = [2nn,3nn).
   Prouvé : tous les indices i*n+j sont < n*n (lemme d'arithmétique non linéaire `lemma_idx`),
   aucun débordement des calculs d'indices (n <= 4096). *)

open FStar.HyperStack.ST
open LowStar.BufferOps
open Bench.Common

module B   = LowStar.Buffer
module HS  = FStar.HyperStack
module U32 = FStar.UInt32
module U64 = FStar.UInt64
module C   = FStar.Int.Cast
module ML  = FStar.Math.Lemmas

#set-options "--fuel 0 --ifuel 0 --z3rlimit 100"

inline_for_extraction noextract
let seed : U64.t = 0x0FEDCBA987654321uL

let lemma_idx (i j n: nat) : Lemma (requires i < n /\ j < n) (ensures i * n + j < n * n) =
  ML.distributivity_add_left i 1 n;
  ML.lemma_mult_le_right n (i + 1) n

let lemma_nn (n: nat) : Lemma (requires n <= 4096) (ensures n * n <= 16777216) =
  ML.lemma_mult_le_right n n 4096;
  ML.lemma_mult_le_left 4096 n 4096

inline_for_extraction noextract
val idx: n:U32.t -> i:U32.t -> j:U32.t -> Pure U32.t
  (requires U32.v n <= 4096 /\ U32.v i < U32.v n /\ U32.v j < U32.v n)
  (ensures fun r -> U32.v r == U32.v i * U32.v n + U32.v j /\ U32.v r < U32.v n * U32.v n)
inline_for_extraction noextract
let idx n i j =
  lemma_idx (U32.v i) (U32.v j) (U32.v n);
  lemma_nn (U32.v n);
  U32.add (U32.mul i n) j

val init_mat: m:B.buffer U32.t -> nn:U32.t -> i:U32.t -> Stack unit
  (requires fun h -> B.live h m /\ U32.v nn <= 16777216 /\ B.length m == 3 * U32.v nn /\ U32.v i <= U32.v nn)
  (ensures fun h0 _ h1 -> B.modifies (B.loc_buffer m) h0 h1)
let rec init_mat m nn i =
  if U32.lt i nn then begin
    let s = U64.add_mod seed (C.uint32_to_uint64 i) in
    m.(i) <- to_u32 (U64.shift_right (mix64 s) 32ul);
    m.(U32.add nn i) <- to_u32 (U64.shift_right (mix64 (U64.add_mod s (C.uint32_to_uint64 nn))) 32ul);
    init_mat m nn (U32.add i 1ul)
  end

val loop_j: m:B.buffer U32.t -> n:U32.t -> nn:U32.t -> i:U32.t -> k:U32.t -> aik:U32.t -> j:U32.t -> Stack unit
  (requires fun h ->
    B.live h m /\ U32.v n <= 4096 /\ U32.v nn == U32.v n * U32.v n /\ B.length m == 3 * U32.v nn /\
    U32.v i < U32.v n /\ U32.v k < U32.v n /\ U32.v j <= U32.v n)
  (ensures fun h0 _ h1 -> B.modifies (B.loc_buffer m) h0 h1)
let rec loop_j m n nn i k aik j =
  if U32.lt j n then begin
    lemma_nn (U32.v n);
    let nn2 = U32.add nn nn in
    let ci = U32.add nn2 (idx n i j) in
    let bi = U32.add nn (idx n k j) in
    m.(ci) <- U32.add_mod m.(ci) (U32.mul_mod aik m.(bi));
    loop_j m n nn i k aik (U32.add j 1ul)
  end

val loop_k: m:B.buffer U32.t -> n:U32.t -> nn:U32.t -> i:U32.t -> k:U32.t -> Stack unit
  (requires fun h ->
    B.live h m /\ U32.v n <= 4096 /\ U32.v nn == U32.v n * U32.v n /\ B.length m == 3 * U32.v nn /\
    U32.v i < U32.v n /\ U32.v k <= U32.v n)
  (ensures fun h0 _ h1 -> B.modifies (B.loc_buffer m) h0 h1)
let rec loop_k m n nn i k =
  if U32.lt k n then begin
    let aik = m.(idx n i k) in
    loop_j m n nn i k aik 0ul;
    loop_k m n nn i (U32.add k 1ul)
  end

val loop_i: m:B.buffer U32.t -> n:U32.t -> nn:U32.t -> i:U32.t -> Stack unit
  (requires fun h ->
    B.live h m /\ U32.v n <= 4096 /\ U32.v nn == U32.v n * U32.v n /\ B.length m == 3 * U32.v nn /\
    U32.v i <= U32.v n)
  (ensures fun h0 _ h1 -> B.modifies (B.loc_buffer m) h0 h1)
let rec loop_i m n nn i =
  if U32.lt i n then begin
    loop_k m n nn i 0ul;
    loop_i m n nn (U32.add i 1ul)
  end

val checksum: m:B.buffer U32.t -> nn:U32.t -> i:U32.t -> ck:U64.t -> Stack U64.t
  (requires fun h -> B.live h m /\ U32.v nn <= 16777216 /\ B.length m == 3 * U32.v nn /\ U32.v i <= U32.v nn)
  (ensures fun h0 _ h1 -> h0 == h1)
let rec checksum m nn i ck =
  if U32.lt i nn then
    checksum m nn (U32.add i 1ul) (fold ck (C.uint32_to_uint64 m.(U32.add (U32.add nn nn) i)))
  else ck

val run: n:U32.t -> ST U64.t
  (requires fun _ -> 1 <= U32.v n /\ U32.v n <= 4096)
  (ensures fun _ _ _ -> True)
let run n =
  lemma_nn (U32.v n);
  ML.pos_times_pos_is_pos (U32.v n) (U32.v n);
  let nn = U32.mul n n in
  let m = B.malloc HS.root 0ul (U32.mul 3ul nn) in
  init_mat m nn 0ul;
  loop_i m n nn 0ul;
  let ck = checksum m nn 0ul ck_init in
  B.free m;
  ck
