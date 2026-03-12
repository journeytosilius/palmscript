# Cookbook: Exchange-Backed Sources

Use named sources when the strategy should fetch historical candles directly from supported exchanges.

```palmscript
interval 1m

source bn = binance.spot("BTCUSDT")
source hl = hyperliquid.perps("BTC")
use hl 1h

plot(bn.close)
plot(hl.1h.close)
```

PalmScript also supports Bybit and Gate source templates:

- `bybit.spot("BTCUSDT")`
- `bybit.usdt_perps("BTCUSDT")`
- `gate.spot("BTC_USDT")`
- `gate.usdt_perps("BTC_USDT")`

Representative checked-in examples:

- `crates/palmscript/examples/strategies/bybit_spot.ps`
- `crates/palmscript/examples/strategies/bybit_usdt_perps_backtest.ps`
- `crates/palmscript/examples/strategies/gate_spot.ps`
- `crates/palmscript/examples/strategies/gate_usdt_perps_backtest.ps`
- `crates/palmscript/examples/strategies/cross_exchange_bybit_gate_spread.ps`

## Try It In The Browser IDE

Open [https://palmscript.dev/app/](https://palmscript.dev/app/), paste the example into the editor, and run it against the available BTCUSDT history in the app.

## What To Watch For

- source-aware scripts must use source-qualified market series
- `use hl 1h` is required before `hl.1h.close`
- the script still has one global base `interval`
- the runtime resolves each required `(source, interval)` feed before execution
- Bybit expects venue-native symbols like `BTCUSDT`
- Gate expects venue-native symbols like `BTC_USDT`
- `run market`, `run backtest`, `run walk-forward`, `run walk-forward-sweep`, and `run optimize` all resolve the same exchange-backed source declarations

Reference:

- [Intervals and Sources](../../reference/intervals-and-sources.md)
