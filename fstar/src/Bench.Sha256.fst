module Bench.Sha256
#lang-pulse
(* SHA-256 d'un message de n octets généré à la volée (spec/workloads.d/sha256.md). 0 <= n <= 2^40.
   Prouvé : mémoire-sûreté (indices de W, K, H et du tampon de queue dans les bornes), absence de débordement
   de size_t, terminaison. Pas de correction fonctionnelle (conformité vérifiée contre `sha256sum`). *)
open Pulse.Lib.Pervasives
open Bench.Common
module V   = Pulse.Lib.Vec
module R   = Pulse.Lib.Reference
module SZ  = FStar.SizeT
module U8  = FStar.UInt8
module U32 = FStar.UInt32
module U64 = FStar.UInt64
module C   = FStar.Int.Cast

#set-options "--z3rlimit 100"

let seed : U64.t = 0x5348413235362D42uL

[@@"opaque_to_smt"]
let rotr (x: U32.t) (k: U32.t{0 < U32.v k /\ U32.v k < 32}) : U32.t =
  U32.logor (U32.shift_right x k) (U32.shift_left x (U32.sub 32ul k))

[@@"opaque_to_smt"]
let big0 (x: U32.t) : U32.t = U32.logxor (U32.logxor (rotr x 2ul) (rotr x 13ul)) (rotr x 22ul)
[@@"opaque_to_smt"]
let big1 (x: U32.t) : U32.t = U32.logxor (U32.logxor (rotr x 6ul) (rotr x 11ul)) (rotr x 25ul)
[@@"opaque_to_smt"]
let small0 (x: U32.t) : U32.t = U32.logxor (U32.logxor (rotr x 7ul) (rotr x 18ul)) (U32.shift_right x 3ul)
[@@"opaque_to_smt"]
let small1 (x: U32.t) : U32.t = U32.logxor (U32.logxor (rotr x 17ul) (rotr x 19ul)) (U32.shift_right x 10ul)
[@@"opaque_to_smt"]
let ch (e f g: U32.t) : U32.t = U32.logxor (U32.logand e f) (U32.logand (U32.lognot e) g)
[@@"opaque_to_smt"]
let maj (a b c: U32.t) : U32.t = U32.logxor (U32.logxor (U32.logand a b) (U32.logand a c)) (U32.logand b c)

[@@"opaque_to_smt"]
let bswap (x: U32.t) : U32.t =
  U32.logor
    (U32.logor (U32.shift_left (U32.logand x 255ul) 24ul) (U32.shift_left (U32.logand x 65280ul) 8ul))
    (U32.logor (U32.logand (U32.shift_right x 8ul) 65280ul) (U32.shift_right x 24ul))

(* Décalage 8*(i mod 8) < 64 pour extraire l'octet i d'un mot de 64 bits. *)
[@@"opaque_to_smt"]
let shamt (i: U64.t) : Pure U32.t (requires True) (ensures fun r -> U32.v r < 64) =
  assert_norm (pow2 32 == 4294967296);
  assert_norm (pow2 64 == 18446744073709551616);
  C.uint64_to_uint32 (U64.mul (U64.rem i 8uL) 8uL)

(* Octet i du message. *)
[@@"opaque_to_smt"]
let byte_at (i: U64.t) : U8.t =
  C.uint64_to_uint8 (U64.shift_right (mix64 (U64.add_mod seed (U64.div i 8uL))) (shamt i))

let sadd (a b: SZ.t) : Pure SZ.t (requires SZ.v a + SZ.v b <= 65535) (ensures fun r -> SZ.v r == SZ.v a + SZ.v b) =
  fits_small (SZ.v a + SZ.v b);
  SZ.add a b

let tlen_of (t: SZ.t) : Pure SZ.t (requires SZ.v t < 64)
  (ensures fun r -> (SZ.v r == 64 \/ SZ.v r == 128) /\ SZ.v t + 8 < SZ.v r + (if SZ.v r = 64 then 1 else 0) /\ SZ.v t < SZ.v r) =
  if SZ.lt t 56sz then 64sz else 128sz

