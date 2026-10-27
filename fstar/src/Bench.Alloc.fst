module Bench.Alloc
#lang-pulse
(* Allocation en masse (spec/workloads.d/alloc.md) : n blocs de 8..256 octets alloués, remplis, sommés ;
   un bloc sur 16 est conservé dans une fenêtre FIFO de W = min(max(1, n/64), 2^25) blocs (le plus ancien est libéré
   quand elle déborde). La fenêtre est une file à deux piles de blocs chaînés ; la propriété de chaque
   bloc est portée par la slprop récursive `owned`. 1 <= n <= 4_000_000_000. *)
open Pulse.Lib.Pervasives
open Bench.Common
module V   = Pulse.Lib.Vec
module R   = Pulse.Lib.Reference
module B   = Pulse.Lib.Box
module SZ  = FStar.SizeT
module U8  = FStar.UInt8
module U32 = FStar.UInt32
module U64 = FStar.UInt64
module C   = FStar.Int.Cast

noeq
type node = {
  blk  : V.vec U8.t;
  next : option node_ptr;
}
and node_ptr = B.box node

(* Compteur fantôme (Peano) : la slprop récursive se déplie par filtrage sur sa forme. *)
type cnt = | Z | S : cnt -> cnt

let rec clen (c: cnt) : nat = match c with | Z -> 0 | S c' -> 1 + clen c'

(* Des blocs chaînés à partir de x, un par S, chacun propriétaire de son bloc d'octets (pleinement alloué). *)
let rec owned ([@@@mkey] x: option node_ptr) (c: cnt) : Tot slprop (decreases c) =
  match c with
  | Z -> pure (x == None)
  | S c' ->
    exists* (p: node_ptr) (nd: node) (s: Seq.seq U8.t).
      pure (x == Some p) ** B.pts_to p nd ** V.pts_to nd.blk s ** pure (V.is_full_vec nd.blk) **
      owned nd.next c'

ghost
fn owned_zero (x: option node_ptr)
  requires pure (x == None)
  ensures owned x Z
{
  fold (owned x Z)
}

ghost
fn owned_succ (x: option node_ptr) (p: node_ptr) (blk: V.vec U8.t) (nx: option node_ptr) (s: Seq.seq U8.t) (c: cnt)
  requires pure (x == Some p) ** B.pts_to p ({ blk = blk; next = nx }) ** V.pts_to blk s ** pure (V.is_full_vec blk) ** owned nx c
  ensures owned x (S c)
{
  fold (owned x (S c))
}

ghost
fn owned_pop_case (x: option node_ptr) (c: cnt)
  requires owned x c ** pure (clen c > 0)
  ensures exists* (p: node_ptr) (nd: node) (s: Seq.seq U8.t) (c': cnt).
      pure (x == Some p /\ clen c' + 1 == clen c) ** B.pts_to p nd ** V.pts_to nd.blk s ** pure (V.is_full_vec nd.blk) ** owned nd.next c'
{
  match c {
    Z -> { unfold (owned x Z); unreachable () }
    S c' -> { unfold (owned x (S c')) }
  }
}

