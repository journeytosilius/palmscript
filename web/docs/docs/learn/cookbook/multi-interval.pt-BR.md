# Cookbook: Estrategia Multi-Intervalo

Este padrao adiciona um contexto mais lento a uma estrategia base mais rapida
ou de mesma velocidade.

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

## Teste No IDE Do Navegador

Abra [https://palmscript.dev/](https://palmscript.dev/), cole o exemplo
no editor e execute-o em um intervalo de datas que cubra varios fechamentos
semanais.

## O Que Observar

- `use spot 1w` e obrigatorio antes de `spot.1w.close`
- valores de intervalos superiores aparecem apenas depois que o candle
  superior fecha completamente
- nenhum candle semanal parcial e exposto
- a indexacao compoe no clock mais lento, nao no clock base

Referencia:

- [Intervalos e Fontes](../../reference/intervals-and-sources.md)
- [Series e Indexacao](../../reference/series-and-indexing.md)
- [Semantica De Avaliacao](../../reference/evaluation-semantics.md)
