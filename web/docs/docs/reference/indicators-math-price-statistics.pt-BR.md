# Math, Price, And Statistics Indicators

Esta pagina define as transformacoes matematicas executaveis, transformacoes de
preco e indicadores orientados a estatistica do PalmScript.

## Transformacoes Matematicas TA-Lib

Estes builtins sao atualmente executaveis:

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

Regras:

- cada um exige exatamente um argumento numerico ou `series<float>`
- se a entrada for uma serie, o tipo de resultado e `series<float>`
- se a entrada for escalar, o tipo de resultado e `float`
- se a entrada for `na`, o resultado e `na`

## Transformacoes Aritmeticas E De Preco TA-Lib

Estes builtins sao atualmente executaveis:

- `add(a, b)`
- `div(a, b)`
- `mult(a, b)`
- `sub(a, b)`
- `avgprice(open, high, low, close)`
- `bop(open, high, low, close)`
- `medprice(high, low)`
- `typprice(high, low, close)`
- `wclprice(high, low, close)`

Regras:

- todos os argumentos devem ser numericos, `series<float>` ou `na`
- se qualquer argumento for uma serie, o tipo de resultado e `series<float>`
- caso contrario, o tipo de resultado e `float`
- se qualquer entrada necessaria for `na`, o resultado e `na`

Regra OHLC adicional:

- `bop` retorna `(close - open) / (high - low)` e retorna `0` quando
  `high - low <= 0`

## `max(series[, length=30])`, `min(series[, length=30])` e `sum(series[, length=30])`

Regras:

- o primeiro argumento deve ser `series<float>`
- a janela trailing opcional usa `30` por padrao
- se fornecida, a janela deve ser um literal inteiro maior ou igual a `2`
- a janela inclui a amostra atual
- se nao houver historico suficiente, o resultado e `na`
- se qualquer amostra necessaria na janela for `na`, o resultado e `na`
- o tipo de resultado e `series<float>`

## `avgdev(series[, length=14])`

Regras:

- o primeiro argumento deve ser `series<float>`
- `length` opcional usa `14` por padrao
- se fornecido, `length` deve ser um literal inteiro maior ou igual a `2`
- o tipo de resultado e `series<float>`
- se nao houver historico suficiente, a amostra atual e `na`
- se a janela necessaria contiver `na`, a amostra atual e `na`

## `maxindex(series[, length=30])` e `minindex(series[, length=30])`

Regras:

- o primeiro argumento deve ser `series<float>`
- `length` opcional usa `30` por padrao
- se fornecido, `length` deve ser um literal inteiro maior ou igual a `2`
- `maxindex` e `minindex` retornam `series<float>` contendo o indice absoluto
  da barra como `f64`
- se nao houver historico suficiente, a amostra atual e `na`
- se a janela necessaria contiver `na`, a amostra atual e `na`

## `minmax(series[, length=30])` e `minmaxindex(series[, length=30])`

Regras:

- o primeiro argumento deve ser `series<float>`
- `length` opcional usa `30` por padrao
- se fornecido, `length` deve ser um literal inteiro maior ou igual a `2`
- `minmax` retorna uma tupla `(min_value, max_value)` na ordem de saida do
  TA-Lib
- `minmaxindex` retorna uma tupla `(min_index, max_index)` na ordem de saida do
  TA-Lib
- saidas tuple-valued devem ser desestruturadas antes de qualquer outro uso
- se nao houver historico suficiente, a amostra atual e `na`
- se a janela necessaria contiver `na`, a amostra atual e `na`

## `stddev(series[, length=5[, deviations=1.0]])` e `var(series[, length=5[, deviations=1.0]])`

Regras:

- o primeiro argumento deve ser `series<float>`
- `length` opcional usa `5` por padrao
- se fornecido, `length` deve ser um literal inteiro
- `stddev` exige `length >= 2`
- `var` permite `length >= 1`
- `deviations` usa `1.0` por padrao
- `stddev` multiplica a raiz quadrada da variancia movel por `deviations`
- `var` ignora o argumento `deviations` para corresponder ao TA-Lib
- o tipo de resultado e `series<float>`
- se nao houver historico suficiente, a amostra atual e `na`
- se a janela necessaria contiver `na`, a amostra atual e `na`

## `beta(series0, series1[, length=5])` e `correl(series0, series1[, length=30])`

Regras:

- ambas as entradas devem ser `series<float>`
- `beta` usa `length=5` por padrao
- `correl` usa `length=30` por padrao
- se fornecido, `length` deve ser um literal inteiro que satisfaca o minimo do
  TA-Lib para aquele builtin
- `beta` segue a formulacao baseada em retornos do TA-Lib, entao sua primeira
  saida so aparece depois de `length + 1` amostras de source
- `correl` retorna a correlacao de Pearson das series brutas pareadas
- o tipo de resultado e `series<float>`
- se nao houver historico suficiente, a amostra atual e `na`
- se a janela necessaria contiver `na`, a amostra atual e `na`
