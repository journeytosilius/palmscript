# Apprendre PalmScript

La documentation publique de PalmScript s'organise autour de :

- le langage pour ecrire des strategies
- des exemples qui montrent comment les scripts sont ecrits et utilises

## Ce Que Vous Faites Avec PalmScript

Flux typique :

1. ecrire un script `.ps`
2. declarer un `interval` de base
3. declarer une ou plusieurs liaisons `source`
4. le valider dans l'IDE navigateur
5. l'executer sur des donnees historiques dans l'application

## Optimisations Longues

Pour les longues recherches de tuning en CLI :

- utilisez `palmscript run optimize ...` quand vous voulez le resultat au premier plan
- utilisez `palmscript runs submit optimize ...` quand la recherche doit continuer dans un etat local durable et conserver chaque candidat termine
- revenez plus tard avec `palmscript runs status <run-id>`, `palmscript runs show <run-id>`, `palmscript runs tail <run-id>` ou `palmscript runs best <run-id> --preset-out best.json`

## Que Lire Ensuite

- Premier flux executable : [Demarrage Rapide](quickstart.md)
- Premiere presentation complete d'une strategie : [Premiere Strategie](first-strategy.md)
- Vue d'ensemble du langage : [Vue d'ensemble du langage](language-overview.md)
- Regles et semantique exactes : [Vue d'ensemble de la Reference](../reference/overview.md)

## Roles De La Documentation

- `Apprendre` explique comment utiliser PalmScript efficacement.
- `Reference` definit ce que signifie PalmScript.
