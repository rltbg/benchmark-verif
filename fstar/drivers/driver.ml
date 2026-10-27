(* Pilote OCaml pour le programme extrait par `fstar.exe --codegen OCaml` (même format de sortie). *)
(* Workload `stream` : lecture du fichier par blocs de 1 Mio (non prouvé) puis noyau Pulse `Bench_Stream.chunk`.
   Le tampon extrait est un `FStar_UInt8.t array` (FStar_UInt8.t = int) : on y copie les octets lus. *)
let stream_run (n_mio : string) : FStar_UInt64.t =
  let dir = try Sys.getenv "BENCH_DATA_DIR" with Not_found -> "/var/tmp/bench-data" in
  let fd = Unix.openfile (Printf.sprintf "%s/stream-%s.json" dir n_mio) [Unix.O_RDONLY] 0 in
  let buf = Bench_Stream.new_buf () in
  let st = Bench_Stream.new_state () in
  let tab = Bench_Stream.new_table () in
  let raw = Bytes.create 1048576 in
  let rec loop () =
    let got = Unix.read fd raw 0 1048576 in
    if got > 0 then begin
      for i = 0 to got - 1 do buf.(i) <- Char.code (Bytes.unsafe_get raw i) done;
      Bench_Stream.chunk buf (FStar_SizeT.uint_to_t (Z.of_int got)) st tab;
      loop ()
    end in
  loop ();
  Unix.close fd;
  Bench_Stream.finish st

(* Workloads parallèles : les threads (Domain.spawn) sont ici, dans le code non prouvé ; les noyaux sont ceux vérifiés par F*. *)
let par_sz z = FStar_SizeT.uint_to_t z
let par_do threads f =
  let t = max 1 (min threads 64) in
  let ds = List.init (t - 1) (fun k -> Domain.spawn (fun () -> f (k + 1))) in
  f 0; List.iter Domain.join ds

let par_matmul nz threads =
  let nn = Z.mul nz nz in
  let a = Bench_Par.u32_alloc (par_sz nn) and b = Bench_Par.u32_alloc (par_sz nn) and c = Bench_Par.u32_alloc (par_sz nn) in
  Bench_Matmul.init_mat a (par_sz nn) (FStar_UInt64.uint_to_t Z.zero);
  Bench_Matmul.init_mat b (par_sz nn) (FStar_UInt64.uint_to_t nn);
  let t = max 1 threads in
  let rowb k = par_sz (Z.div (Z.mul nz (Z.of_int k)) (Z.of_int t)) in
  par_do t (fun k -> Bench_Par.mat_rows a b c (par_sz nz) (rowb k) (rowb (k + 1)));
  Bench_Matmul.checksum c (par_sz nn)

let par_sort nz threads =
  let t = max 1 (min threads 64) in
  let a = Bench_Par.u32_alloc (par_sz nz) and tmp = Bench_Par.u32_alloc (par_sz nz) in
  Bench_Sort.fill a (par_sz nz);
  let bz k = Z.div (Z.mul nz (Z.of_int k)) (Z.of_int t) in
  par_do t (fun k -> Bench_Sort.msort a tmp (par_sz (bz k)) (par_sz (bz (k + 1))));
  let bounds = ref (Array.init (t + 1) bz) in
  while Array.length !bounds > 2 do
    let bs = !bounds in
    let runs = Array.length bs - 1 in
    let pairs = runs / 2 in
    par_do pairs (fun p ->
      let lo = par_sz bs.(2 * p) and mid = par_sz bs.(2 * p + 1) and hi = par_sz bs.(2 * p + 2) in
      Bench_Sort.merge a tmp lo mid hi;
      Bench_Sort.copy_back tmp a lo hi);
    bounds := Array.concat [Array.init pairs (fun p -> bs.(2 * p)); (if runs mod 2 = 1 then [| bs.(runs - 1) |] else [||]); [| nz |]]
  done;
  Bench_Sort.checksum a (par_sz nz)

let par_sha nz threads =
  let t = max 1 (min threads 64) in
  let m = par_sz (Z.div nz (Z.of_int t)) in
  let h = Array.init t (fun _ -> Bench_Par.u32_alloc (par_sz (Z.of_int 8))) in
  let base = Z.of_string "0x5348413235362D42" in
  par_do t (fun k -> Bench_Par.sha_seg (FStar_UInt64.uint_to_t (Z.add base (Z.shift_left (Z.of_int k) 40))) m h.(k));
  let ck = ref Bench_Common.ck_init in
  Array.iter (fun hk -> Array.iter (fun w -> ck := Bench_Common.ckfold !ck (FStar_UInt64.uint_to_t (FStar_UInt32.v w))) hk) h;
  !ck

let () =
  if Array.length Sys.argv < 3 || Array.length Sys.argv > 4 then (prerr_endline "usage: bench <workload> <n> [threads]"; exit 2);
  let threads = if Array.length Sys.argv = 4 then int_of_string Sys.argv.(3) else 1 in ignore threads;
  let n = FStar_SizeT.uint_to_t (Z.of_string Sys.argv.(2)) in
  let t0 = Unix.gettimeofday () in
  let ck =
    match Sys.argv.(1) with
    | "sort" -> Bench_Sort.run n
    | "sieve" -> Bench_Sieve.run n
    | "matmul" -> Bench_Matmul.run n
    | "lines" -> Bench_Lines.run n
    | "hashmap" -> Bench_Hashmap.run n
    | "stream" -> stream_run Sys.argv.(2)
    | "bfs" -> Bench_Bfs.run n
    | "strsort" -> Bench_Strsort.run n
    | "alloc" -> Bench_Alloc.run n
    | "ntt" -> Bench_Ntt.run n
    | "ptrchase" -> Bench_Ptrchase.run n
    | "sha256" -> Bench_Sha256.run n
    | "parmatmul" -> par_matmul (Z.of_string Sys.argv.(2)) threads
    | "parsort" -> par_sort (Z.of_string Sys.argv.(2)) threads
    | "parsha" -> par_sha (Z.of_string Sys.argv.(2)) threads
    (* @@DISPATCH@@ (une ligne `| "<nom>" -> Bench_<Nom>.run n` ; Domain.spawn possible, `threads` disponible) *)
    | _ -> prerr_endline "workload inconnu"; exit 2 in
  let ns = Int64.of_float ((Unix.gettimeofday () -. t0) *. 1e9) in
  Printf.printf "RESULT workload=%s n=%s checksum=%s time_ns=%Ld\n" Sys.argv.(1) Sys.argv.(2)
    (Z.format "%016x" (FStar_UInt64.v ck)) ns
