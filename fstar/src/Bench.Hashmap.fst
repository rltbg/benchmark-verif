module Bench.Hashmap
(* Table de hachage à adressage ouvert, sondage linéaire, couples (clé, valeur) entrelacés
   dans un seul buffer de 2*cap u64 (cap = puissance de 2 >= 2n). n <= 2^26.
   Prouvé : indices dans les bornes (masque `mask_idx`), sondage borné par `cap` essais. *)

open FStar.HyperStack.ST
open LowStar.BufferOps
open Bench.Common

module B   = LowStar.Buffer
module HS  = FStar.HyperStack
module U32 = FStar.UInt32
module U64 = FStar.UInt64
module C   = FStar.Int.Cast

#set-options "--fuel 0 --ifuel 0 --z3rlimit 100"

inline_for_extraction noextract
let seed : U64.t = 0x2468ACE013579BDFuL

let rec find_cap (n2: U32.t{U32.v n2 <= 134217728})
                 (cap: U32.t{2 <= U32.v cap /\ U32.v cap <= 134217728})
                 (lg: U32.t{1 <= U32.v lg /\ U32.v lg <= 27})
  : Tot (r: (U32.t & U32.t){2 <= U32.v (fst r) /\ U32.v (fst r) <= 134217728 /\
                            1 <= U32.v (snd r) /\ U32.v (snd r) <= 27})
        (decreases (27 - U32.v lg)) =
  if U32.lt cap n2 && U32.lt lg 27ul && U32.lte cap 67108864ul
  then find_cap n2 (U32.add cap cap) (U32.add lg 1ul)
  else (cap, lg)

inline_for_extraction noextract
let slot_of (key: U64.t) (lg: U32.t{1 <= U32.v lg /\ U32.v lg <= 27}) (m: U32.t) : Pure U32.t
  (requires True) (ensures fun r -> U32.v r <= U32.v m) =
  let h = U64.mul_mod key 0x9E3779B97F4A7C15uL in
  mask_idx (to_u32 (U64.shift_right h (U32.sub 64ul lg))) m

inline_for_extraction noextract
let key_of (off i: U32.t) : U64.t =
  U64.logor (mix64 (U64.add_mod (U64.add_mod seed (C.uint32_to_uint64 off)) (C.uint32_to_uint64 i))) 1uL

val insert: t:B.buffer U64.t -> cap:U32.t -> m:U32.t -> key:U64.t -> s:U32.t -> tries:U32.t -> i1:U64.t -> Stack unit
  (requires fun h ->
    B.live h t /\ 2 <= U32.v cap /\ U32.v cap <= 134217728 /\ B.length t == 2 * U32.v cap /\
    U32.v m == U32.v cap - 1 /\ U32.v s <= U32.v m)
  (ensures fun h0 _ h1 -> B.modifies (B.loc_buffer t) h0 h1)
let rec insert t cap m key s tries i1 =
  if U32.lt tries cap then begin
    let s2 = U32.add s s in
    let k = t.(s2) in
    if U64.eq k 0uL then begin
      t.(s2) <- key;
      t.(U32.add s2 1ul) <- i1
    end else if U64.eq k key then begin
      t.(U32.add s2 1ul) <- U64.add_mod t.(U32.add s2 1ul) i1
    end else begin
      insert t cap m key (mask_idx (U32.add s 1ul) m) (U32.add tries 1ul) i1
    end
  end

val lookup: t:B.buffer U64.t -> cap:U32.t -> m:U32.t -> key:U64.t -> s:U32.t -> tries:U32.t -> Stack U64.t
  (requires fun h ->
    B.live h t /\ 2 <= U32.v cap /\ U32.v cap <= 134217728 /\ B.length t == 2 * U32.v cap /\
    U32.v m == U32.v cap - 1 /\ U32.v s <= U32.v m)
  (ensures fun h0 _ h1 -> h0 == h1)
let rec lookup t cap m key s tries =
  if U32.lt tries cap then begin
    let s2 = U32.add s s in
    let k = t.(s2) in
    if U64.eq k 0uL then 0uL
    else if U64.eq k key then t.(U32.add s2 1ul)
    else lookup t cap m key (mask_idx (U32.add s 1ul) m) (U32.add tries 1ul)
  end else 0uL

val insert_all: t:B.buffer U64.t -> cap:U32.t -> lg:U32.t -> m:U32.t -> n:U32.t -> i:U32.t -> Stack unit
  (requires fun h ->
    B.live h t /\ 2 <= U32.v cap /\ U32.v cap <= 134217728 /\ B.length t == 2 * U32.v cap /\
    U32.v m == U32.v cap - 1 /\ 1 <= U32.v lg /\ U32.v lg <= 27 /\ U32.v i <= U32.v n /\ U32.v n <= 67108864)
  (ensures fun h0 _ h1 -> B.modifies (B.loc_buffer t) h0 h1)
let rec insert_all t cap lg m n i =
  if U32.lt i n then begin
    let key = key_of 0ul i in
    insert t cap m key (slot_of key lg m) 0ul (C.uint32_to_uint64 (U32.add i 1ul));
    insert_all t cap lg m n (U32.add i 1ul)
  end

val lookup_all: t:B.buffer U64.t -> cap:U32.t -> lg:U32.t -> m:U32.t -> n:U32.t -> off:U32.t -> i:U32.t -> ck:U64.t -> Stack U64.t
  (requires fun h ->
    B.live h t /\ 2 <= U32.v cap /\ U32.v cap <= 134217728 /\ B.length t == 2 * U32.v cap /\
    U32.v m == U32.v cap - 1 /\ 1 <= U32.v lg /\ U32.v lg <= 27 /\ U32.v i <= U32.v n /\ U32.v n <= 67108864 /\
    U32.v off <= 67108864)
  (ensures fun h0 _ h1 -> h0 == h1)
let rec lookup_all t cap lg m n off i ck =
  if U32.lt i n then begin
    let key = key_of off i in
    let v = lookup t cap m key (slot_of key lg m) 0ul in
    lookup_all t cap lg m n off (U32.add i 1ul) (fold ck v)
  end else ck

val run: n:U32.t -> ST U64.t
  (requires fun _ -> 1 <= U32.v n /\ U32.v n <= 67108864)
  (ensures fun _ _ _ -> True)
let run n =
  let (cap, lg) = find_cap (U32.add n n) 2ul 1ul in
  let m = U32.sub cap 1ul in
  let t = B.malloc HS.root 0uL (U32.add cap cap) in
  insert_all t cap lg m n 0ul;
  let ck = lookup_all t cap lg m n 0ul 0ul ck_init in
  let ck = lookup_all t cap lg m n n 0ul ck in
  B.free t;
  ck
