# Momentum, Volume, and Volatility Indicators

This page defines PalmScript's executable momentum, oscillator, volume, and volatility indicators.

## `rsi(series, length)`

Rules:

- it requires exactly two arguments
- the first argument must be `series<float>`
- the second argument must be a positive integer literal
- the result type is `series<float>`
- the series returns `na` until enough history exists to seed the indicator state

## `roc(series[, length=10])`, `mom(series[, length=10])`, `rocp(series[, length=10])`, `rocr(series[, length=10])`, and `rocr100(series[, length=10])`

Rules:

- the first argument must be `series<float>`
- the optional `length` must be a positive integer literal
- omitted `length` uses the TA-Lib default of `10`
- `roc` evaluates as `((series - series[length]) / series[length]) * 100`
- `mom` evaluates as `series - series[length]`
- `rocp` evaluates as `(series - series[length]) / series[length]`
- `rocr` evaluates as `series / series[length]`
- `rocr100` evaluates as `(series / series[length]) * 100`
- if the current or referenced sample is `na`, the result is `na`
- if `series[length]` is `0`, `roc`, `rocp`, `rocr`, and `rocr100` return `na`

## `cmo(series[, length=14])`

Rules:

- the first argument must be `series<float>`
- omitted `length` uses the TA-Lib default of `14`
- if provided, `length` must be an integer literal greater than or equal to `2`
- `cmo` uses TA-Lib's Wilder-style smoothed gain and loss state
- the result type is `series<float>`
- if the smoothed gain and loss sum to `0`, `cmo` returns `0`

## `cci(high, low, close[, length=14])`

Rules:

- the first three arguments must be `series<float>`
- omitted `length` uses the TA-Lib default of `14`
- if provided, `length` must be an integer literal greater than or equal to `2`
- `cci` uses the trailing typical-price average and mean deviation over the requested window
- if the current typical-price delta or mean deviation is `0`, `cci` returns `0`
- the result type is `series<float>`

## `aroon(high, low[, length=14])` and `aroonosc(high, low[, length=14])`

Rules:

- the first two arguments must be `series<float>`
- omitted `length` uses the TA-Lib default of `14`
- if provided, `length` must be an integer literal greater than or equal to `2`
- `aroon` uses a trailing `length + 1` high/low window to match TA-Lib lookback
- `aroon` returns a 2-tuple `(aroon_down, aroon_up)` in TA-Lib output order
- `aroonosc` returns `aroon_up - aroon_down`
- tuple-valued outputs must be destructured before further use

## `willr(high, low, close[, length=14])`

Rules:

- the first three arguments must be `series<float>`
- omitted `length` uses the TA-Lib default of `14`
- if provided, `length` must be an integer literal greater than or equal to `2`
- `willr` uses the trailing highest high and lowest low over the requested window
- the result type is `series<float>`
- if the trailing high-low range is `0`, `willr` returns `0`

## `obv(series, volume)`

Rules:

- both arguments must be `series<float>`
- the first output sample seeds from the current `volume`
- later samples add or subtract the current `volume` based on whether `series` rose or fell from the prior bar
- if the current price or volume sample is `na`, the result is `na`
- the result type is `series<float>`

## `trange(high, low, close)`

Rules:

- all arguments must be `series<float>`
- the first output sample is `na`
- later samples use TA-Lib true range semantics based on current `high`, current `low`, and prior `close`
- if any required sample is `na`, the result is `na`
- the result type is `series<float>`
