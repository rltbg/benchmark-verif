module Bench.Matmul
#lang-pulse
(* C = A*B, matrices n x n de u32 (arithmétique modulo 2^32), boucles i, k, j. 1 <= n <= 4096. *)
open Pulse.Lib.Pervasives
open Bench.Common
module V   = Pulse.Lib.Vec
module R   = Pulse.Lib.Reference
module SZ  = FStar.SizeT
module U32 = FStar.UInt32
module U64 = FStar.UInt64
module ML  = FStar.Math.Lemmas

let seed : U64.t = 0x0FEDCBA987654321uL

let lemma_idx (i j n: nat) : Lemma (requires i < n /\ j < n) (ensures i * n + j < n * n) =
  ML.distributivity_add_left i 1 n;
  ML.lemma_mult_le_right n (i + 1) n

let lemma_nn (n: nat) : Lemma (requires n <= 4096) (ensures n * n <= 16777216) =
  ML.lemma_mult_le_right n n 4096;
  ML.lemma_mult_le_left 4096 n 4096

(* Indice i*n+j, provablement < n*n et sans débordement de size_t. *)
let idx (n i j: SZ.t) : Pure SZ.t
  (requires SZ.fits_u64 /\ SZ.v n <= 4096 /\ SZ.v i < SZ.v n /\ SZ.v j < SZ.v n)
  (ensures fun r -> SZ.v r == SZ.v i * SZ.v n + SZ.v j /\ SZ.v r < SZ.v n * SZ.v n)
= lemma_idx (SZ.v i) (SZ.v j) (SZ.v n);
  lemma_nn (SZ.v n);
  assert_norm (pow2 64 == 18446744073709551616);
  SZ.fits_u64_implies_fits (SZ.v i * SZ.v n);
  SZ.fits_u64_implies_fits (SZ.v i * SZ.v n + SZ.v j);
  SZ.add (SZ.mul i n) j

