// Pilote F# pour le programme extrait par `fstar.exe --codegen FSharp` (même format de sortie).
module Driver

open System.Diagnostics

/// Workload `stream` : lecture du fichier par blocs de 1 Mio (non prouvé) + noyau Pulse `bench_Stream_chunk`.
let streamRun (nMio: uint64) : uint64 =
    let dir = match System.Environment.GetEnvironmentVariable "BENCH_DATA_DIR" with | null -> "/var/tmp/bench-data" | d -> d
    use f = new System.IO.FileStream(sprintf "%s/stream-%d.json" dir nMio, System.IO.FileMode.Open, System.IO.FileAccess.Read, System.IO.FileShare.Read, 0)
    let buf = Custard.bench_Stream_new_buf ()
    let st = Custard.bench_Stream_new_state ()
    let tab = Custard.bench_Stream_new_table ()
    let mutable go = true
    while go do
        let got = f.Read(buf, 0, buf.Length)
        if got = 0 then go <- false else Custard.bench_Stream_chunk buf (uint64 got) st tab
    Custard.bench_Stream_finish st

[<EntryPoint>]
let main argv =
    if argv.Length < 2 || argv.Length > 3 then
        eprintfn "usage: bench-fstar-fsharp <workload> <n> [threads]"
        2
    else
        let n = uint64 argv.[1]
        let threads = if argv.Length = 3 then int argv.[2] else 1
        ignore threads
        let sw = Stopwatch.StartNew()
        let ck : uint64 =
            match argv.[0] with
            | "sort" -> Custard.bench_Sort_run n
            | "sieve" -> Custard.bench_Sieve_run n
            | "matmul" -> Custard.bench_Matmul_run n
            | "lines" -> Custard.bench_Lines_run n
            | "hashmap" -> Custard.bench_Hashmap_run n
            | "stream" -> streamRun n
            | "bfs" -> Custard.bench_Bfs_run n
            // @@DISPATCH@@ (une ligne `| "<nom>" -> Custard.bench_<Nom>_run n` ; si le noyau est parallélisé par le pilote : Tasks/Threads ici, `threads` disponible)
            | _ -> failwith "workload inconnu"
        let ns = int64 (float sw.ElapsedTicks * 1e9 / float Stopwatch.Frequency)
        printfn "RESULT workload=%s n=%d checksum=%016x time_ns=%d" argv.[0] n ck ns
        0
