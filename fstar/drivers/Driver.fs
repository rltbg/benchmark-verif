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

/// Workloads parallèles : les threads sont ici (code non prouvé) ; les noyaux Pulse sont bench_Par_mat_rows, bench_Sort_msort/merge/copy_back, bench_Par_sha_seg.
let parDo (threads: int) (f: int -> unit) =
    let t = max 1 (min threads 64)
    let ths = [| for k in 1 .. t - 1 -> (let th = System.Threading.Thread((fun () -> f k), 16 * 1024 * 1024) in th.Start(); th) |]
    f 0
    for th in ths do th.Join()

let parMatmul (n: uint64) (threads: int) : uint64 =
    let nn = n * n
    let a = Custard.bench_Par_u32_alloc nn
    let b = Custard.bench_Par_u32_alloc nn
    let c = Custard.bench_Par_u32_alloc nn
    Custard.bench_Matmul_init_mat a nn 0UL
    Custard.bench_Matmul_init_mat b nn nn
    let t = max 1 threads
    parDo t (fun k -> Custard.bench_Par_mat_rows a b c n (n * uint64 k / uint64 t) (n * uint64 (k + 1) / uint64 t))
    Custard.bench_Matmul_checksum c nn

let parSort (n: uint64) (threads: int) : uint64 =
    let t = max 1 (min threads 64)
    let a = Custard.bench_Par_u32_alloc n
    let tmp = Custard.bench_Par_u32_alloc n
    Custard.bench_Sort_fill a n
    parDo t (fun k -> Custard.bench_Sort_msort a tmp (n * uint64 k / uint64 t) (n * uint64 (k + 1) / uint64 t))
    let mutable bounds = [| for k in 0 .. t -> n * uint64 k / uint64 t |]
    while bounds.Length > 2 do
        let runs = bounds.Length - 1
        let pairs = runs / 2
        let bs = bounds
        parDo pairs (fun p ->
            let lo, mid, hi = bs.[2 * p], bs.[2 * p + 1], bs.[2 * p + 2]
            Custard.bench_Sort_merge a tmp lo mid hi
            Custard.bench_Sort_copy_back tmp a lo hi)
        bounds <- Array.append [| for p in 0 .. pairs - 1 -> bs.[2 * p] |] (Array.append (if runs % 2 = 1 then [| bs.[runs - 1] |] else [||]) [| n |])
    Custard.bench_Sort_checksum a n

let parSha (n: uint64) (threads: int) : uint64 =
    let t = max 1 (min threads 64)
    let m = n / uint64 t
    let h = [| for _ in 0 .. t - 1 -> Custard.bench_Par_u32_alloc 8UL |]
    parDo t (fun k -> Custard.bench_Par_sha_seg (0x5348413235362D42UL + (uint64 k <<< 40)) m h.[k])
    let mutable ck = Custard.bench_Common_ck_init
    for s in 0 .. t - 1 do
        for w in h.[s] do ck <- Custard.bench_Common_ckfold ck (uint64 w)
    ck

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
            | "strsort" -> Custard.bench_Strsort_run n
            | "alloc" -> Custard.bench_Alloc_run n
            | "ntt" -> Custard.bench_Ntt_run n
            | "ptrchase" -> Custard.bench_Ptrchase_run n
            | "sha256" -> Custard.bench_Sha256_run n
            | "parmatmul" -> parMatmul n threads
            | "parsort" -> parSort n threads
            | "parsha" -> parSha n threads
            // @@DISPATCH@@ (une ligne `| "<nom>" -> Custard.bench_<Nom>_run n` ; si le noyau est parallélisé par le pilote : Tasks/Threads ici, `threads` disponible)
            | _ -> failwith "workload inconnu"
        let ns = int64 (float sw.ElapsedTicks * 1e9 / float Stopwatch.Frequency)
        printfn "RESULT workload=%s n=%d checksum=%016x time_ns=%d" argv.[0] n ck ns
        0