(* Écrit x en k[i] (le rappel de la longueur évite l'accumulation de `Seq.upd` imbriqués dans le contexte SMT). *)
fn put (k: V.vec U32.t) (i: SZ.t) (x: U32.t) (#s0: Ghost.erased (Seq.seq U32.t))
  requires V.pts_to k s0 ** pure (V.length k == 64 /\ SZ.v i < 64)
  ensures exists* s. V.pts_to k s ** pure (Seq.length s == 64)
{
  V.pts_to_len k;
  V.(k.(i) <- x)
}

(* K[0..63] *)
fn init_k (k: V.vec U32.t) (#s0: Ghost.erased (Seq.seq U32.t))
  requires V.pts_to k s0 ** pure (V.length k == 64)
  ensures exists* s. V.pts_to k s ** pure (Seq.length s == 64)
{
  V.pts_to_len k;
  put k 0sz 0x428a2f98ul;
  put k 1sz 0x71374491ul;
  put k 2sz 0xb5c0fbcful;
  put k 3sz 0xe9b5dba5ul;
  put k 4sz 0x3956c25bul;
  put k 5sz 0x59f111f1ul;
  put k 6sz 0x923f82a4ul;
  put k 7sz 0xab1c5ed5ul;
  put k 8sz 0xd807aa98ul;
  put k 9sz 0x12835b01ul;
  put k 10sz 0x243185beul;
  put k 11sz 0x550c7dc3ul;
  put k 12sz 0x72be5d74ul;
  put k 13sz 0x80deb1feul;
  put k 14sz 0x9bdc06a7ul;
  put k 15sz 0xc19bf174ul;
  put k 16sz 0xe49b69c1ul;
  put k 17sz 0xefbe4786ul;
  put k 18sz 0x0fc19dc6ul;
  put k 19sz 0x240ca1ccul;
  put k 20sz 0x2de92c6ful;
  put k 21sz 0x4a7484aaul;
  put k 22sz 0x5cb0a9dcul;
  put k 23sz 0x76f988daul;
  put k 24sz 0x983e5152ul;
  put k 25sz 0xa831c66dul;
  put k 26sz 0xb00327c8ul;
  put k 27sz 0xbf597fc7ul;
  put k 28sz 0xc6e00bf3ul;
  put k 29sz 0xd5a79147ul;
  put k 30sz 0x06ca6351ul;
  put k 31sz 0x14292967ul;
  put k 32sz 0x27b70a85ul;
  put k 33sz 0x2e1b2138ul;
  put k 34sz 0x4d2c6dfcul;
  put k 35sz 0x53380d13ul;
  put k 36sz 0x650a7354ul;
  put k 37sz 0x766a0abbul;
  put k 38sz 0x81c2c92eul;
  put k 39sz 0x92722c85ul;
  put k 40sz 0xa2bfe8a1ul;
  put k 41sz 0xa81a664bul;
  put k 42sz 0xc24b8b70ul;
  put k 43sz 0xc76c51a3ul;
  put k 44sz 0xd192e819ul;
  put k 45sz 0xd6990624ul;
  put k 46sz 0xf40e3585ul;
  put k 47sz 0x106aa070ul;
  put k 48sz 0x19a4c116ul;
  put k 49sz 0x1e376c08ul;
  put k 50sz 0x2748774cul;
  put k 51sz 0x34b0bcb5ul;
  put k 52sz 0x391c0cb3ul;
  put k 53sz 0x4ed8aa4aul;
  put k 54sz 0x5b9cca4ful;
  put k 55sz 0x682e6ff3ul;
  put k 56sz 0x748f82eeul;
  put k 57sz 0x78a5636ful;
  put k 58sz 0x84c87814ul;
  put k 59sz 0x8cc70208ul;
  put k 60sz 0x90befffaul;
  put k 61sz 0xa4506cebul;
  put k 62sz 0xbef9a3f7ul;
  put k 63sz 0xc67178f2ul
}

fn init_h (h: V.vec U32.t) (#s0: Ghost.erased (Seq.seq U32.t))
  requires V.pts_to h s0 ** pure (V.length h == 8)
  ensures exists* s. V.pts_to h s ** pure (Seq.length s == 8)
{
  V.pts_to_len h;
  V.(h.(0sz) <- 0x6a09e667ul); V.(h.(1sz) <- 0xbb67ae85ul); V.(h.(2sz) <- 0x3c6ef372ul); V.(h.(3sz) <- 0xa54ff53aul);
  V.(h.(4sz) <- 0x510e527ful); V.(h.(5sz) <- 0x9b05688cul); V.(h.(6sz) <- 0x1f83d9abul); V.(h.(7sz) <- 0x5be0cd19ul)
}

(* W[0..15] <- bloc b du message : mots 2k et 2k+1 = bswap(bas), bswap(haut) de mix64(seed + 8b + k). *)
fn load_mix_block (w: V.vec U32.t) (b: U64.t) (#s0: Ghost.erased (Seq.seq U32.t))
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
    let x = mix64 (U64.add_mod (U64.add_mod seed (U64.mul_mod b 8uL)) (SZ.sizet_to_uint64 vk));
    let i0 = sadd vk vk;
    let i1 = sadd i0 1sz;
    V.(w.(i0) <- bswap (C.uint64_to_uint32 x));
    V.(w.(i1) <- bswap (hi32 x));
    kk := SZ.add vk 1sz
  }
}

(* W[0..15] <- 16 mots big-endian lus dans tail[off, off+64). *)
fn load_tail_block (w: V.vec U32.t) (tail: V.vec U8.t) (off: SZ.t)
                   (#s0: Ghost.erased (Seq.seq U32.t)) (#st: Ghost.erased (Seq.seq U8.t))
  preserves V.pts_to tail st
  requires V.pts_to w s0 ** pure (V.length w == 64 /\ V.length tail == 128 /\ SZ.v off + 64 <= 128)
  ensures exists* s. V.pts_to w s ** pure (Seq.length s == 64)
{
  V.pts_to_len w;
  V.pts_to_len tail;
  let mut ii = 0sz;
  while (!ii `SZ.lt` 16sz)
  invariant exists* vi s. R.pts_to ii vi ** V.pts_to w s ** pure (SZ.v vi <= 16 /\ Seq.length s == 64)
  decreases (16 - SZ.v !ii)
  {
    let vi = !ii;
    let p = sadd off (sadd vi (sadd vi (sadd vi vi)));
    let b0 = V.(tail.(p));
    let b1 = V.(tail.(sadd p 1sz));
    let b2 = V.(tail.(sadd p 2sz));
    let b3 = V.(tail.(sadd p 3sz));
    V.(w.(vi) <-
        U32.logor
          (U32.logor (U32.shift_left (C.uint8_to_uint32 b0) 24ul) (U32.shift_left (C.uint8_to_uint32 b1) 16ul))
          (U32.logor (U32.shift_left (C.uint8_to_uint32 b2) 8ul) (C.uint8_to_uint32 b3)));
    ii := SZ.add vi 1sz
  }
}

(* Échéancier, 64 rondes, ajout à l'état. W[0..15] déjà chargé. *)
fn compress (h w k: V.vec U32.t) (#sh #sw #sk: Ghost.erased (Seq.seq U32.t))
  preserves V.pts_to k sk
  requires V.pts_to h sh ** V.pts_to w sw ** pure (V.length h == 8 /\ V.length w == 64 /\ V.length k == 64)
  ensures exists* sh' sw'. V.pts_to h sh' ** V.pts_to w sw' ** pure (Seq.length sh' == 8 /\ Seq.length sw' == 64)
{
  V.pts_to_len h;
  V.pts_to_len w;
  V.pts_to_len k;
  let mut t = 16sz;
  while (!t `SZ.lt` 64sz)
  invariant exists* vt s. R.pts_to t vt ** V.pts_to w s ** pure (SZ.v vt >= 16 /\ SZ.v vt <= 64 /\ Seq.length s == 64)
  decreases (64 - SZ.v !t)
  {
    let vt = !t;
    let w16 = V.(w.(SZ.sub vt 16sz));
    let w15 = V.(w.(SZ.sub vt 15sz));
    let w7 = V.(w.(SZ.sub vt 7sz));
    let w2 = V.(w.(SZ.sub vt 2sz));
    V.(w.(vt) <- U32.add_mod (U32.add_mod w16 (small0 w15)) (U32.add_mod w7 (small1 w2)));
    t := SZ.add vt 1sz
  };
  let h0 = V.(h.(0sz)); let h1 = V.(h.(1sz)); let h2 = V.(h.(2sz)); let h3 = V.(h.(3sz));
  let h4 = V.(h.(4sz)); let h5 = V.(h.(5sz)); let h6 = V.(h.(6sz)); let h7 = V.(h.(7sz));
  let mut ra = h0; let mut rb = h1; let mut rc = h2; let mut rd = h3;
  let mut re = h4; let mut rf = h5; let mut rg = h6; let mut rh = h7;
  t := 0sz;
  while (!t `SZ.lt` 64sz)
  invariant exists* vt va vb vc vd ve vf vg vh s.
            R.pts_to t vt ** R.pts_to ra va ** R.pts_to rb vb ** R.pts_to rc vc ** R.pts_to rd vd **
            R.pts_to re ve ** R.pts_to rf vf ** R.pts_to rg vg ** R.pts_to rh vh ** V.pts_to w s **
            pure (SZ.v vt <= 64 /\ Seq.length s == 64)
  decreases (64 - SZ.v !t)
  {
    let vt = !t;
    let va = !ra; let vb = !rb; let vc = !rc; let vd = !rd;
    let ve = !re; let vf = !rf; let vg = !rg; let vh = !rh;
    let kt = V.(k.(vt));
    let wt = V.(w.(vt));
    let t1 = U32.add_mod (U32.add_mod (U32.add_mod vh (big1 ve)) (U32.add_mod (ch ve vf vg) kt)) wt;
    let t2 = U32.add_mod (big0 va) (maj va vb vc);
    rh := vg; rg := vf; rf := ve; re := U32.add_mod vd t1;
    rd := vc; rc := vb; rb := va; ra := U32.add_mod t1 t2;
    t := SZ.add vt 1sz
  };
  let va = !ra; let vb = !rb; let vc = !rc; let vd = !rd;
  let ve = !re; let vf = !rf; let vg = !rg; let vh = !rh;
  V.(h.(0sz) <- U32.add_mod h0 va); V.(h.(1sz) <- U32.add_mod h1 vb);
  V.(h.(2sz) <- U32.add_mod h2 vc); V.(h.(3sz) <- U32.add_mod h3 vd);
  V.(h.(4sz) <- U32.add_mod h4 ve); V.(h.(5sz) <- U32.add_mod h5 vf);
  V.(h.(6sz) <- U32.add_mod h6 vg); V.(h.(7sz) <- U32.add_mod h7 vh)
}

(* Blocs complets du message. *)
fn full_blocks (h w k: V.vec U32.t) (blocks: U64.t) (#sh #sw #sk: Ghost.erased (Seq.seq U32.t))
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
    load_mix_block w vb;
    compress h w k;
    b := U64.add vb 1uL
  }
}

(* Queue (n mod 64 octets), padding et longueur : 1 ou 2 blocs. *)
fn tail_blocks (h w k: V.vec U32.t) (n: SZ.t) (#sh #sw #sk: Ghost.erased (Seq.seq U32.t))
  preserves V.pts_to k sk
  requires V.pts_to h sh ** V.pts_to w sw **
           pure (V.length h == 8 /\ V.length w == 64 /\ V.length k == 64 /\ SZ.fits_u64 /\ SZ.v n <= 1099511627776)
  ensures exists* sh' sw'. V.pts_to h sh' ** V.pts_to w sw' ** pure (Seq.length sh' == 8 /\ Seq.length sw' == 64)
{
  assert_norm (pow2 64 == 18446744073709551616);
  V.pts_to_len h;
  V.pts_to_len w;
  let tail = V.alloc 0uy 128sz;
  V.pts_to_len tail;
  let t = SZ.rem n 64sz;
  let blocks = SZ.div n 64sz;
  let base = U64.mul_mod (SZ.sizet_to_uint64 blocks) 64uL;
  let mut j = 0sz;
  while (!j `SZ.lt` t)
  invariant exists* vj st. R.pts_to j vj ** V.pts_to tail st **
            pure (SZ.v vj <= SZ.v t /\ SZ.v t < 64 /\ Seq.length st == 128)
  decreases (SZ.v t - SZ.v !j)
  {
    let vj = !j;
    V.(tail.(vj) <- byte_at (U64.add_mod base (SZ.sizet_to_uint64 vj)));
    j := SZ.add vj 1sz
  };
  V.(tail.(t) <- 128uy);
  let tlen = tlen_of t;
  let bits = U64.mul_mod (SZ.sizet_to_uint64 n) 8uL;
  let o = SZ.sub tlen 8sz;
  V.(tail.(o) <- C.uint64_to_uint8 (U64.shift_right bits 56ul));
  V.(tail.(sadd o 1sz) <- C.uint64_to_uint8 (U64.shift_right bits 48ul));
  V.(tail.(sadd o 2sz) <- C.uint64_to_uint8 (U64.shift_right bits 40ul));
  V.(tail.(sadd o 3sz) <- C.uint64_to_uint8 (U64.shift_right bits 32ul));
  V.(tail.(sadd o 4sz) <- C.uint64_to_uint8 (U64.shift_right bits 24ul));
  V.(tail.(sadd o 5sz) <- C.uint64_to_uint8 (U64.shift_right bits 16ul));
  V.(tail.(sadd o 6sz) <- C.uint64_to_uint8 (U64.shift_right bits 8ul));
  V.(tail.(sadd o 7sz) <- C.uint64_to_uint8 bits);
  let mut off = 0sz;
  while (!off `SZ.lt` tlen)
  invariant exists* voff sh1 sw1 st. R.pts_to off voff ** V.pts_to h sh1 ** V.pts_to w sw1 ** V.pts_to tail st **
            pure (SZ.v voff <= SZ.v tlen /\ SZ.v voff % 64 == 0 /\ (SZ.v tlen == 64 \/ SZ.v tlen == 128) /\
                  Seq.length sh1 == 8 /\ Seq.length sw1 == 64 /\ Seq.length st == 128)
  decreases (SZ.v tlen - SZ.v !off)
  {
    let voff = !off;
    load_tail_block w tail voff;
    compress h w k;
    off := sadd voff 64sz
  };
  V.free tail
}

fn checksum (h: V.vec U32.t) (#sh: Ghost.erased (Seq.seq U32.t))
  preserves V.pts_to h sh
  requires pure (V.length h == 8)
  returns r: U64.t
{
  V.pts_to_len h;
  let mut i = 0sz;
  let mut ck = ck_init;
  while (!i `SZ.lt` 8sz)
  invariant exists* vi vck. R.pts_to i vi ** R.pts_to ck vck ** pure (SZ.v vi <= 8)
  decreases (8 - SZ.v !i)
  {
    let vi = !i;
    let x = V.(h.(vi));
    ck := ckfold !ck (C.uint32_to_uint64 x);
    i := SZ.add vi 1sz
  };
  !ck
}

fn run (n: SZ.t)
  requires pure (SZ.fits_u64 /\ SZ.v n <= 1099511627776)
  returns r: U64.t
  ensures emp
{
  let h = V.alloc 0ul 8sz;
  let w = V.alloc 0ul 64sz;
  let k = V.alloc 0ul 64sz;
  init_h h;
  init_k k;
  full_blocks h w k (SZ.sizet_to_uint64 (SZ.div n 64sz));
  tail_blocks h w k n;
  let r = checksum h;
  V.free h;
  V.free w;
  V.free k;
  r
}
