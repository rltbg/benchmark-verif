(* Driver OCaml pour les modules extraits par `fstar.exe --codegen OCaml`. *)
let () =
  if Array.length Sys.argv <> 3 then (prerr_endline "usage: bench <sort|sieve|matmul|lines|hashmap> <n>"; exit 2);
  let n = int_of_string Sys.argv.(2) in
  let n32 = Stdint.Uint32.of_int n in
  let t0 = Unix.gettimeofday () in
  let ck =
    match Sys.argv.(1) with
    | "sort" -> Bench_Sort.run n32
    | "sieve" -> Bench_Sieve.run n32
    | "matmul" -> Bench_Matmul.run n32
    | "lines" -> Bench_Lines.run n32
    | "hashmap" -> Bench_Hashmap.run n32
    | _ -> prerr_endline "workload inconnu"; exit 2 in
  let ns = Int64.of_float ((Unix.gettimeofday () -. t0) *. 1e9) in
  Printf.printf "RESULT workload=%s n=%d checksum=%016Lx time_ns=%Ld\n" Sys.argv.(1) n
    (Stdint.Uint64.to_int64 ck) ns
