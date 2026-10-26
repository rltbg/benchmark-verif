module Bench.Hashmap
#lang-pulse
(* Table de hachage à adressage ouvert, sondage linéaire, couples (clé, valeur) entrelacés dans un
   seul tableau de 2*cap u64 (cap = puissance de 2 >= 2n). 1 <= n <= 2^26. *)
open Pulse.Lib.Pervasives
open Bench.Common
module V   = Pulse.Lib.Vec
module R   = Pulse.Lib.Reference
module SZ  = FStar.SizeT
module U32 = FStar.UInt32
module U64 = FStar.UInt64

#set-options "--z3rlimit 40"

let seed : U64.t = 0x2468ACE013579BDFuL

let key_of (off i: U64.t) : U64.t =
  U64.logor (mix64 (U64.add_mod (U64.add_mod seed off) i)) 1uL

(* Position initiale : (key * phi) >> (64 - lg). *)
let slot_raw (key: U64.t) (lg: U32.t) : Pure U64.t (requires 1 <= U32.v lg /\ U32.v lg <= 27) (ensures fun _ -> True) =
  U64.shift_right (U64.mul_mod key 0x9E3779B97F4A7C15uL) (U32.sub 64ul lg)

(* Recherche d'un emplacement libre ou portant déjà `key`, au plus cap essais ; y écrit/ajoute i1. *)
fn insert (t: V.vec U64.t) (cap: SZ.t) (lg: U32.t) (key i1: U64.t) (#s0: Ghost.erased (Seq.seq U64.t))
  requires V.pts_to t s0 **
           pure (SZ.fits_u64 /\ 2 <= SZ.v cap /\ SZ.v cap <= 134217728 /\ V.length t == 2 * SZ.v cap /\
                 SZ.fits (2 * SZ.v cap + 2) /\ 1 <= U32.v lg /\ U32.v lg <= 27)
  ensures exists* s. V.pts_to t s ** pure (Seq.length s == 2 * SZ.v cap)
{
  V.pts_to_len t;
  let sraw = SZ.uint64_to_sizet (slot_raw key lg);
  let mut s = (if SZ.lt sraw cap then sraw else 0sz);
  let mut tries = 0sz;
  let mut fin = false;
  while (
    let vf = !fin;
    let vt = !tries;
    ((not vf) && (vt `SZ.lt` cap))
  )
  invariant exists* vs vt vf sv. R.pts_to s vs ** R.pts_to tries vt ** R.pts_to fin vf ** V.pts_to t sv **
            pure (SZ.v vs < SZ.v cap /\ SZ.v vt <= SZ.v cap /\ Seq.length sv == 2 * SZ.v cap)
  decreases (if !fin then 0 else SZ.v cap - SZ.v !tries + 1)
  {
    let vs = !s;
    let vt = !tries;
    SZ.fits_lte (SZ.v vs + SZ.v vs) (2 * SZ.v cap + 2);
    SZ.fits_lte (SZ.v vs + SZ.v vs + 1) (2 * SZ.v cap + 2);
    let s2 = SZ.add vs vs;
    let k = V.(t.(s2));
    let cur = V.(t.(SZ.add s2 1sz));
    let stop = ((k = 0uL) || (k = key));
    if stop {
      V.(t.(s2) <- key);
      V.(t.(SZ.add s2 1sz) <- (if (k = 0uL) then i1 else U64.add_mod cur i1))
    };
    V.pts_to_len t;
    SZ.fits_lte (SZ.v vs + 1) (2 * SZ.v cap + 2);
    SZ.fits_lte (SZ.v vt + 1) (2 * SZ.v cap + 2);
    let s1 = SZ.add vs 1sz;
    s := (if stop then vs else (if SZ.lt s1 cap then s1 else 0sz));
    tries := (if stop then vt else SZ.add vt 1sz);
    fin := stop
  }
}

fn lookup (t: V.vec U64.t) (cap: SZ.t) (lg: U32.t) (key: U64.t) (#s: Ghost.erased (Seq.seq U64.t))
  preserves V.pts_to t s
  requires pure (SZ.fits_u64 /\ 2 <= SZ.v cap /\ SZ.v cap <= 134217728 /\ V.length t == 2 * SZ.v cap /\
                 SZ.fits (2 * SZ.v cap + 2) /\ 1 <= U32.v lg /\ U32.v lg <= 27)
  returns r: U64.t
{
  V.pts_to_len t;
  let sraw = SZ.uint64_to_sizet (slot_raw key lg);
  let mut s = (if SZ.lt sraw cap then sraw else 0sz);
  let mut tries = 0sz;
  let mut fin = false;
  let mut res = 0uL;
  while (
    let vf = !fin;
    let vt = !tries;
    ((not vf) && (vt `SZ.lt` cap))
  )
  invariant exists* vs vt vf vr. R.pts_to s vs ** R.pts_to tries vt ** R.pts_to fin vf ** R.pts_to res vr **
            pure (SZ.v vs < SZ.v cap /\ SZ.v vt <= SZ.v cap)
  decreases (if !fin then 0 else SZ.v cap - SZ.v !tries + 1)
  {
    let vs = !s;
    let vt = !tries;
    SZ.fits_lte (SZ.v vs + SZ.v vs) (2 * SZ.v cap + 2);
    SZ.fits_lte (SZ.v vs + SZ.v vs + 1) (2 * SZ.v cap + 2);
    let s2 = SZ.add vs vs;
    let k = V.(t.(s2));
    let v = V.(t.(SZ.add s2 1sz));
    let stop = ((k = 0uL) || (k = key));
    SZ.fits_lte (SZ.v vs + 1) (2 * SZ.v cap + 2);
    SZ.fits_lte (SZ.v vt + 1) (2 * SZ.v cap + 2);
    let s1 = SZ.add vs 1sz;
    s := (if stop then vs else (if SZ.lt s1 cap then s1 else 0sz));
    tries := (if stop then vt else SZ.add vt 1sz);
    res := (if (k = key) then v else 0uL);
    fin := stop
  };
  !res
}

fn insert_all (t: V.vec U64.t) (cap: SZ.t) (lg: U32.t) (n: SZ.t) (#s0: Ghost.erased (Seq.seq U64.t))
  requires V.pts_to t s0 **
           pure (SZ.fits_u64 /\ 2 <= SZ.v cap /\ SZ.v cap <= 134217728 /\ V.length t == 2 * SZ.v cap /\
                 SZ.fits (2 * SZ.v cap + 2) /\ 1 <= U32.v lg /\ U32.v lg <= 27 /\ SZ.v n <= 67108864)
  ensures exists* s. V.pts_to t s ** pure (Seq.length s == 2 * SZ.v cap)
{
  V.pts_to_len t;
  let mut i = 0sz;
  while (!i `SZ.lt` n)
  invariant exists* vi s. R.pts_to i vi ** V.pts_to t s ** pure (SZ.v vi <= SZ.v n /\ Seq.length s == 2 * SZ.v cap)
  decreases (SZ.v n - SZ.v !i)
  {
    let vi = !i;
    let i64 = SZ.sizet_to_uint64 vi;
    insert t cap lg (key_of 0uL i64) (U64.add_mod i64 1uL);
    SZ.fits_u64_implies_fits (SZ.v vi + 1);
    i := SZ.add vi 1sz
  }
}

fn lookup_all (t: V.vec U64.t) (cap: SZ.t) (lg: U32.t) (n: SZ.t) (off: U64.t) (ck0: U64.t)
              (#s: Ghost.erased (Seq.seq U64.t))
  preserves V.pts_to t s
  requires pure (SZ.fits_u64 /\ 2 <= SZ.v cap /\ SZ.v cap <= 134217728 /\ V.length t == 2 * SZ.v cap /\
                 SZ.fits (2 * SZ.v cap + 2) /\ 1 <= U32.v lg /\ U32.v lg <= 27 /\ SZ.v n <= 67108864)
  returns r: U64.t
{
  let mut i = 0sz;
  let mut ck = ck0;
  while (!i `SZ.lt` n)
  invariant exists* vi vck. R.pts_to i vi ** R.pts_to ck vck ** pure (SZ.v vi <= SZ.v n)
  decreases (SZ.v n - SZ.v !i)
  {
    let vi = !i;
    let v = lookup t cap lg (key_of off (SZ.sizet_to_uint64 vi));
    ck := ckfold !ck v;
    SZ.fits_u64_implies_fits (SZ.v vi + 1);
    i := SZ.add vi 1sz
  };
  !ck
}

fn run (n: SZ.t)
  requires pure (SZ.fits_u64 /\ SZ.v n >= 1 /\ SZ.v n <= 67108864)
  returns r: U64.t
  ensures emp
{
  assert_norm (pow2 64 == 18446744073709551616);
  SZ.fits_u64_implies_fits (2 * SZ.v n);
  SZ.fits_u64_implies_fits 33554432;
  let lim = SZ.uint_to_t 33554432;
  let n2 = SZ.add n n;
  let mut cap = 2sz;
  let mut lg = 1ul;
  while (
    let vc = !cap;
    let vl = !lg;
    ((vc `SZ.lt` n2) && U32.lt vl 27ul && (vc `SZ.lte` lim))
  )
  invariant exists* vc vl. R.pts_to cap vc ** R.pts_to lg vl **
            pure (2 <= SZ.v vc /\ SZ.v vc <= 134217728 /\ 1 <= U32.v vl /\ U32.v vl <= 27)
  decreases (27 - U32.v !lg)
  {
    let vc = !cap;
    let vl = !lg;
    SZ.fits_u64_implies_fits (SZ.v vc + SZ.v vc);
    cap := SZ.add vc vc;
    lg := U32.add vl 1ul
  };
  let vcap = !cap;
  let vlg = !lg;
  SZ.fits_u64_implies_fits (2 * SZ.v vcap + 2);
  let t = V.alloc 0uL (SZ.add vcap vcap);
  insert_all t vcap vlg n;
  let ck1 = lookup_all t vcap vlg n 0uL ck_init;
  let ck2 = lookup_all t vcap vlg n (SZ.sizet_to_uint64 n) ck1;
  V.free t;
  ck2
}
