# Recetario: Fuentes Respaldadas Por Exchanges

Usa fuentes con nombre cuando la estrategia deba obtener velas historicas
directamente desde exchanges compatibles.

```palmscript
interval 1m

source bn = binance.spot("BTCUSDT")
source bb = bybit.usdt_perps("BTCUSDT")
use bb 1h

plot(bn.close)
plot(bb.1h.close)
```

PalmScript tambien soporta templates de fuente para Bybit y Gate:

- `bybit.spot("BTCUSDT")`
- `bybit.usdt_perps("BTCUSDT")`
- `gate.spot("BTC_USDT")`
- `gate.usdt_perps("BTC_USDT")`

Ejemplos representativos incluidos en el repositorio:

- `crates/palmscript/examples/strategies/bybit_spot.ps`
- `crates/palmscript/examples/strategies/bybit_usdt_perps_backtest.ps`
- `crates/palmscript/examples/strategies/gate_spot.ps`
- `crates/palmscript/examples/strategies/gate_usdt_perps_backtest.ps`
- `crates/palmscript/examples/strategies/cross_exchange_bybit_gate_spread.ps`

## Pruebalo En El IDE Del Navegador

Abre [https://palmscript.dev/app/](https://palmscript.dev/app/), pega el
ejemplo en el editor y ejecutalo sobre el historial disponible de BTCUSDT en la
app.

## Que Debes Vigilar

- los scripts conscientes de fuentes deben usar series de mercado calificadas
  por fuente
- `use bb 1h` es obligatorio antes de `bb.1h.close`
- el script sigue teniendo un unico `interval` base global
- el runtime resuelve cada feed requerido `(source, interval)` antes de la
  ejecucion
- Bybit espera simbolos nativos del venue como `BTCUSDT`
- Gate espera simbolos nativos del venue como `BTC_USDT`
- `run market`, `run backtest`, `run walk-forward`, `run walk-forward-sweep` y
  `run optimize` resuelven las mismas declaraciones de fuente respaldadas por
  exchanges

Referencia:

- [Intervalos y Fuentes](../../reference/intervals-and-sources.md)
