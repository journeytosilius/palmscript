# Builtins

This page defines the builtin functions and predefined market names implemented by PalmScript.

## Builtin Function Set

PalmScript currently provides these callable builtins:

- `sma(series, length)`
- `ema(series, length)`
- `rsi(series, length)`
- `plot(value)`

PalmScript also reserves these predefined market names:

- `open`
- `high`
- `low`
- `close`
- `volume`
- `time`

The predefined market names are identifiers, not callable functions. `close()` is rejected.

## Common Builtin Rules

All builtins are deterministic and side-effect free, except that `plot` writes to the runtime output stream.

Indicator builtins:

- must not perform I/O
- must not depend on wall-clock time
- must follow the update clock of their input series
- must return `series<float>`

## `sma(series, length)`

`sma` computes a simple moving average over a series.

Rules:

- it requires exactly two arguments
- the first argument must be `series<float>`
- the second argument must be a positive integer literal
- the result type is `series<float>`
- if insufficient history exists, the current sample is `na`
- if the required window contains `na`, the current sample is `na`

## `ema(series, length)`

`ema` computes an exponential moving average over a series.

Rules:

- it requires exactly two arguments
- the first argument must be `series<float>`
- the second argument must be a positive integer literal
- the result type is `series<float>`
- the series returns `na` until the seed window is available

## `rsi(series, length)`

`rsi` computes the relative strength index over a series.

Rules:

- it requires exactly two arguments
- the first argument must be `series<float>`
- the second argument must be a positive integer literal
- the result type is `series<float>`
- the series returns `na` until the implementation has enough history to seed the indicator state

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

Indicator builtins follow the update clock of their input series.

Examples:

- `ema(close, 20)` advances on the base clock
- `sma(1w.close, 5)` advances on the weekly clock
- `rsi(hl.1h.close, 14)` advances on source `hl`'s one-hour clock
