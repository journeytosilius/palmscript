# Examples

The canonical examples documentation now lives in the MkDocs site:

- [Multi-Interval Strategy](../docs/learn/cookbook/multi-interval.md)
- [Exchange-Backed Sources](../docs/learn/cookbook/exchange-backed-sources.md)
- [Cross-Source Spread](../docs/learn/cookbook/cross-source-spread.md)
- `Rust Examples` are documented privately in `docs-private/internals/rust-examples.md`

This file remains a short inventory for repository browsing. The canonical public explanation of language behavior lives in `docs/`.

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

Checked-in `.ps` strategies live under `examples/strategies/`.

Representative files:

- `examples/strategies/adaptive_trend_backtest.ps`: adaptive multi-timeframe long-only backtest strategy with optimizer-tuned staged `entry1` / `entry2` market entries, staged `target1` / `target2` profit-taking, and `protect_after_target1 long` stop ratchets
- `examples/strategies/risk_sized_entry_backtest.ps`: staged spot backtest example using `size entry long = risk_pct(...)` to size from stop distance instead of capital fraction
- `examples/strategies/usdm_long_short_backtest.ps`: Binance USD-M BTCUSDT long-biased perp strategy with staged long entries, staged mark-triggered targets, and a post-target mark-triggered stop ratchet
- `examples/strategies/sma_cross.ps`: single-source market-mode strategy
- `examples/strategies/weekly_bias.ps`: single-source supplemental-interval strategy
- `examples/strategies/macd_tuple.ps`: tuple destructuring and `ma_type`
- `examples/strategies/cross_source_spread.ps`: cross-source market-mode strategy
- `examples/strategies/exchange_backed_sources.ps`: source-aware strategy with `use <alias> <interval>`
- `examples/strategies/multi_strategy_backtest.ps`: composite trend, momentum, and breakout backtest strategy using `input`, `const`, and first-class `entry` / `exit` signals
- `examples/strategies/venue_orders_backtest.ps`: backtest strategy using explicit `order` declarations with `limit(...)` and `stop_market(...)`

For runnable public examples and workflow guidance, use the linked docs pages above.

Common commands:

```bash
./palmscript check examples/strategies/sma_cross.ps
./palmscript run market examples/strategies/sma_cross.ps --from 1704067200000 --to 1704153600000
./palmscript run market examples/strategies/volume_breakout.ps --from 1704067200000 --to 1704153600000 --format text
./palmscript run market examples/strategies/weekly_bias.ps --from 1704067200000 --to 1705276800000
./palmscript run market examples/strategies/signal_helpers.ps --from 1704067200000 --to 1704153600000
./palmscript run market examples/strategies/event_memory.ps --from 1704067200000 --to 1704153600000
./palmscript run market examples/strategies/macd_tuple.ps --from 1704067200000 --to 1704153600000
./palmscript run market examples/strategies/cross_source_spread.ps --from 1704067200000 --to 1704153600000
./palmscript run market examples/strategies/exchange_backed_sources.ps --from 1704067200000 --to 1704153600000
./palmscript run backtest examples/strategies/adaptive_trend_backtest.ps --from 1646611200000 --to 1772841600000
./palmscript run walk-forward examples/strategies/adaptive_trend_backtest.ps --from 1646611200000 --to 1772841600000 --train-bars 252 --test-bars 63 --step-bars 63
./palmscript run backtest examples/strategies/multi_strategy_backtest.ps --from 1741348800000 --to 1772884800000 --fee-bps 10 --slippage-bps 2
./palmscript run backtest examples/strategies/venue_orders_backtest.ps --from 1704067200000 --to 1704931200000 --format text
```
