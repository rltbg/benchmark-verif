module Bench.All
(* Module d'agrégation : charge les cinq workloads pour qu'une seule extraction « programme entier »
   (--codegen C|OCaml|FSharp) les contienne tous. Les points d'entrée sont passés à F* par
   --custard_entry (voir build.sh). *)
let sort = Bench.Sort.run
let sieve = Bench.Sieve.run
let matmul = Bench.Matmul.run
let lines = Bench.Lines.run
let hashmap = Bench.Hashmap.run
module Stream_ = Bench.Stream  (* noyau de `stream` : chunk/new_buf/new_state/new_table/finish sont des --custard_entry *)
let bfs = Bench.Bfs.run
(* @@ALL@@ une ligne `let <nom> = Bench.<Nom>.run` par workload étendu *)
