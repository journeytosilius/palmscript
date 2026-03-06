# Intervals and Multi-Interval Semantics

PalmScript supports one base execution interval plus explicit higher or equal interval references.
Those references can come from the legacy base feed or from named exchange-backed sources.

## Declaring Intervals

Every strategy must declare exactly one base interval:

```palmscript
interval 1d
```

Legacy source-less scripts declare additional intervals globally:

```palmscript
interval 1d
use 1w
use 1M
```

Source-aware scripts declare intervals per source alias:

```palmscript
interval 1m
source hl = hyperliquid.perps("BTC")
source bn = binance.spot("BTCUSDT")
use hl 1h
use bn 4h
```

The compiler rejects:

- missing `interval`
- multiple `interval` declarations
- duplicate `use` declarations
- `use` repeating the base interval in legacy scripts
- source-scoped `use` references that repeat the same `(alias, interval)`
- qualified interval references that were not declared with `use`

## Qualified Market Series

Legacy interval-qualified syntax is:

```palmscript
<interval>.<field>
```

Source-qualified syntax is:

```palmscript
<alias>.<field>
<alias>.<interval>.<field>
```

Examples:

- `1w.close`
- `4h.volume`
- `1M.high`
- `hl.close`
- `hl.1h.close`
- `bn.4h.volume`

Allowed fields:

- `open`
- `high`
- `low`
- `close`
- `volume`
- `time`

## No-Lookahead Guarantee

Higher-interval values only become visible after that higher-interval candle fully closes.

If a script runs on `interval 1m` and references `1w.close` or `hl.1h.close`:

- the weekly close stays fixed across the whole week
- it updates only when the weekly candle closes
- partial weekly candles are never exposed

## Equal and Lower Intervals

- referencing the base interval explicitly is allowed
- lower-than-base interval references are rejected
- in source-aware scripts, bare market series like `close` are rejected; use `alias.close`

This keeps the runtime deterministic and avoids ambiguous downsampling semantics inside the VM.

## Indexing

Indexing composes on the referenced interval's own clock:

- `1w.close[0]` is the latest fully closed weekly close
- `1w.close[1]` is the previous weekly close
- `ema(1w.close, 5)[1]` is the prior weekly EMA sample
- `hl.1h.close[0]` is the latest fully closed one-hour close for source `hl`
