# Glossaire

## Intervalle De Base

L'intervalle d'execution declare par `interval <...>`.

## Intervalle Declare

Un intervalle explicitement introduit via `use <alias> <...>` pour une source
nommee.

## Script Source-Aware

Un script qui declare au moins une `source`.

## Template De Source

Un constructeur integre d'exchange / venue comme `binance.spot` ou
`hyperliquid.perps`.

## Mode Marche

Execution contre des flux historiques adosses au marche.

## No Lookahead

La garantie qu'un echantillon d'intervalle superieur ne devient visible
qu'apres la cloture complete de cette bougie.

## Serie De Sortie

Un resultat nomme emis a chaque pas par `export` ou `trigger`.

## Union Des Timestamps De Base

La timeline d'execution construite a partir de l'union des heures d'ouverture
des bougies d'intervalle de base pour toutes les sources declarees.
