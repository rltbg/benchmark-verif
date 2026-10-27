module Bench.Strsort
#lang-pulse
(* Tri fusion stable (descendant, tampon + recopie) d'un tableau d'indices u32 de chaînes de longueur variable (1..32),
   stockées dans un pool d'octets contigu avec des offsets u64 (n+1 entrées). 1 <= n <= 2^28.
   Prouvé : mémoire sûre (indices et plages du pool, en particulier offset+k < taille du pool dans la comparaison), absence de
   débordement non voulu, terminaison. Pas de correction fonctionnelle (le tri n'est pas spécifié) ; les gardes runtime
   (`ia < n`, `offset+len <= taille du pool`) remplacent les invariants de forme entre tableaux. *)
open Pulse.Lib.Pervasives
open Bench.Common
module V   = Pulse.Lib.Vec
module R   = Pulse.Lib.Reference
module SZ  = FStar.SizeT
module U8  = FStar.UInt8
module U32 = FStar.UInt32
module U64 = FStar.UInt64
module C   = FStar.Int.Cast
module ML  = FStar.Math.Lemmas

#set-options "--z3rlimit 60"

let seed_len : U64.t = 0xC0FFEE0000000001uL
let seed_chr : U64.t = 0xC0FFEE0000000002uL

(* Longueur de la chaîne i : 1 + (mix64(seed_len + i) >> 59), dans 1..32. *)
let len_of (i: U64.t) : Pure U64.t (requires True) (ensures fun r -> 1 <= U64.v r /\ U64.v r <= 32) =
  let x = mix64 (U64.add_mod seed_len i) in
  FStar.UInt.shift_right_value_lemma #64 (U64.v x) 59;
  ML.lemma_div_lt (U64.v x) 64 59;
  assert_norm (pow2 5 == 32);
  U64.add 1uL (U64.shift_right x 59ul)

(* Octet k de la chaîne i : 'a' + (mix64(seed_chr + 64 i + k) mod 4). *)
let chr_of (i k: U64.t) : U8.t =
  let x = mix64 (U64.add_mod (U64.add_mod seed_chr (U64.mul_mod 64uL i)) k) in
  U8.add_mod 97uy (C.uint64_to_uint8 (U64.logand x 3uL))

(* off[i+1] = off[i] + len_of i ; renvoie le total (= off[n]) <= 32 n. *)
fn gen_offsets (off: V.vec U64.t) (n: SZ.t) (#s0: Ghost.erased (Seq.seq U64.t))
  requires V.pts_to off s0 ** pure (SZ.fits_u64 /\ V.length off == SZ.v n + 1 /\ SZ.v n <= 268435456 /\ SZ.fits (SZ.v n + 2))
  returns tot: U64.t
  ensures exists* s. V.pts_to off s ** pure (Seq.length s == SZ.v n + 1 /\ U64.v tot <= 32 * SZ.v n)
{
  V.pts_to_len off;
  let mut i = 0sz;
  let mut acc = 0uL;
  while (!i `SZ.lt` n)
  invariant exists* vi vacc s. R.pts_to i vi ** R.pts_to acc vacc ** V.pts_to off s **
            pure (SZ.v vi <= SZ.v n /\ Seq.length s == SZ.v n + 1 /\ U64.v vacc <= 32 * SZ.v vi)
  decreases (SZ.v n - SZ.v !i)
  {
    let vi = !i;
    let vacc = !acc;
    let l = len_of (SZ.sizet_to_uint64 vi);
    let nacc = U64.add vacc l;
    SZ.fits_lte (SZ.v vi + 1) (SZ.v n + 2);
    V.(off.(SZ.add vi 1sz) <- nacc);
    acc := nacc;
    i := SZ.add vi 1sz
  };
  !acc
}

(* Remplit le pool : octets de la chaîne i à partir de la position cumulée ; garde pos+k < plen. *)
fn gen_pool (pool: V.vec U8.t) (n plen: SZ.t) (#s0: Ghost.erased (Seq.seq U8.t))
  requires V.pts_to pool s0 ** pure (SZ.fits_u64 /\ V.length pool == SZ.v plen /\ SZ.v n <= 268435456 /\ SZ.fits (SZ.v n + 2))
  ensures exists* s. V.pts_to pool s ** pure (Seq.length s == SZ.v plen)
{
  V.pts_to_len pool;
  let pl = SZ.sizet_to_uint64 plen;
  let mut i = 0sz;
  let mut pos = 0uL;
  while (!i `SZ.lt` n)
  invariant exists* vi vpos s. R.pts_to i vi ** R.pts_to pos vpos ** V.pts_to pool s **
            pure (SZ.v vi <= SZ.v n /\ Seq.length s == SZ.v plen)
  decreases (SZ.v n - SZ.v !i)
  {
    let vi = !i;
    let vpos = !pos;
    let iu = SZ.sizet_to_uint64 vi;
    let l = len_of iu;
    let mut k = 0uL;
    while (!k `U64.lt` l)
    invariant exists* vk s. R.pts_to k vk ** V.pts_to pool s ** pure (U64.v vk <= U64.v l /\ Seq.length s == SZ.v plen)
    decreases (U64.v l - U64.v !k)
    {
      let vk = !k;
      let p = U64.add_mod vpos vk;
      V.pts_to_len pool;
      if (U64.lt p pl) {
        V.(pool.(SZ.uint64_to_sizet p) <- chr_of iu vk)
      };
      V.pts_to_len pool;
      k := U64.add vk 1uL
    };
    pos := U64.add_mod vpos l;
    SZ.fits_lte (SZ.v vi + 1) (SZ.v n + 2);
    i := SZ.add vi 1sz
  }
}

(* Vrai si (offset, longueur) tient dans le pool. *)
let fits_pool (o l pl: U64.t) : bool = U64.lte o pl && U64.lte l (U64.sub_mod pl o)

(* a <= b pour l'ordre (octets lexicographiques, puis longueur) ; ia, ib hors bornes remplacés par 0. *)
fn lex_le (pool: V.vec U8.t) (off: V.vec U64.t) (n plen: SZ.t) (a b: U32.t)
          (#sp: Ghost.erased (Seq.seq U8.t)) (#so: Ghost.erased (Seq.seq U64.t))
  preserves V.pts_to pool sp ** V.pts_to off so
  requires pure (SZ.fits_u64 /\ SZ.v n >= 1 /\ Seq.length sp == SZ.v plen /\ Seq.length so == SZ.v n + 1 /\
                 SZ.fits (SZ.v n + 2) /\ SZ.fits (SZ.v plen + 1))
  returns r: bool
{
  SZ.fits_u64_implies_fits_32 ();
  let ia0 = SZ.uint32_to_sizet a;
  let ib0 = SZ.uint32_to_sizet b;
  let ia = (if SZ.lt ia0 n then ia0 else 0sz);
  let ib = (if SZ.lt ib0 n then ib0 else 0sz);
  SZ.fits_lte (SZ.v ia + 1) (SZ.v n + 2);
  SZ.fits_lte (SZ.v ib + 1) (SZ.v n + 2);
  let oa = V.(off.(ia));
  let oa1 = V.(off.(SZ.add ia 1sz));
  let ob = V.(off.(ib));
  let ob1 = V.(off.(SZ.add ib 1sz));
  let pl = SZ.sizet_to_uint64 plen;
  let la_raw = U64.sub_mod oa1 oa;
  let lb_raw = U64.sub_mod ob1 ob;
  let oka = fits_pool oa la_raw pl;
  let okb = fits_pool ob lb_raw pl;
  let oaa = (if oka then oa else 0uL);
  let la = (if oka then la_raw else 0uL);
  let obb = (if okb then ob else 0uL);
  let lb = (if okb then lb_raw else 0uL);
  let ma = (if U64.lte la lb then la else lb);
  assert (pure (U64.v oaa + U64.v la <= U64.v pl /\ U64.v obb + U64.v lb <= U64.v pl));
  assert (pure (U64.v ma <= U64.v la /\ U64.v ma <= U64.v lb));
  let sa = SZ.uint64_to_sizet oaa;
  let sb = SZ.uint64_to_sizet obb;
  let m = SZ.uint64_to_sizet ma;
  let mut k = 0sz;
  let mut go = true;
  let mut res = false;
  while (
    let vk = !k;
    let vg = !go;
    (vg && SZ.lt vk m)
  )
  invariant exists* vk vg vr. R.pts_to k vk ** R.pts_to go vg ** R.pts_to res vr ** pure (SZ.v vk <= SZ.v m)
  decreases (if !go then SZ.v m + 1 - SZ.v !k else 0)
  {
    let vk = !k;
    let vr = !res;
    SZ.fits_lte (SZ.v sa + SZ.v vk) (SZ.v plen + 1);
    SZ.fits_lte (SZ.v sb + SZ.v vk) (SZ.v plen + 1);
    SZ.fits_lte (SZ.v vk + 1) (SZ.v plen + 1);
    let ca = V.(pool.(SZ.add sa vk));
    let cb = V.(pool.(SZ.add sb vk));
    let neq = (U8.lt ca cb) || (U8.gt ca cb);
    res := (if neq then U8.lt ca cb else vr);
    go := (not neq);
    k := SZ.add vk 1sz
  };
  let vg = !go;
  if vg { res := U64.lte la lb };
  !res
}

(* Fusionne src[lo,mid) et src[mid,hi) (déjà « triés ») dans dst[lo,hi), stable : à égalité on prend la gauche. *)
fn merge (pool: V.vec U8.t) (off: V.vec U64.t) (n plen: SZ.t) (src dst: V.vec U32.t) (lo mid hi: SZ.t)
         (#sp: Ghost.erased (Seq.seq U8.t)) (#so: Ghost.erased (Seq.seq U64.t))
         (#ss: Ghost.erased (Seq.seq U32.t)) (#sd: Ghost.erased (Seq.seq U32.t))
  preserves V.pts_to pool sp ** V.pts_to off so ** V.pts_to src ss
  requires V.pts_to dst sd **
           pure (SZ.fits_u64 /\ SZ.v n >= 1 /\ Seq.length sp == SZ.v plen /\ Seq.length so == SZ.v n + 1 /\
                 SZ.fits (SZ.v n + 2) /\ SZ.fits (SZ.v plen + 1) /\
                 Seq.length ss == Seq.length sd /\ SZ.v lo <= SZ.v mid /\ SZ.v mid <= SZ.v hi /\ SZ.v hi <= Seq.length ss /\
                 SZ.fits (Seq.length ss + 1))
  ensures exists* sd'. V.pts_to dst sd' ** pure (Seq.length sd' == Seq.length sd)
{
  V.pts_to_len src;
  V.pts_to_len dst;
  let mut i = lo;
  let mut j = mid;
  let mut k = lo;
  while (!k `SZ.lt` hi)
  invariant exists* vi vj vk sd1. R.pts_to i vi ** R.pts_to j vj ** R.pts_to k vk ** V.pts_to dst sd1 **
            pure (Seq.length sd1 == Seq.length sd /\
                  SZ.v lo <= SZ.v vi /\ SZ.v vi <= SZ.v mid /\ SZ.v mid <= SZ.v vj /\ SZ.v vj <= SZ.v hi /\
                  SZ.v vk + SZ.v mid == SZ.v vi + SZ.v vj /\ SZ.v vk <= SZ.v hi)
  decreases (SZ.v hi - SZ.v !k)
  {
    let vi = !i;
    let vj = !j;
    let vk = !k;
    let x = if (vi `SZ.lt` mid) { V.(src.(vi)) } else { 0ul };
    let y = if (vj `SZ.lt` hi) { V.(src.(vj)) } else { 0ul };
    let le = lex_le pool off n plen x y;
    let take_left = (vi `SZ.lt` mid) && ((not (vj `SZ.lt` hi)) || le);
    assert (pure (take_left ==> SZ.v vi < SZ.v mid));
    assert (pure (~take_left ==> SZ.v vj < SZ.v hi));
    V.(dst.(vk) <- (if take_left then x else y));
    SZ.fits_lte (SZ.v vi + 1) (Seq.length ss + 1);
    SZ.fits_lte (SZ.v vj + 1) (Seq.length ss + 1);
    SZ.fits_lte (SZ.v vk + 1) (Seq.length ss + 1);
    i := (if take_left then SZ.add vi 1sz else vi);
    j := (if take_left then vj else SZ.add vj 1sz);
    k := SZ.add vk 1sz
  };
  V.pts_to_len dst
}

(* a[lo,hi) <- tmp[lo,hi). *)
fn copy_back (tmp a: V.vec U32.t) (lo hi: SZ.t)
             (#st: Ghost.erased (Seq.seq U32.t)) (#sa: Ghost.erased (Seq.seq U32.t))
  preserves V.pts_to tmp st
  requires V.pts_to a sa **
           pure (Seq.length st == Seq.length sa /\ SZ.v lo <= SZ.v hi /\ SZ.v hi <= Seq.length sa /\
                 SZ.fits (Seq.length sa + 1))
  ensures exists* sa'. V.pts_to a sa' ** pure (Seq.length sa' == Seq.length sa)
{
  V.pts_to_len tmp;
  V.pts_to_len a;
  let mut i = lo;
  while (!i `SZ.lt` hi)
  invariant exists* vi sa1. R.pts_to i vi ** V.pts_to a sa1 **
            pure (SZ.v lo <= SZ.v vi /\ SZ.v vi <= SZ.v hi /\ Seq.length sa1 == Seq.length sa)
  decreases (SZ.v hi - SZ.v !i)
  {
    let vi = !i;
    let x = V.(tmp.(vi));
    V.(a.(vi) <- x);
    SZ.fits_lte (SZ.v vi + 1) (Seq.length sa + 1);
    i := SZ.add vi 1sz
  };
  V.pts_to_len a
}

fn rec msort (pool: V.vec U8.t) (off: V.vec U64.t) (n plen: SZ.t) (a tmp: V.vec U32.t) (lo hi: SZ.t)
             (#sp: Ghost.erased (Seq.seq U8.t)) (#so: Ghost.erased (Seq.seq U64.t))
             (#sa: Ghost.erased (Seq.seq U32.t)) (#st: Ghost.erased (Seq.seq U32.t))
  preserves V.pts_to pool sp ** V.pts_to off so
  requires V.pts_to a sa ** V.pts_to tmp st **
           pure (SZ.fits_u64 /\ SZ.v n >= 1 /\ Seq.length sp == SZ.v plen /\ Seq.length so == SZ.v n + 1 /\
                 SZ.fits (SZ.v n + 2) /\ SZ.fits (SZ.v plen + 1) /\
                 Seq.length sa == Seq.length st /\ SZ.v lo <= SZ.v hi /\ SZ.v hi <= Seq.length sa /\
                 SZ.fits (Seq.length sa + 1))
  ensures exists* sa' st'. V.pts_to a sa' ** V.pts_to tmp st' **
          pure (Seq.length sa' == Seq.length sa /\ Seq.length st' == Seq.length st)
  decreases (SZ.v hi - SZ.v lo)
{
  if (SZ.lt (SZ.sub hi lo) 2sz) {
    V.pts_to_len a
  } else {
    let mid = SZ.add lo (SZ.div (SZ.sub hi lo) 2sz);
    msort pool off n plen a tmp lo mid;
    msort pool off n plen a tmp mid hi;
    merge pool off n plen a tmp lo mid hi;
    copy_back tmp a lo hi;
    V.pts_to_len a
  }
}

fn init_idx (v: V.vec U32.t) (n: SZ.t) (#s0: Ghost.erased (Seq.seq U32.t))
  requires V.pts_to v s0 ** pure (V.length v == SZ.v n /\ SZ.fits_u64 /\ SZ.fits (SZ.v n + 1))
  ensures exists* s. V.pts_to v s ** pure (Seq.length s == SZ.v n)
{
  V.pts_to_len v;
  let mut i = 0sz;
  while (!i `SZ.lt` n)
  invariant exists* vi s. R.pts_to i vi ** V.pts_to v s ** pure (SZ.v vi <= SZ.v n /\ Seq.length s == SZ.v n)
  decreases (SZ.v n - SZ.v !i)
  {
    let vi = !i;
    V.(v.(vi) <- SZ.sizet_to_uint32 vi);
    SZ.fits_lte (SZ.v vi + 1) (SZ.v n + 1);
    i := SZ.add vi 1sz
  }
}

(* ckfold des indices dans l'ordre trié, puis de la somme des longueurs. *)
fn checksum (v: V.vec U32.t) (n: SZ.t) (tot: U64.t) (#s: Ghost.erased (Seq.seq U32.t))
  preserves V.pts_to v s
  requires pure (V.length v == SZ.v n /\ SZ.fits (SZ.v n + 1))
  returns r: U64.t
{
  V.pts_to_len v;
  let mut i = 0sz;
  let mut ck = ck_init;
  while (!i `SZ.lt` n)
  invariant exists* vi vck. R.pts_to i vi ** R.pts_to ck vck ** pure (SZ.v vi <= SZ.v n)
  decreases (SZ.v n - SZ.v !i)
  {
    let vi = !i;
    let x = V.(v.(vi));
    ck := ckfold !ck (C.uint32_to_uint64 x);
    SZ.fits_lte (SZ.v vi + 1) (SZ.v n + 1);
    i := SZ.add vi 1sz
  };
  ckfold !ck tot
}

fn run (n: SZ.t)
  requires pure (SZ.fits_u64 /\ SZ.v n >= 1 /\ SZ.v n <= 268435456)
  returns r: U64.t
  ensures emp
{
  assert_norm (pow2 64 == 18446744073709551616);
  SZ.fits_u64_implies_fits (SZ.v n + 2);
  SZ.fits_lte (SZ.v n + 1) (SZ.v n + 2);
  let off = V.alloc 0uL (SZ.add n 1sz);
  let tot = gen_offsets off n;
  SZ.fits_u64_implies_fits (U64.v tot + 1);
  let plen = SZ.uint64_to_sizet tot;
  let pool = V.alloc 0uy plen;
  gen_pool pool n plen;
  let a = V.alloc 0ul n;
  let tmp = V.alloc 0ul n;
  init_idx a n;
  V.pts_to_len off;
  V.pts_to_len pool;
  V.pts_to_len a;
  V.pts_to_len tmp;
  msort pool off n plen a tmp 0sz n;
  V.pts_to_len a;
  let ck = checksum a n tot;
  V.free a;
  V.free tmp;
  V.free pool;
  V.free off;
  ck
}
