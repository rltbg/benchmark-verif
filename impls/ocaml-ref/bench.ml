(* Port OCaml idiomatique écrit à la main (non vérifié, accès avec contrôle de bornes).
   Contrôle : montre ce qu'OCaml sait faire sans l'overhead de l'extraction F*. *)

let mix64 x =
  let open Int64 in
  let z = add x 0x9E3779B97F4A7C15L in
  let z = mul (logxor z (shift_right_logical z 30)) 0xBF58476D1CE4E5B9L in
  let z = mul (logxor z (shift_right_logical z 27)) 0x94D049BB133111EBL in
  logxor z (shift_right_logical z 31)

let ck_init = 0xCBF29CE484222325L
let fold ck v = Int64.mul (Int64.logxor ck v) 0x100000001B3L

let s_sort = 0x0123456789ABCDEFL
let s_mat = 0x0FEDCBA987654321L
let s_line = 0x1357924680ACEBDFL
let s_hash = 0x2468ACE013579BDFL

let u32_of_mix x = Int64.to_int (Int64.shift_right_logical x 32)  (* 0 .. 2^32-1 *)

(* int array : 8 octets par u32 (représentation idiomatique d'OCaml) *)
let run_sort n =
  let buf = Array.make (2 * n) 0 in
  for i = 0 to n - 1 do
    buf.(i) <- u32_of_mix (mix64 (Int64.add s_sort (Int64.of_int i)))
  done;
  let so = ref 0 and d = ref n and w = ref 1 in
  while !w < n do
    let lo = ref 0 in
    while !lo < n do
      let mid = if !lo + !w < n then !lo + !w else n in
      let hi = if !lo + 2 * !w < n then !lo + 2 * !w else n in
      let i = ref !lo and j = ref mid and k = ref !lo in
      let s = !so and dd = !d in
      while !i < mid && !j < hi do
        let x = buf.(s + !i) and y = buf.(s + !j) in
        if x <= y then (buf.(dd + !k) <- x; incr i) else (buf.(dd + !k) <- y; incr j);
        incr k
      done;
      while !i < mid do buf.(dd + !k) <- buf.(s + !i); incr i; incr k done;
      while !j < hi do buf.(dd + !k) <- buf.(s + !j); incr j; incr k done;
      lo := !lo + 2 * !w
    done;
    let t = !so in so := !d; d := t;
    w := !w * 2
  done;
  let ck = ref ck_init and ok = ref true in
  for i = 0 to n - 1 do
    if i > 0 && buf.(!so + i - 1) > buf.(!so + i) then ok := false;
    ck := fold !ck (Int64.of_int buf.(!so + i))
  done;
  if !ok then !ck else 0L

let run_sieve n =
  let f = Bytes.make (n + 1) '\000' in
  let i = ref 2 in
  while !i * !i <= n do
    if Bytes.unsafe_get f !i = '\000' then begin
      let j = ref (!i * !i) in
      while !j <= n do Bytes.set f !j '\001'; j := !j + !i done
    end;
    incr i
  done;
  let ck = ref ck_init and cnt = ref 0 in
  for i = 2 to n do
    if Bytes.get f i = '\000' then (ck := fold !ck (Int64.of_int i); incr cnt)
  done;
  fold !ck (Int64.of_int !cnt)

let run_matmul n =
  let nn = n * n in
  let a = Array.make nn 0 and b = Array.make nn 0 and c = Array.make nn 0 in
  for i = 0 to nn - 1 do
    a.(i) <- u32_of_mix (mix64 (Int64.add s_mat (Int64.of_int i)));
    b.(i) <- u32_of_mix (mix64 (Int64.add s_mat (Int64.of_int (nn + i))))
  done;
  for i = 0 to n - 1 do
    for k = 0 to n - 1 do
      let aik = a.(i * n + k) in
      for j = 0 to n - 1 do
        c.(i * n + j) <- (c.(i * n + j) + aik * b.(k * n + j)) land 0xFFFFFFFF
      done
    done
  done;
  let ck = ref ck_init in
  for i = 0 to nn - 1 do ck := fold !ck (Int64.of_int c.(i)) done;
  !ck

let line_k = 1 lsl 20
let line_max = 15

let put_dec buf off v =
  let l = ref 1 and t = ref v in
  while !t >= 10 do incr l; t := !t / 10 done;
  let t = ref v in
  for i = !l - 1 downto 0 do
    Bytes.unsafe_set buf (off + i) (Char.unsafe_chr (48 + !t mod 10)); t := !t / 10
  done;
  off + !l

let run_lines n =
  let txt = Bytes.create (n * line_max) in
  let len = ref 0 in
  for i = 0 to n - 1 do
    let r = mix64 (Int64.add s_line (Int64.of_int i)) in
    let key = Int64.to_int (Int64.shift_right_logical r 44) in
    let v = Int64.to_int (Int64.logand (Int64.shift_right_logical r 8) 0xFFFFFFFFL) mod 1000000 in
    len := put_dec txt !len key;
    Bytes.set txt !len ';'; incr len;
    len := put_dec txt !len v;
    Bytes.set txt !len '\n'; incr len
  done;
  let tab = Array.make (2 * line_k) 0 in
  let p = ref 0 in
  while !p < !len do
    let key = ref 0 and v = ref 0 in
    while Bytes.get txt !p <> ';' do key := !key * 10 + Char.code (Bytes.get txt !p) - 48; incr p done;
    incr p;
    while Bytes.get txt !p <> '\n' do v := !v * 10 + Char.code (Bytes.get txt !p) - 48; incr p done;
    incr p;
    tab.(2 * !key) <- tab.(2 * !key) + !v;
    tab.(2 * !key + 1) <- tab.(2 * !key + 1) + 1
  done;
  let ck = ref ck_init in
  for k = 0 to line_k - 1 do
    ck := fold (fold !ck (Int64.of_int tab.(2 * k))) (Int64.of_int tab.(2 * k + 1))
  done;
  !ck

(* Table : clés u64 et valeurs u64 ne tiennent pas dans un int OCaml 63 bits -> Int64 boîtés
   évités via Bigarray.Array1 int64 (stockage non boxé). *)
let run_hashmap n =
  let lg = ref 1 in
  while (1 lsl !lg) < 2 * n do incr lg done;
  let lg = !lg in
  let cap = 1 lsl lg in
  let mask = cap - 1 in
  let t = Bigarray.Array1.create Bigarray.int64 Bigarray.c_layout (2 * cap) in
  Bigarray.Array1.fill t 0L;
  let slot key = Int64.to_int (Int64.shift_right_logical (Int64.mul key 0x9E3779B97F4A7C15L) (64 - lg)) in
  for i = 0 to n - 1 do
    let key = Int64.logor (mix64 (Int64.add s_hash (Int64.of_int i))) 1L in
    let s = ref (slot key) and fin = ref false in
    while not !fin do
      let k = Bigarray.Array1.get t (2 * !s) in
      if k = 0L then begin
        Bigarray.Array1.set t (2 * !s) key;
        Bigarray.Array1.set t (2 * !s + 1) (Int64.of_int (i + 1)); fin := true
      end else if k = key then begin
        Bigarray.Array1.set t (2 * !s + 1) (Int64.add (Bigarray.Array1.get t (2 * !s + 1)) (Int64.of_int (i + 1)));
        fin := true
      end else s := (!s + 1) land mask
    done
  done;
  let ck = ref ck_init in
  for phase = 0 to 1 do
    for i = 0 to n - 1 do
      let key = Int64.logor (mix64 (Int64.add s_hash (Int64.of_int ((if phase = 1 then n else 0) + i)))) 1L in
      let s = ref (slot key) and v = ref 0L and fin = ref false in
      while not !fin do
        let k = Bigarray.Array1.get t (2 * !s) in
        if k = 0L then fin := true
        else if k = key then (v := Bigarray.Array1.get t (2 * !s + 1); fin := true)
        else s := (!s + 1) land mask
      done;
      ck := fold !ck !v
    done
  done;
  !ck

let () =
  if Array.length Sys.argv <> 3 then (prerr_endline "usage: bench <sort|sieve|matmul|lines|hashmap> <n>"; exit 2);
  let n = int_of_string Sys.argv.(2) in
  let t0 = Unix.gettimeofday () in
  let ck = match Sys.argv.(1) with
    | "sort" -> run_sort n | "sieve" -> run_sieve n | "matmul" -> run_matmul n
    | "lines" -> run_lines n | "hashmap" -> run_hashmap n
    | _ -> prerr_endline "workload inconnu"; exit 2 in
  let ns = Int64.of_float ((Unix.gettimeofday () -. t0) *. 1e9) in
  Printf.printf "RESULT workload=%s n=%d checksum=%016Lx time_ns=%Ld\n" Sys.argv.(1) n ck ns
