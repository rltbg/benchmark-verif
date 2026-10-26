module Bench.Sort
(* Tri fusion ascendant, un seul buffer de 2n u32 : [0,n) et [n,2n) alternent source/destination.
   Prouvé : toutes les lectures/écritures sont dans les bornes, aucun débordement u32
   (n <= 2^30), seul `buf` est modifié. Non prouvé : que le résultat soit trié
   (vérifié à l'exécution par `checksum`, qui renvoie 0 sinon). *)

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
let seed : U64.t = 0x0123456789ABCDEFuL

val gen: buf:B.buffer U32.t -> n:U32.t -> i:U32.t -> Stack unit
  (requires fun h -> B.live h buf /\ U32.v n <= B.length buf /\ U32.v i <= U32.v n)
  (ensures fun h0 _ h1 -> B.modifies (B.loc_buffer buf) h0 h1)
let rec gen buf n i =
  if U32.lt i n then begin
    let r = mix64 (U64.add_mod seed (C.uint32_to_uint64 i)) in
    buf.(i) <- to_u32 (U64.shift_right r 32ul);
    gen buf n (U32.add i 1ul)
  end

(* Recopie la fin du run gauche ; relation k + mid = i + hi (la droite est épuisée). *)
val copy_left: buf:B.buffer U32.t -> n:U32.t -> so:U32.t -> d:U32.t -> mid:U32.t -> hi:U32.t -> i:U32.t -> k:U32.t -> Stack unit
  (requires fun h ->
    B.live h buf /\ B.length buf == 2 * U32.v n /\ U32.v n <= 1073741824 /\
    (U32.v so == 0 /\ U32.v d == U32.v n \/ U32.v so == U32.v n /\ U32.v d == 0) /\
    U32.v i <= U32.v mid /\ U32.v mid <= U32.v hi /\ U32.v hi <= U32.v n /\
    U32.v k + U32.v mid == U32.v i + U32.v hi)
  (ensures fun h0 _ h1 -> B.modifies (B.loc_buffer buf) h0 h1)
let rec copy_left buf n so d mid hi i k =
  if U32.lt i mid then begin
    buf.(U32.add d k) <- buf.(U32.add so i);
    copy_left buf n so d mid hi (U32.add i 1ul) (U32.add k 1ul)
  end

(* Recopie la fin du run droit ; la gauche est épuisée donc k = j. *)
val copy_right: buf:B.buffer U32.t -> n:U32.t -> so:U32.t -> d:U32.t -> hi:U32.t -> j:U32.t -> Stack unit
  (requires fun h ->
    B.live h buf /\ B.length buf == 2 * U32.v n /\ U32.v n <= 1073741824 /\
    (U32.v so == 0 /\ U32.v d == U32.v n \/ U32.v so == U32.v n /\ U32.v d == 0) /\
    U32.v j <= U32.v hi /\ U32.v hi <= U32.v n)
  (ensures fun h0 _ h1 -> B.modifies (B.loc_buffer buf) h0 h1)
let rec copy_right buf n so d hi j =
  if U32.lt j hi then begin
    buf.(U32.add d j) <- buf.(U32.add so j);
    copy_right buf n so d hi (U32.add j 1ul)
  end

val merge_main: buf:B.buffer U32.t -> n:U32.t -> so:U32.t -> d:U32.t -> mid:U32.t -> hi:U32.t ->
                i:U32.t -> j:U32.t -> k:U32.t -> Stack unit
  (requires fun h ->
    B.live h buf /\ B.length buf == 2 * U32.v n /\ U32.v n <= 1073741824 /\
    (U32.v so == 0 /\ U32.v d == U32.v n \/ U32.v so == U32.v n /\ U32.v d == 0) /\
    U32.v i <= U32.v mid /\ U32.v mid <= U32.v j /\ U32.v j <= U32.v hi /\ U32.v hi <= U32.v n /\
    U32.v k + U32.v mid == U32.v i + U32.v j)
  (ensures fun h0 _ h1 -> B.modifies (B.loc_buffer buf) h0 h1)
let rec merge_main buf n so d mid hi i j k =
  if U32.lt i mid && U32.lt j hi then begin
    let x = buf.(U32.add so i) in
    let y = buf.(U32.add so j) in
    if U32.lte x y then begin
      buf.(U32.add d k) <- x;
      merge_main buf n so d mid hi (U32.add i 1ul) j (U32.add k 1ul)
    end else begin
      buf.(U32.add d k) <- y;
      merge_main buf n so d mid hi i (U32.add j 1ul) (U32.add k 1ul)
    end
  end
  else if U32.lt i mid then copy_left buf n so d mid hi i k
  else copy_right buf n so d hi j

val pass: buf:B.buffer U32.t -> n:U32.t -> so:U32.t -> d:U32.t -> w:U32.t -> lo:U32.t -> Stack unit
  (requires fun h ->
    B.live h buf /\ B.length buf == 2 * U32.v n /\ U32.v n <= 1073741824 /\
    (U32.v so == 0 /\ U32.v d == U32.v n \/ U32.v so == U32.v n /\ U32.v d == 0) /\
    1 <= U32.v w /\ U32.v w <= U32.v n /\ U32.v lo <= U32.v n)
  (ensures fun h0 _ h1 -> B.modifies (B.loc_buffer buf) h0 h1)
let rec pass buf n so d w lo =
  if U32.lt lo n then begin
    let w2 = U32.add w w in
    let mid = if U32.lt (U32.add lo w) n then U32.add lo w else n in
    let hi = if U32.lt (U32.add lo w2) n then U32.add lo w2 else n in
    merge_main buf n so d mid hi lo mid lo;
    pass buf n so d w (if U32.lt (U32.add lo w2) n then U32.add lo w2 else n)
  end

(* Renvoie le décalage (0 ou n) de la moitié qui contient le résultat. *)
val sort_rec: buf:B.buffer U32.t -> n:U32.t -> so:U32.t -> d:U32.t -> w:U32.t -> Stack U32.t
  (requires fun h ->
    B.live h buf /\ B.length buf == 2 * U32.v n /\ U32.v n <= 1073741824 /\
    (U32.v so == 0 /\ U32.v d == U32.v n \/ U32.v so == U32.v n /\ U32.v d == 0) /\ 1 <= U32.v w)
  (ensures fun h0 r h1 -> B.modifies (B.loc_buffer buf) h0 h1 /\ (U32.v r == 0 \/ U32.v r == U32.v n))
let rec sort_rec buf n so d w =
  if U32.lt w n then begin
    pass buf n so d w 0ul;
    sort_rec buf n d so (U32.add w w)
  end else so

val checksum: buf:B.buffer U32.t -> n:U32.t -> off:U32.t -> i:U32.t -> prev:U32.t -> ck:U64.t -> ok:bool -> Stack U64.t
  (requires fun h ->
    B.live h buf /\ B.length buf == 2 * U32.v n /\ U32.v n <= 1073741824 /\
    (U32.v off == 0 \/ U32.v off == U32.v n) /\ U32.v i <= U32.v n)
  (ensures fun h0 _ h1 -> h0 == h1)
let rec checksum buf n off i prev ck ok =
  if U32.lt i n then begin
    let x = buf.(U32.add off i) in
    checksum buf n off (U32.add i 1ul) x (fold ck (C.uint32_to_uint64 x)) (ok && U32.lte prev x)
  end else (if ok then ck else 0uL)

val run: n:U32.t -> ST U64.t
  (requires fun _ -> 1 <= U32.v n /\ U32.v n <= 1073741824)
  (ensures fun _ _ _ -> True)
let run n =
  let buf = B.malloc HS.root 0ul (U32.add n n) in
  gen buf n 0ul;
  let off = sort_rec buf n 0ul n 1ul in
  let ck = checksum buf n off 0ul 0ul ck_init true in
  B.free buf;
  ck
