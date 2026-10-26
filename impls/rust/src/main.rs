mod workloads;

use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: {} <sort|sieve|matmul|lines|hashmap> <n>", args[0]);
        std::process::exit(2);
    }
    let n: usize = args[2].parse().expect("n invalide");
    let t0 = Instant::now();
    let ck = match args[1].as_str() {
        "sort" => workloads::run_sort(n),
        "sieve" => workloads::run_sieve(n),
        "matmul" => workloads::run_matmul(n),
        "lines" => workloads::run_lines(n),
        "hashmap" => workloads::run_hashmap(n),
        _ => {
            eprintln!("workload inconnu");
            std::process::exit(2);
        }
    };
    let ns = t0.elapsed().as_nanos();
    println!("RESULT workload={} n={} checksum={:016x} time_ns={}", args[1], n, ck, ns);
}
