# Recetario: Fuentes Respaldadas Por Exchanges

Usa fuentes con nombre cuando la estrategia deba obtener velas historicas
directamente desde exchanges compatibles.

```palmscript
interval 1m

source bn = binance.spot("BTCUSDT")
source hl = hyperliquid.perps("BTC")
use hl 1h

plot(bn.close)
plot(hl.1h.close)
```

## Pruebalo En El IDE Del Navegador

Abre [https://palmscript.dev/app/](https://palmscript.dev/app/), pega el
ejemplo en el editor y ejecutalo sobre el historial disponible de BTCUSDT en la
app.

## Que Debes Vigilar

- los scripts conscientes de fuentes deben usar series de mercado calificadas
  por fuente
- `use hl 1h` es obligatorio antes de `hl.1h.close`
- el script sigue teniendo un unico `interval` base global
- el runtime resuelve cada feed requerido `(source, interval)` antes de la
  ejecucion

Referencia:

- [Intervalos y Fuentes](../../reference/intervals-and-sources.md)
