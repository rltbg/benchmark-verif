module Bench.All
(* Module d'agrégation : charge les cinq workloads pour qu'une seule extraction « programme entier »
   (--codegen C|OCaml|FSharp) les contienne tous. Les points d'entrée sont passés à F* par
   --custard_entry (voir build.sh). *)
let sort = Bench.Sort.run
let sieve = Bench.Sieve.run
let matmul = Bench.Matmul.run
let lines = Bench.Lines.run
let hashmap = Bench.Hashmap.run
(* @@ALL@@ une ligne `let <nom> = Bench.<Nom>.run` par workload étendu *)
