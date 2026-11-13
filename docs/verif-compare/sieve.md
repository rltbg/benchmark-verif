# `sieve` : Creusot contre Verus

Fichiers : `impls/rust/src/sieve.rs` (Creusot), `impls/rust-verus/src/sieve.rs` (Verus).
Journaux : `runs/sieve-creusot.csv`, `runs/sieve-verus.csv`.

**Biais d'ordre.** La version Creusot a été écrite en premier, la version Verus ensuite, avec les
mêmes invariants et les mêmes lemmes. Le temps de mise au point de Verus est donc celui d'une
traduction : la preuve était déjà conçue.

## Cible et résultat (identiques pour les deux outils)

La cible du BRIEF est **atteinte, et dépassée**, par les deux outils :

* `sieve` : à la sortie, pour tout `2 <= k <= n`, `f[k] == 0 <==> is_prime(k)`. `is_prime` est une
  fonction logique : `2 <= p` et aucun `d` de `[2, p)` ne divise `p`.
* `count` (et donc `run`) renvoie `fold(ck_primes(n), num_primes(n))`. `num_primes` est récursive
  et compte les premiers `<= n`. `ck_primes` replie le checksum sur les premiers pris dans l'ordre.
  Le **checksum entier** est donc spécifié, en plus du compteur demandé.
* La terminaison est prouvée partout (`#[check(terminates)]` + `#[variant]` en Creusot, `decreases`
  en Verus), sauf pour `run` en Creusot (voir « Raccourcis »).
* Invariant central, le même dans les deux outils : `f[k] == 0 <==> !crossed(k, i)`, où
  `crossed(k, b) = ∃ d, 2 <= d < b, d*d <= k, d | k`. Quand `f[i] != 0`, les multiples de `i` sont
  déjà barrés par un diviseur `d` de `i`, grâce à la transitivité de la divisibilité. À la sortie,
  `n < i*i`, et le lemme `crossed_iff_composite` conclut (tout composé a un diviseur `<= √k`).

Différence de formulation : en Creusot, l'`ensures` de `count`/`run` est écrit
`exists<c: u64> c@ == num_primes(n@) && result == fold(ck_primes(n as u64), c)`, parce que pearlite
refuse la conversion `Int as u64` (erreur E0605). Verus écrit directement `num_primes(n) as u64`.

### Gardes supprimées

* La boucle externe de l'ancien code Creusot testait `while i <= n && i <= n / i`. Le test `i <= n`
  ne servait qu'à la preuve : il est retiré, et la condition est maintenant `i <= n / i`, comme dans
  `rust-plain`.
* `cnt.wrapping_add(1)` devient `cnt += 1` : l'absence de débordement est prouvée (`cnt <= i`).
* `mark` ne reçoit plus le paramètre `j0`. Elle calcule `i*i` elle-même, et l'absence de
  débordement vient de `i*i <= n`.
* En Creusot, `ckfold` est recopié dans le module avec un contrat (`result == fold(ck, v)`) : celui
  de `common.rs` n'en a pas, et `common.rs` ne devait pas être modifié. En Verus, `common::ckfold`
  a déjà son contrat.

## Chiffres

| | Creusot | Verus |
|---|---|---|
| lignes exécutables / spécification / preuve (*) | 46 / 73 / 43 (ancien module : 39 / 14 / 0) | 50 / 84 / 169 |
| vérification finale à froid | 48,4 s pour la copie `common` + `sieve` + `hashmap` (cache `.why3find` vidé, compilation comprise, charge machine ≈ 11 sur 4 cœurs) ; `sieve` seul n'est pas mesurable séparément avec `cargo creusot` | **2,3 s** (`cargo verus focus --verify-module sieve` après `cargo clean`, dont 0,12 s de SMT) |
| appels pendant la mise au point (jusqu'au premier « ok ») | 9 : 2 erreurs de compilation, 6 échecs (dont 2 relances avec `-x`/`-X` pour *voir* le but), 1 ok | 3 : 1 erreur de compilation, 1 échec (3 buts), 1 ok |
| autres appels | 3 erreurs de l'outil (why3find ne recalibre pas z3 sous charge), puis 2 runs à froid ok | 1 build release (ok), 3 mesures (dont 1 erreur de ligne de commande et 1 sans effet, notées dans le journal) |
| `trusted` / `assume` / `external_body` | aucun | aucun |
| checksum | vérifié (voir plus bas) | vérifié |

(*) Lignes non vides hors commentaires, classées par script. **Exécutable** : signatures, corps,
`use`, `const`. **Spécification** : contrats, invariants, variants, fonctions logiques ou `spec` qui
renvoient une valeur. **Preuve** : lemmes, `proof_assert!`, `proof { }`, `assert … by`,
`snapshot!`, variables fantômes. Le style `verusfmt` (une clause par ligne, `requires` des
`assert … by (nonlinear_arith)` sur plusieurs lignes) gonfle mécaniquement les chiffres Verus.

### Raccourcis restants

* **Creusot** : `run` n'a pas `#[check(terminates)]`, parce que `std::vec::from_elem` (la
  macro `vec!`) n'est pas spécifiée `terminates` dans creusot-std. Pas d'autre raccourci.
