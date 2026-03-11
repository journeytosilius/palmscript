# Recetario: Spread Entre Fuentes

Este patron compara dos mercados con nombre sobre el mismo reloj base.

```palmscript
interval 1m

source spot = binance.spot("BTCUSDT")
source perp = binance.usdm("BTCUSDT")

let spread = spot.close - perp.close
plot(spread)
```

## Por Que Importa

La ejecucion consciente de fuentes construye el reloj base a partir de la union
de los timestamps base de las fuentes declaradas.

Eso significa:

- la estrategia sigue ejecutandose una vez por cada paso del intervalo base
- si una fuente falta en un paso, esa fuente aporta `na`
- las expresiones que dependen de esa entrada faltante tambien propagan `na`
  segun la semantica normal

Referencia:

- [Semantica De Evaluacion](../../reference/evaluation-semantics.md)
- [Intervalos y Fuentes](../../reference/intervals-and-sources.md)
