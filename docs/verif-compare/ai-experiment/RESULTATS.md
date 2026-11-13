# Expérience : quel outil un modèle d'IA (même plus faible) prend-il mieux en main ?

**Protocole.** Quatre exercices identiques dans les deux outils, difficulté croissante : `t1` somme bornée (absence de débordement + valeur),
`t2` recherche dichotomique, `t3` inversion en place (`&mut`), `t4` tri par insertion (trié ∧ permutation). Code exécutable et spécification
fournis (`enonces/`), l'agent n'ajoute que les annotations de preuve. Un agent neuf par (modèle, outil), sans autre contexte que `CONSIGNE.md`,
la documentation, la bibliothèque standard et les exemples de l'outil ; 12 appels au vérificateur au plus par exercice ; journal de chaque appel.
Contrôle a posteriori (par moi) : les 24 fichiers rendus sont **re-vérifiés**, aucune ligne de l'énoncé n'est modifiée ou supprimée (seuls des ajouts),
aucun `assume`/`admit`/`trusted`/`external_body`. Solutions de référence : `solutions-reference/`.

| modèle × outil | t1 | t2 | t3 | t4 | appels au total | durée moyenne d'un appel | durée totale de l'agent | jetons consommés |
|---|--:|--:|--:|--:|--:|--:|--:|--:|
| Haiku × Creusot | 1 | 1 | 1 | 2 | **5** | 55 s | 383 s | 102 k |
| Haiku × Verus | 4 | 1 | 4 | 9 | **18** | 1,4 s | 293 s | 140 k |
| Sonnet × Creusot | 1 | 1 | 1 | 1 | **4** | 66 s | 296 s | 80 k |
| Sonnet × Verus | 1 | 1 | 1 | 2 | **5** | 1,6 s | 61 s | 82 k |
| Opus × Creusot | 1 | 1 | 1 | 1 | **4** | 69 s | 332 s | 82 k |
| Opus × Verus | 1 | 1 | 1 | 2 | **5** | 1,5 s | 74 s | 82 k |

Tous les agents ont tout réussi (24/24). Chiffres d'**une seule exécution** par case : un ordre de grandeur, pas une statistique.

**Ce qu'on en retient.**
* **Creusot demande moins d'essais**, surtout au modèle faible (Haiku : 5 contre 18). Raisons citées par les agents : la spécification de `swap`
  (`exchange`) donne la permutation gratuitement ; les préconditions restent visibles dans les boucles ; pas de déclencheurs de quantificateurs à écrire ;
  le portefeuille de solveurs (Alt-Ergo, Z3, CVC5) absorbe des invariants formulés maladroitement. Seul piège rencontré : déréférencer les `Snapshot` (`*old`).
* **Verus est ~40× plus rapide par essai** (1,5 s contre ~60 s, dont l'essentiel est la recompilation du crate par Creusot) : avec Sonnet et Opus, l'agent finit
  **4 à 5 fois plus vite** en temps réel malgré un essai de plus. Avec Haiku, les essais rapides compensent presque (293 s contre 383 s) mais coûtent 40 % de jetons en plus.
* **Pièges Verus rencontrés par tous les modèles** : boucles isolées (il faut répéter les `requires` dans l'invariant), déclencheurs (`#[trigger]`), conversions
  `usize`→`int`, permutation à prouver soi-même (lemme d'échange sur `to_multiset`, groupe `broadcast` à importer sous son nom complet), et la syntaxe récente
  `final(v)` pour les références mutables (changée en 2026 : un modèle entraîné avant se trompe ; le message d'erreur explique la correction).
* **Messages d'erreur** : jugés précis dans les deux cas (Verus pointe l'invariant ou l'assertion exacte ; Creusot nomme le but qui échoue).
* Verus publie une page officielle de conseils pour les agents d'IA (`docs/guide/src/llmforverusproof.md`, avec un « cheat checker ») ; Creusot n'en a pas.
