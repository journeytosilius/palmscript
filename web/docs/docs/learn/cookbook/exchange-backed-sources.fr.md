# Cookbook: Sources Adossees A Un Exchange

Utilisez des sources nommees lorsque la strategie doit recuperer directement
des chandelles historiques depuis des exchanges pris en charge.

```palmscript
interval 1m

source bn = binance.spot("BTCUSDT")
source hl = hyperliquid.perps("BTC")
use hl 1h

plot(bn.close)
plot(hl.1h.close)
```

## L'Essayer Dans L'IDE Navigateur

Ouvrez [https://palmscript.dev/app/](https://palmscript.dev/app/), collez
l'exemple dans l'editeur, puis executez-le sur l'historique BTCUSDT
disponible dans l'application.

## Points A Surveiller

- les scripts source-aware doivent utiliser des series de marche qualifiees par
  source
- `use hl 1h` est requis avant `hl.1h.close`
- le script conserve un seul `interval` de base global
- le runtime resout chaque flux `(source, interval)` requis avant l'execution

Reference :

- [Intervalles et sources](../../reference/intervals-and-sources.md)
