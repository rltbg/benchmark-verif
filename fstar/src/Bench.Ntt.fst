module Bench.Ntt
#lang-pulse
(* NTT entière modulo p = 2013265921 (voir spec/workloads.d/ntt.md) : bit-reverse, étages de papillons
   à stride variable, table de racines, produit cyclique a*a. n = 2^k, k <= 27 (sinon le résultat est 0).
   Prouvé : mémoire-sûreté (bornes de tous les indices : bit-reverse, `blk*len`, `j*stride`), absence de
   débordement (produits u32 x u32 dans u64), terminaison. La correction fonctionnelle de la transformée
   n'est pas prouvée (elle est vérifiée par un test indépendant contre la convolution naïve). *)
open Pulse.Lib.Pervasives
open Bench.Common
module V   = Pulse.Lib.Vec
module R   = Pulse.Lib.Reference
module SZ  = FStar.SizeT
module U32 = FStar.UInt32
module U64 = FStar.UInt64
module C   = FStar.Int.Cast
module ML  = FStar.Math.Lemmas

let p64 : U64.t = 2013265921uL
let seed : U64.t = 0xA5A5F00D12345678uL

(* a, b < 2^32 : a*b < 2^64, reste < p. *)
let mulmod (a b: U64.t) : Pure U64.t
  (requires U64.v a < 4294967296 /\ U64.v b < 4294967296)
  (ensures fun r -> U64.v r < 2013265921)
= ML.lemma_mult_lt_sqr (U64.v a) (U64.v b) 4294967296;
  assert_norm (4294967296 * 4294967296 == pow2 64);
  U64.rem (U64.mul a b) p64

(* b^e mod p, carré-et-multiplie. *)
let rec powmod (b e: U64.t) : Pure U64.t
  (requires U64.v b < 2013265921)
  (ensures fun r -> U64.v r < 2013265921)
  (decreases (U64.v e))
= if U64.eq e 0uL then 1uL
  else begin
    let r = powmod (mulmod b b) (U64.div e 2uL) in
    if U64.eq (U64.rem e 2uL) 1uL then mulmod r b else r
  end

let fits_lt28 (x: nat) : Lemma (requires SZ.fits_u64 /\ x <= 536870912) (ensures SZ.fits x) =
  assert_norm (pow2 64 == 18446744073709551616);
  SZ.fits_u64_implies_fits x

let rec lemma_pow2_gt (k: nat) : Lemma (ensures pow2 k > k) =
  if k > 0 then lemma_pow2_gt (k - 1)

(* blk < stride, stride*len <= n  ==>  blk*len + len <= n *)
let lemma_blk (blk stride len n: nat) : Lemma (requires blk < stride /\ stride * len <= n) (ensures blk * len + len <= n) =
  ML.lemma_mult_le_right len (blk + 1) stride;
  ML.distributivity_add_left blk 1 len

(* j < half, stride*(2*half) <= n  ==>  j*stride < n/2 + 1 *)
let lemma_tw (j stride half n: nat) : Lemma (requires j < half /\ stride * (2 * half) <= n) (ensures j * stride < n / 2 + 1) =
  ML.lemma_mult_le_right stride (j + 1) half;
  ML.distributivity_add_left j 1 stride;
  ML.swap_mul stride (2 * half);
  ML.paren_mul_right 2 half stride;
  ML.swap_mul half stride

fn fill (a: V.vec U32.t) (n: SZ.t) (#s0: Ghost.erased (Seq.seq U32.t))
  requires V.pts_to a s0 ** pure (V.length a == SZ.v n /\ SZ.fits_u64 /\ SZ.v n <= 134217728)
  ensures exists* s. V.pts_to a s ** pure (Seq.length s == SZ.v n)
{
  V.pts_to_len a;
  let mut i = 0sz;
  while (!i `SZ.lt` n)
  invariant exists* vi s. R.pts_to i vi ** V.pts_to a s ** pure (SZ.v vi <= SZ.v n /\ Seq.length s == SZ.v n)
  decreases (SZ.v n - SZ.v !i)
  {
    let vi = !i;
    let x = U64.rem (mix64 (U64.add_mod seed (SZ.sizet_to_uint64 vi))) p64;
    V.(a.(vi) <- C.uint64_to_uint32 x);
    fits_lt28 (SZ.v vi + 1);
    i := SZ.add vi 1sz
  }
}

(* tw[j] = w^j mod p pour j < n/2. *)
fn twiddles (tw: V.vec U32.t) (n: SZ.t) (w: U64.t) (#s0: Ghost.erased (Seq.seq U32.t))
  requires V.pts_to tw s0 ** pure (V.length tw == SZ.v n / 2 + 1 /\ U64.v w < 2013265921 /\ SZ.fits_u64 /\ SZ.v n <= 134217728)
  ensures exists* s. V.pts_to tw s ** pure (Seq.length s == SZ.v n / 2 + 1)
{
  V.pts_to_len tw;
  let h = SZ.div n 2sz;
  let mut j = 0sz;
  let mut c = 1uL;
  while (!j `SZ.lt` h)
  invariant exists* vj vc s. R.pts_to j vj ** R.pts_to c vc ** V.pts_to tw s **
            pure (SZ.v vj <= SZ.v h /\ U64.v vc < 2013265921 /\ Seq.length s == SZ.v n / 2 + 1)
  decreases (SZ.v h - SZ.v !j)
  {
    let vj = !j;
    let vc = !c;
    V.(tw.(vj) <- C.uint64_to_uint32 vc);
    c := mulmod vc w;
    fits_lt28 (SZ.v vj + 1);
    j := SZ.add vj 1sz
  }
}

(* Permutation bit-reverse sur k bits (n = 2^k). *)
fn bitrev (a: V.vec U32.t) (n k: SZ.t) (#s0: Ghost.erased (Seq.seq U32.t))
  requires V.pts_to a s0 ** pure (V.length a == SZ.v n /\ SZ.v n == pow2 (SZ.v k) /\ SZ.fits_u64 /\ SZ.v n <= 134217728)
  ensures exists* s. V.pts_to a s ** pure (Seq.length s == SZ.v n)
{
  V.pts_to_len a;
  lemma_pow2_gt (SZ.v k);
  let mut i = 0sz;
  while (!i `SZ.lt` n)
  invariant exists* vi s. R.pts_to i vi ** V.pts_to a s ** pure (SZ.v vi <= SZ.v n /\ Seq.length s == SZ.v n)
  decreases (SZ.v n - SZ.v !i)
  {
    let vi = !i;
    let mut x = vi;
    let mut rev = 0sz;
    let mut b = 0sz;
    let mut bnd = 1sz;
    while (!b `SZ.lt` k)
    invariant exists* vx vrev vb vbnd. R.pts_to x vx ** R.pts_to rev vrev ** R.pts_to b vb ** R.pts_to bnd vbnd **
              pure (SZ.v vrev < SZ.v vbnd /\ SZ.v vbnd == pow2 (SZ.v vb) /\ SZ.v vb <= SZ.v k /\ SZ.v vbnd <= SZ.v n)
    decreases (SZ.v k - SZ.v !b)
    {
      let vx = !x;
      let vrev = !rev;
      let vb = !b;
      let vbnd = !bnd;
      ML.pow2_le_compat (SZ.v k) (SZ.v vb + 1);
      assert (pure (pow2 (SZ.v vb + 1) == 2 * pow2 (SZ.v vb)));
      fits_lt28 (SZ.v vrev * 2 + SZ.v vx % 2);
      fits_lt28 (SZ.v vbnd * 2);
      fits_lt28 (SZ.v vb + 1);
      rev := SZ.add (SZ.mul vrev 2sz) (SZ.rem vx 2sz);
      x := SZ.div vx 2sz;
      bnd := SZ.mul vbnd 2sz;
      b := SZ.add vb 1sz
    };
    let vrev = !rev;
    if (SZ.lt vi vrev) {
      let t = V.(a.(vi));
      let u = V.(a.(vrev));
      V.(a.(vi) <- u);
      V.(a.(vrev) <- t)
    };
    V.pts_to_len a;
    fits_lt28 (SZ.v vi + 1);
    i := SZ.add vi 1sz
  }
}

(* Un étage : papillons de demi-longueur half (len = 2 half <= n, stride = n/len). *)
fn stage (a tw: V.vec U32.t) (n half: SZ.t) (#s0 #st: Ghost.erased (Seq.seq U32.t))
  preserves V.pts_to tw st
  requires V.pts_to a s0 **
           pure (SZ.fits_u64 /\ SZ.v n <= 134217728 /\ SZ.v half >= 1 /\ 2 * SZ.v half <= SZ.v n /\
                 V.length a == SZ.v n /\ V.length tw == SZ.v n / 2 + 1)
  ensures exists* s. V.pts_to a s ** pure (Seq.length s == SZ.v n)
{
  V.pts_to_len a;
  V.pts_to_len tw;
  fits_lt28 (2 * SZ.v half);
  let len = SZ.mul half 2sz;
  let stride = SZ.div n len;
  ML.multiply_fractions (SZ.v n) (SZ.v len);
  let mut blk = 0sz;
  while (!blk `SZ.lt` stride)
  invariant exists* vblk s. R.pts_to blk vblk ** V.pts_to a s ** pure (SZ.v vblk <= SZ.v stride /\ Seq.length s == SZ.v n)
  decreases (SZ.v stride - SZ.v !blk)
  {
    let vblk = !blk;
    lemma_blk (SZ.v vblk) (SZ.v stride) (SZ.v len) (SZ.v n);
    fits_lt28 (SZ.v vblk * SZ.v len);
    let start = SZ.mul vblk len;
    let mut j = 0sz;
    while (!j `SZ.lt` half)
    invariant exists* vj s. R.pts_to j vj ** V.pts_to a s ** pure (SZ.v vj <= SZ.v half /\ Seq.length s == SZ.v n)
    decreases (SZ.v half - SZ.v !j)
    {
      let vj = !j;
      lemma_tw (SZ.v vj) (SZ.v stride) (SZ.v half) (SZ.v n);
      fits_lt28 (SZ.v vj * SZ.v stride);
      let ti = SZ.mul vj stride;
      let i0 = SZ.add start vj;
      fits_lt28 (SZ.v start + SZ.v vj + SZ.v half);
      let i1 = SZ.add i0 half;
      let u = C.uint32_to_uint64 V.(a.(i0));
      let x = C.uint32_to_uint64 V.(a.(i1));
      let t = C.uint32_to_uint64 V.(tw.(ti));
      let v = mulmod x t;
      let s = U64.rem (U64.add u v) p64;
      let d = U64.rem (U64.sub (U64.add u p64) v) p64;
      V.(a.(i0) <- C.uint64_to_uint32 s);
      V.(a.(i1) <- C.uint64_to_uint32 d);
      V.pts_to_len a;
      fits_lt28 (SZ.v vj + 1);
      j := SZ.add vj 1sz
    };
    fits_lt28 (SZ.v vblk + 1);
    blk := SZ.add vblk 1sz
  }
}

fn transform (a tw: V.vec U32.t) (n k: SZ.t) (#s0 #st: Ghost.erased (Seq.seq U32.t))
  preserves V.pts_to tw st
  requires V.pts_to a s0 **
           pure (SZ.fits_u64 /\ SZ.v n <= 134217728 /\ SZ.v n == pow2 (SZ.v k) /\
                 V.length a == SZ.v n /\ V.length tw == SZ.v n / 2 + 1)
  ensures exists* s. V.pts_to a s ** pure (Seq.length s == SZ.v n)
{
  bitrev a n k;
  let mut half = 1sz;
  while (
    let vh = !half;
    fits_lt28 (2 * SZ.v vh);
    SZ.lte (SZ.mul vh 2sz) n
  )
  invariant exists* vh s. R.pts_to half vh ** V.pts_to a s ** pure (SZ.v vh >= 1 /\ SZ.v vh <= SZ.v n /\ Seq.length s == SZ.v n)
  decreases (SZ.v n + 1 - SZ.v !half)
  {
    let vh = !half;
    stage a tw n vh;
    fits_lt28 (2 * SZ.v vh);
    half := SZ.mul vh 2sz
  }
}

fn square (a: V.vec U32.t) (n: SZ.t) (#s0: Ghost.erased (Seq.seq U32.t))
  requires V.pts_to a s0 ** pure (V.length a == SZ.v n /\ SZ.fits_u64 /\ SZ.v n <= 134217728)
  ensures exists* s. V.pts_to a s ** pure (Seq.length s == SZ.v n)
{
  V.pts_to_len a;
  let mut i = 0sz;
  while (!i `SZ.lt` n)
  invariant exists* vi s. R.pts_to i vi ** V.pts_to a s ** pure (SZ.v vi <= SZ.v n /\ Seq.length s == SZ.v n)
  decreases (SZ.v n - SZ.v !i)
  {
    let vi = !i;
    let x = C.uint32_to_uint64 V.(a.(vi));
    V.(a.(vi) <- C.uint64_to_uint32 (mulmod x x));
    V.pts_to_len a;
    fits_lt28 (SZ.v vi + 1);
    i := SZ.add vi 1sz
  }
}

(* Renverse a[1..n-1] : échange a[i] et a[n-i] pour 1 <= i, 2i < n. *)
fn reverse (a: V.vec U32.t) (n: SZ.t) (#s0: Ghost.erased (Seq.seq U32.t))
  requires V.pts_to a s0 ** pure (V.length a == SZ.v n /\ SZ.fits_u64 /\ SZ.v n <= 134217728 /\ SZ.v n >= 1)
  ensures exists* s. V.pts_to a s ** pure (Seq.length s == SZ.v n)
{
  V.pts_to_len a;
  let mut i = 1sz;
  while (
    let vi = !i;
    fits_lt28 (2 * SZ.v vi);
    SZ.lt (SZ.mul 2sz vi) n
  )
  invariant exists* vi s. R.pts_to i vi ** V.pts_to a s ** pure (SZ.v vi >= 1 /\ 2 * SZ.v vi <= SZ.v n + 2 /\ Seq.length s == SZ.v n)
  decreases (SZ.v n - SZ.v !i)
  {
    let vi = !i;
    let j = SZ.sub n vi;
    let t = V.(a.(vi));
    let u = V.(a.(j));
    V.(a.(vi) <- u);
    V.(a.(j) <- t);
    V.pts_to_len a;
    fits_lt28 (SZ.v vi + 1);
    i := SZ.add vi 1sz
  }
}

fn checksum (a: V.vec U32.t) (n: SZ.t) (ninv: U64.t) (#s: Ghost.erased (Seq.seq U32.t))
  preserves V.pts_to a s
  requires pure (V.length a == SZ.v n /\ U64.v ninv < 2013265921 /\ SZ.fits_u64 /\ SZ.v n <= 134217728)
  returns r: U64.t
{
  V.pts_to_len a;
  let mut i = 0sz;
  let mut ck = ck_init;
  while (!i `SZ.lt` n)
  invariant exists* vi vck. R.pts_to i vi ** R.pts_to ck vck ** pure (SZ.v vi <= SZ.v n)
  decreases (SZ.v n - SZ.v !i)
  {
    let vi = !i;
    let x = C.uint32_to_uint64 V.(a.(vi));
    ck := ckfold !ck (mulmod x ninv);
    fits_lt28 (SZ.v vi + 1);
    i := SZ.add vi 1sz
  };
  !ck
}

(* k = ceil(log2 n) : plus petit k tel que 2^k >= n. *)
fn ilog2 (n: SZ.t)
  requires pure (SZ.fits_u64 /\ SZ.v n >= 1 /\ SZ.v n <= 134217728)
  returns k: SZ.t
  ensures pure (pow2 (SZ.v k) >= SZ.v n /\ pow2 (SZ.v k) <= 2 * SZ.v n /\ SZ.v k < 268435456)
{
  let mut pw = 1sz;
  let mut k = 0sz;
  while (!pw `SZ.lt` n)
  invariant exists* vpw vk. R.pts_to pw vpw ** R.pts_to k vk **
            pure (SZ.v vpw == pow2 (SZ.v vk) /\ SZ.v vpw <= 2 * SZ.v n /\ SZ.v vk < SZ.v vpw)
  decreases (2 * SZ.v n + 1 - SZ.v !pw)
  {
    let vpw = !pw;
    let vk = !k;
    assert (pure (pow2 (SZ.v vk + 1) == 2 * pow2 (SZ.v vk)));
    lemma_pow2_gt (SZ.v vk + 1);
    fits_lt28 (SZ.v vpw * 2);
    fits_lt28 (SZ.v vk + 1);
    pw := SZ.mul vpw 2sz;
    k := SZ.add vk 1sz
  };
  !k
}

(* 2^k comme size_t (2^k <= 2^28). *)
fn pow2_sz (k: SZ.t)
  requires pure (SZ.fits_u64 /\ pow2 (SZ.v k) <= 268435456 /\ SZ.v k < 268435456)
  returns r: SZ.t
  ensures pure (SZ.v r == pow2 (SZ.v k))
{
  let mut r = 1sz;
  let mut i = 0sz;
  while (!i `SZ.lt` k)
  invariant exists* vr vi. R.pts_to r vr ** R.pts_to i vi ** pure (SZ.v vr == pow2 (SZ.v vi) /\ SZ.v vi <= SZ.v k)
  decreases (SZ.v k - SZ.v !i)
  {
    let vr = !r;
    let vi = !i;
    assert (pure (pow2 (SZ.v vi + 1) == 2 * pow2 (SZ.v vi)));
    ML.pow2_le_compat (SZ.v k) (SZ.v vi + 1);
    fits_lt28 (SZ.v vr * 2);
    fits_lt28 (SZ.v vi + 1);
    r := SZ.mul vr 2sz;
    i := SZ.add vi 1sz
  };
  !r
}

fn run (n: SZ.t)
  requires pure (SZ.fits_u64)
  returns r: U64.t
  ensures emp
{
  fits_lt28 134217728;
  let lim = SZ.uint_to_t 134217728;
  if (SZ.eq n 0sz || SZ.lt lim n) {
    0uL
  } else {
    let k = ilog2 n;
    let pw = pow2_sz k;
    if (SZ.ne pw n) {
      0uL
    } else {
      fits_lt28 (SZ.v n);
      fits_lt28 (SZ.v n / 2 + 1);
      let a = V.alloc 0ul n;
      let h1 = SZ.add (SZ.div n 2sz) 1sz;
      let tw = V.alloc 0ul h1;
      fill a n;
      let w = powmod 31uL (U64.div 2013265920uL (SZ.sizet_to_uint64 n));
      twiddles tw n w;
      transform a tw n k;
      square a n;
      transform a tw n k;
      reverse a n;
      let ninv = powmod (SZ.sizet_to_uint64 n) 2013265919uL;
      let r = checksum a n ninv;
      V.free a;
      V.free tw;
      r
    }
  }
}
