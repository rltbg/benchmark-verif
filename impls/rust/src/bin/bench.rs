use bench_rust::*;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 || args.len() > 4 {
        eprintln!("usage: {} <workload> <n> [threads]", args[0]);
        std::process::exit(2);
    }
    let threads: usize = args.get(3).map_or(1, |t| t.parse().expect("threads"));
    let n: usize = args[2].parse().expect("n invalide");
    let t0 = Instant::now();
    let ck = match args[1].as_str() {
        "sort" => sort::run(n),
        "sieve" => sieve::run(n),
        "matmul" => matmul::run(n),
        "lines" => lines::run(n),
        "hashmap" => hashmap::run(n),
        "sha256" => sha256::run(n, threads),
        "ptrchase" => ptrchase::run(n, threads),
        "stream" => stream::run(n, threads),
        "ntt" => ntt::run(n, threads),
        "parmatmul" => parmatmul::run(n, threads),
        "parsort" => parsort::run(n, threads),
        "parsha" => parsha::run(n, threads),
        "bfs" => bfs::run(n, threads),
        "strsort" => strsort::run(n, threads),
        "alloc" => alloc::run(n),
        // @@DISPATCH@@ (une ligne `"<nom>" => <nom>::run(n, threads),`)
        _ => {
            eprintln!("workload inconnu");
            std::process::exit(2);
        }
    };
    let _ = threads;
    let ns = t0.elapsed().as_nanos();
    println!("RESULT workload={} n={} checksum={:016x} time_ns={}", args[1], n, ck, ns);
}
