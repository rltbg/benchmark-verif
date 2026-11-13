//! Point d'entrée : analyse des arguments et chronométrage (code non vérifié, hors `verus!`) ;
//! chaque workload est un module vérifié exposant `pub fn run(n: usize, threads: usize) -> u64`.
use std::time::Instant;

pub mod common;
// @@MODS@@ (une ligne `pub mod <nom>;`)
pub mod ptrchase;
pub mod alloc;
pub mod sieve;
pub mod hashmap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 || args.len() > 4 {
        eprintln!("usage: {} <workload> <n> [threads]", args[0]);
        std::process::exit(2);
    }
    let threads: usize = args.get(3).map_or(1, |t| t.parse().expect("threads"));
    let n: usize = args[2].parse().expect("n invalide");
    let t0 = Instant::now();
    let ck: u64 = match args[1].as_str() {
        // @@DISPATCH@@ (une ligne `"<nom>" => <nom>::run(n, threads),`)
        "ptrchase" => ptrchase::run(n, threads),
        "alloc" => alloc::run(n, threads),
        "sieve" => sieve::run(n, threads),
        "hashmap" => hashmap::run(n, threads),
        _ => {
            eprintln!("workload inconnu");
            std::process::exit(2);
        }
    };
    let _ = (n, threads);
    let ns = t0.elapsed().as_nanos();
    println!("RESULT workload={} n={} checksum={:016x} time_ns={}", args[1], n, ck, ns);
}
