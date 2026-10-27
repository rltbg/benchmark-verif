#### CPU et RAM, `large`

Cellule : temps CPU (user+sys) · utilisation d'un cœur · pic RAM (RSS) · RAM ÷ taille idéale des structures · fautes de page mineures.

| workload | C natif | Zig 0.17 | Rust | Rust + Creusot | F*→C | F*→F# | F*→OCaml (corrigé) | F*→OCaml (livré) |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| sort | 5.4 s · 99 % · 246 Mo · ×1.01 · 63 k | 4.8 s · 99 % · 246 Mo · ×1.01 · 63 k | 6.1 s · 99 % · 246 Mo · ×1.01 · 63 k | 6.5 s · 99 % · 246 Mo · ×1.01 · 63 k | 5.9 s · 100 % · 246 Mo · ×1.01 · 63 k | 9.0 s · 99 % · 273 Mo · ×1.12 · 64 k | 458.6 s · 99 % · 1267 Mo · ×5.19 · 324 k | — |
| sieve | 17.4 s · 94 % · 955 Mo · ×1.00 · 244 k | 18.0 s · 98 % · 955 Mo · ×1.00 · 244 k | 17.5 s · 100 % · 956 Mo · ×1.00 · 244 k | 17.9 s · 95 % · 956 Mo · ×1.00 · 244 k | 19.1 s · 99 % · 955 Mo · ×1.00 · 244 k | 18.8 s · 100 % · 983 Mo · ×1.03 · 246 k | 480.3 s · 99 % · 7648 Mo · ×8.02 · 1957 k | — |
| matmul | 2.5 s · 99 % · 73 Mo · ×1.02 · 25 k | 3.2 s · 99 % · 73 Mo · ×1.02 · 18 k | 2.5 s · 99 % · 73 Mo · ×1.03 · 25 k | 3.3 s · 99 % · 73 Mo · ×1.03 · 25 k | 2.9 s · 98 % · 73 Mo · ×1.02 · 25 k | 15.1 s · 99 % · 101 Mo · ×1.41 · 20 k | — | — |
| lines | 3.5 s · 99 % · 545 Mo · ×0.93 · 143 k | 3.8 s · 99 % · 545 Mo · ×0.93 · 143 k | 2.9 s · 96 % · 546 Mo · ×0.93 · 143 k | 4.8 s · 99 % · 546 Mo · ×0.93 · 143 k | 3.9 s · 99 % · 545 Mo · ×0.93 · 143 k | 4.6 s · 99 % · 617 Mo · ×1.05 · 153 k | 325.9 s · 99 % · 6469 Mo · ×11.00 · 1655 k | — |
| hashmap | 2.8 s · 100 % · 513 Mo · ×1.00 · 262 k | 2.8 s · 100 % · 514 Mo · ×1.00 · 131 k | 3.4 s · 98 % · 514 Mo · ×1.00 · 262 k | 3.3 s · 99 % · 514 Mo · ×1.00 · 262 k | 3.2 s · 99 % · 514 Mo · ×1.00 · 131 k | 4.7 s · 98 % · 541 Mo · ×1.06 · 133 k | 120.9 s · 99 % · 1259 Mo · ×2.46 · 321 k | — |

#### Empreinte disque, démarrage, construction

| colonne | exécutable | runtime à installer | démarrage (run minuscule) | RAM au démarrage | compilation | vérification à froid |
|---|--:|--:|--:|--:|--:|--:|
| C natif | 20 Ko | 0 Mo | 6 ms | 1.7 Mo | 0.3 s | — |
| Zig 0.17 | 67 Ko | 0 Mo | 6 ms | 1.7 Mo | 0.3 s | — |
| Rust | 395 Ko | 0 Mo | 5 ms | 1.9 Mo | 0.1 s | — |
| Rust + Creusot | 396 Ko | 0 Mo | 6 ms | 1.9 Mo | 0.1 s | 16 s |
| F*→C | 24 Ko | 0 Mo | 5 ms | 1.7 Mo | 4.0 s | 236 s |
| F*→F# | 50 Ko | 72 Mo | 86 ms | 28.8 Mo | 22.6 s | 236 s |
| F*→OCaml (corrigé) | 5059 Ko | 0 Mo | — | — | 11.5 s | 236 s |
| F*→OCaml (livré) | 5077 Ko | 0 Mo | — | — | 10.1 s | 236 s |
