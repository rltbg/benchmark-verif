//! Les cinq workloads de `spec/WORKLOADS.md`, vérifiés avec Creusot.
//!
//! * Absence de panique (indices dans les bornes) et de débordement arithmétique (hors `wrapping_*`)
//!   pour toutes les fonctions : c'est vérifié par Creusot, pas supposé.
//! * Les corrections fonctionnelles prouvées sont indiquées dans l'en-tête de chaque module.
//! * Un `cargo build` normal efface tous les contrats : le code compilé est du Rust ordinaire.
pub mod bfs;
pub mod common;
pub mod hashmap;
pub mod lines;
pub mod matmul;
pub mod sieve;
pub mod sort;
pub mod sha256;
pub mod ptrchase;
pub mod stream;
pub mod ntt;
pub mod parmatmul;
pub mod parsort;
pub mod parsha;
pub mod strsort;
pub mod alloc;
// @@MODS@@ (une ligne `pub mod <nom>;` ; chaque module expose `pub fn run(n: usize, threads: usize) -> u64`, contrats Creusot sur les noyaux)
