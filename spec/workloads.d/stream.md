# stream — flux d'octets lu depuis un fichier (E/S disque réelles)

**Paramètre** `n` : taille du fichier d'entrée en **Mio**. Unité de débit : octets/s (`n·2^20` octets traités).
Mémoire de la spécification : un tampon de 1 Mio + l'état (≈ 1 Mio + quelques Ko), indépendant de `n`.

## Fichier d'entrée
`/var/tmp/bench-data/stream-<n>.json` (répertoire remplaçable par la variable d'environnement `BENCH_DATA_DIR`), produit par
`harness/genstream.c` (`gcc -O2 -o results/.genstream harness/genstream.c && results/.genstream <n>`), **hors du temps mesuré**
(hook `prepare` de `spec/workloads.d/stream.json`). C'est les `n·2^20` premiers octets (le dernier enregistrement est éventuellement
tronqué) du flux infini de lignes JSON suivant. Le cache de pages est vidé (`echo 3 > /proc/sys/vm/drop_caches`) avant chaque run.

Générateur : `draw()` = `mix64(0xF11E000000000001 + j)`, `j` incrémenté à chaque tirage (compteur global, départ 0).
Un enregistrement, dans l'ordre des tirages :
```
{"id":<u64 décimal = draw()>,"name":"<k mots>","tags":[<t tags>],"v":<nombre>,"ok":<true|false|null>}\n
```
* `k = 1 + draw()%3` mots séparés par une espace, chaque mot `WORDS[draw()%16]` avec
  `WORDS = alpha bravo charlie delta echo foxtrot golf hotel india juliet kilo lima mike november oscar papa` ; puis, si `draw()%8 == 0`,
  la suite de 6 octets ` \"q\"` (espace, antislash, guillemet, q, antislash, guillemet : teste l'échappement) ;
* `t = draw()%4` tags `"t<draw()%1000>"` séparés par des virgules ;
* `d = draw()` ; `v` selon `d%4` : 0 → entier (`-` si bit 8 de `d`) de valeur `(d>>16) % 1000000007` ; 1 → `((d>>16)%10000).` puis 6 chiffres
  `(d>>40)%1000000` zéro-complétés ; 2 → `((d>>16)%1000).((d>>30)%1000)e` puis `-` si bit 9 de `d` sinon `+`, puis `(d>>50)%30` ; 3 → `0` ;
* `ok` : `draw()%3` = 0 → `true`, 1 → `false`, 2 → `null`.

## Noyau (la seule partie prouvée en F\* et Creusot)
Un **pilote** (non prouvé) ouvre le fichier, le lit par blocs de **1 048 576 octets** avec `read(2)` (le dernier bloc est plus court) et appelle
`chunk(buf, len, st, tab)` sur chaque bloc ; l'état `st` (16 u64) persiste entre les blocs. Le temps mesuré couvre ouverture, lectures et noyau
(pas la génération du fichier ni le calcul de `tab`). `tab` = table CRC-32 de 256 entrées (polynôme réfléchi 0xEDB88320).

Slots de `st` : 0 registre CRC (initial `0xFFFFFFFF`), 1 mode, 2 profondeur, 3 profondeur max, 4 objets, 5 chaînes, 6 nombres, 7 tableaux,
8 mots (littéraux), 9 lignes, 10 somme des id, 11 somme de tous les entiers, 12 accumulateur, 13 `isint`, 14 `gotid`, 15 octets lus.
Tous les compteurs sont modulo 2^64. Pour chaque octet `c` (CRC : `crc = tab[(crc ^ c) & 0xFF] ^ (crc >> 8)` pour tous les octets) :

1. **Fin de nombre** — si mode = 3 (nombre) et `c` n'est ni chiffre ni l'un de `. e E + -` : `nums++` ; si `isint` : `allsum += acc` et, si profondeur = 1 et
   `gotid = 0`, `idsum += acc` ; si profondeur = 1 : `gotid = 1` ; mode ← 0.
   **Fin de mot** — si mode = 4 (mot) et `c` n'est pas dans `a..z` : `lits++` ; mode ← 0. (Dans les deux cas `c` est ensuite traité en mode 0.)
2. Selon le mode : 2 (après `\`) → mode 1 ; 1 (chaîne) → `\` : mode 2, `"` : mode 0, sinon mode 1 ; 3 (nombre) → chiffre : `acc = acc·10 + (c−'0')`, sinon
   (un de `. e E + -`) `isint = 0` ; 4 (mot) → rien ;
3. mode 0 : `"` → `strs++`, mode 1 ; `{` → `objs++`, profondeur++, max ; `[` → `arrs++`, profondeur++, max ; `}` ou `]` → profondeur−− si > 0 ;
   `\n` → `lines++`, `gotid = 0` ; chiffre → mode 3, `acc = c−'0'`, `isint = 1` ; `-` → mode 3, `acc = 0`, `isint = 1` ; `a..z` → mode 4 ; autre → rien.

La fin de fichier ne finalise rien (un nombre ou mot en cours est ignoré).

## Checksum
`ck = CK_INIT` ; `ck = ckfold(ck, v)` pour `v` dans l'ordre : `crc ^ 0xFFFFFFFF`, objets, chaînes, nombres, tableaux, mots, profondeur max, lignes,
somme des id, somme de tous les entiers, octets lus.

Référence indépendante : `zlib.crc32` + un automate Python (voir `spec/checksums.d/stream.json`, vérifié pour n = 1, 4, 16 et 256).

## Colonnes
* C natif, Zig, Rust : lecture (`read`/`File::read`) + automate à la main.
* Rust + Creusot : `chunk` prouvé (bornes, absence de débordement, terminaison) ; `run` (ouverture et lecture du fichier) **`#[trusted]`**.
* F\* → C / F# / OCaml : `Bench.Stream.chunk` (Pulse, mémoire-sûreté) extrait ; la lecture du fichier est écrite dans `fstar/drivers/{driver_stream.h,Driver.fs,driver.ml}`
  (non prouvée). Côté OCaml, le tampon extrait est un `FStar_UInt8.t array` : le pilote y copie chaque octet lu (coût inclus dans le temps mesuré).
