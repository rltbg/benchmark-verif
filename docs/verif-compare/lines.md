# `lines` : Creusot contre Verus

Fichiers : `impls/rust/src/lines.rs` (Creusot) et `impls/rust-verus/src/lines.rs` (Verus).
Journaux : `runs/lines-creusot.csv`, `runs/lines-verus.csv`.

**Biais d'ordre.** La version Creusot a été écrite en premier. La version Verus en reprend le modèle,
les lemmes et la structure de l'invariant : son temps de mise au point est celui d'une traduction.

## Cible et résultat (identiques pour les deux outils)

La cible du BRIEF est **atteinte et dépassée** par les deux outils : l'agrégat par clé est prouvé,
le lemme d'aller-retour décimal aussi, et le texte est relié au générateur `mix64`.

* **Modèle.**
  * `dec(v)` : écriture décimale ASCII sans zéro de tête, en `Seq<Int>` (Creusot) ou en `Seq<u8>` (Verus).
  * `num(s, j)` : valeur des `j` premiers chiffres, par le schéma de Horner.
  * `line(k, v) = dec(k) ';' dec(v) '\n'` ; `text(n)` concatène les `n` premières lignes.
  * Le i-ème tirage vaut `mix64(SEED + i)`, avec un `mix64` logique écrit en vecteurs de bits.
    La clé est `r >> 44`, la valeur `((r >> 8) & 0xFFFFFFFF) mod 10^6`.
  * `sum_v(n, k)` et `cnt(n, k)` : somme des valeurs et nombre des lignes de clé `k`.
* **Lemme d'aller-retour** `num_dec` : `num(dec(v), |dec(v)|) = v`. Tout préfixe de `dec(v)` vaut
  au plus `v`, ce qui sert à prouver l'absence de débordement pendant l'analyse.
* **`generate`** : le buffer est exactement `text(n)`.
* **`parse`** : `tab[2k] = sum_v(n, k)` et `tab[2k+1] = cnt(n, k)` pour toute clé `k < 2^20`.
  La preuve suit la machine à états octet par octet ; l'invariant situe `p` dans la ligne `i`.
* **Terminaison** prouvée partout (`#[check(terminates)]` et `#[variant]` ; `decreases`), sauf `run`
  en Creusot : `vec!` n'y est pas `terminates`.
* `checksum` n'a pas de spécification fonctionnelle : la cible ne la demande pas.

### Gardes et calculs supprimés

* **Masque `key % 2^20` à l'indexation de `tab`** : supprimé. On prouve `r >> 44 < 2^20` par
  `#[bitwise_proof]` (Creusot) ou `by (bit_vector)` (Verus), puis que la clé relue est cette clé.
* **Garde `p < len`** : il n'y a plus de longueur utile.
  * Avant, le texte était écrit dans un buffer pré-rempli de `15n+20` zéros, à des positions
    calculées, et `parse` comparait `p` à une longueur rendue par `generate`.
  * Maintenant, `generate` pousse les octets (`Vec::with_capacity` puis `push`, comme `rust-plain`)
    et `parse` parcourt `txt` en entier. La seule borne est celle de la boucle, `p < txt.len()`.
  * Le remplissage initial de `15n` octets disparaît aussi.
* **Additions `wrapping_*`** dans `parse` : remplacées par `+`. On prouve qu'elles ne débordent pas
  (somme `<= 999 999·n < 2^64`, compte `<= n`, accumulateurs `<= v < 10^7`).
  Le code machine est le même que celui de `wrapping_add` en mode release.
* **`ndigits` / `write_dec`** (écriture de droite à gauche à une position calculée) : remplacés par
  le `put_dec` de `rust-plain`, qui remplit un tampon local de 10 octets puis le recopie à l'envers.

## Chiffres

| | Creusot | Verus |
|---|---|---|
| lignes exécutables / spécification / preuve (*) | 96 / 74 / 156 (ancien : 99 / 10 / 11) | 89 / 91 / 277 |
| vérification finale à froid | **63 s** sur une copie sans `proof.json` ni cache why3find (charge ≈ 5). 28,5 s avec `--no-cache` en rejouant les `proof.json` | **2,1 s** (`cargo verus focus --verify-module lines --time`, dont 0,77 s de vérification) |
| appels jusqu'au premier « ok » | 10 : 2 erreurs de compilation, 7 échecs (dont 2 relances `-x`/`-X` pour voir les buts), 1 ok | 10 : 2 erreurs de compilation, 7 échecs (dont 1 avec `--expand-errors`), 1 ok |
| autres appels | 3 ok : complet avec cache, `--no-cache`, copie vierge | 1 build release ok, 1 erreur de ligne de commande, 2 mesures à froid |
| `trusted` / `assume` / `external_body` | aucun | aucun |
| checksum | `lines:10000 = ed29565c33f35ce6` ; aussi 10⁶ et 4·10⁶, identiques à `rust-plain` | idem |
| temps d'exécution (4·10⁶ lignes, meilleur de 5) | 226 ms (plain 222 ms, ancien Creusot 266 ms) | 245 ms (mesures bruitées : machine partagée) |

