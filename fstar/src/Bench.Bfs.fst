module Bench.Bfs
#lang-pulse
(* Graphe creux aléatoire (m = 16 n arêtes), CSR par tri par comptage, puis BFS depuis 0.
   Spécification : spec/workloads.d/bfs.md. 1 <= n <= 100_000_000.
   Prouvé : mémoire-sûreté et absence de débordement. Les invariants « offsets <= m », « sommet < n »,
   « file <= n » ne sont pas prouvés mais gardés à l'exécution (gardes toujours vraies sur ces entrées). *)
open Pulse.Lib.Pervasives
open Bench.Common
module V   = Pulse.Lib.Vec
module R   = Pulse.Lib.Reference
module SZ  = FStar.SizeT
module U32 = FStar.UInt32
module U64 = FStar.UInt64
module C   = FStar.Int.Cast

#set-options "--z3rlimit 60"

let seed : U64.t = 0xB5F0000000000001uL
let inf : U32.t = 0xFFFFFFFFul

(* Extrémité k (0 : source, 1 : cible) de l'arête j, ramenée dans [0, nn). *)
let endp (j k: U64.t) (nn: U64.t{U64.v nn <> 0}) : r: U64.t{U64.v r < U64.v nn} =
  U64.rem (mix64 (U64.add_mod seed (U64.add_mod (U64.mul_mod 2uL j) k))) nn

(* Passe 1 : off[u+1] += 1 pour chaque arête. *)
fn count_deg (off: V.vec U32.t) (n m: SZ.t) (nn: U64.t) (#s0: Ghost.erased (Seq.seq U32.t))
  requires V.pts_to off s0 **
           pure (SZ.fits_u64 /\ SZ.v n >= 1 /\ U64.v nn == SZ.v n /\ V.length off == SZ.v n + 1 /\
                 SZ.fits (SZ.v n + 1) /\ SZ.fits (SZ.v m + 1))
  ensures exists* s. V.pts_to off s ** pure (Seq.length s == SZ.v n + 1)
{
  V.pts_to_len off;
  let mut j = 0sz;
  while (!j `SZ.lt` m)
  invariant exists* vj s. R.pts_to j vj ** V.pts_to off s **
            pure (SZ.v vj <= SZ.v m /\ Seq.length s == SZ.v n + 1)
  decreases (SZ.v m - SZ.v !j)
  {
    let vj = !j;
    let u = SZ.uint64_to_sizet (endp (SZ.sizet_to_uint64 vj) 0uL nn);
    SZ.fits_lte (SZ.v u + 1) (SZ.v n + 1);
    let idx = SZ.add u 1sz;
    let c = V.(off.(idx));
    V.(off.(idx) <- U32.add_mod c 1ul);
    SZ.fits_lte (SZ.v vj + 1) (SZ.v m + 1);
    j := SZ.add vj 1sz
  }
}

(* Passe 2 : sommes préfixes (modulo 2^32). *)
fn prefix (off: V.vec U32.t) (n: SZ.t) (#s0: Ghost.erased (Seq.seq U32.t))
  requires V.pts_to off s0 ** pure (SZ.v n >= 1 /\ V.length off == SZ.v n + 1 /\ SZ.fits (SZ.v n + 1))
  ensures exists* s. V.pts_to off s ** pure (Seq.length s == SZ.v n + 1)
{
  V.pts_to_len off;
  let mut i = 1sz;
  while (!i `SZ.lte` n)
  invariant exists* vi s. R.pts_to i vi ** V.pts_to off s **
            pure (1 <= SZ.v vi /\ SZ.v vi <= SZ.v n + 1 /\ Seq.length s == SZ.v n + 1)
  decreases (SZ.v n + 1 - SZ.v !i)
  {
    let vi = !i;
    let a = V.(off.(vi));
    let b = V.(off.(SZ.sub vi 1sz));
    V.(off.(vi) <- U32.add_mod a b);
    SZ.fits_lte (SZ.v vi + 1) (SZ.v n + 1);
    i := SZ.add vi 1sz
  }
}

(* cur[i] <- off[i] pour i < n. *)
fn copy_cur (off cur: V.vec U32.t) (n: SZ.t) (#so: Ghost.erased (Seq.seq U32.t)) (#sc: Ghost.erased (Seq.seq U32.t))
  preserves V.pts_to off so
  requires V.pts_to cur sc ** pure (V.length off == SZ.v n + 1 /\ V.length cur == SZ.v n /\ SZ.fits (SZ.v n + 1))
  ensures exists* s. V.pts_to cur s ** pure (Seq.length s == SZ.v n)
{
  V.pts_to_len off;
  V.pts_to_len cur;
  let mut i = 0sz;
  while (!i `SZ.lt` n)
  invariant exists* vi s. R.pts_to i vi ** V.pts_to cur s **
            pure (SZ.v vi <= SZ.v n /\ Seq.length s == SZ.v n)
  decreases (SZ.v n - SZ.v !i)
  {
    let vi = !i;
    let x = V.(off.(vi));
    V.(cur.(vi) <- x);
    SZ.fits_lte (SZ.v vi + 1) (SZ.v n + 1);
    i := SZ.add vi 1sz
  }
}

(* Passe 3 : adj[cur[u]++] = v ; garde `cur[u] < m` (toujours vraie). *)
fn fill (cur adj: V.vec U32.t) (n m: SZ.t) (nn: U64.t) (#sc: Ghost.erased (Seq.seq U32.t)) (#sa: Ghost.erased (Seq.seq U32.t))
  requires V.pts_to cur sc ** V.pts_to adj sa **
           pure (SZ.fits_u64 /\ SZ.v n >= 1 /\ U64.v nn == SZ.v n /\ V.length cur == SZ.v n /\ V.length adj == SZ.v m /\
                 SZ.fits (SZ.v m + 1))
  ensures exists* s1 s2. V.pts_to cur s1 ** V.pts_to adj s2 ** pure (Seq.length s1 == SZ.v n /\ Seq.length s2 == SZ.v m)
{
  SZ.fits_u64_implies_fits_32 ();
  V.pts_to_len cur;
  V.pts_to_len adj;
  let mut j = 0sz;
  while (!j `SZ.lt` m)
  invariant exists* vj s1 s2. R.pts_to j vj ** V.pts_to cur s1 ** V.pts_to adj s2 **
            pure (SZ.v vj <= SZ.v m /\ Seq.length s1 == SZ.v n /\ Seq.length s2 == SZ.v m)
  decreases (SZ.v m - SZ.v !j)
  {
    let vj = !j;
    let j64 = SZ.sizet_to_uint64 vj;
    let u = SZ.uint64_to_sizet (endp j64 0uL nn);
    let v = endp j64 1uL nn;
    let p = V.(cur.(u));
    let ps = SZ.uint32_to_sizet p;
    let inb = SZ.lt ps m;
    if inb {
      V.(adj.(ps) <- C.uint64_to_uint32 v)
    };
    V.(cur.(u) <- U32.add_mod p 1ul);
    V.pts_to_len adj;
    SZ.fits_lte (SZ.v vj + 1) (SZ.v m + 1);
    j := SZ.add vj 1sz
  }
}

(* Relaxe les arêtes e0..e1 (bornées par m) du sommet courant : renvoie le nouveau `tail` (<= n). *)
fn relax (adj dist q: V.vec U32.t) (n m e0 e1 t0: SZ.t) (du: U32.t)
         (#sa: Ghost.erased (Seq.seq U32.t)) (#sd: Ghost.erased (Seq.seq U32.t)) (#sq: Ghost.erased (Seq.seq U32.t))
  preserves V.pts_to adj sa
  requires V.pts_to dist sd ** V.pts_to q sq **
           pure (SZ.fits_u64 /\ SZ.v n >= 1 /\ V.length adj == SZ.v m /\ V.length dist == SZ.v n /\ V.length q == SZ.v n /\
                 SZ.v t0 <= SZ.v n /\ SZ.fits (SZ.v n + 1) /\ SZ.fits (SZ.v m + 1))
  returns t: SZ.t
  ensures exists* s1 s2. V.pts_to dist s1 ** V.pts_to q s2 **
          pure (Seq.length s1 == SZ.v n /\ Seq.length s2 == SZ.v n /\ SZ.v t <= SZ.v n /\ SZ.v t >= SZ.v t0)
{
  SZ.fits_u64_implies_fits_32 ();
  V.pts_to_len adj;
  V.pts_to_len dist;
  V.pts_to_len q;
  let mut e = (if SZ.lte e0 m then e0 else m);
  let mut tail = t0;
  while (
    let ve = !e;
    (SZ.lt ve e1 && SZ.lt ve m)
  )
  invariant exists* ve vt s1 s2. R.pts_to e ve ** R.pts_to tail vt ** V.pts_to dist s1 ** V.pts_to q s2 **
            pure (SZ.v ve <= SZ.v m /\ SZ.v vt <= SZ.v n /\ SZ.v vt >= SZ.v t0 /\ Seq.length s1 == SZ.v n /\ Seq.length s2 == SZ.v n)
  decreases (SZ.v m - SZ.v !e)
  {
    let ve = !e;
    let vt = !tail;
    let w32 = V.(adj.(ve));
    let ws = SZ.uint32_to_sizet w32;
    let wok = SZ.lt ws n;
    let wsafe = (if wok then ws else 0sz);
    let dw = V.(dist.(wsafe));
    let fresh = (wok && (dw = inf) && SZ.lt vt n);
    if fresh {
      V.(dist.(wsafe) <- du);
      V.(q.(vt) <- w32)
    };
    V.pts_to_len dist;
    V.pts_to_len q;
    SZ.fits_lte (SZ.v vt + 1) (SZ.v n + 1);
    SZ.fits_lte (SZ.v ve + 1) (SZ.v m + 1);
    tail := (if fresh then SZ.add vt 1sz else vt);
    e := SZ.add ve 1sz
  };
  !tail
}

(* BFS depuis 0 ; renvoie le nombre de sommets atteints (<= n). Gardes : u < n, e <= m. *)
fn bfs (off adj dist q: V.vec U32.t) (n m: SZ.t)
       (#so: Ghost.erased (Seq.seq U32.t)) (#sa: Ghost.erased (Seq.seq U32.t))
       (#sd: Ghost.erased (Seq.seq U32.t)) (#sq: Ghost.erased (Seq.seq U32.t))
  preserves V.pts_to off so ** V.pts_to adj sa
  requires V.pts_to dist sd ** V.pts_to q sq **
           pure (SZ.fits_u64 /\ SZ.v n >= 1 /\ V.length off == SZ.v n + 1 /\ V.length adj == SZ.v m /\
                 V.length dist == SZ.v n /\ V.length q == SZ.v n /\ SZ.fits (SZ.v n + 1) /\ SZ.fits (SZ.v m + 1))
  returns t: SZ.t
  ensures exists* s1 s2. V.pts_to dist s1 ** V.pts_to q s2 **
          pure (Seq.length s1 == SZ.v n /\ Seq.length s2 == SZ.v n /\ SZ.v t <= SZ.v n)
{
  SZ.fits_u64_implies_fits_32 ();
  V.pts_to_len off;
  V.pts_to_len dist;
  V.pts_to_len q;
  V.(dist.(0sz) <- 0ul);
  V.(q.(0sz) <- 0ul);
  let mut head = 0sz;
  let mut tail = 1sz;
  while (
    let vh = !head;
    let vt = !tail;
    SZ.lt vh vt
  )
  invariant exists* vh vt s1 s2. R.pts_to head vh ** R.pts_to tail vt ** V.pts_to dist s1 ** V.pts_to q s2 **
            pure (SZ.v vh <= SZ.v vt /\ SZ.v vt <= SZ.v n /\ Seq.length s1 == SZ.v n /\ Seq.length s2 == SZ.v n)
  decreases (SZ.v n - SZ.v !head)
  {
    let vh = !head;
    let vt = !tail;
    let u32 = V.(q.(vh));
    let us = SZ.uint32_to_sizet u32;
    let uok = SZ.lt us n;
    let usafe = (if uok then us else 0sz);
    let du = U32.add_mod V.(dist.(usafe)) 1ul;
    SZ.fits_lte (SZ.v usafe + 1) (SZ.v n + 1);
    let e0 = SZ.uint32_to_sizet V.(off.(usafe));
    let e1 = SZ.uint32_to_sizet V.(off.(SZ.add usafe 1sz));
    let t = relax adj dist q n m e0 e1 vt du;
    SZ.fits_lte (SZ.v vh + 1) (SZ.v n + 1);
    head := SZ.add vh 1sz;
    tail := t
  };
  !tail
}

(* ck = fold(dist[i]) puis fold(atteints) puis fold(profondeur max). *)
fn checksum (dist: V.vec U32.t) (n t: SZ.t) (#sd: Ghost.erased (Seq.seq U32.t))
  preserves V.pts_to dist sd
  requires pure (SZ.v n >= 1 /\ V.length dist == SZ.v n /\ SZ.fits (SZ.v n + 1))
  returns r: U64.t
{
  V.pts_to_len dist;
  let mut i = 0sz;
  let mut ck = ck_init;
  let mut maxd = 0ul;
  while (!i `SZ.lt` n)
  invariant exists* vi vck vm. R.pts_to i vi ** R.pts_to ck vck ** R.pts_to maxd vm ** pure (SZ.v vi <= SZ.v n)
  decreases (SZ.v n - SZ.v !i)
  {
    let vi = !i;
    let d = V.(dist.(vi));
    ck := ckfold !ck (C.uint32_to_uint64 d);
    maxd := (if ((not (d = inf)) && U32.gt d !maxd) then d else !maxd);
    SZ.fits_lte (SZ.v vi + 1) (SZ.v n + 1);
    i := SZ.add vi 1sz
  };
  let c1 = ckfold !ck (SZ.sizet_to_uint64 t);
  ckfold c1 (C.uint32_to_uint64 !maxd)
}

fn run (n: SZ.t)
  requires pure (SZ.fits_u64 /\ SZ.v n >= 1 /\ SZ.v n <= 100000000)
  returns r: U64.t
  ensures emp
{
  assert_norm (pow2 64 == 18446744073709551616);
  SZ.fits_u64_implies_fits (SZ.v n + 1);
  SZ.fits_u64_implies_fits (16 * SZ.v n + 1);
  SZ.fits_u64_implies_fits 16;
  let m = SZ.mul 16sz n;
  let nn = SZ.sizet_to_uint64 n;
  let off = V.alloc 0ul (SZ.add n 1sz);
  let adj = V.alloc 0ul m;
  count_deg off n m nn;
  prefix off n;
  let cur = V.alloc 0ul n;
  copy_cur off cur n;
  fill cur adj n m nn;
  V.free cur;
  let dist = V.alloc inf n;
  let q = V.alloc 0ul n;
  let t = bfs off adj dist q n m;
  let r = checksum dist n t;
  V.free off;
  V.free adj;
  V.free dist;
  V.free q;
  r
}
