# Consigne de l'expérience

Tu as quatre exercices de vérification formelle de programmes Rust (t1 à t4, difficulté croissante) dans ton dossier de travail.
Chaque fichier contient un énoncé en tête. Le code exécutable et la spécification sont donnés ; tu dois ajouter les annotations
de preuve pour que le vérificateur accepte le programme.

Règles strictes :
* Travaille uniquement dans ton dossier de travail. Tu peux lire la documentation et la bibliothèque standard de l'outil
  (chemins ci-dessous) et ses exemples. N'ouvre aucun autre fichier de /tmp/claude-0 ni de /home/user/benchmark-verif.
* Ne modifie ni la spécification ni le code exécutable ; aucune hypothèse non prouvée (voir l'en-tête de chaque fichier).
* Au plus 12 appels au vérificateur par exercice. Après 12 échecs, abandonne l'exercice et passe au suivant.
* Après CHAQUE appel au vérificateur, ajoute une ligne à `journal.csv` : `t<N>,<numéro d'essai>,<durée en s>,<ok|echec|erreur_syntaxe>,<note très courte sur l'erreur>`.
* Fais les exercices dans l'ordre t1, t2, t3, t4.

Rapport final (≤ 15 lignes) : pour chaque exercice, réussi ou non et en combien d'appels ; ce qui t'a le plus gêné ; ce qui t'a aidé.
