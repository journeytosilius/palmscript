# First Strategy

This strategy runs on one-minute bars, computes two moving averages, exports a trend state, and plots the close only when the fast average is above the slow average.

```palmscript
interval 1m

let fast = ema(close, 5)
let slow = sma(close, 10)

export trend = fast > slow

if trend {
    plot(close)
} else {
    plot(na)
}
```

## What This Introduces

- `interval 1m` sets the base execution clock
- `close` is a base market series in a source-less script
- `let` binds reusable expressions
- `export` emits a named output series
- `plot` emits chart-style numeric output
- `if / else` controls which values are emitted

## Run It

```bash
target/debug/palmscript run csv examples/strategies/sma_cross.palm \
  --bars examples/data/minute_bars.csv
```

## Extend It With Higher-Timeframe Context

```palmscript
interval 1d
use 1w

let weekly_basis = ema(1w.close, 8)
export bullish = close > weekly_basis
plot(close)
```

For the exact rules behind `1w.close`, indexing, and no-lookahead behavior, see:

- [Series and Indexing](../reference/series-and-indexing.md)
- [Intervals and Sources](../reference/intervals-and-sources.md)
- [Evaluation Semantics](../reference/evaluation-semantics.md)
