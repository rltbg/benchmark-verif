(* Pilote OCaml pour le programme extrait par `fstar.exe --codegen OCaml` (même format de sortie). *)
let () =
  if Array.length Sys.argv <> 3 then (prerr_endline "usage: bench <sort|sieve|matmul|lines|hashmap> <n>"; exit 2);
  let n = FStar_SizeT.uint_to_t (Z.of_string Sys.argv.(2)) in
  let t0 = Unix.gettimeofday () in
  let ck =
    match Sys.argv.(1) with
    | "sort" -> Bench_Sort.run n
    | "sieve" -> Bench_Sieve.run n
    | "matmul" -> Bench_Matmul.run n
    | "lines" -> Bench_Lines.run n
    | "hashmap" -> Bench_Hashmap.run n
    | _ -> prerr_endline "workload inconnu"; exit 2 in
  let ns = Int64.of_float ((Unix.gettimeofday () -. t0) *. 1e9) in
  Printf.printf "RESULT workload=%s n=%s checksum=%s time_ns=%Ld\n" Sys.argv.(1) Sys.argv.(2)
    (Z.format "%016x" (FStar_UInt64.v ck)) ns
