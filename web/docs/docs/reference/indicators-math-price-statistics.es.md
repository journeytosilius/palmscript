# Indicadores De Matematicas, Precio Y Estadistica

Esta pagina define las transformaciones matematicas ejecutables, las
transformaciones de precio y los indicadores orientados a estadistica de
PalmScript.

## Transformaciones Matematicas TA-Lib

Estos builtins son actualmente ejecutables:

- `acos(real)`
- `asin(real)`
- `atan(real)`
- `ceil(real)`
- `cos(real)`
- `cosh(real)`
- `exp(real)`
- `floor(real)`
- `ln(real)`
- `log10(real)`
- `sin(real)`
- `sinh(real)`
- `sqrt(real)`
- `tan(real)`
- `tanh(real)`

Reglas:

- cada uno requiere exactamente un argumento numerico o `series<float>`
- si la entrada es una serie, el tipo de resultado es `series<float>`
- si la entrada es escalar, el tipo de resultado es `float`
- si la entrada es `na`, el resultado es `na`

## Operaciones Aritmeticas TA-Lib Y Transformaciones De Precio

Estos builtins son actualmente ejecutables:

- `add(a, b)`
- `div(a, b)`
- `mult(a, b)`
- `sub(a, b)`
- `avgprice(open, high, low, close)`
- `bop(open, high, low, close)`
- `medprice(high, low)`
- `typprice(high, low, close)`
- `wclprice(high, low, close)`

Reglas:

- todos los argumentos deben ser numericos, `series<float>` o `na`
- si algun argumento es una serie, el tipo de resultado es `series<float>`
- en caso contrario el tipo de resultado es `float`
- si cualquier entrada requerida es `na`, el resultado es `na`

Regla adicional de OHLC:

- `bop` devuelve `(close - open) / (high - low)` y devuelve `0` cuando
  `high - low <= 0`

## `max(series[, length=30])`, `min(series[, length=30])` y `sum(series[, length=30])`

Reglas:

- el primer argumento debe ser `series<float>`
- la ventana opcional usa `30` por defecto
- si se provee, la ventana debe ser un literal entero mayor o igual a `2`
- la ventana incluye la muestra actual
- si no existe suficiente historial, el resultado es `na`
- si cualquier muestra de la ventana requerida es `na`, el resultado es `na`
- el tipo de resultado es `series<float>`

## `avgdev(series[, length=14])`

Reglas:

- el primer argumento debe ser `series<float>`
- la opcion `length` usa `14` por defecto
- si se provee, `length` debe ser un literal entero mayor o igual a `2`
- el tipo de resultado es `series<float>`
- si no existe suficiente historial, la muestra actual es `na`
- si la ventana requerida contiene `na`, la muestra actual es `na`

## `maxindex(series[, length=30])` y `minindex(series[, length=30])`

Reglas:

- el primer argumento debe ser `series<float>`
- la opcion `length` usa `30` por defecto
- si se provee, `length` debe ser un literal entero mayor o igual a `2`
- `maxindex` y `minindex` devuelven `series<float>` que contienen el indice
  absoluto de barra como `f64`
- si no existe suficiente historial, la muestra actual es `na`
- si la ventana requerida contiene `na`, la muestra actual es `na`

## `minmax(series[, length=30])` y `minmaxindex(series[, length=30])`

Reglas:

- el primer argumento debe ser `series<float>`
- la opcion `length` usa `30` por defecto
- si se provee, `length` debe ser un literal entero mayor o igual a `2`
- `minmax` devuelve una tupla de 2 valores `(min_value, max_value)` en el orden
  de salida de TA-Lib
- `minmaxindex` devuelve una tupla de 2 valores `(min_index, max_index)` en el
  orden de salida de TA-Lib
- las salidas tuple deben destructurarse antes de cualquier otro uso
- si no existe suficiente historial, la muestra actual es `na`
- si la ventana requerida contiene `na`, la muestra actual es `na`

## `stddev(series[, length=5[, deviations=1.0]])` y `var(series[, length=5[, deviations=1.0]])`

Reglas:

- el primer argumento debe ser `series<float>`
- la opcion `length` usa `5` por defecto
- si se provee, `length` debe ser un literal entero
- `stddev` requiere `length >= 2`
- `var` permite `length >= 1`
- `deviations` usa `1.0` por defecto
- `stddev` multiplica la raiz cuadrada de la varianza movil por `deviations`
- `var` ignora el argumento `deviations` para coincidir con TA-Lib
- el tipo de resultado es `series<float>`
- si no existe suficiente historial, la muestra actual es `na`
- si la ventana requerida contiene `na`, la muestra actual es `na`

## `beta(series0, series1[, length=5])` y `correl(series0, series1[, length=30])`

Reglas:

- ambas entradas deben ser `series<float>`
- `beta` usa `length=5` por defecto
- `correl` usa `length=30` por defecto
- si se provee, `length` debe ser un literal entero que satisfaga el minimo
  TA-Lib del builtin correspondiente
- `beta` sigue la formulacion beta basada en ratio de retornos de TA-Lib, por
  lo que produce salida por primera vez despues de `length + 1` muestras fuente
- `correl` devuelve la correlacion de Pearson de las series de entrada
  emparejadas en crudo
- el tipo de resultado es `series<float>`
- si no existe suficiente historial, la muestra actual es `na`
- si la ventana requerida contiene `na`, la muestra actual es `na`
