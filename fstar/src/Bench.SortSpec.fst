module Bench.SortSpec
(* Spécification et lemmes (purs, effacés à l'extraction) du tri fusion :
   « trié » et « permutation » sur une plage, et les pas de la fusion et de la récursion. *)
module U32 = FStar.UInt32
module S   = FStar.Seq
module P   = FStar.Seq.Properties

let v (x: U32.t) : nat = U32.v x

let sorted_range (s: seq32) (lo hi: nat) : prop =
  hi <= S.length s /\
  (forall (i j: nat). lo <= i /\ i <= j /\ j < hi ==> v (S.index s i) <= v (S.index s j))

let perm_range (a b: seq32) (lo hi: nat) : prop =
  lo <= hi /\ hi <= S.length a /\ hi <= S.length b /\
  P.permutation U32.t (S.slice a lo hi) (S.slice b lo hi)

(* a et b coïncident hors de [lo, hi). *)
let frame (a b: seq32) (lo hi: nat) : prop =
  S.length a == S.length b /\
  (forall (p: nat). p < S.length a /\ (p < lo \/ hi <= p) ==> S.index a p == S.index b p)

(* Les tranches [lo,hi) de a et b sont égales (version totale : porte ses propres garde-fous). *)
let slice_eq (a b: seq32) (lo hi: nat) : prop =
  lo <= hi /\ hi <= S.length a /\ hi <= S.length b /\ S.slice a lo hi == S.slice b lo hi

let perm_trans (a b c: seq32)
  : Lemma (requires P.permutation U32.t a b /\ P.permutation U32.t b c)
          (ensures P.permutation U32.t a c) = ()

let perm_refl (a: seq32) : Lemma (P.permutation U32.t a a) = ()

#set-options "--z3rlimit 100"

let count_app (y: U32.t) (a b: seq32) : Lemma (P.count y (S.append a b) == P.count y a + P.count y b) =
  P.lemma_append_count_aux y a b

let count_snoc (y: U32.t) (a: seq32) (w: U32.t) : Lemma (P.count y (S.snoc a w) == P.count y a + P.count y (S.create 1 w)) =
  S.lemma_eq_intro (S.snoc a w) (S.append a (S.create 1 w));
  count_app y a (S.create 1 w)

let count_cons (y: U32.t) (w: U32.t) (a: seq32) : Lemma (P.count y (S.cons w a) == P.count y (S.create 1 w) + P.count y a) =
  S.lemma_eq_intro (S.cons w a) (S.append (S.create 1 w) a);
  count_app y (S.create 1 w) a

(* Pas de la fusion : la tête w de la file gauche passe à la fin de l'accumulateur. *)
let step_left (sd ss: seq32) (lo k i mid j hi: nat) (w: U32.t)
  : Lemma
    (requires lo <= i /\ lo <= hi /\ lo <= k /\ k < S.length sd /\ i < mid /\ mid <= hi /\ hi <= S.length ss /\ j <= hi /\ w == S.index ss i /\
              P.permutation U32.t (S.append (S.append (S.slice sd lo k) (S.slice ss i mid)) (S.slice ss j hi))
                                  (S.slice ss lo hi))
    (ensures
      (let sd' = S.upd sd k w in
       P.permutation U32.t (S.append (S.append (S.slice sd' lo (k+1)) (S.slice ss (i+1) mid)) (S.slice ss j hi))
                           (S.slice ss lo hi)))
= let sd' = S.upd sd k w in
  let acc = S.slice sd lo k in
  let r1 = S.slice ss (i+1) mid in
  let r2 = S.slice ss j hi in
  S.lemma_eq_intro (S.slice sd' lo (k+1)) (S.snoc acc w);
  S.lemma_eq_intro (S.slice ss i mid) (S.cons w r1);
  let aux (y: U32.t) : Lemma (P.count y (S.append (S.append (S.slice sd' lo (k+1)) r1) r2) ==
                              P.count y (S.append (S.append acc (S.slice ss i mid)) r2)) =
    count_app y (S.append (S.snoc acc w) r1) r2; count_app y (S.snoc acc w) r1; count_snoc y acc w;
    count_app y (S.append acc (S.cons w r1)) r2; count_app y acc (S.cons w r1); count_cons y w r1
  in
  FStar.Classical.forall_intro aux

(* Idem pour la tête w de la file droite. *)
let step_right (sd ss: seq32) (lo k i mid j hi: nat) (w: U32.t)
  : Lemma
    (requires lo <= i /\ lo <= hi /\ lo <= k /\ k < S.length sd /\ i <= mid /\ mid <= j /\ j < hi /\ hi <= S.length ss /\ w == S.index ss j /\
              P.permutation U32.t (S.append (S.append (S.slice sd lo k) (S.slice ss i mid)) (S.slice ss j hi))
                                  (S.slice ss lo hi))
    (ensures
      (let sd' = S.upd sd k w in
       P.permutation U32.t (S.append (S.append (S.slice sd' lo (k+1)) (S.slice ss i mid)) (S.slice ss (j+1) hi))
                           (S.slice ss lo hi)))
= let sd' = S.upd sd k w in
  let acc = S.slice sd lo k in
  let r1 = S.slice ss i mid in
  let r2 = S.slice ss (j+1) hi in
  S.lemma_eq_intro (S.slice sd' lo (k+1)) (S.snoc acc w);
  S.lemma_eq_intro (S.slice ss j hi) (S.cons w r2);
  let aux (y: U32.t) : Lemma (P.count y (S.append (S.append (S.slice sd' lo (k+1)) r1) r2) ==
                              P.count y (S.append (S.append acc r1) (S.slice ss j hi))) =
    count_app y (S.append (S.snoc acc w) r1) r2; count_app y (S.snoc acc w) r1; count_snoc y acc w;
    count_app y (S.append acc r1) (S.cons w r2); count_app y acc r1; count_cons y w r2
  in
  FStar.Classical.forall_intro aux

(* ------------------------------------------------------------------ invariant de la fusion *)

(* sd = destination, ss = source ; i, j parcourent les deux runs de ss, k remplit sd. *)
[@@"opaque_to_smt"]
let merge_inv (sd ss: seq32) (lo mid hi i j k: nat) : prop =
  lo <= i /\ i <= mid /\ mid <= j /\ j <= hi /\ hi <= S.length ss /\ hi <= S.length sd /\
  k + mid == i + j /\ k <= hi /\
  sorted_range sd lo k /\
  (forall (p q: nat). lo <= p /\ p < k /\ i <= q /\ q < mid ==> v (S.index sd p) <= v (S.index ss q)) /\
  (forall (p q: nat). lo <= p /\ p < k /\ j <= q /\ q < hi ==> v (S.index sd p) <= v (S.index ss q)) /\
  P.permutation U32.t (S.append (S.append (S.slice sd lo k) (S.slice ss i mid)) (S.slice ss j hi))
                      (S.slice ss lo hi)

let frame_len (a b: seq32) (lo hi: nat) : Lemma (requires frame a b lo hi) (ensures S.length a == S.length b) = ()
let frame_refl (a: seq32) (lo hi: nat) : Lemma (frame a a lo hi) = ()
let frame_upd (a b: seq32) (lo hi k: nat) (w: U32.t)
  : Lemma (requires frame a b lo hi /\ lo <= k /\ k < hi /\ k < S.length a)
          (ensures frame (S.upd a k w) b lo hi) = ()
let perm_range_refl (a: seq32) (lo hi: nat)
  : Lemma (requires lo <= hi /\ hi <= S.length a) (ensures perm_range a a lo hi) = ()
let sorted_small (s: seq32) (lo hi: nat)
  : Lemma (requires lo <= hi /\ hi <= S.length s /\ hi - lo < 2) (ensures sorted_range s lo hi) = ()
let slice_eq_intro (a b: seq32) (lo hi: nat)
  : Lemma (requires lo <= hi /\ hi <= S.length a /\ hi <= S.length b /\
                    (forall (p: nat). lo <= p /\ p < hi ==> S.index a p == S.index b p))
          (ensures slice_eq a b lo hi)
= S.lemma_eq_intro (S.slice a lo hi) (S.slice b lo hi)

let merge_init (sd ss: seq32) (lo mid hi: nat)
  : Lemma (requires lo <= mid /\ mid <= hi /\ hi <= S.length ss /\ hi <= S.length sd)
          (ensures merge_inv sd ss lo mid hi lo mid lo)
= reveal_opaque (`%merge_inv) merge_inv;
  S.lemma_eq_intro (S.slice sd lo lo) S.empty;
  S.lemma_eq_intro (S.append (S.append (S.slice sd lo lo) (S.slice ss lo mid)) (S.slice ss mid hi)) (S.slice ss lo hi)

let merge_step_left (sd ss: seq32) (lo mid hi i j k: nat)
  : Lemma
    (requires merge_inv sd ss lo mid hi i j k /\ sorted_range ss lo mid /\ sorted_range ss mid hi /\
              i < mid /\ k < hi /\ k < S.length sd /\ i < S.length ss /\ (j >= hi \/ (j < S.length ss /\ v (S.index ss i) <= v (S.index ss j))))
    (ensures merge_inv (S.upd sd k (S.index ss i)) ss lo mid hi (i+1) j (k+1))
= reveal_opaque (`%merge_inv) merge_inv;
  let w = S.index ss i in
  let sd' = S.upd sd k w in
  step_left sd ss lo k i mid j hi w;
  assert (sorted_range sd' lo (k+1));
  assert (forall (p q: nat). lo <= p /\ p < k + 1 /\ i + 1 <= q /\ q < mid ==> v (S.index sd' p) <= v (S.index ss q));
  assert (forall (p q: nat). lo <= p /\ p < k + 1 /\ j <= q /\ q < hi ==> v (S.index sd' p) <= v (S.index ss q))

let merge_step_right (sd ss: seq32) (lo mid hi i j k: nat)
  : Lemma
    (requires merge_inv sd ss lo mid hi i j k /\ sorted_range ss lo mid /\ sorted_range ss mid hi /\
              j < hi /\ k < hi /\ k < S.length sd /\ j < S.length ss /\ (i >= mid \/ (i < S.length ss /\ v (S.index ss j) < v (S.index ss i))))
    (ensures merge_inv (S.upd sd k (S.index ss j)) ss lo mid hi i (j+1) (k+1))
= reveal_opaque (`%merge_inv) merge_inv;
  let w = S.index ss j in
  let sd' = S.upd sd k w in
  step_right sd ss lo k i mid j hi w;
  assert (sorted_range sd' lo (k+1));
  assert (forall (p q: nat). lo <= p /\ p < k + 1 /\ i <= q /\ q < mid ==> v (S.index sd' p) <= v (S.index ss q));
  assert (forall (p q: nat). lo <= p /\ p < k + 1 /\ j + 1 <= q /\ q < hi ==> v (S.index sd' p) <= v (S.index ss q))

(* Pas unique, paramétré par le choix runtime : évite tout branchement autour des lemmes. *)
let merge_step (sd ss: seq32) (lo mid hi i j k: nat) (take_left: bool)
  : Lemma
    (requires merge_inv sd ss lo mid hi i j k /\ sorted_range ss lo mid /\ sorted_range ss mid hi /\
              k < hi /\ k < S.length sd /\
              (take_left ==> (i < mid /\ i < S.length ss /\ (j >= hi \/ (j < S.length ss /\ v (S.index ss i) <= v (S.index ss j))))) /\
              (~take_left ==> (j < hi /\ j < S.length ss /\ (i >= mid \/ (i < S.length ss /\ v (S.index ss j) < v (S.index ss i))))))
    (ensures
      (take_left ==> merge_inv (S.upd sd k (S.index ss i)) ss lo mid hi (i+1) j (k+1)) /\
      (~take_left ==> merge_inv (S.upd sd k (S.index ss j)) ss lo mid hi i (j+1) (k+1)))
= if take_left then merge_step_left sd ss lo mid hi i j k else merge_step_right sd ss lo mid hi i j k

(* En sortie (k = hi) : i = mid, j = hi, dst[lo,hi) est trié et permutation de src[lo,hi). *)
let merge_done (sd ss: seq32) (lo mid hi i j k: nat)
  : Lemma (requires merge_inv sd ss lo mid hi i j k /\ k == hi)
          (ensures sorted_range sd lo hi /\ perm_range sd ss lo hi)
= reveal_opaque (`%merge_inv) merge_inv;
  S.lemma_eq_intro (S.slice ss mid mid) S.empty;
  S.lemma_eq_intro (S.slice ss hi hi) S.empty;
  S.lemma_eq_intro (S.append (S.append (S.slice sd lo hi) (S.slice ss i mid)) (S.slice ss j hi)) (S.slice sd lo hi)

(* ------------------------------------------------------------------ récursion du tri *)

let frame_trans (a b c: seq32) (lo1 hi1 lo2 hi2: nat)
  : Lemma (requires frame a b lo1 hi1 /\ frame b c lo2 hi2 /\ hi1 <= lo2)
          (ensures True) = ()

(* Un run trié dont la plage est hors d'un cadre reste trié. *)
let sorted_frame (a b: seq32) (lo hi flo fhi: nat)
  : Lemma (requires sorted_range b lo hi /\ frame a b flo fhi /\ (hi <= flo \/ fhi <= lo) /\ hi <= S.length a)
          (ensures sorted_range a lo hi) = ()

(* Hors du cadre, les tranches sont égales. *)
let slice_frame (a b: seq32) (lo hi flo fhi: nat)
  : Lemma (requires frame a b flo fhi /\ (hi <= flo \/ fhi <= lo) /\ lo <= hi /\ hi <= S.length a)
          (ensures S.slice a lo hi == S.slice b lo hi)
= S.lemma_eq_intro (S.slice a lo hi) (S.slice b lo hi)

let perm_split (a b: seq32) (lo mid hi: nat)
  : Lemma (requires lo <= mid /\ mid <= hi /\ hi <= S.length a /\ hi <= S.length b /\
                    P.permutation U32.t (S.slice a lo mid) (S.slice b lo mid) /\
                    P.permutation U32.t (S.slice a mid hi) (S.slice b mid hi))
          (ensures perm_range a b lo hi)
= P.append_permutations (S.slice a lo mid) (S.slice a mid hi) (S.slice b lo mid) (S.slice b mid hi);
  S.lemma_eq_intro (S.append (S.slice a lo mid) (S.slice a mid hi)) (S.slice a lo hi);
  S.lemma_eq_intro (S.append (S.slice b lo mid) (S.slice b mid hi)) (S.slice b lo hi)

(* Un run trié, hors du cadre de modification, reste trié. *)
let sorted_frame_l (a b: seq32) (lo hi flo fhi: nat)
  : Lemma (requires lo <= hi /\ hi <= S.length a /\ sorted_range b lo hi /\ frame a b flo fhi /\ (hi <= flo \/ fhi <= lo))
          (ensures sorted_range a lo hi)
= ()

let sorted_of_slice_eq (a t: seq32) (lo hi: nat)
  : Lemma (requires lo <= hi /\ hi <= S.length a /\ hi <= S.length t /\ sorted_range t lo hi /\ S.slice a lo hi == S.slice t lo hi)
          (ensures sorted_range a lo hi)
= let aux (p: nat) : Lemma (requires lo <= p /\ p < hi)
                           (ensures p < S.length a /\ p < S.length t /\ S.index a p == S.index t p) =
    assert (S.index (S.slice a lo hi) (p - lo) == S.index a p);
    assert (S.index (S.slice t lo hi) (p - lo) == S.index t p)
  in
  FStar.Classical.forall_intro (FStar.Classical.move_requires aux)

(* Bilan de msort sur [lo,hi) : o = état initial, s1/s2 après chaque moitié, t = tmp après fusion, a3 = résultat. *)
let msort_combine (o s1 s2 t a3: seq32) (lo mid hi: nat)
  : Lemma
    (requires lo <= mid /\ mid <= hi /\ hi <= S.length o /\
              S.length o == S.length s1 /\ S.length s1 == S.length s2 /\ S.length s2 == S.length a3 /\ hi <= S.length t /\
              sorted_range s1 lo mid /\ perm_range s1 o lo mid /\ frame s1 o lo mid /\
              sorted_range s2 mid hi /\ perm_range s2 s1 mid hi /\ frame s2 s1 mid hi /\
              sorted_range t lo hi /\ perm_range t s2 lo hi /\
              frame a3 s2 lo hi /\ slice_eq a3 t lo hi)
    (ensures sorted_range a3 lo hi /\ perm_range a3 o lo hi /\ frame a3 o lo hi)
= // permutation : a3[lo,hi) = t[lo,hi) ~ s2[lo,hi) ~ o[lo,hi)
  slice_frame s2 s1 lo mid mid hi;
  slice_frame s1 o mid hi lo mid;
  perm_split s2 o lo mid hi;
  assert (P.permutation U32.t (S.slice a3 lo hi) (S.slice t lo hi));
  assert (P.permutation U32.t (S.slice t lo hi) (S.slice s2 lo hi));
  assert (P.permutation U32.t (S.slice s2 lo hi) (S.slice o lo hi));
  sorted_of_slice_eq a3 t lo hi


