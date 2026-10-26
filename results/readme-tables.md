### `xlarge`

| workload (n) | C natif | Zig 0.17 | Rust | Rust + Creusot | F*→C | F*→F# |
|---|--:|--:|--:|--:|--:|--:|
| sort (100 000 000) | 6.20 M/s · 16.12 s · 764 Mo | 7.12 M/s · 14.04 s · 764 Mo | 6.00 M/s · 16.66 s · 765 Mo | 5.74 M/s · 17.41 s · 765 Mo | 6.64 M/s · 15.06 s · 764 Mo | 4.32 M/s · 23.14 s · 812 Mo |
| sieve (3 000 000 000) | 48.79 M/s · 61.48 s · 2862 Mo | 49.40 M/s · 60.73 s · 2863 Mo | 49.86 M/s · 60.17 s · 2863 Mo | 42.10 M/s · 71.26 s · 2863 Mo | 50.62 M/s · 59.26 s · 2862 Mo | **échec** |
| matmul (4 096) | 2.50 G/s · 27.53 s · 193 Mo | 1.14 G/s · 60.40 s · 194 Mo | 1.59 G/s · 43.14 s · 194 Mo | 2.87 G/s · 23.94 s · 194 Mo | 1.57 G/s · 43.90 s · 193 Mo | 513.56 M/s · 133.81 s · 351 Mo |
| lines (100 000 000) | 14.00 M/s · 7.14 s · 1336 Mo | 15.40 M/s · 6.49 s · 1336 Mo | 16.07 M/s · 6.22 s · 1337 Mo | 14.16 M/s · 7.06 s · 1337 Mo | 18.31 M/s · 5.46 s · 1336 Mo | 7.96 M/s · 12.56 s · 1609 Mo |
| hashmap (48 000 000) | 6.74 M/s · 21.38 s · 1025 Mo | 5.71 M/s · 25.22 s · 1026 Mo | 6.11 M/s · 23.58 s · 1026 Mo | 6.06 M/s · 23.76 s · 1026 Mo | 5.23 M/s · 27.53 s · 1026 Mo | 3.83 M/s · 37.56 s · 1187 Mo |

Moyenne géométrique du temps relatif au meilleur : C natif ×1.12 (5/5), Rust + Creusot ×1.16 (5/5), F*→C ×1.20 (5/5), Rust ×1.22 (5/5), Zig 0.17 ×1.29 (5/5), F*→F# ×2.47 (4/5).

### `large`

| workload (n) | C natif | Zig 0.17 | Rust | Rust + Creusot | F*→C | F*→F# | F*→OCaml (corrigé) | F*→OCaml (livré) |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| sort (32 000 000) | 5.98 M/s · 5.35 s · 246 Mo | 7.34 M/s · 4.36 s · 246 Mo | 6.44 M/s · 4.97 s · 246 Mo | 6.18 M/s · 5.18 s · 246 Mo | 6.91 M/s · 4.63 s · 246 Mo | 4.81 M/s · 6.66 s · 294 Mo | 68.88 k/s · 464.57 s · 1267 Mo | délai dépassé |
| sieve (1 000 000 000) | 53.45 M/s · 18.71 s · 955 Mo | 52.67 M/s · 18.98 s · 955 Mo | 52.72 M/s · 18.97 s · 956 Mo | 51.99 M/s · 19.23 s · 956 Mo | 53.44 M/s · 18.71 s · 955 Mo | 42.82 M/s · 23.35 s · 982 Mo | 2.05 M/s · 487.39 s · 7648 Mo | délai dépassé |
| matmul (2 500) | 3.39 G/s · 4.60 s · 73 Mo | 2.36 G/s · 6.62 s · 73 Mo | 2.41 G/s · 6.48 s · 73 Mo | 3.19 G/s · 4.89 s · 73 Mo | 3.00 G/s · 5.21 s · 73 Mo | 713.59 M/s · 21.90 s · 231 Mo | délai dépassé | délai dépassé |
| lines (40 000 000) | 15.05 M/s · 2.66 s · 545 Mo | 13.36 M/s · 2.99 s · 545 Mo | 16.00 M/s · 2.50 s · 545 Mo | 12.36 M/s · 3.24 s · 546 Mo | 16.57 M/s · 2.41 s · 545 Mo | 7.98 M/s · 5.01 s · 751 Mo | 121.47 k/s · 329.29 s · 6469 Mo | délai dépassé |
| hashmap (16 000 000) | 16.64 M/s · 2.89 s · 513 Mo | 15.78 M/s · 3.04 s · 514 Mo | 13.88 M/s · 3.46 s · 514 Mo | 14.19 M/s · 3.38 s · 514 Mo | 15.51 M/s · 3.10 s · 514 Mo | 5.69 M/s · 8.44 s · 675 Mo | 392.43 k/s · 122.31 s · 1259 Mo | délai dépassé |

