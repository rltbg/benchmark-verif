module Bench.SortSpec
(* Interface abstraite de la spécification du tri fusion : les prédicats sont opaques pour le client
   (code Pulse), qui n'utilise que les lemmes ci-dessous. Cela évite que les quantificateurs de
   « trié », « permutation » et « cadre » se déplient dans les requêtes SMT du code. *)
module U32 = FStar.UInt32
module S   = FStar.Seq
module P   = FStar.Seq.Properties

type seq32 = S.seq U32.t

val sorted_range (s: seq32) (lo hi: nat) : prop      (* s[lo,hi) est croissant *)
val perm_range (a b: seq32) (lo hi: nat) : prop      (* a[lo,hi) est une permutation de b[lo,hi) *)
val frame (a b: seq32) (lo hi: nat) : prop           (* a et b coïncident hors de [lo,hi) ; même longueur *)
val slice_eq (a b: seq32) (lo hi: nat) : prop        (* a[lo,hi) == b[lo,hi) *)
val merge_inv (sd ss: seq32) (lo mid hi i j k: nat) : prop

val frame_len (a b: seq32) (lo hi: nat) : Lemma (requires frame a b lo hi) (ensures S.length a == S.length b)
val frame_refl (a: seq32) (lo hi: nat) : Lemma (frame a a lo hi)
val frame_upd (a b: seq32) (lo hi k: nat) (w: U32.t)
  : Lemma (requires frame a b lo hi /\ lo <= k /\ k < hi /\ k < S.length a)
          (ensures frame (S.upd a k w) b lo hi)
val perm_range_refl (a: seq32) (lo hi: nat)
  : Lemma (requires lo <= hi /\ hi <= S.length a) (ensures perm_range a a lo hi)
val sorted_small (s: seq32) (lo hi: nat)
  : Lemma (requires lo <= hi /\ hi <= S.length s /\ hi - lo < 2) (ensures sorted_range s lo hi)
val slice_eq_intro (a b: seq32) (lo hi: nat)
  : Lemma (requires lo <= hi /\ hi <= S.length a /\ hi <= S.length b /\
                    (forall (p: nat). lo <= p /\ p < hi ==> S.index a p == S.index b p))
          (ensures slice_eq a b lo hi)

val merge_init (sd ss: seq32) (lo mid hi: nat)
  : Lemma (requires lo <= mid /\ mid <= hi /\ hi <= S.length ss /\ hi <= S.length sd)
          (ensures merge_inv sd ss lo mid hi lo mid lo)

(* Pas unique, paramétré par le choix runtime. *)
val merge_step (sd ss: seq32) (lo mid hi i j k: nat) (take_left: bool)
  : Lemma
    (requires merge_inv sd ss lo mid hi i j k /\ sorted_range ss lo mid /\ sorted_range ss mid hi /\
              k < hi /\ k < S.length sd /\
              (take_left ==> (i < mid /\ i < S.length ss /\
                              (j >= hi \/ (j < S.length ss /\ U32.v (S.index ss i) <= U32.v (S.index ss j))))) /\
              (~take_left ==> (j < hi /\ j < S.length ss /\
                               (i >= mid \/ (i < S.length ss /\ U32.v (S.index ss j) < U32.v (S.index ss i))))))
    (ensures
      (take_left ==> merge_inv (S.upd sd k (S.index ss i)) ss lo mid hi (i+1) j (k+1)) /\
      (~take_left ==> merge_inv (S.upd sd k (S.index ss j)) ss lo mid hi i (j+1) (k+1)))

val merge_done (sd ss: seq32) (lo mid hi i j k: nat)
  : Lemma (requires merge_inv sd ss lo mid hi i j k /\ k == hi)
          (ensures sorted_range sd lo hi /\ perm_range sd ss lo hi)

val sorted_frame_l (a b: seq32) (lo hi flo fhi: nat)
  : Lemma (requires lo <= hi /\ hi <= S.length a /\ sorted_range b lo hi /\ frame a b flo fhi /\ (hi <= flo \/ fhi <= lo))
          (ensures sorted_range a lo hi)

(* Bilan de msort sur [lo,hi) : o = état initial, s1/s2 après chaque moitié, t = tmp après fusion, a3 = résultat. *)
val msort_combine (o s1 s2 t a3: seq32) (lo mid hi: nat)
  : Lemma
    (requires lo <= mid /\ mid <= hi /\ hi <= S.length o /\
              S.length o == S.length s1 /\ S.length s1 == S.length s2 /\ S.length s2 == S.length a3 /\ hi <= S.length t /\
              sorted_range s1 lo mid /\ perm_range s1 o lo mid /\ frame s1 o lo mid /\
              sorted_range s2 mid hi /\ perm_range s2 s1 mid hi /\ frame s2 s1 mid hi /\
              sorted_range t lo hi /\ perm_range t s2 lo hi /\
              frame a3 s2 lo hi /\ slice_eq a3 t lo hi)
    (ensures sorted_range a3 lo hi /\ perm_range a3 o lo hi /\ frame a3 o lo hi)
