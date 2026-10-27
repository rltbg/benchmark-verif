### `xlarge`

| workload (n) | C natif | Zig 0.17 | Rust | Rust + Creusot | F*→C | F*→F# |
|---|--:|--:|--:|--:|--:|--:|
| sort (100 000 000) | 5.40 M/s · 18.54 s · 764 Mo | 5.64 M/s · 17.72 s · 764 Mo | 4.42 M/s · 22.61 s · 765 Mo | 4.47 M/s · 22.35 s · 765 Mo | 5.36 M/s · 18.64 s · 764 Mo | 3.58 M/s · 27.90 s · 792 Mo |
| sieve (3 000 000 000) | 51.30 M/s · 58.48 s · 2862 Mo | 53.38 M/s · 56.20 s · 2863 Mo | 49.32 M/s · 60.83 s · 2863 Mo | 54.21 M/s · 55.34 s · 2863 Mo | 56.02 M/s · 53.55 s · 2862 Mo | **échec** |
| matmul (4 096) | 5.94 G/s · 11.57 s · 193 Mo | 5.09 G/s · 13.50 s · 194 Mo | 4.91 G/s · 13.99 s · 194 Mo | 5.54 G/s · 12.41 s · 194 Mo | 5.13 G/s · 13.40 s · 193 Mo | 877.06 M/s · 78.35 s · 221 Mo |
| lines (100 000 000) | 10.32 M/s · 9.69 s · 1336 Mo | 9.01 M/s · 11.10 s · 1336 Mo | 10.36 M/s · 9.65 s · 1337 Mo | 9.47 M/s · 10.56 s · 1337 Mo | 11.58 M/s · 8.64 s · 1336 Mo | 7.12 M/s · 14.05 s · 1476 Mo |
| hashmap (48 000 000) | 6.07 M/s · 23.72 s · 1025 Mo | 5.86 M/s · 24.58 s · 1026 Mo | 5.97 M/s · 24.14 s · 1026 Mo | 6.63 M/s · 21.73 s · 1026 Mo | 5.86 M/s · 24.56 s · 1026 Mo | 4.38 M/s · 32.87 s · 1053 Mo |

Moyenne géométrique du temps relatif au meilleur : F*→C ×1.07 (5/5), C natif ×1.07 (5/5), Rust + Creusot ×1.11 (5/5), Zig 0.17 ×1.12 (5/5), Rust ×1.17 (5/5), F*→F# ×2.26 (4/5).

### `large`

| workload (n) | C natif | Zig 0.17 | Rust | Rust + Creusot | F*→C | F*→F# | F*→OCaml (corrigé) | F*→OCaml (livré) |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| sort (32 000 000) | 5.89 M/s · 5.43 s · 246 Mo | 6.58 M/s · 4.86 s · 246 Mo | 5.23 M/s · 6.12 s · 246 Mo | 4.90 M/s · 6.53 s · 246 Mo | 5.44 M/s · 5.89 s · 246 Mo | 3.56 M/s · 8.98 s · 273 Mo | 68.88 k/s · 464.57 s · 1267 Mo | délai dépassé |
| sieve (1 000 000 000) | 54.28 M/s · 18.42 s · 955 Mo | 54.53 M/s · 18.34 s · 955 Mo | 57.12 M/s · 17.51 s · 956 Mo | 53.19 M/s · 18.80 s · 956 Mo | 51.80 M/s · 19.30 s · 955 Mo | 53.32 M/s · 18.75 s · 983 Mo | 2.05 M/s · 487.39 s · 7648 Mo | délai dépassé |
| matmul (2 500) | 6.13 G/s · 2.55 s · 73 Mo | 4.91 G/s · 3.18 s · 73 Mo | 6.22 G/s · 2.51 s · 73 Mo | 4.70 G/s · 3.32 s · 73 Mo | 5.33 G/s · 2.93 s · 73 Mo | 1.04 G/s · 15.09 s · 101 Mo | délai dépassé | délai dépassé |
| lines (40 000 000) | 11.24 M/s · 3.56 s · 545 Mo | 10.42 M/s · 3.84 s · 545 Mo | 13.27 M/s · 3.01 s · 546 Mo | 8.21 M/s · 4.87 s · 546 Mo | 10.28 M/s · 3.89 s · 545 Mo | 8.80 M/s · 4.55 s · 617 Mo | 121.47 k/s · 329.29 s · 6469 Mo | délai dépassé |
| hashmap (16 000 000) | 17.14 M/s · 2.80 s · 513 Mo | 17.03 M/s · 2.82 s · 514 Mo | 14.11 M/s · 3.40 s · 514 Mo | 14.53 M/s · 3.30 s · 514 Mo | 15.06 M/s · 3.19 s · 514 Mo | 10.27 M/s · 4.68 s · 541 Mo | 392.43 k/s · 122.31 s · 1259 Mo | délai dépassé |