(*) Comptage approximatif par script, hors lignes vides et commentaires :
* les sections « Modèle » comptent comme spécification et « Lemmes » comme preuve ;
* dans le code, `requires`/`ensures` comptent comme spécification ;
* `invariant`, `variant`/`decreases`, `proof_assert!`, `snapshot!`, `proof { }` et `assert` comptent
  comme preuve ; le reste (accolades et signatures comprises) est exécutable.

Le style `verusfmt` (une clause par ligne) gonfle les chiffres Verus. En Creusot, `mix64` est
recopié dans le module avec son contrat logique, car celui de `common.rs` n'en a pas.

## Ce qui a coûté le plus

### Creusot

1. **Pas de conversion `Int → u64` dans la logique** (`i as u64` y est refusé).
   * La suite des tirages ne peut donc pas être une fonction logique de l'indice.
   * Contournement : une suite fantôme `rs: Seq<u64>` construite par `generate`, et caractérisée par
     `forall<j: u64> j@ < n ==> rs[j@] == mix64_l(SEED + j)`.
   * Il faut alors un lemme `text_snoc` : `text` ne dépend que des `i` premiers tirages.
2. **Localiser un but en échec.**
   * Sans option, why3find ne donne que « `vc_parse` ✘ (44/48) » sur la ligne de la fonction.
   * Il faut relancer avec `--why3find-arg=-x` (sous `script`, car l'option exige un terminal) pour
     obtenir la ligne, et avec `-X` pour voir la tâche (par exemple `in_bounds(tab[2k+1] + 1)`).
3. **Les indices concaténés.** L'égalité `txt = text(i) ++ line(...)` après quatre écritures, et la
   lecture de l'octet `p` dans la ligne `i`, ont demandé des assertions morceau par morceau. Par
   exemple, l'indice `t1.len() + q` a dû être réécrit `t0.len() + |kd| + 1 + q` pour que le
   déclencheur s'applique. Deux tours.

### Verus

1. **Les déclencheurs.**
   * L'invariant de la table `forall k. tab[2k] == sum ∧ #[trigger] tab[2k+1] == cnt` ne prouvait que
     la moitié « compte » : avec `--expand-errors`, le but de la conjonction « somme » ne contient
     pas le terme `tab[2k+1]`.
   * Deux `#[trigger]` dans la même formule forment un **multi-déclencheur**, ce qui n'est pas le
     comportement voulu.
   * La solution : deux `forall` séparés, déclenchés par `sum_v(i, k)` et par `cnt(i, k)`.
2. **Le dépassement de `rlimit`** sur la boucle de `parse`. Résolu en rendant opaques
   (`#[verifier::opaque]`) le prédicat d'agrégat `agg_ok` et le tirage `draw` (qui contient
   `spec_mix64`), puis en ajoutant deux lemmes `lemma_agg_get` et `lemma_agg_step`. On passe de
   l'échec par `rlimit` à 1 s.
3. **`loop` avec `break`.** Il faut répartir les invariants entre `invariant_except_break` et
   `invariant`, avec un `ensures` de boucle. Le message d'erreur l'indique lui-même (« hint: use
   `invariant_except_break` »).
4. **Une borne non inductive.** `dec_shape` (`v < 10^6 ⇒ |dec v| <= 6`) passait en Creusot, où
   Why3 déplie davantage. En Verus, il a fallu écrire les sept bornes intermédiaires (`v < 10^k`).

## Qualité des messages d'erreur

* **Verus** est nettement meilleur pour localiser :
  * la clause d'invariant ou d'`ensures` fautive est soulignée ;
  * `--expand-errors` découpe une conjonction et marque ✔/✘ chaque partie ;
  * les erreurs de type (`nat` passé là où un indice `int` est attendu) viennent de rustc et sont
    claires.
* **Creusot** :
  * le résumé par défaut donne un nombre de buts par fonction et la ligne de la fonction ;
  * `-x` donne la ligne de l'assertion ou de l'invariant (étiquetée « loop invariant #2 ») ;
  * `-X` donne la tâche Why3 complète, lisible mais longue (500 à 800 lignes) ;
  * une erreur d'appel de lemme (précondition) n'est pas distinguée d'un échec de preuve.

## Ce qu'un modèle plus faible aurait probablement raté

* **Creusot** :
  * le contournement de l'absence de `Int → u64` (suite fantôme et lemme `text_snoc`) ;
  * le besoin de `#[bitwise_proof]`, documenté seulement dans des tests ;
  * les assertions de découpage de `bytes_are` par régions.
* **Verus** :
  * le diagnostic du déclencheur absent d'une branche de conjonction, et la confusion
    multi-déclencheur ;
  * le recours à `#[verifier::opaque]` pour sortir d'un `rlimit`, plutôt qu'une hausse du `rlimit`.
* **Les deux** :
  * la spécification elle-même, c'est-à-dire le choix de `num` sur un préfixe (`num(s, j)`, sans
    `subsequence`) pour éviter les égalités de sous-suites ;
  * l'invariant de la machine à états, qui situe `p` dans la ligne `i` par l'offset
    `p - |text(i)|`.
