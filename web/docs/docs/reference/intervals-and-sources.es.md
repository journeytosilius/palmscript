# Intervalos y Fuentes

Esta pagina define las reglas normativas de intervalos y fuentes en PalmScript.

## Intervalos Soportados

PalmScript acepta los literales de intervalo listados en la
[Tabla De Intervalos](intervals.md). Los intervalos distinguen mayusculas y
minusculas.

## Intervalo Base

Todo script declara exactamente un intervalo base:

```palmscript
interval 1m
```

El intervalo base define el reloj de ejecucion.

## Fuentes Con Nombre

Los scripts ejecutables declaran una o mas fuentes con nombre respaldadas por
exchanges:

```palmscript
interval 1m
source hl = hyperliquid.perps("BTC")
source bn = binance.spot("BTCUSDT")
use hl 1h

plot(bn.close - hl.1h.close)
```

Reglas:

- al menos una declaracion `source` es obligatoria
- las series de mercado deben estar calificadas por fuente
- cada fuente declarada aporta un feed base en el intervalo base del script
- `use <alias> <interval>` declara un intervalo adicional para esa fuente
- `<alias>.<field>` se refiere a esa fuente en el intervalo base
- `<alias>.<interval>.<field>` se refiere a esa fuente en el intervalo
  nombrado
- las referencias a intervalos inferiores al base se rechazan

## Templates De Fuente Soportados

PalmScript soporta actualmente estos templates de primera clase:

- `binance.spot("<symbol>")`
- `binance.usdm("<symbol>")`
- `hyperliquid.spot("<symbol>")`
- `hyperliquid.perps("<symbol>")`

El soporte de intervalos depende del template:

- `binance.spot` acepta todos los intervalos PalmScript soportados
- `binance.usdm` acepta todos los intervalos PalmScript soportados
- `hyperliquid.spot` rechaza `1s` y `6h`
- `hyperliquid.perps` rechaza `1s` y `6h`

Las restricciones operativas de carga tambien dependen del template:

- la API REST de Hyperliquid solo expone las `5000` velas mas recientes por
  feed
- el modo mercado rechaza cualquier solicitud de feed de Hyperliquid que exceda
  esa ventana de retencion
- los feeds de Binance se paginan internamente y no tienen el mismo limite de
  retencion para toda la ventana

## Conjunto De Campos De Fuente

Todos los templates de fuente se normalizan en los mismos campos canonicos de
mercado:

- `time`
- `open`
- `high`
- `low`
- `close`
- `volume`

Reglas:

- `time` es la hora de apertura de la vela en Unix milliseconds UTC
- los campos de precio y volumen son numericos
- los campos extra especificos de cada venue no se exponen en el lenguaje

## Intervalos Iguales, Superiores E Inferiores

PalmScript distingue tres casos para un intervalo referenciado respecto al
intervalo base:

- intervalo igual: valido
- intervalo superior: valido si se declara con `use <alias> <interval>`
- intervalo inferior: rechazado

## Semantica De Runtime

En modo mercado:

- PalmScript obtiene directamente desde las venues los feeds requeridos
  `(source, interval)`
- la linea temporal base de ejecucion es la union de los tiempos de apertura de
  barras del intervalo base para todas las fuentes declaradas
- si una fuente no tiene barra base en un paso de la linea temporal, esa fuente
  aporta `na` en ese paso
- los intervalos lentos de una fuente retienen su ultimo valor completamente
  cerrado hasta su siguiente frontera de cierre

## Garantia Sin Lookahead

PalmScript no debe exponer una vela de intervalo superior antes de que esa vela
haya cerrado por completo.

Esto aplica a intervalos calificados por fuente como `hl.1h.close`.

## Reglas De Alineacion De Runtime

Los feeds preparados deben alinearse con sus intervalos declarados.

El runtime rechaza feeds que esten:

- desalineados con la frontera del intervalo
- desordenados
- duplicados en un mismo tiempo de apertura de intervalo