Moyenne géométrique du temps relatif au meilleur : F*→C ×1.05 (5/5), C natif ×1.06 (5/5), Zig 0.17 ×1.14 (5/5), Rust ×1.15 (5/5), Rust + Creusot ×1.15 (5/5), F*→F# ×2.23 (5/5), F*→OCaml (corrigé) ×63.29 (4/5).

### `medium`

| workload (n) | C natif | Zig 0.17 | Rust | Rust + Creusot | F*→C | F*→F# | F*→OCaml (corrigé) | F*→OCaml (livré) |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| sort (8 000 000) | 6.21 M/s · 1.29 s · 63 Mo | 8.13 M/s · 0.98 s · 63 Mo | 6.60 M/s · 1.21 s · 63 Mo | 6.37 M/s · 1.26 s · 63 Mo | 7.40 M/s · 1.08 s · 62 Mo | 4.65 M/s · 1.72 s · 109 Mo | 85.29 k/s · 93.79 s · 322 Mo | délai dépassé |
| sieve (200 000 000) | 60.12 M/s · 3.33 s · 192 Mo | 60.00 M/s · 3.33 s · 192 Mo | 60.49 M/s · 3.31 s · 193 Mo | 59.55 M/s · 3.36 s · 193 Mo | 62.80 M/s · 3.18 s · 192 Mo | 46.41 M/s · 4.31 s · 219 Mo | 2.40 M/s · 83.31 s · 1536 Mo | 430.51 k/s · 464.56 s · 1592 Mo |
| matmul (1 600) | 3.19 G/s · 1.28 s · 31 Mo | 2.25 G/s · 1.82 s · 31 Mo | 2.59 G/s · 1.58 s · 31 Mo | 3.03 G/s · 1.35 s · 31 Mo | 3.20 G/s · 1.28 s · 31 Mo | 816.45 M/s · 5.02 s · 115 Mo | délai dépassé | délai dépassé |
| lines (10 000 000) | 11.54 M/s · 0.87 s · 149 Mo | 13.53 M/s · 0.74 s · 149 Mo | 16.06 M/s · 0.62 s · 150 Mo | 9.47 M/s · 1.06 s · 150 Mo | 12.16 M/s · 0.82 s · 149 Mo | 7.59 M/s · 1.32 s · 322 Mo | 132.24 k/s · 75.62 s · 1635 Mo | délai dépassé |
| hashmap (4 000 000) | 20.06 M/s · 0.60 s · 129 Mo | 18.02 M/s · 0.67 s · 130 Mo | 15.48 M/s · 0.78 s · 130 Mo | 15.61 M/s · 0.77 s · 130 Mo | 18.60 M/s · 0.65 s · 130 Mo | 5.99 M/s · 2.00 s · 291 Mo | 448.65 k/s · 26.75 s · 320 Mo | 61.42 k/s · 195.39 s · 347 Mo |

Moyenne géométrique du temps relatif au meilleur : F*→C ×1.09 (5/5), C natif ×1.14 (5/5), Zig 0.17 ×1.15 (5/5), Rust ×1.15 (5/5), Rust + Creusot ×1.25 (5/5), F*→F# ×2.31 (5/5), F*→OCaml (corrigé) ×60.67 (4/5), F*→OCaml (livré) ×218.28 (2/5).

### `small`

| workload (n) | C natif | Zig 0.17 | Rust | Rust + Creusot | F*→C | F*→F# | F*→OCaml (corrigé) | F*→OCaml (livré) |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| sort (1 000 000) | 7.21 M/s · 0.14 s · 9 Mo | 8.66 M/s · 0.12 s · 9 Mo | 7.75 M/s · 0.13 s · 10 Mo | 7.53 M/s · 0.13 s · 10 Mo | 7.65 M/s · 0.13 s · 9 Mo | 3.16 M/s · 0.32 s · 63 Mo | 106.57 k/s · 9.38 s · 46 Mo | 14.85 k/s · 67.36 s · 58 Mo |
| sieve (20 000 000) | 135.27 M/s · 0.15 s · 21 Mo | 149.44 M/s · 0.13 s · 21 Mo | 135.93 M/s · 0.15 s · 21 Mo | 147.97 M/s · 0.14 s · 21 Mo | 148.78 M/s · 0.13 s · 21 Mo | 92.31 M/s · 0.22 s · 48 Mo | 2.68 M/s · 7.45 s · 160 Mo | 479.79 k/s · 41.68 s · 165 Mo |
| matmul (512) | 4.29 G/s · 0.03 s · 4 Mo | 2.70 G/s · 0.05 s · 5 Mo | 2.75 G/s · 0.05 s · 5 Mo | 3.85 G/s · 0.03 s · 5 Mo | 3.85 G/s · 0.03 s · 4 Mo | 422.28 M/s · 0.32 s · 37 Mo | 4.18 M/s · 32.14 s · 77 Mo | 668.22 k/s · 200.86 s · 81 Mo |
| lines (1 000 000) | 7.26 M/s · 0.14 s · 31 Mo | 10.14 M/s · 0.10 s · 31 Mo | 10.00 M/s · 0.10 s · 31 Mo | 7.82 M/s · 0.13 s · 31 Mo | 9.15 M/s · 0.11 s · 31 Mo | 4.99 M/s · 0.20 s · 151 Mo | 128.38 k/s · 7.79 s · 184 Mo | 13.82 k/s · 72.37 s · 197 Mo |
| hashmap (500 000) | 29.99 M/s · 0.05 s · 17 Mo | 31.52 M/s · 0.05 s · 18 Mo | 26.27 M/s · 0.06 s · 18 Mo | 23.53 M/s · 0.06 s · 18 Mo | 28.51 M/s · 0.05 s · 18 Mo | 6.67 M/s · 0.22 s · 171 Mo | 518.71 k/s · 2.89 s · 47 Mo | 66.34 k/s · 22.61 s · 50 Mo |

