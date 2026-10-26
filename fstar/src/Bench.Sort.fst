module Bench.Sort
#lang-pulse
(* Tri fusion descendant (récursif) avec tableau temporaire et recopie. 1 <= n <= 2^30.
   Prouvé (correction fonctionnelle complète) : `msort` trie la plage [lo,hi), le résultat en est une
   permutation, rien d'autre n'est modifié ; plus mémoire sûre, absence de débordement, terminaison. *)
open Pulse.Lib.Pervasives
open Bench.Common
open Bench.SortSpec
module V   = Pulse.Lib.Vec
module R   = Pulse.Lib.Reference
module SZ  = FStar.SizeT
module U32 = FStar.UInt32
module U64 = FStar.UInt64

#set-options "--z3rlimit 100"

let seed : U64.t = 0x0123456789ABCDEFuL

fn fill (v: V.vec U32.t) (n: SZ.t) (#s0: Ghost.erased (Seq.seq U32.t))
  requires V.pts_to v s0 ** pure (V.length v == SZ.v n)
  ensures exists* s. V.pts_to v s ** pure (Seq.length s == SZ.v n)
{
  V.pts_to_len v;
  let mut i = 0sz;
  while (!i `SZ.lt` n)
  invariant exists* vi s. R.pts_to i vi ** V.pts_to v s ** pure (SZ.v vi <= SZ.v n /\ Seq.length s == SZ.v n)
  decreases (SZ.v n - SZ.v !i)
  {
    let vi = !i;
    V.(v.(vi) <- hi32 (mix64 (U64.add_mod seed (SZ.sizet_to_uint64 vi))));
    i := SZ.add vi 1sz
  }
}

(* Fusionne les runs triés src[lo,mid) et src[mid,hi) dans dst[lo,hi). *)
fn merge (src dst: V.vec U32.t) (lo mid hi: SZ.t)
         (#ss: Ghost.erased (Seq.seq U32.t)) (#sd: Ghost.erased (Seq.seq U32.t))
  preserves V.pts_to src ss
  requires V.pts_to dst sd **
           pure (Seq.length ss == Seq.length sd /\ SZ.v lo <= SZ.v mid /\ SZ.v mid <= SZ.v hi /\ SZ.v hi <= Seq.length ss /\
                 sorted_range ss (SZ.v lo) (SZ.v mid) /\ sorted_range ss (SZ.v mid) (SZ.v hi) /\
                 SZ.fits (Seq.length ss + 1))
  ensures exists* sd'. V.pts_to dst sd' **
          pure (Seq.length sd' == Seq.length sd /\ sorted_range sd' (SZ.v lo) (SZ.v hi) /\
                perm_range sd' ss (SZ.v lo) (SZ.v hi) /\ frame sd' sd (SZ.v lo) (SZ.v hi))
{
  V.pts_to_len src;
  V.pts_to_len dst;
  merge_init sd ss (SZ.v lo) (SZ.v mid) (SZ.v hi);
  frame_refl sd (SZ.v lo) (SZ.v hi);
  let mut i = lo;
  let mut j = mid;
  let mut k = lo;
  while (!k `SZ.lt` hi)
  invariant exists* vi vj vk sd1. R.pts_to i vi ** R.pts_to j vj ** R.pts_to k vk ** V.pts_to dst sd1 **
            pure (merge_inv sd1 ss (SZ.v lo) (SZ.v mid) (SZ.v hi) (SZ.v vi) (SZ.v vj) (SZ.v vk) /\
                  frame sd1 sd (SZ.v lo) (SZ.v hi) /\
                  SZ.v lo <= SZ.v vi /\ SZ.v vi <= SZ.v mid /\ SZ.v mid <= SZ.v vj /\ SZ.v vj <= SZ.v hi /\
                  SZ.v vk + SZ.v mid == SZ.v vi + SZ.v vj /\ SZ.v vk <= SZ.v hi)
  decreases (SZ.v hi - SZ.v !k)
  {
    with vi0 vj0 vk0 sd1. assert (R.pts_to i vi0 ** R.pts_to j vj0 ** R.pts_to k vk0 ** V.pts_to dst sd1);
    frame_len sd1 sd (SZ.v lo) (SZ.v hi);
    let vi = !i;
    let vj = !j;
    let vk = !k;
    let x = if (vi `SZ.lt` mid) { V.(src.(vi)) } else { 0ul };
    let y = if (vj `SZ.lt` hi) { V.(src.(vj)) } else { 0ul };
    let take_left = (vi `SZ.lt` mid) && ((not (vj `SZ.lt` hi)) || U32.lte x y);
    assert (pure (take_left ==> SZ.v vi < SZ.v mid));
    assert (pure (~take_left ==> SZ.v vj < SZ.v hi));
    assert (pure (SZ.v vi < SZ.v mid ==> SZ.v vi < Seq.length ss));
    assert (pure (SZ.v vj < SZ.v hi ==> SZ.v vj < Seq.length ss));
    assert (pure (take_left ==> (SZ.v vj >= SZ.v hi \/ (SZ.v vj < Seq.length ss /\ SZ.v vi < Seq.length ss /\ U32.v (Seq.index ss (SZ.v vi)) <= U32.v (Seq.index ss (SZ.v vj))))));
    assert (pure (~take_left ==> (SZ.v vi >= SZ.v mid \/ (SZ.v vi < Seq.length ss /\ SZ.v vj < Seq.length ss /\ U32.v (Seq.index ss (SZ.v vj)) < U32.v (Seq.index ss (SZ.v vi))))));
    merge_step sd1 ss (SZ.v lo) (SZ.v mid) (SZ.v hi) (SZ.v vi) (SZ.v vj) (SZ.v vk) take_left;
    V.(dst.(vk) <- (if take_left then x else y));
    frame_upd sd1 sd (SZ.v lo) (SZ.v hi) (SZ.v vk) (if take_left then x else y);
    SZ.fits_lte (SZ.v vi + 1) (Seq.length ss + 1);
    SZ.fits_lte (SZ.v vj + 1) (Seq.length ss + 1);
    SZ.fits_lte (SZ.v vk + 1) (Seq.length ss + 1);
    i := (if take_left then SZ.add vi 1sz else vi);
    j := (if take_left then vj else SZ.add vj 1sz);
    k := SZ.add vk 1sz
  };
  with vi vj vk sd1. assert (R.pts_to i vi ** R.pts_to j vj ** R.pts_to k vk ** V.pts_to dst sd1);
  merge_done sd1 ss (SZ.v lo) (SZ.v mid) (SZ.v hi) (SZ.v vi) (SZ.v vj) (SZ.v vk);
  frame_len sd1 sd (SZ.v lo) (SZ.v hi);
  V.pts_to_len dst
}

(* a[lo,hi) <- tmp[lo,hi). *)
fn copy_back (tmp a: V.vec U32.t) (lo hi: SZ.t)
             (#st: Ghost.erased (Seq.seq U32.t)) (#sa: Ghost.erased (Seq.seq U32.t))
  preserves V.pts_to tmp st
  requires V.pts_to a sa **
           pure (Seq.length st == Seq.length sa /\ SZ.v lo <= SZ.v hi /\ SZ.v hi <= Seq.length sa /\
                 SZ.fits (Seq.length sa + 1))
  ensures exists* sa'. V.pts_to a sa' **
          pure (Seq.length sa' == Seq.length sa /\ slice_eq sa' st (SZ.v lo) (SZ.v hi) /\
                frame sa' sa (SZ.v lo) (SZ.v hi))
{
  V.pts_to_len tmp;
  V.pts_to_len a;
  frame_refl sa (SZ.v lo) (SZ.v hi);
  let mut i = lo;
  while (!i `SZ.lt` hi)
  invariant exists* vi sa1. R.pts_to i vi ** V.pts_to a sa1 **
            pure (SZ.v lo <= SZ.v vi /\ SZ.v vi <= SZ.v hi /\ Seq.length sa1 == Seq.length sa /\
                  frame sa1 sa (SZ.v lo) (SZ.v hi) /\
                  (forall (p: nat). SZ.v lo <= p /\ p < SZ.v vi ==> Seq.index sa1 p == Seq.index st p))
  decreases (SZ.v hi - SZ.v !i)
  {
    with vi0 sa1. assert (R.pts_to i vi0 ** V.pts_to a sa1);
    frame_len sa1 sa (SZ.v lo) (SZ.v hi);
    let vi = !i;
    let x = V.(tmp.(vi));
    V.(a.(vi) <- x);
    frame_upd sa1 sa (SZ.v lo) (SZ.v hi) (SZ.v vi) x;
    SZ.fits_lte (SZ.v vi + 1) (Seq.length sa + 1);
    i := SZ.add vi 1sz
  };
  with vi sa1. assert (R.pts_to i vi ** V.pts_to a sa1);
  frame_len sa1 sa (SZ.v lo) (SZ.v hi);
  slice_eq_intro sa1 st (SZ.v lo) (SZ.v hi);
  V.pts_to_len a
}

fn rec msort (a tmp: V.vec U32.t) (lo hi: SZ.t)
             (#sa: Ghost.erased (Seq.seq U32.t)) (#st: Ghost.erased (Seq.seq U32.t))
  requires V.pts_to a sa ** V.pts_to tmp st **
           pure (Seq.length sa == Seq.length st /\ SZ.v lo <= SZ.v hi /\ SZ.v hi <= Seq.length sa /\
                 SZ.fits (Seq.length sa + 1))
  ensures exists* sa' st'. V.pts_to a sa' ** V.pts_to tmp st' **
          pure (Seq.length sa' == Seq.length sa /\ Seq.length st' == Seq.length st /\
                sorted_range sa' (SZ.v lo) (SZ.v hi) /\ perm_range sa' sa (SZ.v lo) (SZ.v hi) /\
                frame sa' sa (SZ.v lo) (SZ.v hi))
  decreases (SZ.v hi - SZ.v lo)
{
  if (SZ.lt (SZ.sub hi lo) 2sz) {
    perm_range_refl sa (SZ.v lo) (SZ.v hi);
    sorted_small sa (SZ.v lo) (SZ.v hi);
    frame_refl sa (SZ.v lo) (SZ.v hi);
    V.pts_to_len a
  } else {
    let mid = SZ.add lo (SZ.div (SZ.sub hi lo) 2sz);
    msort a tmp lo mid;
    with s1 t1. assert (V.pts_to a s1 ** V.pts_to tmp t1);
    msort a tmp mid hi;
    with s2 t2. assert (V.pts_to a s2 ** V.pts_to tmp t2);
    sorted_frame_l s2 s1 (SZ.v lo) (SZ.v mid) (SZ.v mid) (SZ.v hi);
    merge a tmp lo mid hi;
    with t3. assert (V.pts_to tmp t3);
    copy_back tmp a lo hi;
    with s3. assert (V.pts_to a s3);
    msort_combine sa s1 s2 t3 s3 (SZ.v lo) (SZ.v mid) (SZ.v hi);
    V.pts_to_len a
  }
}

(* fold des éléments dans l'ordre ; 0 si l'ordre n'est pas croissant. *)
fn checksum (v: V.vec U32.t) (n: SZ.t) (#s: Ghost.erased (Seq.seq U32.t))
  preserves V.pts_to v s
  requires pure (V.length v == SZ.v n /\ SZ.fits (SZ.v n + 1))
  returns r: U64.t
{
  V.pts_to_len v;
  let mut i = 0sz;
  let mut ck = ck_init;
  let mut prev = 0ul;
  let mut ok = true;
  while (!i `SZ.lt` n)
  invariant exists* vi vck vprev vok. R.pts_to i vi ** R.pts_to ck vck ** R.pts_to prev vprev ** R.pts_to ok vok **
            pure (SZ.v vi <= SZ.v n)
  decreases (SZ.v n - SZ.v !i)
  {
    let vi = !i;
    let x = V.(v.(vi));
    let vp = !prev;
    let vok = !ok;
    ok := (vok && U32.lte vp x);
    prev := x;
    ck := ckfold !ck (FStar.Int.Cast.uint32_to_uint64 x);
    SZ.fits_lte (SZ.v vi + 1) (SZ.v n + 1);
    i := SZ.add vi 1sz
  };
  let vok = !ok;
  let vck = !ck;
  (if vok then vck else 0uL)
}

fn run (n: SZ.t)
  requires pure (SZ.fits_u64 /\ SZ.v n >= 1 /\ SZ.v n <= 1073741824)
  returns r: U64.t
  ensures emp
{
  assert_norm (pow2 64 == 18446744073709551616);
  SZ.fits_u64_implies_fits (SZ.v n + 1);
  let a = V.alloc 0ul n;
  let tmp = V.alloc 0ul n;
  fill a n;
  V.pts_to_len a;
  V.pts_to_len tmp;
  msort a tmp 0sz n;
  with sa. assert (V.pts_to a sa);
  V.pts_to_len a;
  let ck = checksum a n;
  V.free a;
  V.free tmp;
  ck
}
