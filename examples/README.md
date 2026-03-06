# Examples

The canonical examples documentation now lives in the MkDocs site:

- [CSV Strategies](../docs/learn/cookbook/csv-strategies.md)
- [Multi-Interval Strategy](../docs/learn/cookbook/multi-interval.md)
- [Exchange-Backed Sources](../docs/learn/cookbook/exchange-backed-sources.md)
- [Cross-Source Spread](../docs/learn/cookbook/cross-source-spread.md)
- [Rust Examples](../docs/internals/rust-examples.md)

This file remains a short inventory for repository browsing.

## Rust Examples

Run from the repository root:

```bash
cargo run --example sma
cargo run --example rsi
cargo run --example step_engine
cargo run --example multi_interval
cargo run --example monthly_trend
```

## CLI Strategies

Checked-in `.palm` strategies live under `examples/strategies/`.

Common commands:

```bash
./palmscript check examples/strategies/sma_cross.palm
./palmscript run csv examples/strategies/sma_cross.palm --bars examples/data/minute_bars.csv
./palmscript run csv examples/strategies/volume_breakout.palm --bars examples/data/minute_bars.csv --format text
./palmscript run csv examples/strategies/weekly_bias.palm --bars /path/to/daily_bars.csv
./palmscript run market strategy.palm --from 1704067200000 --to 1704153600000
```
