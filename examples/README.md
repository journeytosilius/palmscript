# Examples

The canonical examples documentation now lives in the MkDocs site:

- [CSV Strategies](../docs/learn/cookbook/csv-strategies.md)
- [Multi-Interval Strategy](../docs/learn/cookbook/multi-interval.md)
- [Exchange-Backed Sources](../docs/learn/cookbook/exchange-backed-sources.md)
- [Cross-Source Spread](../docs/learn/cookbook/cross-source-spread.md)
- [Rust Examples](../docs/internals/rust-examples.md)

This file remains a short inventory for repository browsing. The canonical explanation of language behavior lives in `docs/`.

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

Representative files:

- `examples/strategies/sma_cross.palm`: source-less single-interval strategy
- `examples/strategies/weekly_bias.palm`: source-less supplemental-interval strategy
- `examples/strategies/macd_tuple.palm`: tuple destructuring and `ma_type`
- `examples/strategies/cross_source_spread.palm`: source-aware market-mode strategy
- `examples/strategies/exchange_backed_sources.palm`: source-aware strategy with `use <alias> <interval>`

For runnable commands and workflow guidance, use the linked docs pages above.

Common commands:

```bash
./palmscript check examples/strategies/sma_cross.palm
./palmscript run csv examples/strategies/sma_cross.palm --bars examples/data/minute_bars.csv
./palmscript run csv examples/strategies/volume_breakout.palm --bars examples/data/minute_bars.csv --format text
./palmscript run csv examples/strategies/weekly_bias.palm --bars /path/to/daily_bars.csv
./palmscript run csv examples/strategies/signal_helpers.palm --bars examples/data/minute_bars.csv
./palmscript run csv examples/strategies/event_memory.palm --bars examples/data/minute_bars.csv
./palmscript run csv examples/strategies/macd_tuple.palm --bars examples/data/minute_bars.csv
./palmscript run market examples/strategies/cross_source_spread.palm --from 1704067200000 --to 1704153600000
./palmscript run market examples/strategies/exchange_backed_sources.palm --from 1704067200000 --to 1704153600000
```
