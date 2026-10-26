// Port F# idiomatique (impératif, tableaux .NET), écrit à la main et non vérifié.
// Contrôle pour la colonne « F* -> F# » : plancher de ce que .NET sait faire.
module Bench

open System
open System.Diagnostics

let inline mix64 (x: uint64) : uint64 =
    let z = x + 0x9E3779B97F4A7C15UL
    let z = (z ^^^ (z >>> 30)) * 0xBF58476D1CE4E5B9UL
    let z = (z ^^^ (z >>> 27)) * 0x94D049BB133111EBUL
    z ^^^ (z >>> 31)

let ckInit = 0xCBF29CE484222325UL
let inline fold (ck: uint64) (v: uint64) : uint64 = (ck ^^^ v) * 0x100000001B3UL

let sSort = 0x0123456789ABCDEFUL
let sMat = 0x0FEDCBA987654321UL
let sLine = 0x1357924680ACEBDFUL
let sHash = 0x2468ACE013579BDFUL

let runSort (n: int) : uint64 =
    let buf : uint32[] = Array.zeroCreate (2 * n)
    for i in 0 .. n - 1 do
        buf.[i] <- uint32 (mix64 (sSort + uint64 i) >>> 32)
    let mutable so = 0
    let mutable d = n
    let mutable w = 1
    while w < n do
        let mutable lo = 0
        while lo < n do
            let mid = if lo + w < n then lo + w else n
            let hi = if lo + 2 * w < n then lo + 2 * w else n
            let mutable i = lo
            let mutable j = mid
            let mutable k = lo
            while i < mid && j < hi do
                let x = buf.[so + i]
                let y = buf.[so + j]
                if x <= y then
                    buf.[d + k] <- x
                    i <- i + 1
                else
                    buf.[d + k] <- y
                    j <- j + 1
                k <- k + 1
            while i < mid do
                buf.[d + k] <- buf.[so + i]
                i <- i + 1
                k <- k + 1
            while j < hi do
                buf.[d + k] <- buf.[so + j]
                j <- j + 1
                k <- k + 1
            lo <- lo + 2 * w
        let t = so
        so <- d
        d <- t
        w <- w * 2
    let mutable ck = ckInit
    let mutable ok = true
    for i in 0 .. n - 1 do
        if i > 0 && buf.[so + i - 1] > buf.[so + i] then ok <- false
        ck <- fold ck (uint64 buf.[so + i])
    if ok then ck else 0UL

let runSieve (n: int) : uint64 =
    let f : byte[] = Array.zeroCreate (n + 1)
    let mutable i = 2L
    while i * i <= int64 n do
        if f.[int i] = 0uy then
            let mutable j = int (i * i)
            while j <= n do
                f.[j] <- 1uy
                j <- j + int i
        i <- i + 1L
    let mutable ck = ckInit
    let mutable cnt = 0UL
    for i in 2 .. n do
        if f.[i] = 0uy then
            ck <- fold ck (uint64 i)
            cnt <- cnt + 1UL
    fold ck cnt

let runMatmul (n: int) : uint64 =
    let nn = n * n
    let a : uint32[] = Array.zeroCreate nn
    let b : uint32[] = Array.zeroCreate nn
    let c : uint32[] = Array.zeroCreate nn
    for i in 0 .. nn - 1 do
        a.[i] <- uint32 (mix64 (sMat + uint64 i) >>> 32)
        b.[i] <- uint32 (mix64 (sMat + uint64 (nn + i)) >>> 32)
    for i in 0 .. n - 1 do
        for k in 0 .. n - 1 do
            let aik = a.[i * n + k]
            for j in 0 .. n - 1 do
                c.[i * n + j] <- c.[i * n + j] + aik * b.[k * n + j]
    let mutable ck = ckInit
    for i in 0 .. nn - 1 do
        ck <- fold ck (uint64 c.[i])
    ck

let lineK = 1 <<< 20
let lineMax = 15

