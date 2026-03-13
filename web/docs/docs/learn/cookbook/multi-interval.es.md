# Recetario: Estrategia Multi-Intervalo

Este patron agrega contexto mas lento a una estrategia base mas rapida o igual.

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

## Pruebalo En El IDE Del Navegador

Abre [https://palmscript.dev/](https://palmscript.dev/), pega el
ejemplo en el editor y ejecutalo sobre un rango de fechas que cubra multiples
cierres semanales.

## Que Debes Vigilar

- `use spot 1w` es obligatorio antes de `spot.1w.close`
- los valores de intervalos superiores solo aparecen cuando la vela superior ya
  cerro por completo
- no se expone ninguna vela semanal parcial
- la indexacion compone sobre el reloj del intervalo lento, no sobre el reloj
  base

Referencia:

- [Intervalos y Fuentes](../../reference/intervals-and-sources.md)
- [Series E Indexacion](../../reference/series-and-indexing.md)
- [Semantica De Evaluacion](../../reference/evaluation-semantics.md)
