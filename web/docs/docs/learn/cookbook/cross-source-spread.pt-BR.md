# Cookbook: Spread Entre Fontes

Este padrao compara dois mercados nomeados no mesmo clock base.

```palmscript
interval 1m

source spot = binance.spot("BTCUSDT")
source perp = binance.usdm("BTCUSDT")

let spread = spot.close - perp.close
plot(spread)
```

## Por Que Isso Importa

A execucao source-aware constroi o clock base a partir da uniao dos timestamps
base das fontes declaradas.

Isso significa:

- a estrategia ainda executa uma vez por passo do intervalo base
- se uma fonte estiver ausente em um passo, essa fonte contribui com `na`
- expressoes que dependem dessa entrada ausente tambem propagam `na` pela
  semantica normal

Referencia:

- [Semantica De Avaliacao](../../reference/evaluation-semantics.md)
- [Intervalos e Fontes](../../reference/intervals-and-sources.md)
