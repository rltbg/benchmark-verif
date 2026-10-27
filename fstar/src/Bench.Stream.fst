module Bench.Stream
#lang-pulse
(* Flux d'octets : CRC-32 (table de 256 entrées, polynôme 0xEDB88320) + automate de tokenisation JSON-lines.
   Le noyau `chunk` traite un bloc de len octets (len <= taille du tampon) et conserve son état dans un
   tableau de 16 u64 (donc l'automate traverse les frontières de blocs). Le pilote (non prouvé) lit le
   fichier par blocs de 1 Mio ; voir spec/workloads.d/stream.md pour la définition exacte des champs.

   Prouvé : mémoire-sûreté (index du tampon et de la table dans les bornes), terminaison. Aucun
   débordement n'est possible : tous les compteurs utilisent l'arithmétique modulo 2^64. *)
open Pulse.Lib.Pervasives
open Bench.Common
module V   = Pulse.Lib.Vec
module R   = Pulse.Lib.Reference
module SZ  = FStar.SizeT
module U8  = FStar.UInt8
module U32 = FStar.UInt32
module U64 = FStar.UInt64
module C   = FStar.Int.Cast

#set-options "--z3rlimit 100"

(* ---- classification d'un octet ---- *)
let is_digit (c: U8.t) : bool = U8.gte c 48uy && U8.lte c 57uy
let is_lower (c: U8.t) : bool = U8.gte c 97uy && U8.lte c 122uy
(* '.', 'e', 'E', '+', '-' *)
let is_numpunct (c: U8.t) : bool = c = 46uy || c = 101uy || c = 69uy || c = 43uy || c = 45uy

let b2u (b: bool) : U64.t = if b then 1uL else 0uL

(* ---- transition (fonctions pures, toutes totales) ----
   m : 0 normal, 1 chaîne, 2 chaîne après '\', 3 nombre, 4 mot (true/false/null). *)
let fin3 (c: U8.t) (m: U64.t) : bool = m = 3uL && not (is_digit c || is_numpunct c)
let fin4 (c: U8.t) (m: U64.t) : bool = m = 4uL && not (is_lower c)
let m0_of (c: U8.t) (m: U64.t) : U64.t = if fin3 c m || fin4 c m then 0uL else m

let nmode (c: U8.t) (m: U64.t) : U64.t =
  let m0 = m0_of c m in
  if m0 = 2uL then 1uL
  else if m0 = 1uL then (if c = 92uy then 2uL else if c = 34uy then 0uL else 1uL)
  else if m0 = 3uL then 3uL
  else if m0 = 4uL then 4uL
  else if c = 34uy then 1uL
  else if is_digit c || c = 45uy then 3uL
  else if is_lower c then 4uL
  else 0uL

let nacc (c: U8.t) (m acc: U64.t) : U64.t =
  let m0 = m0_of c m in
  if m0 = 3uL then (if is_digit c then U64.add_mod (U64.mul_mod acc 10uL) (C.uint8_to_uint64 (U8.sub_mod c 48uy)) else acc)
  else if m0 = 0uL && is_digit c then C.uint8_to_uint64 (U8.sub_mod c 48uy)
  else if m0 = 0uL && c = 45uy then 0uL
  else acc

let nisint (c: U8.t) (m isint: U64.t) : U64.t =
  let m0 = m0_of c m in
  if m0 = 3uL then (if is_digit c then isint else 0uL)
  else if m0 = 0uL && (is_digit c || c = 45uy) then 1uL
  else isint

let nnums (c: U8.t) (m nums: U64.t) : U64.t = U64.add_mod nums (b2u (fin3 c m))
let nlits (c: U8.t) (m lits: U64.t) : U64.t = U64.add_mod lits (b2u (fin4 c m))
let nidsum (c: U8.t) (m d isint gotid acc idsum: U64.t) : U64.t =
  if fin3 c m && isint = 1uL && d = 1uL && gotid = 0uL then U64.add_mod idsum acc else idsum
let nallsum (c: U8.t) (m isint acc allsum: U64.t) : U64.t =
  if fin3 c m && isint = 1uL then U64.add_mod allsum acc else allsum
let ngotid (c: U8.t) (m d gotid: U64.t) : U64.t =
  if m0_of c m = 0uL && c = 10uy then 0uL
  else if fin3 c m && d = 1uL then 1uL
  else gotid
let nobjs (c: U8.t) (m objs: U64.t) : U64.t = U64.add_mod objs (b2u (m0_of c m = 0uL && c = 123uy))
let narrs (c: U8.t) (m arrs: U64.t) : U64.t = U64.add_mod arrs (b2u (m0_of c m = 0uL && c = 91uy))
let nstrs (c: U8.t) (m strs: U64.t) : U64.t = U64.add_mod strs (b2u (m0_of c m = 0uL && c = 34uy))
let nlines (c: U8.t) (m lines: U64.t) : U64.t = U64.add_mod lines (b2u (m0_of c m = 0uL && c = 10uy))
let ndepth (c: U8.t) (m d: U64.t) : U64.t =
  if m0_of c m = 0uL then
    (if c = 123uy || c = 91uy then U64.add_mod d 1uL
     else if (c = 125uy || c = 93uy) && U64.gt d 0uL then U64.sub_mod d 1uL
     else d)
  else d
let nmaxd (maxd d': U64.t) : U64.t = if U64.gt d' maxd then d' else maxd

(* x & 255 <= 255 : indice provablement dans la table de 256 entrées. *)
let mask8 (x: U32.t) : Pure U32.t (requires True) (ensures fun r -> U32.v r <= 255) =
  FStar.UInt.logand_le #32 (U32.v x) 255;
  U32.logand x 255ul

(* Un pas du CRC-32 octet par octet. *)
let crc_idx (crc: U32.t) (c: U8.t) : Pure U32.t (requires True) (ensures fun r -> U32.v r <= 255) =
  mask8 (U32.logxor crc (C.uint8_to_uint32 c))

(* ---- noyau ---- *)
fn chunk (buf: V.vec U8.t) (len: SZ.t) (st: V.vec U64.t) (tab: V.vec U32.t)
         (#sb: Ghost.erased (Seq.seq U8.t)) (#ss: Ghost.erased (Seq.seq U64.t)) (#stab: Ghost.erased (Seq.seq U32.t))
  preserves V.pts_to buf sb ** V.pts_to tab stab
  requires V.pts_to st ss **
           pure (SZ.fits_u64 /\ SZ.v len <= V.length buf /\ V.length st == 16 /\ V.length tab == 256)
  ensures exists* ss'. V.pts_to st ss' ** pure (Seq.length ss' == 16)
{
  V.pts_to_len buf;
  V.pts_to_len st;
  V.pts_to_len tab;
  assert_norm (pow2 64 == 18446744073709551616);
  SZ.fits_u64_implies_fits_32 ();
  let mut crc = C.uint64_to_uint32 V.(st.(0sz));
  let mut mode = V.(st.(1sz));
  let mut depth = V.(st.(2sz));
  let mut maxd = V.(st.(3sz));
  let mut objs = V.(st.(4sz));
  let mut strs = V.(st.(5sz));
  let mut nums = V.(st.(6sz));
  let mut arrs = V.(st.(7sz));
  let mut lits = V.(st.(8sz));
  let mut lines = V.(st.(9sz));
  let mut idsum = V.(st.(10sz));
  let mut allsum = V.(st.(11sz));
  let mut acc = V.(st.(12sz));
  let mut isint = V.(st.(13sz));
  let mut gotid = V.(st.(14sz));
  let mut p = 0sz;
  while (!p `SZ.lt` len)
  invariant exists* vp vcrc vm vd vmx vo vs vn va vl vln vid vall vacc vint vgot.
            R.pts_to p vp ** R.pts_to crc vcrc ** R.pts_to mode vm ** R.pts_to depth vd ** R.pts_to maxd vmx **
            R.pts_to objs vo ** R.pts_to strs vs ** R.pts_to nums vn ** R.pts_to arrs va ** R.pts_to lits vl **
            R.pts_to lines vln ** R.pts_to idsum vid ** R.pts_to allsum vall ** R.pts_to acc vacc **
            R.pts_to isint vint ** R.pts_to gotid vgot **
            pure (SZ.v vp <= SZ.v len)
  decreases (SZ.v len - SZ.v !p)
  {
    let vp = !p;
    let c = V.(buf.(vp));
    let vcrc = !crc;
    let vm = !mode;
    let vd = !depth;
    let vacc = !acc;
    let vint = !isint;
    let vgot = !gotid;
    let ix = crc_idx vcrc c;
    let t = V.(tab.(SZ.uint32_to_sizet ix));
    crc := U32.logxor t (U32.shift_right vcrc 8ul);
    idsum := nidsum c vm vd vint vgot vacc !idsum;
    allsum := nallsum c vm vint vacc !allsum;
    gotid := ngotid c vm vd vgot;
    nums := nnums c vm !nums;
    lits := nlits c vm !lits;
    objs := nobjs c vm !objs;
    arrs := narrs c vm !arrs;
    strs := nstrs c vm !strs;
    lines := nlines c vm !lines;
    let d' = ndepth c vm vd;
    depth := d';
    maxd := nmaxd !maxd d';
    mode := nmode c vm;
    acc := nacc c vm vacc;
    isint := nisint c vm vint;
    SZ.fits_lte (SZ.v vp + 1) (SZ.v len);
    p := SZ.add vp 1sz
  };
  V.(st.(0sz) <- C.uint32_to_uint64 !crc);
  V.(st.(1sz) <- !mode);
  V.(st.(2sz) <- !depth);
  V.(st.(3sz) <- !maxd);
  V.(st.(4sz) <- !objs);
  V.(st.(5sz) <- !strs);
  V.(st.(6sz) <- !nums);
  V.(st.(7sz) <- !arrs);
  V.(st.(8sz) <- !lits);
  V.(st.(9sz) <- !lines);
  V.(st.(10sz) <- !idsum);
  V.(st.(11sz) <- !allsum);
  V.(st.(12sz) <- !acc);
  V.(st.(13sz) <- !isint);
  V.(st.(14sz) <- !gotid);
  let b = V.(st.(15sz));
  V.(st.(15sz) <- U64.add_mod b (SZ.sizet_to_uint64 len));
  V.pts_to_len st
}

(* ---- allocation de l'état et construction de la table CRC (appelés par le pilote) ---- *)
fn new_buf (u: unit)
  requires pure (SZ.fits_u64)
  returns v: V.vec U8.t
  ensures exists* s. V.pts_to v s ** pure (Seq.length s == 1048576)
{
  assert_norm (pow2 64 == 18446744073709551616);
  SZ.fits_u64_implies_fits 1048576;
  V.alloc 0uy (SZ.uint_to_t 1048576)
}

fn new_state (u: unit)
  requires emp
  returns v: V.vec U64.t
  ensures exists* s. V.pts_to v s ** pure (Seq.length s == 16)
{
  let v = V.alloc 0uL 16sz;
  V.pts_to_len v;
  V.(v.(0sz) <- 0xFFFFFFFFuL);
  V.pts_to_len v;
  v
}

(* CRC-32 d'un octet i : 8 rondes de décalage conditionnel. *)
[@@"opaque_to_smt"]
let crc_round (x: U32.t) : U32.t =
  if U32.eq (U32.logand x 1ul) 1ul then U32.logxor (U32.shift_right x 1ul) 0xEDB88320ul
  else U32.shift_right x 1ul

[@@"opaque_to_smt"]
let crc_entry (i: U32.t) : U32.t =
  crc_round (crc_round (crc_round (crc_round (crc_round (crc_round (crc_round (crc_round i)))))))

fn new_table (u: unit)
  requires pure (SZ.fits_u64)
  returns v: V.vec U32.t
  ensures exists* s. V.pts_to v s ** pure (Seq.length s == 256)
{
  assert_norm (pow2 64 == 18446744073709551616);
  SZ.fits_u64_implies_fits_32 ();
  let v = V.alloc 0ul 256sz;
  V.pts_to_len v;
  let mut k = 0sz;
  while (!k `SZ.lt` 256sz)
  invariant exists* vk s. R.pts_to k vk ** V.pts_to v s ** pure (SZ.v vk <= 256 /\ Seq.length s == 256)
  decreases (256 - SZ.v !k)
  {
    let vk = !k;
    V.pts_to_len v;
    V.(v.(vk) <- crc_entry (SZ.sizet_to_uint32 vk));
    SZ.fits_lte (SZ.v vk + 1) 256;
    k := SZ.add vk 1sz
  };
  V.pts_to_len v;
  v
}

(* Checksum final : crc, objets, chaînes, nombres, tableaux, mots, profondeur max, lignes, somme des id,
   somme de tous les entiers, nombre d'octets. *)
fn finish (st: V.vec U64.t) (#ss: Ghost.erased (Seq.seq U64.t))
  preserves V.pts_to st ss
  requires pure (V.length st == 16)
  returns r: U64.t
{
  V.pts_to_len st;
  let crc = U64.logxor V.(st.(0sz)) 0xFFFFFFFFuL;
  let r = ckfold ck_init crc;
  let r = ckfold r V.(st.(4sz));
  let r = ckfold r V.(st.(5sz));
  let r = ckfold r V.(st.(6sz));
  let r = ckfold r V.(st.(7sz));
  let r = ckfold r V.(st.(8sz));
  let r = ckfold r V.(st.(3sz));
  let r = ckfold r V.(st.(9sz));
  let r = ckfold r V.(st.(10sz));
  let r = ckfold r V.(st.(11sz));
  ckfold r V.(st.(15sz))
}
