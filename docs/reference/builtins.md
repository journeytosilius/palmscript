# Builtins

This page defines the builtin functions and predefined market names implemented by PalmScript.

## Builtin Function Set

PalmScript currently provides these callable builtins:

- `sma(series, length)`
- `ema(series, length)`
- `rsi(series, length)`
- `plot(value)`
- `above(a, b)`
- `below(a, b)`
- `between(x, low, high)`
- `outside(x, low, high)`
- `cross(a, b)`
- `crossover(a, b)`
- `crossunder(a, b)`
- `change(series, length)`
- `roc(series, length)`
- `highest(series, length)`
- `lowest(series, length)`
- `rising(series, length)`
- `falling(series, length)`
- `barssince(condition)`
- `valuewhen(condition, source, occurrence)`

PalmScript also reserves these predefined market names:

- `open`
- `high`
- `low`
- `close`
- `volume`
- `time`

The predefined market names are identifiers, not callable functions. `close()` is rejected.

## Common Builtin Rules

Rules:

- all builtins are deterministic
- builtins must not perform I/O, access time, or access the network
- `plot` writes to the output stream; all other builtins are pure
- helper builtins propagate `na` unless a more specific rule below overrides that behavior
- helper builtins follow the update clocks implied by their series arguments

## Indicators

### `sma(series, length)`

Rules:

- it requires exactly two arguments
- the first argument must be `series<float>`
- the second argument must be a positive integer literal
- the result type is `series<float>`
- if insufficient history exists, the current sample is `na`
- if the required window contains `na`, the current sample is `na`

### `ema(series, length)`

Rules:

- it requires exactly two arguments
- the first argument must be `series<float>`
- the second argument must be a positive integer literal
- the result type is `series<float>`
- the series returns `na` until the seed window is available

### `rsi(series, length)`

Rules:

- it requires exactly two arguments
- the first argument must be `series<float>`
- the second argument must be a positive integer literal
- the result type is `series<float>`
- the series returns `na` until enough history exists to seed the indicator state

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

### `roc(series, length)`

Rules:

- it requires exactly two arguments
- the first argument must be `series<float>`
- the second argument must be a positive integer literal
- it evaluates as `((series - series[length]) / series[length]) * 100`
- if the current or referenced sample is `na`, the result is `na`
- if `series[length]` is `0`, the result is `na`
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

- `ema(close, 20)` advances on the base clock
- `highest(1w.close, 5)` advances on the weekly clock
- `crossover(hl.close, bn.close)` advances when either referenced source series advances
- `barssince(close > close[1])` advances on the clock of that condition series
- `valuewhen(trigger_series, hl.1h.close, 0)` advances on the clock of `trigger_series`