Moyenne géométrique du temps relatif au meilleur : C natif ×1.07 (5/5), Rust ×1.09 (5/5), Zig 0.17 ×1.11 (5/5), F*→C ×1.18 (5/5), Rust + Creusot ×1.29 (5/5), F*→F# ×1.97 (5/5), F*→OCaml (corrigé) ×59.68 (4/5).

### `medium`

| workload (n) | C natif | Zig 0.17 | Rust | Rust + Creusot | F*→C | F*→F# | F*→OCaml (corrigé) | F*→OCaml (livré) |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| sort (8 000 000) | 6.33 M/s · 1.26 s · 63 Mo | 7.36 M/s · 1.09 s · 63 Mo | 5.76 M/s · 1.39 s · 63 Mo | 5.56 M/s · 1.44 s · 63 Mo | 6.44 M/s · 1.24 s · 62 Mo | 4.18 M/s · 1.91 s · 90 Mo | 85.29 k/s · 93.79 s · 322 Mo | délai dépassé |
| sieve (200 000 000) | 65.58 M/s · 3.05 s · 192 Mo | 62.35 M/s · 3.21 s · 192 Mo | 66.15 M/s · 3.02 s · 193 Mo | 60.67 M/s · 3.30 s · 193 Mo | 64.39 M/s · 3.11 s · 192 Mo | 65.47 M/s · 3.05 s · 220 Mo | 2.40 M/s · 83.31 s · 1536 Mo | 430.51 k/s · 464.56 s · 1592 Mo |
| matmul (1 600) | 6.03 G/s · 0.68 s · 31 Mo | 5.83 G/s · 0.70 s · 31 Mo | 6.02 G/s · 0.68 s · 31 Mo | 5.17 G/s · 0.79 s · 31 Mo | 5.83 G/s · 0.70 s · 31 Mo | 1.03 G/s · 3.97 s · 58 Mo | délai dépassé | délai dépassé |
| lines (10 000 000) | 10.90 M/s · 0.92 s · 149 Mo | 12.18 M/s · 0.82 s · 149 Mo | 15.70 M/s · 0.64 s · 150 Mo | 10.40 M/s · 0.96 s · 150 Mo | 16.57 M/s · 0.60 s · 149 Mo | 8.57 M/s · 1.17 s · 188 Mo | 132.24 k/s · 75.62 s · 1635 Mo | délai dépassé |
| hashmap (4 000 000) | 18.83 M/s · 0.64 s · 129 Mo | 20.48 M/s · 0.59 s · 130 Mo | 18.83 M/s · 0.64 s · 130 Mo | 17.26 M/s · 0.70 s · 130 Mo | 20.59 M/s · 0.58 s · 130 Mo | 11.70 M/s · 1.03 s · 157 Mo | 448.65 k/s · 26.75 s · 320 Mo | 61.42 k/s · 195.39 s · 347 Mo |

Moyenne géométrique du temps relatif au meilleur : F*→C ×1.04 (5/5), Rust ×1.08 (5/5), Zig 0.17 ×1.08 (5/5), C natif ×1.14 (5/5), Rust + Creusot ×1.26 (5/5), F*→F# ×2.04 (5/5), F*→OCaml (corrigé) ×60.81 (4/5), F*→OCaml (livré) ×226.95 (2/5).

### `small`

