module Bench.Lines
#lang-pulse
(* Débit « lignes/seconde » : génère n lignes ASCII « clé;valeur\n » dans un buffer, puis les relit
   octet par octet et agrège somme/compte par clé (2^20 clés). 1 <= n <= 268_435_455. *)
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

let seed : U64.t = 0x1357924680ACEBDFuL

(* Nombre de chiffres décimaux (au plus 7 ; au plus 6 si v < 10^6). *)
[@@"opaque_to_smt"]
let ndigits (v: U32.t) : Pure SZ.t (requires True)
  (ensures fun r -> 1 <= SZ.v r /\ SZ.v r <= 7 /\ (U32.v v < 1000000 ==> SZ.v r <= 6)) =
  if U32.lt v 10ul then 1sz
  else if U32.lt v 100ul then 2sz
  else if U32.lt v 1000ul then 3sz
  else if U32.lt v 10000ul then 4sz
  else if U32.lt v 100000ul then 5sz
  else if U32.lt v 1000000ul then 6sz
  else 7sz

(* Écrit les l chiffres de v dans txt[off, off+l), de droite à gauche. *)
fn write_dec (txt: V.vec U8.t) (off: SZ.t) (v: U32.t) (l: SZ.t) (#s0: Ghost.erased (Seq.seq U8.t))
  requires V.pts_to txt s0 ** pure (SZ.v l >= 1 /\ SZ.v off + SZ.v l <= V.length txt)
  ensures exists* s. V.pts_to txt s ** pure (Seq.length s == Seq.length s0)
{
  V.pts_to_len txt;
  let mut c = l;
  let mut x = v;
  while (!c `SZ.gt` 0sz)
  invariant exists* vc vx s. R.pts_to c vc ** R.pts_to x vx ** V.pts_to txt s **
            pure (SZ.v vc <= SZ.v l /\ Seq.length s == Seq.length s0)
  decreases (SZ.v !c)
  {
    let vc = !c;
    let vx = !x;
    SZ.fits_lte (SZ.v off + SZ.v vc - 1) (SZ.v off + SZ.v l);
    let p = SZ.add off (SZ.sub vc 1sz);
    V.(txt.(p) <- C.uint32_to_uint8 (U32.add_mod 48ul (U32.rem vx 10ul)));
    x := U32.div vx 10ul;
    c := SZ.sub vc 1sz
  }
}

(* Génère les n lignes ; rend la longueur utile du texte. *)
fn gen (txt: V.vec U8.t) (n: SZ.t) (#s0: Ghost.erased (Seq.seq U8.t))
  requires V.pts_to txt s0 ** pure (SZ.fits_u64 /\ SZ.v n <= 268435455 /\ V.length txt == 15 * SZ.v n + 20)
  returns len: SZ.t
  ensures (exists* s. V.pts_to txt s ** pure (Seq.length s == 15 * SZ.v n + 20)) ** pure (SZ.v len <= 15 * SZ.v n)
{
  V.pts_to_len txt;
  assert_norm (pow2 64 == 18446744073709551616);
  SZ.fits_u64_implies_fits (15 * SZ.v n + 20);
  let mut i = 0sz;
  let mut off = 0sz;
  while (!i `SZ.lt` n)
  invariant exists* vi voff s. R.pts_to i vi ** R.pts_to off voff ** V.pts_to txt s **
            pure (SZ.v vi <= SZ.v n /\ SZ.v voff <= 15 * SZ.v vi /\ Seq.length s == 15 * SZ.v n + 20)
  decreases (SZ.v n - SZ.v !i)
  {
    let vi = !i;
    let voff = !off;
    let r = mix64 (U64.add_mod seed (SZ.sizet_to_uint64 vi));
    let key = mask20 (C.uint64_to_uint32 (U64.shift_right r 44ul));
    let vl = C.uint64_to_uint32 (U64.rem (U64.logand (U64.shift_right r 8ul) 0xFFFFFFFFuL) 1000000uL);
    let l1 = ndigits key;
    write_dec txt voff key l1;
    SZ.fits_u64_implies_fits (SZ.v voff + 15);
    let off1 = SZ.add voff l1;
    V.(txt.(off1) <- 59uy);
    let off2 = SZ.add off1 1sz;
    let l2 = ndigits vl;
    write_dec txt off2 vl l2;
    let off3 = SZ.add off2 l2;
    V.(txt.(off3) <- 10uy);
    SZ.fits_u64_implies_fits (SZ.v vi + 1);
    off := SZ.add off3 1sz;
    i := SZ.add vi 1sz
  };
  !off
}

(* tab[2k] = somme des valeurs, tab[2k+1] = nombre de lignes de la clé k.
   Machine à états : un octet par itération (clé jusqu'à ';', puis valeur jusqu'à '\n'). *)
fn parse (txt: V.vec U8.t) (tab: V.vec U64.t) (len: SZ.t)
         (#st: Ghost.erased (Seq.seq U8.t)) (#sb: Ghost.erased (Seq.seq U64.t))
  preserves V.pts_to txt st
  requires V.pts_to tab sb **
           pure (SZ.fits_u64 /\ SZ.v len <= V.length txt /\ V.length tab == 2097152)
  ensures exists* sb'. V.pts_to tab sb' ** pure (Seq.length sb' == 2097152)
{
  V.pts_to_len txt;
  V.pts_to_len tab;
  assert_norm (pow2 64 == 18446744073709551616);
  SZ.fits_u64_implies_fits_32 ();
  let mut p = 0sz;
  let mut in_val = false;
  let mut key = 0ul;
  let mut vl = 0ul;
  while (!p `SZ.lt` len)
  invariant exists* vp vin vk vv sb1. R.pts_to p vp ** R.pts_to in_val vin ** R.pts_to key vk ** R.pts_to vl vv **
            V.pts_to tab sb1 ** pure (SZ.v vp <= SZ.v len /\ Seq.length sb1 == 2097152)
  decreases (SZ.v len - SZ.v !p)
  {
    let vp = !p;
    let c = V.(txt.(vp));
    let dgt = C.uint8_to_uint32 (U8.sub_mod c 48uy);
    let vin = !in_val;
    let k = !key;
    let v = !vl;
    if vin {
      if (c = 10uy) {
        let kk = SZ.uint32_to_sizet (mask20 k);
        SZ.fits_u64_implies_fits (2 * SZ.v kk + 1);
        let k2 = SZ.add kk kk;
        let s = V.(tab.(k2));
        V.(tab.(k2) <- U64.add_mod s (C.uint32_to_uint64 v));
        let cnt = V.(tab.(SZ.add k2 1sz));
        V.(tab.(SZ.add k2 1sz) <- U64.add_mod cnt 1uL);
        in_val := false;
        key := 0ul;
        vl := 0ul
      } else {
        vl := U32.add_mod (U32.mul_mod v 10ul) dgt
      }
    } else {
      if (c = 59uy) {
        in_val := true
      } else {
        key := U32.add_mod (U32.mul_mod k 10ul) dgt
      }
    };
    V.pts_to_len tab;
    SZ.fits_lte (SZ.v vp + 1) (SZ.v len);
    p := SZ.add vp 1sz
  }
}

fn checksum (tab: V.vec U64.t) (#s: Ghost.erased (Seq.seq U64.t))
  preserves V.pts_to tab s
  requires pure (SZ.fits_u64 /\ V.length tab == 2097152)
  returns r: U64.t
{
  V.pts_to_len tab;
  assert_norm (pow2 64 == 18446744073709551616);
  SZ.fits_u64_implies_fits 1048576;
  let lim = SZ.uint_to_t 1048576;
  let mut k = 0sz;
  let mut ck = ck_init;
  while (!k `SZ.lt` lim)
  invariant exists* vk vck. R.pts_to k vk ** R.pts_to ck vck ** pure (SZ.v vk <= 1048576)
  decreases (1048576 - SZ.v !k)
  {
    let vk = !k;
    SZ.fits_u64_implies_fits (2 * SZ.v vk + 1);
    let k2 = SZ.add vk vk;
    let a = V.(tab.(k2));
    let b = V.(tab.(SZ.add k2 1sz));
    ck := ckfold (ckfold !ck a) b;
    k := SZ.add vk 1sz
  };
  !ck
}

fn run (n: SZ.t)
  requires pure (SZ.fits_u64 /\ SZ.v n >= 1 /\ SZ.v n <= 268435455)
  returns r: U64.t
  ensures emp
{
  assert_norm (pow2 64 == 18446744073709551616);
  SZ.fits_u64_implies_fits (15 * SZ.v n + 20);
  SZ.fits_u64_implies_fits 2097152;
  SZ.fits_u64_implies_fits (15 * SZ.v n);
  let sz = SZ.add (SZ.mul 15sz n) 20sz;
  let txt = V.alloc 0uy sz;
  let len = gen txt n;
  let tab = V.alloc 0uL (SZ.uint_to_t 2097152);
  V.pts_to_len txt;
  parse txt tab len;
  let r = checksum tab;
  V.free txt;
  V.free tab;
  r
}
