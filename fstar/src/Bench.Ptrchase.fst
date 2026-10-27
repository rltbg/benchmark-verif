module Bench.Ptrchase
#lang-pulse
(* ptrchase : arène par indices (2n u64 : clé ; liens gauche|droite<<32), BST non équilibré,
   recherches, puis chasse de pointeurs sur une permutation de Sattolo (voir spec/workloads.d/ptrchase.md).
   1 <= n <= 2^31 - 2. Prouvé : mémoire-sûreté, bornes, terminaison. Les liens lus dans l'arène sont
   gardés (`c < cnt`, `v < n`) : pas d'invariant quantifié sur l'arène. *)
open Pulse.Lib.Pervasives
open Bench.Common
module V   = Pulse.Lib.Vec
module R   = Pulse.Lib.Reference
module SZ  = FStar.SizeT
module U32 = FStar.UInt32
module U64 = FStar.UInt64

#set-options "--z3rlimit 60"

let seed1 : U64.t = 0xD00D000000000001uL
let seed2 : U64.t = 0xD00D000000000002uL
let seed3 : U64.t = 0xD00D000000000003uL
let nilnil : U64.t = 0xFFFFFFFFFFFFFFFFuL

(* size_t -> u64 sans perte (valeurs < 2^32). *)
let s2u (x: SZ.t) : Pure U64.t (requires SZ.v x < 4294967296) (ensures fun y -> U64.v y == SZ.v x) =
  assert_norm (pow2 64 == 18446744073709551616);
  SZ.sizet_to_uint64 x

(* Pré-condition commune : n tient dans un u32 et 2n+2 dans un size_t. *)
let ok_n (n: SZ.t) : prop =
  SZ.fits_u64 /\ 1 <= SZ.v n /\ SZ.v n <= 2147483646 /\ SZ.fits (2 * SZ.v n + 2)

