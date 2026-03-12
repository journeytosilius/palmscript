# Cookbook: Ecart Entre Sources

Ce motif compare deux marches nommes sur la meme horloge de base.

```palmscript
interval 1m

source spot = binance.spot("BTCUSDT")
source perp = binance.usdm("BTCUSDT")

let spread = spot.close - perp.close
plot(spread)
```

## Pourquoi C'est Important

L'execution source-aware construit l'horloge de base a partir de l'union des
timestamps de base des sources declarees.

Cela signifie :

- la strategie s'execute toujours une fois par pas d'intervalle de base
- si une source manque sur un pas, cette source contribue `na`
- les expressions qui dependent de cette entree manquante propagent aussi `na`
  selon la semantique normale

Reference :

- [Semantique d'evaluation](../../reference/evaluation-semantics.md)
- [Intervalles et sources](../../reference/intervals-and-sources.md)
