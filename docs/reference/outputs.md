# Outputs

This page defines the user-visible output forms in PalmScript.

## Output Forms

PalmScript exposes three output-producing constructs:

- `plot(value)`
- `export name = expr`
- `trigger name = expr`
- `entry long = expr`, `exit long = expr`, `entry short = expr`, `exit short = expr`
- `protect long = order_spec`, `protect short = order_spec`, `target long = order_spec`, `target short = order_spec`

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

## First-Class Strategy Signals

PalmScript exposes first-class strategy signal declarations for the built-in backtester:

```palmscript
entry long = spot.close > spot.high[1]
exit long = spot.close < ema(spot.close, 20)
entry short = spot.close < spot.low[1]
exit short = spot.close > ema(spot.close, 20)
```

Rules:

- the four declarations are top-level only
- each expression must evaluate to `bool`, `series<bool>`, or `na`
- they compile to trigger outputs with explicit signal-role metadata
- runtime event emission follows the same `true`/`false`/`na` rules as ordinary triggers

## Order Declarations

PalmScript also exposes top-level order declarations that parameterize how the built-in backtester executes a signal role:

```palmscript
entry long = spot.close > spot.high[1]
exit long = spot.close < ema(spot.close, 20)

order entry long = limit(spot.close[1], tif.gtc, false)
order exit long = stop_market(lowest(spot.low, 5)[1], trigger_ref.last)
```

Rules:

- order declarations are top-level only
- there may be at most one `order` declaration per signal role
- missing `order` declarations default to `market()`
- numeric order fields such as `price`, `trigger_price`, and `expire_time_ms` are evaluated by the runtime as hidden internal series
- `tif.<variant>` and `trigger_ref.<variant>` are typed enum literals checked at compile time
- venue-specific compatibility checks run when the backtest starts, based on the execution `source`

## Attached Exits

PalmScript also exposes first-class attached exits that keep the discretionary `exit` signal free:

```palmscript
entry long = spot.close > spot.high[1]
exit long = spot.close < ema(spot.close, 20)
protect long = stop_market(position.entry_price - 2 * atr(spot.high, spot.low, spot.close, 14), trigger_ref.last)
target long = take_profit_market(
    highest_since(position_event.long_entry_fill, spot.high) + 4,
    trigger_ref.last
)
```

Rules:

- attached exits are top-level only
- `protect` and `target` are optional per side
- they arm only after a matching entry fill exists
- they are reevaluated once per execution bar while that position remains open
- `protect` and `target` for one side are OCO: if one fills, the other is cancelled
- if both become fillable on the same execution bar, `protect` wins deterministically
- `position.*` is available only inside `protect` and `target` declarations
- `position_event.*` is a backtest-driven series namespace that exposes actual fill events such as `position_event.long_entry_fill`
- outside backtests, `position_event.*` is defined but evaluates to `false` on every step

## Legacy Trigger Compatibility

Legacy backtest scripts that use trigger names are still supported temporarily:

- `trigger long_entry = ...`
- `trigger long_exit = ...`
- `trigger short_entry = ...`
- `trigger short_exit = ...`

Compatibility rules:

- if a script declares any first-class `entry` / `exit` signals, the backtester uses those roles directly
- if a script declares no first-class signals, the backtester falls back to the legacy trigger names above
- ordinary `trigger` declarations remain valid for alerting or non-backtest consumers

See [Backtesting](../tooling/backtesting.md) for the Rust API and execution
model.

## Runtime Output Collections

Over a full run, the runtime accumulates:

- `plots`
- `exports`
- `triggers`
- `order_fields`
- `trigger_events`
- `alerts`

`alerts` currently exist in the runtime output structures but are not produced by a first-class PalmScript language construct.

## Output Time And Bar Index

Each output sample is tagged with:

- the current `bar_index`
- the current step `time`

In source-aware runs, the step time is the open time of the current base-clock step.