* **Verus** : aucun. `vec![0u8; n+1]` s'appuie sur l'`assume_specification` de `from_elem` fournie
  par vstd, comme toute bibliothèque standard.
* **Les deux outils** : `main` n'est pas vérifié, donc la précondition `2 <= n <= 4·10⁹` de `run`
  n'y est pas contrôlée. C'est le contrat du workload.

## Ce qui a coûté le plus

**Creusot : l'arithmétique modulaire.** Les prouveurs (alt-ergo, z3, cvc5, cvc4 via why3find, 1 s
par but) n'instancient pas d'eux-mêmes les lemmes `ComputerDivision` de Why3.

* `k == d*q ==> k % d == 0` échoue dans un contexte chargé, mais passe seul. Il a fallu des lemmes
  d'essai pour s'en rendre compte, puis un lemme `mod_mul_add(d, q, r) : (d*q + r) % d == r`.
* Ce lemme n'est utilisé que si le terme a **exactement** la forme `(d*q + r) % d`. Il a fallu
  écrire `proof_assert!({ mod_mul_add(i, j/i+1, 0); (i*(j/i+1)+0) % i == 0 })`, avec le `+ 0`.
* Trouver le but en échec demande `--why3find-arg=-X`. Le message par défaut dit seulement
  `line 132 characters 1-49: proof failed, Goal vc_mark ✘ (16/17)`, en pointant la signature de la
  fonction, sans dire lequel des 17 sous-buts échoue.

**Verus : trois corrections.**

* Nouvelle syntaxe des références mutables : `final(f)` dans les `ensures`.
* Arithmétique non linéaire en petits pas : prouver `q < d` avant `q*q <= k`, car
  `by (nonlinear_arith)` ne voit que ses `requires`.
* `j += i` : Verus ne suppose pas que `usize` fait 64 bits. Il a fallu prouver `i <= 2^16`, d'où
  `j + i < 2^32`.

Les messages pointent la bonne assertion ou le bon débordement. Chaque appel prend environ 3 s.

## Qualité des messages d'erreur

* **Creusot** : la position est grossière (une fonction et un compte de sous-buts). La tâche Why3
  s'obtient avec `-X`, lisible mais longue (des centaines de lignes d'axiomes). Les erreurs de
  compilation pearlite sont claires : `@` hors `pearlite!`, cast `Int → u64`, fonction
  non-`terminates` appelée.
* **Verus** : messages précis (assertion, postcondition avec le point de sortie, débordement), et
  `--expand-errors` détaille les conjonctions.

## Ce qu'un modèle plus faible aurait probablement raté

* **En Creusot** :
  * que l'échec vient du filtrage syntaxique des axiomes, d'où l'astuce `+ 0` ou une instance
    explicite ;
  * qu'il faut `-X` pour identifier le sous-but ;
  * que `@` n'est permis que dans `pearlite!` ;
  * que l'on ne peut pas convertir `Int` en `u64`, d'où la postcondition existentielle.
* **En Verus** :
  * `final(f)` contre `old(f)` ;
  * le contexte vide de `nonlinear_arith`, qui oblige à passer les hypothèses par `requires` ;
  * la largeur inconnue de `usize` ;
  * l'interdiction de `%` dans un déclencheur, d'où la fonction `divides`.
* **Dans les deux outils** : la *bonne* formulation de l'invariant (`crossed(k, i)` avec `d*d <= k`)
  et le cas « `i` déjà barré », qui utilise la transitivité de la divisibilité. Sans eux on se
  replie sur « pas de panique ».

## Checksums et performances

| n | Creusot | Verus | rust-plain / référence |
|---|---|---|---|
| 2 | 08395307b4f1348c | 08395307b4f1348c | 08395307b4f1348c |
| 1 000 000 (smoke) | 0c28b4b013f8d676 | 0c28b4b013f8d676 | 0c28b4b013f8d676 (`spec/checksums.json`) |
| 20 000 000 (small) | 3cc7c532bcb3506c | 3cc7c532bcb3506c | 3cc7c532bcb3506c |
| 200 000 000 | 69d55ea2e02061e8 | 69d55ea2e02061e8 | 69d55ea2e02061e8 (`spec/checksums.json`) |

Temps à n = 2·10⁸ : environ 3,3 s pour chacune des trois versions (machine partagée et chargée,
les écarts restent dans le bruit). Le code exécuté est celui de `rust-plain`.
