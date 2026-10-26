// Driver F# pour les modules extraits par `fstar.exe --codegen FSharp` (EXPÉRIMENTAL).
module Driver

open System.Diagnostics

[<EntryPoint>]
let main argv =
    if argv.Length <> 2 then
        eprintfn "usage: bench-fstar-fsharp <sort|sieve|matmul|lines|hashmap> <n>"
        2
    else
        let n = uint32 argv.[1]
        let sw = Stopwatch.StartNew()
        let ck : uint64 =
            match argv.[0] with
            | "sort" -> Bench.Sort.run n
            | "sieve" -> Bench.Sieve.run n
            | "matmul" -> Bench.Matmul.run n
            | "lines" -> Bench.Lines.run n
            | "hashmap" -> Bench.Hashmap.run n
            | _ -> failwith "workload inconnu"
        let ns = int64 (float sw.ElapsedTicks * 1e9 / float Stopwatch.Frequency)
        printfn "RESULT workload=%s n=%d checksum=%016x time_ns=%d" argv.[0] n ck ns
        0