(* Ajoute un bloc en tête de la pile `st` (de n blocs). *)
fn push_blk (st: R.ref (option node_ptr)) (nr: R.ref SZ.t) (blk: V.vec U8.t)
  (#s: Ghost.erased (Seq.seq U8.t)) (#x0: Ghost.erased (option node_ptr)) (#n0: Ghost.erased SZ.t) (#c0: Ghost.erased cnt)
  requires R.pts_to st x0 ** R.pts_to nr n0 ** owned x0 c0 ** V.pts_to blk s **
           pure (SZ.fits_u64 /\ V.is_full_vec blk /\ clen c0 == SZ.v n0 /\ SZ.v n0 < 1000000000)
  ensures exists* x1 n1 c1. R.pts_to st x1 ** R.pts_to nr n1 ** owned x1 c1 **
           pure (SZ.v n1 == SZ.v n0 + 1 /\ clen c1 == SZ.v n1)
{
  let x = !st;
  let n = !nr;
  let p = B.alloc ({ blk = blk; next = x });
  owned_succ (Some p) p blk x s c0;
  st := Some p;
  assert_norm (pow2 64 == 18446744073709551616);
  SZ.fits_u64_implies_fits (SZ.v n + 1);
  nr := SZ.add n 1sz
}

(* Libère le bloc de tête de la pile (n >= 1 blocs). *)
fn pop_free (st: R.ref (option node_ptr)) (nr: R.ref SZ.t)
  (#x0: Ghost.erased (option node_ptr)) (#n0: Ghost.erased SZ.t) (#c0: Ghost.erased cnt)
  requires R.pts_to st x0 ** R.pts_to nr n0 ** owned x0 c0 ** pure (clen c0 == SZ.v n0 /\ SZ.v n0 > 0)
  ensures exists* x1 n1 c1. R.pts_to st x1 ** R.pts_to nr n1 ** owned x1 c1 **
           pure (SZ.v n1 + 1 == SZ.v n0 /\ clen c1 == SZ.v n1)
{
  let x = !st;
  owned_pop_case x c0;
  with p nd s cr. _;
  match x {
    None -> { unreachable () }
    Some q -> {
      rewrite each p as q;
      let nv = B.op_Bang q;
      V.free nv.blk;
      B.free q;
      st := nv.next;
      let n = !nr;
      nr := SZ.sub n 1sz
    }
  }
}

(* Retourne la pile `bk` sur la pile `fr` : tous les blocs de `bk` passent (en ordre inverse) au-dessus de `fr`. *)
fn move_all (fr: R.ref (option node_ptr)) (nf: R.ref SZ.t) (bk: R.ref (option node_ptr)) (nb: R.ref SZ.t)
  (#f0: Ghost.erased (option node_ptr)) (#n0: Ghost.erased SZ.t) (#cf0: Ghost.erased cnt)
  (#b0: Ghost.erased (option node_ptr)) (#m0: Ghost.erased SZ.t) (#cb0: Ghost.erased cnt)
  requires R.pts_to fr f0 ** R.pts_to nf n0 ** owned f0 cf0 ** R.pts_to bk b0 ** R.pts_to nb m0 ** owned b0 cb0 **
           pure (SZ.fits_u64 /\ clen cf0 == SZ.v n0 /\ clen cb0 == SZ.v m0 /\ SZ.v n0 + SZ.v m0 < 1000000000)
  ensures exists* f1 n1 cf1 b1 m1 cb1. R.pts_to fr f1 ** R.pts_to nf n1 ** owned f1 cf1 ** R.pts_to bk b1 ** R.pts_to nb m1 ** owned b1 cb1 **
           pure (clen cf1 == SZ.v n1 /\ clen cb1 == SZ.v m1 /\ SZ.v m1 == 0 /\ SZ.v n1 == SZ.v n0 + SZ.v m0)
{
  while (
    let m = !nb;
    SZ.lt 0sz m
  )
  invariant exists* fv nfv cf bv nbv cb. R.pts_to fr fv ** R.pts_to nf nfv ** owned fv cf ** R.pts_to bk bv ** R.pts_to nb nbv ** owned bv cb **
            pure (clen cf == SZ.v nfv /\ clen cb == SZ.v nbv /\ SZ.v nfv + SZ.v nbv == SZ.v n0 + SZ.v m0)
  decreases (SZ.v !nb)
  {
    let bv = !bk;
    owned_pop_case bv _;
    with p nd s cr. _;
    match bv {
      None -> { unreachable () }
      Some q -> {
        rewrite each p as q;
        let nv = B.op_Bang q;
        let fv = !fr;
        B.op_Colon_Equals q ({ blk = nv.blk; next = fv });
        owned_succ (Some q) q nv.blk fv s _;
        fr := Some q;
        bk := nv.next;
        let a = !nf;
        let m = !nb;
        assert_norm (pow2 64 == 18446744073709551616);
        SZ.fits_u64_implies_fits (SZ.v a + 1);
        nf := SZ.add a 1sz;
        nb := SZ.sub m 1sz
      }
    }
  }
}

(* La file : deux piles (front = les plus anciens, bk = les plus récents) ; au plus b blocs (b <= 2^25 + 1). *)
let qinv (fr: R.ref (option node_ptr)) (nf: R.ref SZ.t) (bk: R.ref (option node_ptr)) (nb: R.ref SZ.t) (b: nat) : slprop =
  exists* (fv: option node_ptr) (nfv: SZ.t) (cf: cnt) (bv: option node_ptr) (nbv: SZ.t) (cb: cnt).
    R.pts_to fr fv ** R.pts_to nf nfv ** owned fv cf ** R.pts_to bk bv ** R.pts_to nb nbv ** owned bv cb **
    pure (clen cf == SZ.v nfv /\ clen cb == SZ.v nbv /\ SZ.v nfv + SZ.v nbv <= b /\ b <= 33554433)

(* idem, avec en plus : la pile front est non vide. *)
let qfront (fr: R.ref (option node_ptr)) (nf: R.ref SZ.t) (bk: R.ref (option node_ptr)) (nb: R.ref SZ.t) (b: nat) : slprop =
  exists* (fv: option node_ptr) (nfv: SZ.t) (cf: cnt) (bv: option node_ptr) (nbv: SZ.t) (cb: cnt).
    R.pts_to fr fv ** R.pts_to nf nfv ** owned fv cf ** R.pts_to bk bv ** R.pts_to nb nbv ** owned bv cb **
    pure (clen cf == SZ.v nfv /\ clen cb == SZ.v nbv /\ SZ.v nfv > 0 /\ SZ.v nfv + SZ.v nbv <= b /\ b <= 33554433)

(* File de <= w+1 blocs -> file de <= w blocs : libère le plus ancien si la file a w+1 blocs. *)
fn trim (fr: R.ref (option node_ptr)) (nf: R.ref SZ.t) (bk: R.ref (option node_ptr)) (nb: R.ref SZ.t) (w: SZ.t)
  requires qinv fr nf bk nb (SZ.v w + 1) ** pure (SZ.fits_u64 /\ 1 <= SZ.v w /\ SZ.v w <= 33554432)
  ensures qinv fr nf bk nb (SZ.v w)
{
  unfold (qinv fr nf bk nb (SZ.v w + 1));
  let a = !nf;
  let m = !nb;
  assert_norm (pow2 64 == 18446744073709551616);
  SZ.fits_u64_implies_fits (SZ.v a + SZ.v m);
  let tot = SZ.add a m;
  if (SZ.lt w tot) {
    if (a = 0sz) {
      move_all fr nf bk nb;
      fold (qfront fr nf bk nb (SZ.v w + 1))
    } else {
      fold (qfront fr nf bk nb (SZ.v w + 1))
    };
    unfold (qfront fr nf bk nb (SZ.v w + 1));
    pop_free fr nf;
    fold (qinv fr nf bk nb (SZ.v w))
  } else {
    fold (qinv fr nf bk nb (SZ.v w))
  }
}

(* Un bloc conservé dans la fenêtre de w blocs (le plus ancien est libéré quand elle déborde). *)
fn keep_blk (fr: R.ref (option node_ptr)) (nf: R.ref SZ.t) (bk: R.ref (option node_ptr)) (nb: R.ref SZ.t) (w: SZ.t)
  (blk: V.vec U8.t) (#s: Ghost.erased (Seq.seq U8.t))
  requires qinv fr nf bk nb (SZ.v w) ** V.pts_to blk s **
           pure (SZ.fits_u64 /\ V.is_full_vec blk /\ 1 <= SZ.v w /\ SZ.v w <= 33554432)
  ensures qinv fr nf bk nb (SZ.v w)
{
  unfold (qinv fr nf bk nb (SZ.v w));
  push_blk bk nb blk;
  fold (qinv fr nf bk nb (SZ.v w + 1));
  trim fr nf bk nb w
}

ghost
fn owned_nil (x: option node_ptr) (c: cnt)
  requires owned x c ** pure (clen c == 0)
  ensures pure (x == None)
{
  match c {
    Z -> { unfold (owned x Z) }
    S c' -> { unfold (owned x (S c')); unreachable () }
  }
}

(* Libère toute la file (front puis bk). *)
fn drain (fr: R.ref (option node_ptr)) (nf: R.ref SZ.t) (bk: R.ref (option node_ptr)) (nb: R.ref SZ.t) (w: SZ.t)
  requires qinv fr nf bk nb (SZ.v w) ** pure (SZ.fits_u64 /\ SZ.v w <= 33554432)
  ensures exists* (fv: option node_ptr) (nfv: SZ.t) (bv: option node_ptr) (nbv: SZ.t).
          R.pts_to fr fv ** R.pts_to nf nfv ** R.pts_to bk bv ** R.pts_to nb nbv
{
  unfold (qinv fr nf bk nb (SZ.v w));
  move_all fr nf bk nb;
  while (
    let a = !nf;
    SZ.lt 0sz a
  )
  invariant exists* (fv: option node_ptr) (nfv: SZ.t) (cf: cnt). R.pts_to fr fv ** R.pts_to nf nfv ** owned fv cf ** pure (clen cf == SZ.v nfv)
  decreases (SZ.v !nf)
  {
    pop_free fr nf
  };
  with fv nfv cf. assert (R.pts_to fr fv ** R.pts_to nf nfv ** owned fv cf ** pure (clen cf == SZ.v nfv));
  owned_nil fv cf;
  with bv nbv cb. assert (R.pts_to bk bv ** R.pts_to nb nbv ** owned bv cb ** pure (clen cb == SZ.v nbv));
  owned_nil bv cb
}

let seed : U64.t = 0xA110C00000000001uL

(* x & 31 <= 31 : taille de bloc 8 + 8*(x & 31) <= 256. *)
let low5 (x: U64.t) : Pure U64.t (requires True) (ensures fun r -> U64.v r <= 31) =
  FStar.UInt.logand_le #64 (U64.v x) 31;
  U64.logand x 31uL

(* Remplit b[k] = (i + k) mod 251 puis renvoie la somme des octets (lue dans le bloc). *)
fn fill_sum (b: V.vec U8.t) (sz: SZ.t) (i: U64.t) (#s0: Ghost.erased (Seq.seq U8.t))
  requires V.pts_to b s0 ** pure (V.length b == SZ.v sz /\ SZ.v sz <= 256 /\ SZ.fits_u64)
  returns r: U64.t
  ensures exists* s1. V.pts_to b s1 ** pure (Seq.length s1 == SZ.v sz)
{
  V.pts_to_len b;
  assert_norm (pow2 64 == 18446744073709551616);
  let mut k = 0sz;
  let mut v = C.uint64_to_uint8 (U64.rem i 251uL);
  while (!k `SZ.lt` sz)
  invariant exists* vk vv sv. R.pts_to k vk ** R.pts_to v vv ** V.pts_to b sv **
            pure (SZ.v vk <= SZ.v sz /\ Seq.length sv == SZ.v sz)
  decreases (SZ.v sz - SZ.v !k)
  {
    let vk = !k;
    let vv = !v;
    V.(b.(vk) <- vv);
    v := (if vv = 250uy then 0uy else U8.add_mod vv 1uy);
    SZ.fits_u64_implies_fits (SZ.v vk + 1);
    k := SZ.add vk 1sz
  };
  let mut j = 0sz;
  let mut sum = 0uL;
  while (!j `SZ.lt` sz)
  invariant exists* vj vsum sv. R.pts_to j vj ** R.pts_to sum vsum ** V.pts_to b sv **
            pure (SZ.v vj <= SZ.v sz /\ Seq.length sv == SZ.v sz)
  decreases (SZ.v sz - SZ.v !j)
  {
    let vj = !j;
    let x = V.(b.(vj));
    sum := U64.add_mod !sum (C.uint8_to_uint64 x);
    SZ.fits_u64_implies_fits (SZ.v vj + 1);
    j := SZ.add vj 1sz
  };
  V.pts_to_len b;
  !sum
}

fn run (n: SZ.t)
  requires pure (SZ.fits_u64 /\ SZ.v n >= 1 /\ SZ.v n <= 4000000000)
  returns r: U64.t
  ensures emp
{
  assert_norm (pow2 64 == 18446744073709551616);
  SZ.fits_u64_implies_fits 33554432;
  SZ.fits_u64_implies_fits 256;
  let wmax = SZ.uint_to_t 33554432;
  let w0 = SZ.div n 64sz;
  let w1 = (if SZ.lt 0sz w0 then w0 else 1sz);
  let w = (if SZ.lt w1 wmax then w1 else wmax);
  let mut fr : option node_ptr = None;
  let mut nf = 0sz;
  let mut bk : option node_ptr = None;
  let mut nb = 0sz;
  owned_zero None;
  owned_zero None;
  fold (qinv fr nf bk nb (SZ.v w));
  let mut ck = ck_init;
  let mut kept = 0uL;
  let mut i = 0sz;
  while (!i `SZ.lt` n)
  invariant exists* vi vck vkept. R.pts_to i vi ** R.pts_to ck vck ** R.pts_to kept vkept **
            qinv fr nf bk nb (SZ.v w) ** pure (SZ.v vi <= SZ.v n)
  decreases (SZ.v n - SZ.v !i)
  {
    let vi = !i;
    let u = SZ.sizet_to_uint64 vi;
    let r = mix64 (U64.add_mod seed u);
    let k5 = low5 r;
    let kk = SZ.uint64_to_sizet k5;
    SZ.fits_lte (SZ.v kk) 31;
    let sl = SZ.add 8sz (SZ.mul 8sz kk);
    let blk = V.alloc 0uy sl;
    let sum = fill_sum blk sl u;
    ck := ckfold !ck sum;
    if (SZ.rem vi 16sz = 0sz) {
      keep_blk fr nf bk nb w blk;
      kept := U64.add_mod !kept 1uL
    } else {
      V.free blk
    };
    SZ.fits_u64_implies_fits (SZ.v vi + 1);
    i := SZ.add vi 1sz
  };
  drain fr nf bk nb w;
  ckfold !ck !kept
}