Moyenne géométrique du temps relatif au meilleur : F*→C ×1.09 (5/5), Zig 0.17 ×1.10 (5/5), C natif ×1.14 (5/5), Rust + Creusot ×1.18 (5/5), Rust ×1.18 (5/5), F*→F# ×3.37 (5/5), F*→OCaml (corrigé) ×117.42 (5/5), F*→OCaml (livré) ×835.33 (5/5).

### `tiny`

| workload (n) | C natif | Zig 0.17 | Rust | Rust + Creusot | F*→C | F*→F# | F*→OCaml (corrigé) | F*→OCaml (livré) |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| sort (200 000) | 8.92 M/s · 0.02 s · 3 Mo | 10.83 M/s · 0.02 s · 3 Mo | 8.62 M/s · 0.02 s · 3 Mo | 8.48 M/s · 0.02 s · 3 Mo | 10.01 M/s · 0.02 s · 3 Mo | 1.41 M/s · 0.14 s · 48 Mo | 123.11 k/s · 1.62 s · 15 Mo | 16.34 k/s · 12.24 s · 17 Mo |
| sieve (5 000 000) | 149.35 M/s · 0.03 s · 6 Mo | 162.05 M/s · 0.03 s · 6 Mo | 158.15 M/s · 0.03 s · 7 Mo | 160.61 M/s · 0.03 s · 7 Mo | 177.09 M/s · 0.03 s · 6 Mo | 104.13 M/s · 0.05 s · 33 Mo | 2.88 M/s · 1.74 s · 45 Mo | 489.85 k/s · 10.21 s · 47 Mo |
| matmul (128) | 2.31 G/s · 0.00 s · 2 Mo | 1.64 G/s · 0.00 s · 2 Mo | 3.16 G/s · 0.00 s · 2 Mo | 2.64 G/s · 0.00 s · 2 Mo | 3.07 G/s · 0.00 s · 2 Mo | 84.84 M/s · 0.02 s · 29 Mo | 3.88 M/s · 0.54 s · 10 Mo | 720.44 k/s · 2.91 s · 12 Mo |
| lines (50 000) | 1.73 M/s · 0.03 s · 18 Mo | 1.68 M/s · 0.03 s · 18 Mo | 1.98 M/s · 0.03 s · 19 Mo | 1.95 M/s · 0.03 s · 19 Mo | 2.16 M/s · 0.02 s · 18 Mo | 1.46 M/s · 0.03 s · 50 Mo | 65.64 k/s · 0.76 s · 31 Mo | 6.60 k/s · 7.57 s · 32 Mo |
| hashmap (100 000) | 30.80 M/s · 0.01 s · 5 Mo | 39.80 M/s · 0.01 s · 5 Mo | 35.51 M/s · 0.01 s · 6 Mo | 37.79 M/s · 0.01 s · 6 Mo | 44.16 M/s · 0.01 s · 5 Mo | 4.73 M/s · 0.06 s · 58 Mo | 538.82 k/s · 0.56 s · 16 Mo | 69.80 k/s · 4.30 s · 16 Mo |

Moyenne géométrique du temps relatif au meilleur : F*→C ×1.02 (5/5), Rust ×1.14 (5/5), Rust + Creusot ×1.17 (5/5), Zig 0.17 ×1.25 (5/5), C natif ×1.29 (5/5), F*→F# ×5.83 (5/5), F*→OCaml (corrigé) ×103.50 (5/5), F*→OCaml (livré) ×736.97 (5/5).

