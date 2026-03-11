# Tipos y Valores

PalmScript opera sobre numeros escalares, booleanos escalares, literales enum
tipados, series de esos valores, `na` y `void`.

## Tipos Concretos

La implementacion distingue estos tipos concretos:

- `float`
- `bool`
- `ma_type`
- `tif`
- `trigger_ref`
- `position_side`
- `exit_kind`
- `series<float>`
- `series<bool>`
- `void`

`void` es el tipo de resultado de expresiones como `plot(...)` que no producen
un valor reutilizable.

## Valores Primitivos

Los valores de PalmScript tienen las siguientes formas en runtime:

- los valores numericos son `f64`
- los valores booleanos son `true` o `false`
- los valores `ma_type.<variant>` son literales enum tipados
- los valores `tif.<variant>` son literales enum tipados
- los valores `trigger_ref.<variant>` son literales enum tipados
- los valores `position_side.<variant>` son literales enum tipados
- los valores `exit_kind.<variant>` son literales enum tipados
- `na` es el sentinel de valor faltante
- `void` no es un literal escribible por el usuario

Superficie actual de enums tipados:

- `ma_type.sma`
- `ma_type.ema`
- `ma_type.wma`
- `ma_type.dema`
- `ma_type.tema`
- `ma_type.trima`
- `ma_type.kama`
- `ma_type.mama`
- `ma_type.t3`
- `tif.gtc`
- `tif.ioc`
- `tif.fok`
- `tif.gtd`
- `trigger_ref.last`
- `trigger_ref.mark`
- `trigger_ref.index`
- `position_side.long`
- `position_side.short`
- `exit_kind.protect`
- `exit_kind.target`
- `exit_kind.signal`
- `exit_kind.reversal`

Todas las variantes actuales de `ma_type` son ejecutables a traves de los
builtins de medias moviles estilo TA-Lib; consulta [Superficie TA-Lib](ta-lib.md).
Los valores `tif`, `trigger_ref`, `position_side` y `exit_kind` existen hoy
para parametrizar declaraciones de orden de backtest y el estado de posicion y
salida impulsado por backtests.

## Tipos De Series

Los valores de series son secuencias indexadas en el tiempo.

Un tipo de serie:

- avanza sobre un reloj de actualizacion
- retiene historial acotado
- expone su muestra actual cuando se usa en expresiones
- puede producir `na` en una muestra dada

Los campos de mercado son valores de serie. Los indicadores, helpers de senales
y builtins de memoria de eventos tambien pueden devolver valores de serie.

Algunos builtins tambien pueden devolver tuplas de tamano fijo de valores de
serie. En la implementacion actual, los resultados tuple solo se soportan como
resultados inmediatos de builtins y deben destructurarse con `let (...) = ...`.

Ejemplo:

```palm
let (line, signal, hist) = macd(spot.close, 12, 26, 9)
plot(hist)
```

Limites actuales del soporte de tuplas:

- los valores tuple solo se producen por builtins especificos
- los valores tuple no pueden almacenarse como valores reutilizables ordinarios
- las expresiones tuple no pueden pasarse directamente a `plot`, `export`,
  `trigger`, condiciones o expresiones posteriores
- la destructuracion de tuplas es la unica forma soportada de consumir un
  resultado tuple

## `na`

`na` forma parte de la semantica normal del lenguaje. No es una excepcion de
runtime.

`na` puede surgir por:

- historial insuficiente para indexacion
- warm-up de indicadores
- datos faltantes en un paso del reloj base consciente de fuentes
- aritmetica o comparaciones donde un operando ya es `na`
- uso explicito del literal `na`

PalmScript tambien expone `na(value)` como predicado builtin distinto del
literal `na` desnudo:

- `na` por si solo es el literal de valor faltante
- `na(expr)` devuelve `bool` o `series<bool>` segun el argumento
- `nz(value[, fallback])` y `coalesce(value, fallback)` son los helpers
  principales para tratar nulos

## Combinacion De Series Y Escalares

PalmScript permite mezclar escalares y series en expresiones cuando el operador
subyacente acepta esas categorias de operandos.

Reglas:

- si cualquiera de los operandos aceptados es `series<float>`, la aritmetica
  produce `series<float>`
- si cualquiera de los operandos aceptados es `series<bool>`, las operaciones
  logicas producen `series<bool>`
- si cualquiera de los operandos aceptados es `series<float>`, las comparaciones
  numericas producen `series<bool>`
- la igualdad sobre cualquier operando de serie produce `series<bool>`

Esto es lifting de valores, no materializacion implicita de una serie sin
limite. La evaluacion sigue los relojes de actualizacion descritos en
[Semantica De Evaluacion](evaluation-semantics.md).

## `na` En El Type Checking

`na` se acepta en cualquier lugar donde mas tarde pueda requerirse una
expresion numerica o booleana, sujeta a la construccion circundante.

Ejemplos:

- `plot(na)` es valido
- `export x = na` es valido
- `trigger t = na` es valido
- `if na { ... } else { ... }` es valido
- `ma(spot.close, 20, ma_type.ema)` es valido

## Logica Booleana

`and` y `or` usan la logica de tres valores de PalmScript.

No coercionan `na` a `false`. Su tabla de verdad en runtime se define en
[Semantica De Evaluacion](evaluation-semantics.md).

## Normalizacion De Salidas

Las declaraciones de salida normalizan sus tipos de valor de la siguiente
manera:

- `export` sobre numerico, serie numerica o `na` produce `series<float>`
- `export` sobre bool o serie bool produce `series<bool>`
- las salidas `trigger`, `entry` y `exit` siempre producen `series<bool>`

Consulta [Salidas](outputs.md) para el comportamiento exacto de las salidas.
