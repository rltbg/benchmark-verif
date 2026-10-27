module Bench.Par
#lang-pulse
(* Noyaux séquentiels des workloads parallèles `parmatmul`, `parsort`, `parsha` (spec/workloads.d/par*.md).
   Les threads ne sont PAS dans ce module : ils sont dans les pilotes (fstar/drivers/*) qui appellent ces points d'entrée sur des plages
   disjointes d'un même tableau. Prouvé : mémoire-sûreté, indices dans les bornes (n <= 16384 pour le produit matriciel), absence de débordement,
   terminaison. Pas de correction fonctionnelle (voir Bench.Sort pour le tri). Points d'entrée Custard :
     Bench.Par.u32_alloc / u32_free          allocation de tableaux partagés par les threads
     Bench.Par.mat_rows                      lignes i0..i1 de C = A*B
     Bench.Par.sha_seg                       condensé SHA-256 d'un segment (réutilise Bench.Sha256.compress / tail_blocks)
   plus Bench.Matmul.init_mat / checksum et Bench.Sort.fill / msort / merge / copy_back / checksum déjà prouvés. *)
open Pulse.Lib.Pervasives
open Bench.Common
module V   = Pulse.Lib.Vec
module R   = Pulse.Lib.Reference
module SZ  = FStar.SizeT
module U32 = FStar.UInt32
module U64 = FStar.UInt64
module C   = FStar.Int.Cast
module ML  = FStar.Math.Lemmas
module S   = Bench.Sha256

#set-options "--z3rlimit 100"

fn u32_alloc (n: SZ.t)
  requires emp
  returns v: V.vec U32.t
  ensures V.pts_to v (Seq.create (SZ.v n) 0ul) ** pure (V.length v == SZ.v n /\ V.is_full_vec v)
{
  V.alloc 0ul n
}

fn u32_free (v: V.vec U32.t) (#s: Ghost.erased (Seq.seq U32.t))
  requires V.pts_to v s ** pure (V.is_full_vec v)
  ensures emp
{
  V.free v
}

(* ---------- produit matriciel : lignes i0..i1 ---------- *)

let lemma_idx (i j n: nat) : Lemma (requires i < n /\ j < n) (ensures i * n + j < n * n) =
  ML.distributivity_add_left i 1 n;
  ML.lemma_mult_le_right n (i + 1) n

let lemma_nn (n: nat) : Lemma (requires n <= 16384) (ensures n * n <= 268435456) =
  ML.lemma_mult_le_right n n 16384;
  ML.lemma_mult_le_left 16384 n 16384

let idx (n i j: SZ.t) : Pure SZ.t
  (requires SZ.fits_u64 /\ SZ.v n <= 16384 /\ SZ.v i < SZ.v n /\ SZ.v j < SZ.v n)
  (ensures fun r -> SZ.v r == SZ.v i * SZ.v n + SZ.v j /\ SZ.v r < SZ.v n * SZ.v n)
= lemma_idx (SZ.v i) (SZ.v j) (SZ.v n);
  lemma_nn (SZ.v n);
  assert_norm (pow2 64 == 18446744073709551616);
  SZ.fits_u64_implies_fits (SZ.v i * SZ.v n);
  SZ.fits_u64_implies_fits (SZ.v i * SZ.v n + SZ.v j);
  SZ.add (SZ.mul i n) j

fn loop_j (a b c: V.vec U32.t) (n i k: SZ.t) (aik: U32.t)
          (#sa #sb #sc: Ghost.erased (Seq.seq U32.t))
  preserves V.pts_to a sa ** V.pts_to b sb
  requires V.pts_to c sc **
           pure (SZ.fits_u64 /\ SZ.v n <= 16384 /\ SZ.v i < SZ.v n /\ SZ.v k < SZ.v n /\
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
    SZ.fits_lte (SZ.v vj + 1) 16385;
    j := SZ.add vj 1sz
  }
}

fn loop_k (a b c: V.vec U32.t) (n i: SZ.t)
          (#sa #sb #sc: Ghost.erased (Seq.seq U32.t))
  preserves V.pts_to a sa ** V.pts_to b sb
  requires V.pts_to c sc **
           pure (SZ.fits_u64 /\ SZ.v n <= 16384 /\ SZ.v i < SZ.v n /\
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
    SZ.fits_lte (SZ.v vk + 1) 16385;
    k := SZ.add vk 1sz
  }
}

(* Lignes i0..i1 de C = A*B (C supposé nul sur ces lignes) : appelé par chaque thread sur sa plage. *)
fn mat_rows (a b c: V.vec U32.t) (n i0 i1: SZ.t)
            (#sa #sb #sc: Ghost.erased (Seq.seq U32.t))
  preserves V.pts_to a sa ** V.pts_to b sb
  requires V.pts_to c sc **
           pure (SZ.fits_u64 /\ SZ.v n <= 16384 /\ SZ.v i1 <= SZ.v n /\
                 V.length a == SZ.v n * SZ.v n /\ V.length b == SZ.v n * SZ.v n /\ V.length c == SZ.v n * SZ.v n)
  ensures exists* sc'. V.pts_to c sc' ** pure (Seq.length sc' == SZ.v n * SZ.v n)
{
  V.pts_to_len c;
  let mut i = i0;
  while (!i `SZ.lt` i1)
  invariant exists* vi sc1. R.pts_to i vi ** V.pts_to c sc1 **
            pure (SZ.v vi <= SZ.v i1 \/ SZ.v i0 > SZ.v i1) /\ pure (Seq.length sc1 == SZ.v n * SZ.v n)
  decreases (SZ.v i1 - SZ.v !i)
  {
    let vi = !i;
    loop_k a b c n vi;
    SZ.fits_lte (SZ.v vi + 1) 16385;
    i := SZ.add vi 1sz
  }
}

(* ---------- SHA-256 d'un segment ---------- *)

(* W[0..15] <- bloc b du segment de base `sb` : mots 2k, 2k+1 = bswap(bas), bswap(haut) de mix64(sb + 8b + k). *)
fn load_mix_block_seeded (w: V.vec U32.t) (sb b: U64.t) (#s0: Ghost.erased (Seq.seq U32.t))
  requires V.pts_to w s0 ** pure (V.length w == 64)
  ensures exists* s. V.pts_to w s ** pure (Seq.length s == 64)
{
  V.pts_to_len w;
  let mut kk = 0sz;
  while (!kk `SZ.lt` 8sz)
  invariant exists* vk s. R.pts_to kk vk ** V.pts_to w s ** pure (SZ.v vk <= 8 /\ Seq.length s == 64)
  decreases (8 - SZ.v !kk)
  {
    let vk = !kk;
    let x = mix64 (U64.add_mod (U64.add_mod sb (U64.mul_mod b 8uL)) (SZ.sizet_to_uint64 vk));
    let i0 = S.sadd vk vk;
    let i1 = S.sadd i0 1sz;
    V.(w.(i0) <- S.bswap (C.uint64_to_uint32 x));
    V.(w.(i1) <- S.bswap (hi32 x));
    kk := SZ.add vk 1sz
  }
}

fn full_blocks_seeded (h w k: V.vec U32.t) (sb blocks: U64.t) (#sh #sw #sk: Ghost.erased (Seq.seq U32.t))
  preserves V.pts_to k sk
  requires V.pts_to h sh ** V.pts_to w sw ** pure (V.length h == 8 /\ V.length w == 64 /\ V.length k == 64)
  ensures exists* sh' sw'. V.pts_to h sh' ** V.pts_to w sw' ** pure (Seq.length sh' == 8 /\ Seq.length sw' == 64)
{
  V.pts_to_len h;
  V.pts_to_len w;
  let mut b = 0uL;
  while (!b `U64.lt` blocks)
  invariant exists* vb sh1 sw1. R.pts_to b vb ** V.pts_to h sh1 ** V.pts_to w sw1 **
            pure (U64.v vb <= U64.v blocks /\ Seq.length sh1 == 8 /\ Seq.length sw1 == 64)
  decreases (U64.v blocks - U64.v !b)
  {
    let vb = !b;
    load_mix_block_seeded w sb vb;
    S.compress h w k;
    b := U64.add vb 1uL
  }
}

(* Condensé SHA-256 des m octets (m multiple de 64, m <= 2^40) du segment de base sb, écrit dans out (8 mots). *)
fn sha_seg (sb: U64.t) (m: SZ.t) (out: V.vec U32.t) (#so: Ghost.erased (Seq.seq U32.t))
  requires V.pts_to out so ** pure (V.length out == 8 /\ SZ.fits_u64 /\ SZ.v m <= 1099511627776)
  ensures exists* s. V.pts_to out s ** pure (Seq.length s == 8)
{
  let h = V.alloc 0ul 8sz;
  let w = V.alloc 0ul 64sz;
  let k = V.alloc 0ul 64sz;
  S.init_h h;
  S.init_k k;
  full_blocks_seeded h w k sb (SZ.sizet_to_uint64 (SZ.div m 64sz));
  S.tail_blocks h w k m;
  V.pts_to_len h;
  V.pts_to_len out;
  let mut i = 0sz;
  while (!i `SZ.lt` 8sz)
  invariant exists* vi so1 sh1. R.pts_to i vi ** V.pts_to out so1 ** V.pts_to h sh1 **
            pure (SZ.v vi <= 8 /\ Seq.length so1 == 8 /\ Seq.length sh1 == 8)
  decreases (8 - SZ.v !i)
  {
    let vi = !i;
    let x = V.(h.(vi));
    V.(out.(vi) <- x);
    i := SZ.add vi 1sz
  };
  V.free h;
  V.free w;
  V.free k
}
