module Bench.Sieve
#lang-pulse
(* Crible d'Ératosthène, 1 octet par entier. 2 <= n <= 4_000_000_000. *)
open Pulse.Lib.Pervasives
open Bench.Common
module V   = Pulse.Lib.Vec
module R   = Pulse.Lib.Reference
module SZ  = FStar.SizeT
module U8  = FStar.UInt8
module U64 = FStar.UInt64
module ML  = FStar.Math.Lemmas

(* Marque les multiples de i à partir de j (j <= n). *)
fn mark (f: V.vec U8.t) (n i j0: SZ.t) (#s0: Ghost.erased (Seq.seq U8.t))
  requires V.pts_to f s0 ** pure (V.length f == SZ.v n + 1 /\ SZ.v j0 <= SZ.v n + 1 /\ SZ.v i >= 1 /\ SZ.v i <= SZ.v n /\ SZ.fits (2 * SZ.v n + 2))
  ensures exists* s. V.pts_to f s ** pure (Seq.length s == SZ.v n + 1)
{
  V.pts_to_len f;
  let mut j = j0;
  while (!j `SZ.lte` n)
  invariant exists* vj s. R.pts_to j vj ** V.pts_to f s **
            pure (SZ.v vj <= SZ.v n + SZ.v i /\ Seq.length s == SZ.v n + 1)
  decreases (if SZ.v !j <= SZ.v n then SZ.v n + 1 - SZ.v !j else 0)
  {
    let vj = !j;
    V.(f.(vj) <- 1uy);
    SZ.fits_lte (SZ.v vj + SZ.v i) (2 * SZ.v n + 2);
    j := SZ.add vj i;
  }
}

(* Pour chaque i >= 2 tel que i*i <= n (testé sous la forme i <= n/i) et i premier, marque ses multiples. *)
fn sieve_outer (f: V.vec U8.t) (n: SZ.t) (#s0: Ghost.erased (Seq.seq U8.t))
  requires V.pts_to f s0 ** pure (V.length f == SZ.v n + 1 /\ SZ.v n >= 2 /\ SZ.fits (2 * SZ.v n + 2))
  ensures exists* s. V.pts_to f s ** pure (Seq.length s == SZ.v n + 1)
{
  V.pts_to_len f;
  let mut i = 2sz;
  while (
    let vi = !i;
    (SZ.lte vi n && SZ.lte vi (SZ.div n vi))
  )
  invariant exists* vi s. R.pts_to i vi ** V.pts_to f s **
            pure (SZ.v vi >= 2 /\ SZ.v vi <= SZ.v n + 1 /\ Seq.length s == SZ.v n + 1)
  decreases (SZ.v n + 1 - SZ.v !i)
  {
    let vi = !i;
    let fi = V.(f.(vi));
    if (fi = 0uy) {
      ML.multiply_fractions (SZ.v n) (SZ.v vi);
      ML.lemma_mult_le_left (SZ.v vi) (SZ.v vi) (SZ.v n / SZ.v vi);
      SZ.fits_lte (SZ.v vi * SZ.v vi) (SZ.v n);
      let j0 = SZ.mul vi vi;
      mark f n vi j0
    };
    V.pts_to_len f;
    SZ.fits_lte (SZ.v vi + 1) (2 * SZ.v n + 2);
    i := SZ.add vi 1sz
  }
}

(* ck = fold des premiers p (dans l'ordre), puis du nombre de premiers. *)
fn count (f: V.vec U8.t) (n: SZ.t) (#s: Ghost.erased (Seq.seq U8.t))
  preserves V.pts_to f s
  requires pure (V.length f == SZ.v n + 1 /\ SZ.v n >= 2 /\ SZ.fits (2 * SZ.v n + 2))
  returns r: U64.t
{
  V.pts_to_len f;
  let mut i = 2sz;
  let mut ck = ck_init;
  let mut cnt = 0uL;
  while (!i `SZ.lte` n)
  invariant exists* vi vck vcnt. R.pts_to i vi ** R.pts_to ck vck ** R.pts_to cnt vcnt **
            pure (SZ.v vi >= 2 /\ SZ.v vi <= SZ.v n + 1)
  decreases (SZ.v n + 1 - SZ.v !i)
  {
    let vi = !i;
    let fi = V.(f.(vi));
    if (fi = 0uy) {
      ck := ckfold !ck (SZ.sizet_to_uint64 vi);
      cnt := U64.add_mod !cnt 1uL
    };
    SZ.fits_lte (SZ.v vi + 1) (2 * SZ.v n + 2);
    i := SZ.add vi 1sz
  };
  ckfold !ck !cnt
}

fn run (n: SZ.t)
  requires pure (SZ.fits_u64 /\ SZ.v n >= 2 /\ SZ.v n <= 4000000000)
  returns r: U64.t
  ensures emp
{
  assert_norm (pow2 64 == 18446744073709551616);
  SZ.fits_u64_implies_fits (SZ.v n + 1);
  SZ.fits_u64_implies_fits (2 * SZ.v n + 2);
  let f = V.alloc 0uy (SZ.add n 1sz);
  sieve_outer f n;
  let r = count f n;
  V.free f;
  r
}
