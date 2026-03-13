# Cookbook: Strategie Multi-Intervalle

Ce motif ajoute un contexte plus lent a une strategie de base plus rapide ou
de meme vitesse.

```palmscript
interval 1d
source spot = binance.spot("BTCUSDT")
use spot 1w

let weekly_basis = ema(spot.1w.close, 8)

if spot.close > weekly_basis {
    plot(1)
} else {
    plot(0)
}
```

## L'Essayer Dans L'IDE Navigateur

Ouvrez [https://palmscript.dev/](https://palmscript.dev/), collez
l'exemple dans l'editeur, puis executez-le sur une plage de dates qui couvre
plusieurs clotures hebdomadaires.

## Points A Surveiller

- `use spot 1w` est requis avant `spot.1w.close`
- les valeurs d'intervalle superieur n'apparaissent qu'apres la cloture
  complete de la bougie superieure
- aucune bougie hebdomadaire partielle n'est exposee
- l'indexation se compose sur l'horloge plus lente, pas sur l'horloge de base

Reference :

- [Intervalles et sources](../../reference/intervals-and-sources.md)
- [Series et indexation](../../reference/series-and-indexing.md)
- [Semantique d'evaluation](../../reference/evaluation-semantics.md)
