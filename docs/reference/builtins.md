# Builtins

This page defines PalmScript's shared builtin rules and the non-indicator builtin helpers.

Indicator-specific contracts live in the dedicated [Indicators](indicators.md) section.

## Executable Builtins vs Reserved Names

PalmScript exposes three related surfaces:

- executable builtin helpers and outputs documented on this page
- executable indicators documented in the [Indicators](indicators.md) section
- a broader reserved TA-Lib catalog described in [TA-Lib Surface](ta-lib.md)

Not every reserved TA-Lib name is executable today. Reserved-but-not-yet-executable names produce deterministic compile diagnostics instead of being treated as unknown identifiers.

## Builtin Categories

PalmScript currently exposes these builtin categories:

- indicators: [Trend and Overlap](indicators-trend-and-overlap.md), [Momentum, Volume, and Volatility](indicators-momentum-volume-volatility.md), and [Math, Price, and Statistics](indicators-math-price-statistics.md)
- relational helpers: `above`, `below`, `between`, `outside`
- crossing helpers: `cross`, `crossover`, `crossunder`
- series and window helpers: `change`, `highest`, `lowest`, `rising`, `falling`
- event-memory helpers: `barssince`, `valuewhen`
- outputs: `plot`

Market fields are selected through source-qualified series such as `spot.open`, `spot.close`, or `hl.1h.volume`. Only identifiers are callable, so `spot.close()` is rejected.

## Tuple-Valued Builtins

The current executable tuple-valued builtins are:

- `macd(series, fast_length, slow_length, signal_length)` documented in [Trend and Overlap](indicators-trend-and-overlap.md)
- `minmax(series[, length=30])` documented in [Math, Price, and Statistics](indicators-math-price-statistics.md)
- `minmaxindex(series[, length=30])` documented in [Math, Price, and Statistics](indicators-math-price-statistics.md)
- `aroon(high, low[, length=14])` documented in [Momentum, Volume, and Volatility](indicators-momentum-volume-volatility.md)

All tuple-valued builtin results must be destructured immediately with `let (...) = ...` before further use.

## Common Builtin Rules

Rules:

- all builtins are deterministic
- builtins must not perform I/O, access time, or access the network
- `plot` writes to the output stream; all other builtins are pure
- builtin helpers and indicators propagate `na` unless a more specific rule overrides that behavior
- builtin results follow the update clocks implied by their series arguments

## Relational Helpers

### `above(a, b)` and `below(a, b)`

Rules:

- both arguments must be numeric, `series<float>`, or `na`
- `above(a, b)` evaluates as `a > b`
- `below(a, b)` evaluates as `a < b`
- if any required input is `na`, the result is `na`
- if either input is a series, the result type is `series<bool>`
- otherwise the result type is `bool`

### `between(x, low, high)` and `outside(x, low, high)`

Rules:

- all arguments must be numeric, `series<float>`, or `na`
- `between(x, low, high)` evaluates as `low < x and x < high`
- `outside(x, low, high)` evaluates as `x < low or x > high`
- if any required input is `na`, the result is `na`
- if any argument is a series, the result type is `series<bool>`
- otherwise the result type is `bool`

## Crossing Helpers

### `crossover(a, b)`

Rules:

- both arguments must be numeric, `series<float>`, or `na`
- at least one argument must be `series<float>`
- scalar arguments are treated as thresholds, so their prior sample is their current value
- it evaluates as current `a > b` and prior `a[1] <= b[1]`
- if any required current or prior sample is `na`, the result is `na`
- the result type is `series<bool>`

### `crossunder(a, b)`

Rules:

- both arguments must be numeric, `series<float>`, or `na`
- at least one argument must be `series<float>`
- scalar arguments are treated as thresholds, so their prior sample is their current value
- it evaluates as current `a < b` and prior `a[1] >= b[1]`
- if any required current or prior sample is `na`, the result is `na`
- the result type is `series<bool>`

### `cross(a, b)`

Rules:

- both arguments follow the same contract as `crossover` and `crossunder`
- it evaluates as `crossover(a, b) or crossunder(a, b)`
- if any required current or prior sample is `na`, the result is `na`
- the result type is `series<bool>`

## Series and Window Helpers

### `change(series, length)`

Rules:

- it requires exactly two arguments
- the first argument must be `series<float>`
- the second argument must be a positive integer literal
- it evaluates as `series - series[length]`
- if the current or referenced sample is `na`, the result is `na`
- the result type is `series<float>`

### `highest(series, length)` and `lowest(series, length)`

Rules:

- the first argument must be `series<float>`
- the second argument must be a positive integer literal
- the window includes the current sample
- if insufficient history exists, the result is `na`
- if any sample in the required window is `na`, the result is `na`
- the result type is `series<float>`

### `rising(series, length)` and `falling(series, length)`

Rules:

- the first argument must be `series<float>`
- the second argument must be a positive integer literal
- `rising(series, length)` means the current sample is strictly greater than every prior sample in the trailing `length` bars
- `falling(series, length)` means the current sample is strictly less than every prior sample in the trailing `length` bars
- if insufficient history exists, the result is `na`
- if any required sample is `na`, the result is `na`
- the result type is `series<bool>`

## Event Memory Helpers

### `barssince(condition)`

Rules:

- it requires exactly one argument
- the argument must be `series<bool>`
- it returns `0` on bars where the current condition sample is `true`
- it increments on each update of the condition's own clock after the last true event
- it returns `na` until the first true event
- if the current condition sample is `na`, the current output is `na`
- the result type is `series<float>`

### `valuewhen(condition, source, occurrence)`

Rules:

- it requires exactly three arguments
- the first argument must be `series<bool>`
- the second argument must be `series<float>` or `series<bool>`
- the third argument must be a non-negative integer literal
- occurrence `0` means the most recent true event
- the result type matches the second argument type
- it returns `na` until enough matching true events exist
- if the current condition sample is `na`, the current output is `na`
- when the current condition sample is `true`, the current `source` sample is captured for future occurrences

## `plot(value)`

`plot` emits a plot point for the current step.

Rules:

- it requires exactly one argument
- the argument must be numeric, `series<float>`, or `na`
- the expression result type is `void`
- `plot` must not be called inside a user-defined function body

At runtime:

- numeric values are recorded as plot points
- `na` records a plot point with no numeric value

## Update Clocks

Builtin results follow the update clocks of their inputs.

Examples:

- `ema(spot.close, 20)` advances on the base clock
- `highest(spot.1w.close, 5)` advances on the weekly clock
- `crossover(hl.close, bn.close)` advances when either referenced source series advances
- `barssince(spot.close > spot.close[1])` advances on the clock of that condition series
- `valuewhen(trigger_series, hl.1h.close, 0)` advances on the clock of `trigger_series`
