# Math, Price, and Statistics Indicators

This page defines PalmScript's executable math transforms, price transforms, and statistics-oriented indicators.

## TA-Lib Math Transforms

These builtins are currently executable:

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

Rules:

- each requires exactly one numeric or `series<float>` argument
- if the input is a series, the result type is `series<float>`
- if the input is scalar, the result type is `float`
- if the input is `na`, the result is `na`

## TA-Lib Arithmetic and Price Transforms

These builtins are currently executable:

- `add(a, b)`
- `div(a, b)`
- `mult(a, b)`
- `sub(a, b)`
- `avgprice(open, high, low, close)`
- `bop(open, high, low, close)`
- `medprice(high, low)`
- `typprice(high, low, close)`
- `wclprice(high, low, close)`

Rules:

- all arguments must be numeric, `series<float>`, or `na`
- if any argument is a series, the result type is `series<float>`
- otherwise the result type is `float`
- if any required input is `na`, the result is `na`

Additional OHLC rule:

- `bop` returns `(close - open) / (high - low)` and returns `0` when `high - low <= 0`

## `max(series[, length=30])`, `min(series[, length=30])`, and `sum(series[, length=30])`

Rules:

- the first argument must be `series<float>`
- the optional trailing window defaults to `30`
- if provided, the window must be an integer literal greater than or equal to `2`
- the window includes the current sample
- if insufficient history exists, the result is `na`
- if any sample in the required window is `na`, the result is `na`
- the result type is `series<float>`

## `avgdev(series[, length=14])`

Rules:

- the first argument must be `series<float>`
- the optional `length` defaults to `14`
- if provided, `length` must be an integer literal greater than or equal to `2`
- the result type is `series<float>`
- if insufficient history exists, the current sample is `na`
- if the required window contains `na`, the current sample is `na`

## `maxindex(series[, length=30])` and `minindex(series[, length=30])`

Rules:

- the first argument must be `series<float>`
- the optional `length` defaults to `30`
- if provided, `length` must be an integer literal greater than or equal to `2`
- `maxindex` and `minindex` return `series<float>` containing the absolute bar index as `f64`
- if insufficient history exists, the current sample is `na`
- if the required window contains `na`, the current sample is `na`

## `minmax(series[, length=30])` and `minmaxindex(series[, length=30])`

Rules:

- the first argument must be `series<float>`
- the optional `length` defaults to `30`
- if provided, `length` must be an integer literal greater than or equal to `2`
- `minmax` returns a 2-tuple `(min_value, max_value)` in TA-Lib output order
- `minmaxindex` returns a 2-tuple `(min_index, max_index)` in TA-Lib output order
- tuple-valued outputs must be destructured before further use
- if insufficient history exists, the current sample is `na`
- if the required window contains `na`, the current sample is `na`

## `stddev(series[, length=5[, deviations=1.0]])` and `var(series[, length=5[, deviations=1.0]])`

Rules:

- the first argument must be `series<float>`
- the optional `length` defaults to `5`
- if provided, `length` must be an integer literal
- `stddev` requires `length >= 2`
- `var` allows `length >= 1`
- `deviations` defaults to `1.0`
- `stddev` multiplies the square root of the rolling variance by `deviations`
- `var` ignores the `deviations` argument to match TA-Lib
- the result type is `series<float>`
- if insufficient history exists, the current sample is `na`
- if the required window contains `na`, the current sample is `na`

## `beta(series0, series1[, length=5])` and `correl(series0, series1[, length=30])`

Rules:

- both inputs must be `series<float>`
- `beta` defaults to `length=5`
- `correl` defaults to `length=30`
- if provided, `length` must be an integer literal that satisfies the TA-Lib minimum for that builtin
- `beta` follows TA-Lib's return-ratio formulation, so it first yields output after `length + 1` source samples
- `correl` returns the Pearson correlation of the paired raw input series
- the result type is `series<float>`
- if insufficient history exists, the current sample is `na`
- if the required window contains `na`, the current sample is `na`
