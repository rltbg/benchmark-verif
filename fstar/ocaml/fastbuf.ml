(* Réalisation OCaml *native* des buffers Low* (tableau OCaml + décalage), O(1) par accès.
   La réalisation livrée dans ulib (LowStar_Monotonic_Buffer.ml) stocke le contenu dans une
   `FStar_Seq_Base.seq`, qui est une *liste chaînée* : index/upd y coûtent O(n).
   Ce module est substitué par `FASTBUF=1 fstar/build.sh ocaml` (réécriture des références
   LowStar_Buffer.* / LowStar_Monotonic_Buffer.* du code extrait).
   Il garde volontairement la représentation générique 'a array : les u8/u32/u64 restent des
   Stdint boxés, comme pour tout code OCaml extrait de F*. *)

type 'a buffer = { arr : 'a array; off : int }

let to_i = Stdint.Uint32.to_int

let mmalloc _r init len = { arr = Array.make (to_i len) init; off = 0 }
let mgcmalloc = mmalloc
let malloc _r _ init len = mmalloc () init len
let gcmalloc = malloc
let free _ = ()
let index b i = Array.unsafe_get b.arr (b.off + to_i i)
let upd' b i v = Array.unsafe_set b.arr (b.off + to_i i) v
let upd = upd'
let op_Array_Access = index
let op_Array_Assignment = upd
let sub b i _len = { b with off = b.off + to_i i }
let offset b i = { b with off = b.off + to_i i }
