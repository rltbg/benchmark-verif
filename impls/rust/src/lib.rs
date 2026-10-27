//! Les cinq workloads de `spec/WORKLOADS.md`, vérifiés avec Creusot.
//!
//! * Absence de panique (indices dans les bornes) et de débordement arithmétique (hors `wrapping_*`)
//!   pour toutes les fonctions : c'est vérifié par Creusot, pas supposé.
//! * Les corrections fonctionnelles prouvées sont indiquées dans l'en-tête de chaque module.
//! * Un `cargo build` normal efface tous les contrats : le code compilé est du Rust ordinaire.
pub mod common;
pub mod hashmap;
pub mod lines;
pub mod matmul;
pub mod sieve;
pub mod sort;
// @@MODS@@ (une ligne `pub mod <nom>;` ; chaque module expose `pub fn run(n: usize, threads: usize) -> u64`, contrats Creusot sur les noyaux)
