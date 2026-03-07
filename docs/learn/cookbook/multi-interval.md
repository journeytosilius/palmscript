# Cookbook: Multi-Interval Strategy

This pattern adds slower context to a faster or equal base strategy.

```palmscript
interval 1d
use 1w

let weekly_basis = ema(1w.close, 8)

if close > weekly_basis {
    plot(1)
} else {
    plot(0)
}
```

Related checked-in example: [`examples/strategies/weekly_bias.palm`](https://github.com/journeytosilius/palmscript/blob/main/examples/strategies/weekly_bias.palm)

## Run It

```bash
palmscript run csv examples/strategies/weekly_bias.palm \
  --bars /path/to/daily_bars.csv
```

## What To Watch For

- `use 1w` is required before `1w.close`
- higher-interval values appear only after the higher candle fully closes
- no partial weekly candle is exposed
- indexing composes on the slower interval clock, not the base clock

Reference:

- [Intervals and Sources](../../reference/intervals-and-sources.md)
- [Series and Indexing](../../reference/series-and-indexing.md)
- [Evaluation Semantics](../../reference/evaluation-semantics.md)