| workload (n) | C natif | Zig 0.17 | Rust | Rust + Creusot | F*→C | F*→F# | F*→OCaml (corrigé) | F*→OCaml (livré) |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| sort (1 000 000) | 7.64 M/s · 0.13 s · 9 Mo | 7.57 M/s · 0.13 s · 9 Mo | 6.66 M/s · 0.15 s · 10 Mo | 6.73 M/s · 0.15 s · 10 Mo | 6.49 M/s · 0.15 s · 9 Mo | 4.89 M/s · 0.20 s · 37 Mo | 106.57 k/s · 9.38 s · 46 Mo | 14.85 k/s · 67.36 s · 58 Mo |
| sieve (20 000 000) | 154.35 M/s · 0.13 s · 21 Mo | 162.35 M/s · 0.12 s · 21 Mo | 147.94 M/s · 0.14 s · 21 Mo | 145.12 M/s · 0.14 s · 21 Mo | 157.43 M/s · 0.13 s · 21 Mo | 146.49 M/s · 0.14 s · 48 Mo | 2.68 M/s · 7.45 s · 160 Mo | 479.79 k/s · 41.68 s · 165 Mo |
| matmul (512) | 7.79 G/s · 0.02 s · 4 Mo | 7.97 G/s · 0.02 s · 4 Mo | 6.14 G/s · 0.02 s · 5 Mo | 4.28 G/s · 0.03 s · 5 Mo | 6.46 G/s · 0.02 s · 4 Mo | 1.11 G/s · 0.12 s · 32 Mo | 4.18 M/s · 32.14 s · 77 Mo | 668.22 k/s · 200.86 s · 81 Mo |
| lines (1 000 000) | 9.27 M/s · 0.11 s · 31 Mo | 10.42 M/s · 0.10 s · 31 Mo | 10.36 M/s · 0.10 s · 31 Mo | 8.23 M/s · 0.12 s · 31 Mo | 9.13 M/s · 0.11 s · 31 Mo | 7.79 M/s · 0.13 s · 59 Mo | 128.38 k/s · 7.79 s · 184 Mo | 13.82 k/s · 72.37 s · 197 Mo |
| hashmap (500 000) | 31.45 M/s · 0.05 s · 17 Mo | 39.80 M/s · 0.04 s · 18 Mo | 30.88 M/s · 0.05 s · 18 Mo | 31.13 M/s · 0.05 s · 18 Mo | 32.56 M/s · 0.05 s · 18 Mo | 18.92 M/s · 0.08 s · 45 Mo | 518.71 k/s · 2.89 s · 47 Mo | 66.34 k/s · 22.61 s · 50 Mo |

Moyenne géométrique du temps relatif au meilleur : Zig 0.17 ×1.00 (5/5), C natif ×1.09 (5/5), F*→C ×1.16 (5/5), Rust ×1.16 (5/5), Rust + Creusot ×1.31 (5/5), F*→F# ×2.04 (5/5), F*→OCaml (corrigé) ×138.85 (5/5), F*→OCaml (livré) ×987.79 (5/5).

### `tiny`

| workload (n) | C natif | Zig 0.17 | Rust | Rust + Creusot | F*→C | F*→F# |
|---|--:|--:|--:|--:|--:|--:|
| sort (200 000) | 8.38 M/s · 0.02 s · 3 Mo | 9.37 M/s · 0.02 s · 3 Mo | 7.86 M/s · 0.03 s · 3 Mo | 7.28 M/s · 0.03 s · 3 Mo | 8.09 M/s · 0.02 s · 3 Mo | 5.17 M/s · 0.04 s · 30 Mo |
| sieve (5 000 000) | 177.59 M/s · 0.03 s · 6 Mo | 186.86 M/s · 0.03 s · 6 Mo | 185.19 M/s · 0.03 s · 7 Mo | 165.58 M/s · 0.03 s · 7 Mo | 187.10 M/s · 0.03 s · 6 Mo | 153.18 M/s · 0.03 s · 33 Mo |
| matmul (128) | 4.24 G/s · 0.00 s · 2 Mo | 4.99 G/s · 0.00 s · 2 Mo | 5.72 G/s · 0.00 s · 2 Mo | 2.77 G/s · 0.00 s · 2 Mo | 4.53 G/s · 0.00 s · 2 Mo | 455.66 M/s · 0.00 s · 29 Mo |
| lines (50 000) | 1.73 M/s · 0.03 s · 18 Mo | 2.04 M/s · 0.02 s · 18 Mo | 2.31 M/s · 0.02 s · 19 Mo | 2.01 M/s · 0.02 s · 19 Mo | 2.20 M/s · 0.02 s · 18 Mo | 2.34 M/s · 0.02 s · 46 Mo |
| hashmap (100 000) | 35.29 M/s · 0.01 s · 5 Mo | 48.29 M/s · 0.01 s · 5 Mo | 37.37 M/s · 0.01 s · 6 Mo | 35.65 M/s · 0.01 s · 6 Mo | 50.79 M/s · 0.01 s · 5 Mo | 19.19 M/s · 0.02 s · 33 Mo |

Moyenne géométrique du temps relatif au meilleur : Zig 0.17 ×1.07 (5/5), F*→C ×1.09 (5/5), Rust ×1.11 (5/5), C natif ×1.25 (5/5), Rust + Creusot ×1.38 (5/5), F*→F# ×2.36 (5/5).

