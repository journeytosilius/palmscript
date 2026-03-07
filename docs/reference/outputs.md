# Outputs

This page defines the user-visible output forms in PalmScript.

## Output Forms

PalmScript exposes three output-producing constructs:

- `plot(value)`
- `export name = expr`
- `trigger name = expr`

`plot` is a builtin call. `export` and `trigger` are declarations.

## `plot`

`plot` emits a plot point for the current step.

Rules:

- the argument must be numeric, `series<float>`, or `na`
- the current step contributes one plot point per executed `plot` call
- `plot` does not create a reusable language binding
- `plot` is not allowed inside user-defined function bodies

## `export`

`export` publishes a named output series:

```palmscript
export trend = ema(spot.close, 20) > ema(spot.close, 50)
```

Rules:

- it is top-level only
- the name must be unique within the current scope
- the expression may evaluate to numeric, bool, series numeric, series bool, or `na`
- `void` is rejected

Type normalization:

- numeric, series numeric, and `na` exports become `series<float>`
- bool and series bool exports become `series<bool>`

## `trigger`

`trigger` publishes a named boolean output series:

```palmscript
trigger long_entry = spot.close > spot.high[1]
```

Rules:

- it is top-level only
- the expression must evaluate to `bool`, `series<bool>`, or `na`
- the output type is always `series<bool>`

Runtime event rule:

- a trigger event is emitted for a step only when the current trigger sample is `true`
- `false` and `na` do not emit trigger events

## Backtester Trigger Contract

The PalmScript language does not attach trading semantics directly to `trigger`.

The library backtester interprets trigger names externally. Its default signal
contract is:

- `trigger long_entry = ...`
- `trigger long_exit = ...`
- `trigger short_entry = ...`
- `trigger short_exit = ...`

Rules for that contract:

- trigger names are matched by the backtester, not by the compiler or VM
- scripts may still declare additional triggers for alerts or other consumers
- if no configured trigger names match the compiled trigger outputs, the
  backtester fails validation
- trigger events are observed on the runtime step clock, then scheduled onto the
  next eligible execution-source bar by the backtester

See [Backtesting](../tooling/backtesting.md) for the Rust API and execution
model.

## Runtime Output Collections

Over a full run, the runtime accumulates:

- `plots`
- `exports`
- `triggers`
- `trigger_events`
- `alerts`

`alerts` currently exist in the runtime output structures but are not produced by a first-class PalmScript language construct.

## Output Time And Bar Index

Each output sample is tagged with:

- the current `bar_index`
- the current step `time`

In source-aware runs, the step time is the open time of the current base-clock step.
