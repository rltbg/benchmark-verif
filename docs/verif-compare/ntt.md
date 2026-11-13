# `ntt` : Creusot contre Verus

Fichiers : `impls/rust/src/ntt.rs` (Creusot), `impls/rust-verus/src/ntt.rs` (Verus).
Journaux : `runs/ntt-creusot.csv`, `runs/ntt-verus.csv`.

**Biais d'ordre.** Creusot d'abord pour la cible de base, puis Verus ; pour la partie « au-delà »
(`powmod`, racines), **Verus d'abord** (avec `vstd::arithmetic::power`), puis Creusot.

## Cible et résultat

Cible : bornes et valeurs < p ; la permutation bit-reverse est une involution ; au-delà si possible
(transformée = DFT modulaire). **Base atteinte dans les deux outils ; au-delà partiellement, au même niveau
dans les deux.** Aucun `trusted`/`assume`/`external_body`.

* **Valeurs < p** (`below_p`) : garanties par `fill`, `twiddles`, chaque `stage`, `transform`, `square`,
  `reverse` ; chaque `r[i] = a[i]·ninv mod p` du checksum est < p par construction.
* **Bit-reverse** : `rev_bits(x, k) = (x mod 2)·2^(k-1) + rev_bits(x/2, k-1)` (fonction logique qui suit la
  boucle du programme `rev = 2·rev + x mod 2 ; x /= 2`). Lemmes prouvés par induction : `rev_bound`
  (`< 2^k`), `rev_top` (`rev(y + t·2^k, k+1) = 2·rev(y, k) + t`), **`rev_involutive`**
  (`rev(rev(x, k), k) = x` sur `[0, 2^k)`). La boucle interne a l'invariant
  `rev(i, k) = rev·2^(k-b) + rev(x, k-b)` (lemme `rev_step`), et **`bitrev` garantit `a'[x] = a[rev(x, k)]`**
  (invariant : les paires `{x, rev x}` avec `x < i ∨ rev x < i` sont échangées, les autres intactes).
* **Au-delà** : `powmod(b, e) = b^e mod p` (invariant `r·b^e ≡ b0^e0`) ; donc `w = g^((p-1)/n) mod p`,
  `ninv = n^(p-2) mod p` ; et `twiddles` garantit `tw[j] = w^j mod p`.
* **Non prouvé** : que la transformée soit la DFT. Il manque (a) la sémantique des papillons de `stage`
  (énoncé par blocs `blk·len + j`, disjonction de blocs : faisable, même technique que les lignes de `matmul`,
  non fait faute de temps), (b) la récurrence de Cooley-Tukey (somme découpée pair/impair sur les indices
  bit-reverse) et (c) que `w` est une racine primitive n-ième de l'unité, ce qui demande
  `31^((p-1)/2) ≡ -1 mod p` : un fait numérique sur un exposant de 10⁹, hors de portée des solveurs sans
  calcul exécuté (`assert by (compute)` en Verus serait à essayer avec une fonction carré-et-multiplie spec).
* Gardes : `run` garde `n > 2^27` et `pw != n` (n pas une puissance de 2) parce que **la spécification
  l'exige** (résultat 0). L'ancien test `n == 0` a été retiré (comme dans `rust-plain`, `n = 0` donne
  `pw = 1 != 0`), l'invariant devient `pw <= 2n ∨ pw = 1`.

### Ce qui diffère entre les outils

* **Puissances.** Verus : `vstd::arithmetic::power::pow` et ses lemmes (`lemma_pow_adds`,
  `lemma_pow_multiplies`, `lemma_pow_mod_noop`) + `lemma_mul_mod_noop_general` : `powmod` et `twiddles`
  vérifiés **au premier essai** (1 appel). Creusot : pas de bibliothèque équivalente utilisée ; il a fallu
  écrire et prouver `pw`, `pw_nonneg`, `pw_succ`, `pw_add`, `pw_sq`, `mod_shift`, `mul_mod`, `pw_mod`,
  `powmod_odd`, `powmod_even` (5 appels en échec + 3 appels `-X`/`-t 5` avant le succès).
* **Dépliage récursif en Creusot.** `pw(b, 1) = b` échouait dans le contexte de `powmod_step` (l'axiome
  de définition de `pw` se déclenche en boucle sur tous les termes `pw b e`) : solution, lemme isolé
  `pw_succ` (petit contexte). Un lemme dont l'`ensures` contient un `if` (`powmod_step`) ne s'instanciait pas
  dans `powmod`, même à 5 s : solution, deux lemmes `powmod_odd`/`powmod_even` appelés dans chaque branche.
* **Verus** : `const P` doit être `pub` pour apparaître dans une `pub open spec fn` ; `P - 1` passé à un lemme
  non linéaire n'était pas évalué (débordement de `r * b` non prouvé) : constantes littérales `2013265920`.
  Fin de boucle : `rev * pow2(0) == rev` demande `by (nonlinear_arith)` (multiplication non linéaire désactivée).

## Chiffres

| | Creusot | Verus |
|---|---|---|
| lignes exécutables / spécification / preuve (*) | 135 / 142 / 148 (ancien module : 134 / 67 / 19) | 164 / 169 / 218 |
| vérification finale à froid | 42,3 s (module seul, `--no-cache`) ; 59,1 s avec `-f` | **1,2 s** pour le module (3,7 s mur pour les 3 modules) |
| appels, base (valeurs < p, bit-reverse) | 2 : 1 erreur de compilation (nom `rev` masqué par la variable `rev`), 1 ok | 3 : 1 erreur de compilation (`const` privée dans une spec publique), 1 échec (5 erreurs), 1 ok |
| appels, au-delà (`powmod`, `tw[j] = w^j`) | 8 : 7 échecs (dont 4 appels isolés `-X`/`-t 5`), 1 ok | 1 : ok |
| autres appels | 2 mesures à froid | 1 build release, 1 mesure |
| `trusted` / `assume` / `external_body` | aucun | aucun |
| checksum | 1024 : `05b245e2ab8b4741`, 4096 : `b05563bb5c4a4cc8`, 65536 : `ee80fab50f65ba9a`, 2^20 : `0edb5c31c12641a4` (= `spec/checksums.d/ntt.json`) ; n = 1000 et 0 → 0 | idem |
| performance (n = 2^20) | 115–160 ms, comme `rust-plain` (machine chargée) | idem |

(*) Même méthode que `matmul.md` ; format `verusfmt` côté Verus.

## Expérience

* **Ce qui a coûté le plus.** Creusot : l'arithmétique des puissances sans bibliothèque, et la fragilité
  des déclencheurs (définitions récursives qui bouclent, lemmes avec `if`). Le bit-reverse, lui, est passé
  en Creusot dès la première version qui compilait. Verus : quasiment rien.
* **Messages d'erreur.** Creusot : `lines 127-197: proof failed` pour un lemme entier ; les buts réels
  (`pw b 1 = b * pw b 0`) n'apparaissent qu'avec `-X`. Verus : assertion ou débordement localisé.
* **Ce qu'une IA plus faible aurait raté.** (1) L'invariant de la boucle bit-reverse sous forme
  « accumulateur » (`rev(i) = rev·2^(k-b) + rev(x, k-b)`) plutôt qu'une formule par bits. (2) L'invariant
  par paires de `bitrev`, qui a besoin de l'involution *pendant* la boucle (pour `x = rev i`). (3) En Creusot,
  isoler les dépliages récursifs et éviter les `if` dans les conclusions de lemmes ; savoir que relancer avec
  plus de temps ne sert à rien quand c'est un problème de déclencheur.
