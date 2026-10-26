// Pilote F# pour le programme extrait par `fstar.exe --codegen FSharp` (même format de sortie).
module Driver

open System.Diagnostics

[<EntryPoint>]
let main argv =
    if argv.Length <> 2 then
        eprintfn "usage: bench-fstar-fsharp <sort|sieve|matmul|lines|hashmap> <n>"
        2
    else
        let n = uint64 argv.[1]
        let sw = Stopwatch.StartNew()
        let ck : uint64 =
            match argv.[0] with
            | "sort" -> Custard.bench_Sort_run n
            | "sieve" -> Custard.bench_Sieve_run n
            | "matmul" -> Custard.bench_Matmul_run n
            | "lines" -> Custard.bench_Lines_run n
            | "hashmap" -> Custard.bench_Hashmap_run n
            | _ -> failwith "workload inconnu"
        let ns = int64 (float sw.ElapsedTicks * 1e9 / float Stopwatch.Frequency)
        printfn "RESULT workload=%s n=%d checksum=%016x time_ns=%d" argv.[0] n ck ns
        0
