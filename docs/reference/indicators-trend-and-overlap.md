# Trend and Overlap Indicators

This page defines PalmScript's executable trend and overlap indicators.

## `sma(series, length)`

Rules:

- it requires exactly two arguments
- the first argument must be `series<float>`
- the second argument must be a positive integer literal
- the result type is `series<float>`
- if insufficient history exists, the current sample is `na`
- if the required window contains `na`, the current sample is `na`

## `ema(series, length)`

Rules:

- it requires exactly two arguments
- the first argument must be `series<float>`
- the second argument must be a positive integer literal
- the result type is `series<float>`
- the series returns `na` until the seed window is available

## `ma(series, length, ma_type)`

Rules:

- it requires exactly three arguments
- the first argument must be `series<float>`
- the second argument must be a positive integer literal
- the third argument must be a typed `ma_type.<variant>` value
- the result type is `series<float>`
- `ma_type.sma`, `ma_type.ema`, and `ma_type.wma` are currently implemented

## `apo(series[, fast_length=12[, slow_length=26[, ma_type=ma_type.sma]]])` and `ppo(series[, fast_length=12[, slow_length=26[, ma_type=ma_type.sma]]])`

Rules:

- the first argument must be `series<float>`
- `fast_length` and `slow_length` default to `12` and `26`
- if provided, `fast_length` and `slow_length` must be integer literals greater than or equal to `2`
- if provided, the fourth argument must be a typed `ma_type.<variant>` value
- omitted `ma_type` defaults to `ma_type.sma`
- `apo` returns `fast_ma - slow_ma`
- `ppo` returns `((fast_ma - slow_ma) / slow_ma) * 100`
- if the slow moving average is `0`, `ppo` returns `0`
- `ma_type.sma`, `ma_type.ema`, and `ma_type.wma` are currently implemented
- the result type is `series<float>`

## `macd(series, fast_length, slow_length, signal_length)`

Rules:

- it requires exactly four arguments
- the first argument must be `series<float>`
- the remaining arguments must be positive integer literals
- the result type is a 3-tuple of series values in TA-Lib order: `(macd_line, signal, histogram)`
- the result must be destructured before it can be used in `plot`, `export`, conditions, or further expressions

## `wma(series[, length=30])`

Rules:

- the first argument must be `series<float>`
- the optional `length` defaults to `30`
- if provided, `length` must be an integer literal greater than or equal to `2`
- the result type is `series<float>`
- if insufficient history exists, the current sample is `na`
- if the required window contains `na`, the current sample is `na`

## `midpoint(series[, length=14])` and `midprice(high, low[, length=14])`

Rules:

- `midpoint` requires `series<float>` as the first argument
- `midprice` requires `series<float>` for both `high` and `low`
- the optional trailing window defaults to `14`
- if provided, the window must be an integer literal greater than or equal to `2`
- the window includes the current sample
- if insufficient history exists, the result is `na`
- if any required sample in the window is `na`, the result is `na`
- the result type is `series<float>`

## `linearreg(series[, length=14])`, `linearreg_angle(series[, length=14])`, `linearreg_intercept(series[, length=14])`, `linearreg_slope(series[, length=14])`, and `tsf(series[, length=14])`

Rules:

- the first argument must be `series<float>`
- the optional `length` defaults to `14`
- if provided, `length` must be an integer literal greater than or equal to `2`
- if insufficient history exists, the current sample is `na`
- if the required window contains `na`, the current sample is `na`
- `linearreg` returns the fitted value at the current bar
- `linearreg_angle` returns the fitted slope angle
- `linearreg_intercept` returns the fitted intercept
- `linearreg_slope` returns the fitted slope
- `tsf` returns the one-step-ahead forecast
- the result type is `series<float>`