(* m[i] = hi32(mix64(seed + off + i)) pour i < len. *)
fn init_mat (m: V.vec U32.t) (len: SZ.t) (off: U64.t) (#s0: Ghost.erased (Seq.seq U32.t))
  requires V.pts_to m s0 ** pure (V.length m == SZ.v len)
  ensures exists* s. V.pts_to m s ** pure (Seq.length s == SZ.v len)
{
  V.pts_to_len m;
  let mut i = 0sz;
  while (!i `SZ.lt` len)
  invariant exists* vi s. R.pts_to i vi ** V.pts_to m s ** pure (SZ.v vi <= SZ.v len /\ Seq.length s == SZ.v len)
  decreases (SZ.v len - SZ.v !i)
  {
    let vi = !i;
    V.(m.(vi) <- hi32 (mix64 (U64.add_mod (U64.add_mod seed off) (SZ.sizet_to_uint64 vi))));
    i := SZ.add vi 1sz
  }
}

fn loop_j (a b c: V.vec U32.t) (n i k: SZ.t) (aik: U32.t)
          (#sa #sb #sc: Ghost.erased (Seq.seq U32.t))
  preserves V.pts_to a sa ** V.pts_to b sb
  requires V.pts_to c sc **
           pure (SZ.fits_u64 /\ SZ.v n <= 4096 /\ SZ.v i < SZ.v n /\ SZ.v k < SZ.v n /\
                 V.length a == SZ.v n * SZ.v n /\ V.length b == SZ.v n * SZ.v n /\ V.length c == SZ.v n * SZ.v n)
  ensures exists* sc'. V.pts_to c sc' ** pure (Seq.length sc' == SZ.v n * SZ.v n)
{
  V.pts_to_len a;
  V.pts_to_len b;
  V.pts_to_len c;
  let mut j = 0sz;
  while (!j `SZ.lt` n)
  invariant exists* vj sc1. R.pts_to j vj ** V.pts_to c sc1 **
            pure (SZ.v vj <= SZ.v n /\ Seq.length sc1 == SZ.v n * SZ.v n)
  decreases (SZ.v n - SZ.v !j)
  {
    let vj = !j;
    let ci = idx n i vj;
    let bi = idx n k vj;
    let bv = V.(b.(bi));
    let cv = V.(c.(ci));
    V.(c.(ci) <- U32.add_mod cv (U32.mul_mod aik bv));
    fits_small (SZ.v vj + 1);
    j := SZ.add vj 1sz
  }
}

fn loop_k (a b c: V.vec U32.t) (n i: SZ.t)
          (#sa #sb #sc: Ghost.erased (Seq.seq U32.t))
  preserves V.pts_to a sa ** V.pts_to b sb
  requires V.pts_to c sc **
           pure (SZ.fits_u64 /\ SZ.v n <= 4096 /\ SZ.v i < SZ.v n /\
                 V.length a == SZ.v n * SZ.v n /\ V.length b == SZ.v n * SZ.v n /\ V.length c == SZ.v n * SZ.v n)
  ensures exists* sc'. V.pts_to c sc' ** pure (Seq.length sc' == SZ.v n * SZ.v n)
{
  V.pts_to_len a;
  V.pts_to_len c;
  let mut k = 0sz;
  while (!k `SZ.lt` n)
  invariant exists* vk sc1. R.pts_to k vk ** V.pts_to c sc1 **
            pure (SZ.v vk <= SZ.v n /\ Seq.length sc1 == SZ.v n * SZ.v n)
  decreases (SZ.v n - SZ.v !k)
  {
    let vk = !k;
    let aik = V.(a.(idx n i vk));
    loop_j a b c n i vk aik;
    fits_small (SZ.v vk + 1);
    k := SZ.add vk 1sz
  }
}

fn loop_i (a b c: V.vec U32.t) (n: SZ.t)
          (#sa #sb #sc: Ghost.erased (Seq.seq U32.t))
  preserves V.pts_to a sa ** V.pts_to b sb
  requires V.pts_to c sc **
           pure (SZ.fits_u64 /\ SZ.v n <= 4096 /\
                 V.length a == SZ.v n * SZ.v n /\ V.length b == SZ.v n * SZ.v n /\ V.length c == SZ.v n * SZ.v n)
  ensures exists* sc'. V.pts_to c sc' ** pure (Seq.length sc' == SZ.v n * SZ.v n)
{
  V.pts_to_len c;
  let mut i = 0sz;
  while (!i `SZ.lt` n)
  invariant exists* vi sc1. R.pts_to i vi ** V.pts_to c sc1 **
            pure (SZ.v vi <= SZ.v n /\ Seq.length sc1 == SZ.v n * SZ.v n)
  decreases (SZ.v n - SZ.v !i)
  {
    let vi = !i;
    loop_k a b c n vi;
    fits_small (SZ.v vi + 1);
    i := SZ.add vi 1sz
  }
}

fn checksum (c: V.vec U32.t) (nn: SZ.t) (#s: Ghost.erased (Seq.seq U32.t))
  preserves V.pts_to c s
  requires pure (V.length c == SZ.v nn /\ SZ.fits (SZ.v nn + 1))
  returns r: U64.t
{
  V.pts_to_len c;
  let mut i = 0sz;
  let mut ck = ck_init;
  while (!i `SZ.lt` nn)
  invariant exists* vi vck. R.pts_to i vi ** R.pts_to ck vck ** pure (SZ.v vi <= SZ.v nn)
  decreases (SZ.v nn - SZ.v !i)
  {
    let vi = !i;
    let x = V.(c.(vi));
    ck := ckfold !ck (FStar.Int.Cast.uint32_to_uint64 x);
    SZ.fits_lte (SZ.v vi + 1) (SZ.v nn + 1);
    i := SZ.add vi 1sz
  };
  !ck
}

fn run (n: SZ.t)
  requires pure (SZ.fits_u64 /\ SZ.v n >= 1 /\ SZ.v n <= 4096)
  returns r: U64.t
  ensures emp
{
  assert_norm (pow2 64 == 18446744073709551616);
  lemma_nn (SZ.v n);
  SZ.fits_u64_implies_fits (SZ.v n * SZ.v n);
  SZ.fits_u64_implies_fits (SZ.v n * SZ.v n + 1);
  let nn = SZ.mul n n;
  let a = V.alloc 0ul nn;
  let b = V.alloc 0ul nn;
  let c = V.alloc 0ul nn;
  init_mat a nn 0uL;
  init_mat b nn (SZ.sizet_to_uint64 nn);
  loop_i a b c n;
  let r = checksum c nn;
  V.free a;
  V.free b;
  V.free c;
  r
}