let putDec (buf: byte[]) (off: int) (v: uint64) : int =
    let mutable l = 1
    let mutable t = v
    while t >= 10UL do
        l <- l + 1
        t <- t / 10UL
    t <- v
    for i in l - 1 .. -1 .. 0 do
        buf.[off + i] <- byte (48UL + t % 10UL)
        t <- t / 10UL
    off + l

let runLines (n: int) : uint64 =
    let txt : byte[] = Array.zeroCreate (n * lineMax)
    let mutable len = 0
    for i in 0 .. n - 1 do
        let r = mix64 (sLine + uint64 i)
        let key = r >>> 44
        let v = ((r >>> 8) &&& 0xFFFFFFFFUL) % 1000000UL
        len <- putDec txt len key
        txt.[len] <- byte ';'
        len <- len + 1
        len <- putDec txt len v
        txt.[len] <- byte '\n'
        len <- len + 1
    let tab : uint64[] = Array.zeroCreate (2 * lineK)
    let mutable p = 0
    while p < len do
        let mutable key = 0
        let mutable v = 0UL
        while txt.[p] <> byte ';' do
            key <- key * 10 + int (txt.[p] - 48uy)
            p <- p + 1
        p <- p + 1
        while txt.[p] <> byte '\n' do
            v <- v * 10UL + uint64 (txt.[p] - 48uy)
            p <- p + 1
        p <- p + 1
        tab.[2 * key] <- tab.[2 * key] + v
        tab.[2 * key + 1] <- tab.[2 * key + 1] + 1UL
    let mutable ck = ckInit
    for k in 0 .. lineK - 1 do
        ck <- fold (fold ck tab.[2 * k]) tab.[2 * k + 1]
    ck

let runHashmap (n: int) : uint64 =
    let mutable lg = 1
    while (1L <<< lg) < 2L * int64 n do
        lg <- lg + 1
    let cap = 1 <<< lg
    let mask = cap - 1
    let t : uint64[] = Array.zeroCreate (2 * cap)
    let inline slot (key: uint64) = int ((key * 0x9E3779B97F4A7C15UL) >>> (64 - lg))
    for i in 0 .. n - 1 do
        let key = mix64 (sHash + uint64 i) ||| 1UL
        let mutable s = slot key
        let mutable fin = false
        while not fin do
            let k = t.[2 * s]
            if k = 0UL then
                t.[2 * s] <- key
                t.[2 * s + 1] <- uint64 i + 1UL
                fin <- true
            elif k = key then
                t.[2 * s + 1] <- t.[2 * s + 1] + uint64 i + 1UL
                fin <- true
            else
                s <- (s + 1) &&& mask
    let mutable ck = ckInit
    for phase in 0 .. 1 do
        for i in 0 .. n - 1 do
            let off = if phase = 1 then n else 0
            let key = mix64 (sHash + uint64 (off + i)) ||| 1UL
            let mutable s = slot key
            let mutable v = 0UL
            let mutable fin = false
            while not fin do
                let k = t.[2 * s]
                if k = 0UL then fin <- true
                elif k = key then
                    v <- t.[2 * s + 1]
                    fin <- true
                else
                    s <- (s + 1) &&& mask
            ck <- fold ck v
    ck

[<EntryPoint>]
let main argv =
    if argv.Length <> 2 then
        eprintfn "usage: bench-fsharp <sort|sieve|matmul|lines|hashmap> <n>"
        2
    else
        let n = int argv.[1]
        let sw = Stopwatch.StartNew()
        let ck =
            match argv.[0] with
            | "sort" -> runSort n
            | "sieve" -> runSieve n
            | "matmul" -> runMatmul n
            | "lines" -> runLines n
            | "hashmap" -> runHashmap n
            | _ -> failwith "workload inconnu"
        let ns = int64 (float sw.ElapsedTicks * 1e9 / float Stopwatch.Frequency)
        printfn "RESULT workload=%s n=%d checksum=%016x time_ns=%d" argv.[0] n ck ns
        0
