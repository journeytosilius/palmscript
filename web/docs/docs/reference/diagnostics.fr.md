# Diagnostics

PalmScript expose des diagnostics et des erreurs depuis trois couches
publiques.

## 1. Diagnostics De Compilation

Les diagnostics de compilation sont des echecs au niveau source avec des spans.

Classes de diagnostics :

- erreurs lexicales
- erreurs de parsing
- erreurs de types et de resolution de noms
- erreurs structurelles a la compilation

Exemples :

- `interval` manquant ou duplique
- template `source` non pris en charge
- alias de source inconnu
- reference d'intervalle `use` non declaree
- reference a un intervalle inferieur a l'intervalle de base
- liaisons dupliquees
- recursion de fonction invalide
- arite ou type d'argument builtin invalide

Ces diagnostics apparaissent via :

- le panneau de diagnostics de l'editeur dans l'IDE navigateur
- les requetes de backtest emises par l'application hebergee

## 2. Erreurs De Recuperation Marche

Apres une compilation reussie, la preparation du runtime peut echouer pendant
l'assemblage des flux historiques requis.

Exemples :

- la fenetre temporelle demandee est invalide
- le script n'a aucune declaration `source`
- une requete exchange echoue
- la reponse du venue est mal formee
- un flux requis ne renvoie aucune donnee dans la fenetre demandee
- un symbole ne peut pas etre resolu par le venue choisi

## 3. Erreurs D'Execution

Les erreurs d'execution surviennent apres le debut de la preparation des flux
ou pendant l'execution.

Exemples :

- erreurs d'alignement des flux
- flux runtime manquants ou dupliques
- epuisement du budget d'instructions
- stack underflow
- incompatibilite de type pendant l'execution
- slot local ou de serie invalide
- depassement de capacite d'historique
- incompatibilite de type de sortie pendant la collecte des sorties

## Responsabilite Des Couches

La couche responsable d'un echec fait partie du contrat :

- la validite syntaxique et semantique releve de la compilation
- la validite exchange / reseau / reponse releve de la recuperation marche
- la coherence des flux et la validite d'execution relevent du runtime

PalmScript echoue explicitement au lieu de degrader silencieusement la
semantique.
