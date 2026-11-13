# stream — Creusot contre Verus

Fichiers : `impls/rust/src/stream.rs` (+ `verif/bench_rust_rlib/stream/`), `impls/rust-verus/src/stream.rs`.
Journaux : `runs/stream-creusot.csv`, `runs/stream-verus.csv`.

## Cible et résultat

Cible : « le CRC calculé égale le CRC-32 logique du flux ; les compteurs du tokeniseur correspondent à un automate
logique ». **Atteinte dans les deux outils**, et au-delà du noyau : la boucle de lecture elle-même est vérifiée,
seules l'ouverture du fichier et l'appel `read` ne le sont pas.

* `crc_seq(crc, s)` : CRC-32 **bit à bit** (8 décalages conditionnels par octet, polynôme réfléchi 0xEDB88320) —
  la table n'apparaît pas dans la spécification ; `crc32(s) = crc_seq(0xFFFFFFFF, s) ^ 0xFFFFFFFF`.
  `crc_table` est prouvée (`tab[i] = crc_iter(i, 8)`) et le lemme bit à bit `crc8_split`
  (`crc8(x) = crc8(x & 0xff) ^ (x >> 8)`) justifie la mise à jour par table du code.
* `tok_seq(t, s)` : repliement de l'automate `tok_step = tok_act ∘ tok_end`, transcription des règles 1–3 de
  `spec/workloads.d/stream.md` sur un état `Tok` (mode, profondeur, max, 6 compteurs, sommes, accumulateur, drapeaux).
* `chunk(buf, len, st, tab)` garantit `crc' = crc_seq(crc, buf[..len])`, `tok' = tok_seq(tok, buf[..len])`,
  `total' = total + len` (mod 2^64) ; lemmes de concaténation `crc_seq(a ++ b) = crc_seq(crc_seq(a), b)` (idem `tok_seq`).
* `run` : invariant « l'état est celui de l'automate sur `seen` », `seen` = concaténation fantôme des blocs lus ;
  garantie finale `∃ s. résultat == ck_stream(s)` = checksum (CRC-32 de s, compteurs de `tok_seq(tok0, s)`,
  |s| mod 2^64). On ne peut pas dire plus sans modèle du fichier : `s` est « ce que `read` a rendu ».

Non vérifié (listé en tête des modules) : ouverture (`BENCH_DATA_DIR`, `File::open`) et `read` — Creusot `#[trusted]`
(`open_input`, `read_block`), Verus `#[verifier::external_body]` (type opaque `Input`, `Input::open`,
`Input::read_block`) ; seul contrat : `read <= buf.len()`, longueur du tampon conservée. Verus : `run` porte en plus
`#[verifier::exec_allows_no_decreases_clause]` (la fin de la boucle de lecture dépend du fichier) ; en Creusot `run`
n'a simplement pas `#[check(terminates)]`. Toutes les autres fonctions terminent (prouvé).

Changements de code par rapport à l'ancienne version Creusot (alignés sur `rust-plain`) : état en `struct`
(`State { crc, tok: Tok, total }`) au lieu d'un `Vec<u64>` de 16 cases, booléens pour `isint`/`gotid`, table `[u32; 256]`
sur la pile ; transition dans `tok_step_exec` `#[inline(always)]` appliquée à une copie locale de `Tok` (registres).
Le mode 0 est la branche « sinon », comme dans `rust-plain` (le mode reste dans 0..4).

**Checksums vérifiés** : `stream:1 = 4a0908c08b30ffe1`, `stream:4 = ab4c19c0e991cc61` (Creusot et Verus).
Temps n = 4 (min de 6, cache chaud) : plain 15,6 ms, Creusot 14,2 ms, Verus 15,1 ms.

## Chiffres

| | Creusot | Verus |
|---|---|---|
| lignes exec / spec / preuve (≈) | 183 / 158 / 58 | 234 / 151 / 86 |
| vérification finale à froid | 36,9 s mur (20 fichiers, why3find `-f --no-cache`) | 3,6 s (`verify-crate-time`, module seul) |
| appels (dev) | 3 (+ final) : 1 erreur de compilation, 1 échec | 6 (+ final) : 1 erreur de compilation, 4 échecs |
| hors preuve | `open_input`, `read_block` (`#[trusted]`) | `Input` + 2 méthodes (`external_body`), `exec_allows_no_decreases_clause` sur `run` |

## Ce qui a coûté le plus

* **Creusot** (fait après sha256, avec ses leçons : conversion `u64_of` fermée dans un sous-module, lemmes en
  `bitwise_proof`) : seulement la borne de l'indice de table — `(x & 0xff) < 256` n'est pas connu hors mode bit à bit
  (« index in bounds » ; l'ancien code contournait avec `% 256`) — et le cas `b` vide des lemmes de concaténation
  (`a ++ [] == a` par extensionnalité). `derive(Clone)` est ambigu avec le prélude Creusot (il faut
  `creusot_std::prelude::Clone`). La syntaxe `Tok { champ: …, ..t }` fonctionne en logique.
* **Verus** : 4 échecs sur la mise à jour CRC. Cause réelle : j'avais instancié le lemme
  `(x ^ c) >> 8 == x >> 8` avec `x := crc ^ c` au lieu de `x := crc` — le lemme « passait » mais ne servait à rien.
  Trouvé en découpant l'assertion en étapes ; Verus a pointé l'étape fausse immédiatement.
  `|s| mod 2^64` : `lemma_add_mod_noop` de vstd ; `reveal_with_fuel(crc_iter, 9)` pour déplier 8 décalages.

## Messages d'erreur

Creusot : sous-but « index in bounds » visible seulement via la session Why3. Verus : chaque assertion intermédiaire
fausse est désignée par sa ligne ; « invariant not satisfied at end of loop body » localise la boucle.

## Ce qu'un modèle plus faible aurait probablement raté

* Spécifier le CRC *sans la table* (sinon la « preuve » est une tautologie) et prouver l'équivalence table/bit à bit
  par un lemme de linéarité en vecteurs de bits.
* Aller au-delà du noyau : rendre `run` vérifiable en isolant seulement `open`/`read` dans des fonctions de confiance,
  avec un état fantôme `seen` et des lemmes de concaténation pour composer les blocs.
* Le compteur d'octets modulo 2^64 (en Creusot : définir la conversion `Int → u64` soi-même et prouver
  `u64_of(x + v) == u64_of(x) + v`).
