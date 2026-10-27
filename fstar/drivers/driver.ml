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
    (* @@DISPATCH@@ (une ligne `| "<nom>" -> Bench_<Nom>.run n` ; Domain.spawn possible, `threads` disponible) *)
    | _ -> prerr_endline "workload inconnu"; exit 2 in
  let ns = Int64.of_float ((Unix.gettimeofday () -. t0) *. 1e9) in
  Printf.printf "RESULT workload=%s n=%s checksum=%s time_ns=%Ld\n" Sys.argv.(1) Sys.argv.(2)
    (Z.format "%016x" (FStar_UInt64.v ck)) ns
