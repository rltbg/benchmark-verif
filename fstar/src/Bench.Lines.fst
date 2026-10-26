module Bench.Lines
(* Débit « lignes/seconde » : génère n lignes ASCII « clé;valeur\n » dans un buffer,
   puis les relit octet par octet et agrège somme/compte par clé (2^20 clés).
   Prouvé : mémoire sûre, pas de débordement des offsets (n <= 268_435_455), txt et tab disjoints.
   Le parseur est une machine à états gardée par `p < len` : une ligne mal formée ne peut
   jamais sortir du buffer (coût : une comparaison par octet, absente de la version C de contrôle). *)

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

inline_for_extraction noextract
let seed : U64.t = 0x1357924680ACEBDFuL

inline_for_extraction noextract
let ndigits (v: U32.t) : Pure U32.t
  (requires True)
  (ensures fun r -> 1 <= U32.v r /\ U32.v r <= 7 /\ (U32.v v < 1000000 ==> U32.v r <= 6)) =
  if U32.lt v 10ul then 1ul
  else if U32.lt v 100ul then 2ul
  else if U32.lt v 1000ul then 3ul
  else if U32.lt v 10000ul then 4ul
  else if U32.lt v 100000ul then 5ul
  else if U32.lt v 1000000ul then 6ul
  else 7ul

(* Écrit les chiffres de v de droite à gauche à partir de la position off+pos. *)
val write_digits: txt:B.buffer U8.t -> off:U32.t -> v:U32.t -> pos:U32.t -> Stack unit
  (requires fun h -> B.live h txt /\ U32.v off + U32.v pos < B.length txt /\ B.length txt <= 4294967295)
  (ensures fun h0 _ h1 -> B.modifies (B.loc_buffer txt) h0 h1)
let rec write_digits txt off v pos =
  txt.(U32.add off pos) <- C.uint32_to_uint8 (U32.add_mod 48ul (U32.rem v 10ul));
  if U32.gt pos 0ul then write_digits txt off (U32.div v 10ul) (U32.sub pos 1ul)

val gen: txt:B.buffer U8.t -> n:U32.t -> i:U32.t -> off:U32.t -> Stack U32.t
  (requires fun h ->
    B.live h txt /\ U32.v n <= 268435455 /\ B.length txt == 15 * U32.v n + 20 /\
    U32.v i <= U32.v n /\ U32.v off <= 15 * U32.v i)
  (ensures fun h0 r h1 -> B.modifies (B.loc_buffer txt) h0 h1 /\ U32.v r <= 15 * U32.v n)
let rec gen txt n i off =
  if U32.lt i n then begin
    let r = mix64 (U64.add_mod seed (C.uint32_to_uint64 i)) in
    let key = mask_idx (to_u32 (U64.shift_right r 44ul)) 1048575ul in
    let vl = to_u32 (U64.rem (U64.logand (U64.shift_right r 8ul) 0xFFFFFFFFuL) 1000000uL) in
    let l1 = ndigits key in
    write_digits txt off key (U32.sub l1 1ul);
    let off1 = U32.add off l1 in
    txt.(off1) <- 59uy;                       (* ';'  *)
    let off2 = U32.add off1 1ul in
    let l2 = ndigits vl in
    write_digits txt off2 vl (U32.sub l2 1ul);
    let off3 = U32.add off2 l2 in
    txt.(off3) <- 10uy;                       (* '\n' *)
    gen txt n (U32.add i 1ul) (U32.add off3 1ul)
  end else off

(* tab[2k] = somme des valeurs, tab[2k+1] = nombre de lignes de la clé k. *)
val parse: txt:B.buffer U8.t -> tab:B.buffer U64.t -> len:U32.t -> p:U32.t -> in_val:bool -> key:U32.t -> vl:U32.t -> Stack unit
  (requires fun h ->
    B.live h txt /\ B.live h tab /\ B.disjoint txt tab /\
    U32.v len <= B.length txt /\ B.length tab == 2097152 /\ U32.v p <= U32.v len)
  (ensures fun h0 _ h1 -> B.modifies (B.loc_buffer tab) h0 h1)
let rec parse txt tab len p in_val key vl =
  if U32.lt p len then begin
    let c = txt.(p) in
    let p1 = U32.add p 1ul in
    let dgt = C.uint8_to_uint32 (U8.sub_mod c 48uy) in
    if not in_val then begin
      if U8.eq c 59uy then parse txt tab len p1 true key 0ul
      else parse txt tab len p1 false (U32.add_mod (U32.mul_mod key 10ul) dgt) vl
    end else begin
      if U8.eq c 10uy then begin
        let k = mask_idx key 1048575ul in
        let k2 = U32.add k k in
        tab.(k2) <- U64.add_mod tab.(k2) (C.uint32_to_uint64 vl);
        tab.(U32.add k2 1ul) <- U64.add_mod tab.(U32.add k2 1ul) 1uL;
        parse txt tab len p1 false 0ul 0ul
      end else parse txt tab len p1 true key (U32.add_mod (U32.mul_mod vl 10ul) dgt)
    end
  end

val checksum: tab:B.buffer U64.t -> k:U32.t -> ck:U64.t -> Stack U64.t
  (requires fun h -> B.live h tab /\ B.length tab == 2097152 /\ U32.v k <= 1048576)
  (ensures fun h0 _ h1 -> h0 == h1)
let rec checksum tab k ck =
  if U32.lt k 1048576ul then begin
    let k2 = U32.add k k in
    checksum tab (U32.add k 1ul) (fold (fold ck tab.(k2)) tab.(U32.add k2 1ul))
  end else ck

val run: n:U32.t -> ST U64.t
  (requires fun _ -> 1 <= U32.v n /\ U32.v n <= 268435455)
  (ensures fun _ _ _ -> True)
let run n =
  let txt = B.malloc HS.root 0uy (U32.add (U32.mul 15ul n) 20ul) in
  let len = gen txt n 0ul 0ul in
  let tab = B.malloc HS.root 0uL 2097152ul in
  parse txt tab len 0ul false 0ul 0ul;
  let ck = checksum tab 0ul ck_init in
  B.free txt;
  B.free tab;
  ck