(* Insère `nk` ; cnt (>= 1, < n) est lu/écrit dans cntr. Renvoie le nombre de nœuds visités. *)
fn insert_one (a: V.vec U64.t) (n: SZ.t) (cntr: R.ref SZ.t) (nk: U64.t)
              (#s0: Ghost.erased (Seq.seq U64.t)) (#c0: Ghost.erased SZ.t)
  requires V.pts_to a s0 ** R.pts_to cntr c0 **
           pure (ok_n n /\ V.length a == 2 * SZ.v n /\ 1 <= SZ.v c0 /\ SZ.v c0 < SZ.v n)
  returns r: SZ.t
  ensures exists* s c. V.pts_to a s ** R.pts_to cntr c **
          pure (Seq.length s == 2 * SZ.v n /\ SZ.v c0 <= SZ.v c /\ SZ.v c <= SZ.v c0 + 1 /\ SZ.v r <= SZ.v n)
{
  V.pts_to_len a;
  let mut cur = 0sz;
  let mut v = 0sz;
  let mut fin = false;
  while (
    let vf = !fin;
    let vv = !v;
    let vc = !cntr;
    ((not vf) && (vv `SZ.lt` vc))
  )
  invariant exists* vcur vv vf vc sv. R.pts_to cur vcur ** R.pts_to v vv ** R.pts_to fin vf ** R.pts_to cntr vc **
            V.pts_to a sv **
            pure (SZ.v vcur < SZ.v vc /\ SZ.v vv <= SZ.v vc /\ SZ.v c0 <= SZ.v vc /\ SZ.v vc <= SZ.v c0 + 1 /\
                  (vf \/ SZ.v vc == SZ.v c0) /\ Seq.length sv == 2 * SZ.v n)
  decreases (SZ.v n - SZ.v !v)
  {
    let vcur = !cur;
    let vv = !v;
    let vc = !cntr;
    let vf = !fin;
    SZ.fits_lte (SZ.v vcur + SZ.v vcur) (2 * SZ.v n + 2);
    SZ.fits_lte (SZ.v vcur + SZ.v vcur + 1) (2 * SZ.v n + 2);
    SZ.fits_lte (SZ.v vv + 1) (2 * SZ.v n + 2);
    SZ.fits_lte (SZ.v vc + 1) (2 * SZ.v n + 2);
    SZ.fits_lte (SZ.v vc + SZ.v vc) (2 * SZ.v n + 2);
    SZ.fits_lte (SZ.v vc + SZ.v vc + 1) (2 * SZ.v n + 2);
    let i2 = SZ.add vcur vcur;
    let key = V.(a.(i2));
    let lnk = V.(a.(SZ.add i2 1sz));
    let eq = (nk = key);
    let left = U64.lt nk key;
    let c = (if left then U64.logand lnk 0xFFFFFFFFuL else U64.shift_right lnk 32ul);
    let vc64 = s2u vc;
    let inr = ((not eq) && U64.lt c vc64);
    let attach = ((not vf) && (not eq) && (not (U64.lt c vc64)));
    if attach {
      let ic = SZ.add vc vc;
      V.(a.(ic) <- nk);
      V.(a.(SZ.add ic 1sz) <- nilnil);
      V.(a.(SZ.add i2 1sz) <-
           (if left then U64.logor (U64.logand lnk 0xFFFFFFFF00000000uL) vc64
                    else U64.logor (U64.logand lnk 0xFFFFFFFFuL) (U64.shift_left vc64 32ul)))
    };
    V.pts_to_len a;
    cur := (if inr then SZ.uint64_to_sizet c else vcur);
    cntr := (if attach then SZ.add vc 1sz else vc);
    v := SZ.add vv 1sz;
    fin := (not inr)
  };
  !v
}

(* n insertions ; renvoie la somme des longueurs de chemin ; cnt final dans cntr. *)
fn build (a: V.vec U64.t) (n: SZ.t) (cntr: R.ref SZ.t) (#s0: Ghost.erased (Seq.seq U64.t)) (#c0: Ghost.erased SZ.t)
  requires V.pts_to a s0 ** R.pts_to cntr c0 ** pure (ok_n n /\ V.length a == 2 * SZ.v n)
  returns plen: U64.t
  ensures exists* s c. V.pts_to a s ** R.pts_to cntr c **
          pure (Seq.length s == 2 * SZ.v n /\ 1 <= SZ.v c /\ SZ.v c <= SZ.v n)
{
  V.pts_to_len a;
  V.(a.(0sz) <- U64.logor (mix64 seed1) 1uL);
  V.(a.(1sz) <- nilnil);
  cntr := 1sz;
  let mut i = 1sz;
  let mut plen = 0uL;
  while (!i `SZ.lt` n)
  invariant exists* vi vp vc s. R.pts_to i vi ** R.pts_to plen vp ** R.pts_to cntr vc ** V.pts_to a s **
            pure (1 <= SZ.v vi /\ SZ.v vi <= SZ.v n /\ 1 <= SZ.v vc /\ SZ.v vc <= SZ.v vi /\ Seq.length s == 2 * SZ.v n)
  decreases (SZ.v n - SZ.v !i)
  {
    let vi = !i;
    let nk = U64.logor (mix64 (U64.add_mod seed1 (s2u vi))) 1uL;
    let v = insert_one a n cntr nk;
    plen := U64.add_mod !plen (s2u v);
    SZ.fits_lte (SZ.v vi + 1) (2 * SZ.v n + 2);
    i := SZ.add vi 1sz
  };
  !plen
}

(* Une recherche : (visites << 1) | trouvée. *)
fn search (a: V.vec U64.t) (n cnt: SZ.t) (key: U64.t) (#s: Ghost.erased (Seq.seq U64.t))
  preserves V.pts_to a s
  requires pure (ok_n n /\ V.length a == 2 * SZ.v n /\ 1 <= SZ.v cnt /\ SZ.v cnt <= SZ.v n)
  returns r: U64.t
{
  V.pts_to_len a;
  let mut cur = 0sz;
  let mut v = 0sz;
  let mut found = 0uL;
  let mut fin = false;
  while (
    let vf = !fin;
    let vv = !v;
    ((not vf) && (vv `SZ.lt` cnt))
  )
  invariant exists* vcur vv vf vfound. R.pts_to cur vcur ** R.pts_to v vv ** R.pts_to fin vf ** R.pts_to found vfound **
            pure (SZ.v vcur < SZ.v cnt /\ SZ.v vv <= SZ.v cnt)
  decreases (SZ.v cnt - SZ.v !v)
  {
    let vcur = !cur;
    let vv = !v;
    SZ.fits_lte (SZ.v vcur + SZ.v vcur) (2 * SZ.v n + 2);
    SZ.fits_lte (SZ.v vcur + SZ.v vcur + 1) (2 * SZ.v n + 2);
    SZ.fits_lte (SZ.v vv + 1) (2 * SZ.v n + 2);
    let i2 = SZ.add vcur vcur;
    let kc = V.(a.(i2));
    let lnk = V.(a.(SZ.add i2 1sz));
    let eq = (key = kc);
    let c = (if (U64.lt key kc) then U64.logand lnk 0xFFFFFFFFuL else U64.shift_right lnk 32ul);
    let inr = ((not eq) && U64.lt c (s2u cnt));
    cur := (if inr then SZ.uint64_to_sizet c else vcur);
    v := SZ.add vv 1sz;
    found := (if eq then 1uL else 0uL);
    fin := (not inr)
  };
  let vv = !v;
  U64.logor (U64.shift_left (s2u vv) 1ul) !found
}

(* n succès puis n/2 échecs. *)
fn search_all (a: V.vec U64.t) (n cnt: SZ.t) (#s: Ghost.erased (Seq.seq U64.t))
  preserves V.pts_to a s
  requires pure (ok_n n /\ V.length a == 2 * SZ.v n /\ 1 <= SZ.v cnt /\ SZ.v cnt <= SZ.v n)
  returns r: U64.t
{
  let mut k = 0sz;
  let mut ck = ck_init;
  let n64 = s2u n;
  while (!k `SZ.lt` n)
  invariant exists* vk vck. R.pts_to k vk ** R.pts_to ck vck ** pure (SZ.v vk <= SZ.v n)
  decreases (SZ.v n - SZ.v !k)
  {
    let vk = !k;
    let j = U64.rem (U64.mul_mod 3uL (s2u vk)) n64;
    let r = search a n cnt (U64.logor (mix64 (U64.add_mod seed1 j)) 1uL);
    ck := ckfold !ck r;
    SZ.fits_lte (SZ.v vk + 1) (2 * SZ.v n + 2);
    k := SZ.add vk 1sz
  };
  let half = SZ.div n 2sz;
  let mut m = 0sz;
  while (!m `SZ.lt` half)
  invariant exists* vm vck. R.pts_to m vm ** R.pts_to ck vck ** pure (SZ.v vm <= SZ.v half)
  decreases (SZ.v half - SZ.v !m)
  {
    let vm = !m;
    let r = search a n cnt (U64.logand (mix64 (U64.add_mod seed3 (s2u vm))) 0xFFFFFFFFFFFFFFFEuL);
    ck := ckfold !ck r;
    SZ.fits_lte (SZ.v vm + 1) (2 * SZ.v n + 2);
    m := SZ.add vm 1sz
  };
  !ck
}

(* Permutation de Sattolo de [0, n) dans `nx` (longueur n). *)
fn sattolo (nx: V.vec U32.t) (n: SZ.t) (#s0: Ghost.erased (Seq.seq U32.t))
  requires V.pts_to nx s0 ** pure (ok_n n /\ V.length nx == SZ.v n)
  ensures exists* s. V.pts_to nx s ** pure (Seq.length s == SZ.v n)
{
  V.pts_to_len nx;
  let mut i = 0sz;
  while (!i `SZ.lt` n)
  invariant exists* vi s. R.pts_to i vi ** V.pts_to nx s ** pure (SZ.v vi <= SZ.v n /\ Seq.length s == SZ.v n)
  decreases (SZ.v n - SZ.v !i)
  {
    let vi = !i;
    V.(nx.(vi) <- SZ.sizet_to_uint32 vi);
    V.pts_to_len nx;
    SZ.fits_lte (SZ.v vi + 1) (2 * SZ.v n + 2);
    i := SZ.add vi 1sz
  };
  i := SZ.sub n 1sz;
  while (0sz `SZ.lt` !i)
  invariant exists* vi s. R.pts_to i vi ** V.pts_to nx s ** pure (SZ.v vi < SZ.v n /\ Seq.length s == SZ.v n)
  decreases (SZ.v !i)
  {
    let vi = !i;
    let ji = s2u vi;
    let j = SZ.uint64_to_sizet (U64.rem (mix64 (U64.add_mod seed2 ji)) ji);
    let vj = (if (j `SZ.lt` n) then j else 0sz);
    let ti = V.(nx.(vi));
    let tj = V.(nx.(vj));
    V.(nx.(vi) <- tj);
    V.(nx.(vj) <- ti);
    V.pts_to_len nx;
    i := SZ.sub vi 1sz
  }
}

(* 4n sauts ; renvoie r3. *)
fn chase (nx: V.vec U32.t) (n: SZ.t) (#s: Ghost.erased (Seq.seq U32.t))
  preserves V.pts_to nx s
  requires pure (ok_n n /\ V.length nx == SZ.v n)
  returns r: U64.t
{
  V.pts_to_len nx;
  let ntot = U64.mul_mod 4uL (s2u n);
  let mut cur = 0sz;
  let mut sum = 0uL;
  let mut h = 0uL;
  while (U64.lt !h ntot)
  invariant exists* vcur vsum vh. R.pts_to cur vcur ** R.pts_to sum vsum ** R.pts_to h vh **
            pure (SZ.v vcur < SZ.v n)
  decreases (if U64.v !h < U64.v ntot then U64.v ntot - U64.v !h else 0)
  {
    let vcur = !cur;
    SZ.fits_u64_implies_fits_32 ();
    let v = SZ.uint32_to_sizet V.(nx.(vcur));
    let nc = (if (v `SZ.lt` n) then v else vcur);
    cur := nc;
    sum := U64.add_mod !sum (s2u nc);
    h := U64.add_mod !h 1uL
  };
  let vcur = !cur;
  let vsum = !sum;
  ckfold (ckfold ck_init (s2u vcur)) vsum
}

fn run (n: SZ.t)
  requires pure (SZ.fits_u64 /\ SZ.v n >= 1 /\ SZ.v n <= 2147483646)
  returns r: U64.t
  ensures emp
{
  assert_norm (pow2 64 == 18446744073709551616);
  SZ.fits_u64_implies_fits (2 * SZ.v n + 2);
  let n2 = SZ.add n n;
  let a = V.alloc 0uL n2;
  let nx = V.alloc 0ul n;
  let mut cntr = 1sz;
  let plen = build a n cntr;
  let cnt = !cntr;
  let r1 = ckfold (ckfold ck_init (s2u cnt)) plen;
  let r2 = search_all a n cnt;
  sattolo nx n;
  let r3 = chase nx n;
  V.free a;
  V.free nx;
  ckfold (ckfold (ckfold ck_init r1) r2) r3
}
