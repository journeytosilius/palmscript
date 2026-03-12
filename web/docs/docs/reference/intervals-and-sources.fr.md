# Intervalles Et Sources

Cette page definit les regles normatives des intervalles et des sources dans
PalmScript.

## Intervalles Pris En Charge

PalmScript accepte les litteraux d'intervalle listes dans la
[Table des intervalles](intervals.md). Les intervalles sont sensibles a la
casse.

## Intervalle De Base

Chaque script declare exactement un intervalle de base :

```palmscript
interval 1m
```

L'intervalle de base definit l'horloge d'execution.

## Sources Nommees

Les scripts executables declarent une ou plusieurs sources nommees adossees a
des exchanges :

```palmscript
interval 1m
source hl = hyperliquid.perps("BTC")
source bn = binance.spot("BTCUSDT")
use hl 1h

plot(bn.close - hl.1h.close)
```

Regles :

- au moins une declaration `source` est requise
- les series de marche doivent etre qualifiees par source
- chaque source declaree fournit un flux de base sur l'intervalle de base du
  script
- `use <alias> <interval>` declare un intervalle supplementaire pour cette
  source
- `<alias>.<field>` reference cette source sur l'intervalle de base
- `<alias>.<interval>.<field>` reference cette source sur l'intervalle nomme
- les references a un intervalle inferieur a l'intervalle de base sont
  rejetees

## Templates De Source Pris En Charge

PalmScript prend actuellement en charge ces templates de premiere classe :

- `binance.spot("<symbol>")`
- `binance.usdm("<symbol>")`
- `hyperliquid.spot("<symbol>")`
- `hyperliquid.perps("<symbol>")`

La prise en charge des intervalles depend du template :

- `binance.spot` accepte tous les intervalles PalmScript pris en charge
- `binance.usdm` accepte tous les intervalles PalmScript pris en charge
- `hyperliquid.spot` rejette `1s` et `6h`
- `hyperliquid.perps` rejette `1s` et `6h`

Les contraintes operationnelles de recuperation dependent aussi du template :

- l'API REST Hyperliquid n'expose que les `5000` bougies les plus recentes par
  flux
- le mode marche rejette toute demande de flux Hyperliquid qui depasse cette
  fenetre de retention
- les flux Binance sont pagines en interne et n'ont pas la meme limite de
  retention sur la fenetre complete

## Ensemble Des Champs De Source

Tous les templates de source sont normalises vers les memes champs de marche
canoniques :

- `time`
- `open`
- `high`
- `low`
- `close`
- `volume`

Regles :

- `time` est l'heure d'ouverture de la bougie en millisecondes Unix UTC
- les champs de prix et de volume sont numeriques
- les champs supplementaires specifiques au venue ne sont pas exposes dans le
  langage

## Intervalles Egaux, Superieurs Et Inferieurs

PalmScript distingue trois cas pour un intervalle reference relativement a
l'intervalle de base :

- intervalle egal : valide
- intervalle superieur : valide s'il est declare avec `use <alias> <interval>`
- intervalle inferieur : rejete

## Semantique Runtime

En mode marche :

- PalmScript recupere directement depuis les venues les flux `(source, interval)`
  requis
- la timeline d'execution de base est l'union des heures d'ouverture des barres
  d'intervalle de base de toutes les sources declarees
- si une source n'a pas de barre de base a une etape de la timeline, cette
  source fournit `na` pour cette etape
- les intervalles de source plus lents conservent leur derniere valeur
  completement cloturee jusqu'a leur prochaine frontiere de cloture

## Garantie Sans Lookahead

PalmScript ne doit pas exposer une bougie d'intervalle superieur avant sa
cloture complete.

Cela s'applique aux intervalles qualifies source-aware comme `hl.1h.close`.

## Regles D'Alignement Runtime

Les flux prepares doivent etre alignes sur leurs intervalles declares.

Le runtime rejette les flux qui sont :

- mal alignes sur la frontiere de l'intervalle
- non tries
- dupliques pour une meme heure d'ouverture d'intervalle
